mod error;

pub use error::{DbError, Result};

use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::{
    collections::{BTreeMap, HashMap},
    ffi::OsStr,
    fs::{self, File, OpenOptions},
    io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    ops::Range,
    path::{Path, PathBuf},
};

const COMPACT_WATERFLOW: u64 = 1024 * 1024;

pub struct KvStore {
    path: PathBuf,
    writer: LogWriter<File>,
    cur_gen: u64,
    need_compact: u64,
    readers: HashMap<u64, LogReader<File>>,
    index: BTreeMap<String, CommandPos>,
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        fs::create_dir_all(&path)?;

        let mut readers = HashMap::new();
        let mut index = BTreeMap::new();
        // load files from specific directory and build index
        let gen_list = get_gen_list(&path)?;
        let mut need_compact = 0;
        for &gen in &gen_list {
            let mut reader = LogReader::new(File::open(log_path(&path, gen))?)?;
            need_compact += load_logfile(gen, &mut reader, &mut index)?;
            readers.insert(gen, reader);
        }
        let cur_gen = gen_list.last().unwrap_or(&0) + 1;
        let writer = new_logfile(&path, cur_gen, &mut readers)?;

        Ok(KvStore {
            path,
            readers,
            writer,
            cur_gen,
            index,
            need_compact,
        })
    }
    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        let cmd = KvsCommand::set(key.clone(), val);
        let pos = self.writer.pos;
        serde_json::to_writer(& mut self.writer, &cmd)?;
        self.writer.flush()?;

        if let Some(old_cmd) = self
            .index
            .insert(key, (self.cur_gen, pos..self.writer.pos).into())
        {
            self.need_compact += old_cmd.len;
        }

        if self.need_compact > COMPACT_WATERFLOW {
            self.compact()?;
        }
        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(cmd_pos) = self.index.get(&key) {
            let reader = self
                .readers
                .get_mut(&cmd_pos.gen)
                .expect("fatal error: reader can not find");
            reader.seek(SeekFrom::Start(cmd_pos.pos))?;

            let cmd_reader = reader.take(cmd_pos.len);
            if let KvsCommand::Set { value, .. } = serde_json::from_reader(cmd_reader)? {
                Ok(Some(value))
            } else {
                Err(DbError::UnexpectCommandErr)
            }
        } else {
            Ok(None)
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if let Some(cmd_pos) = self.index.remove(&key) {
            let cmd = KvsCommand::remove(key.clone());
            serde_json::to_writer(&mut self.writer, &cmd);
            self.writer.flush()?;

            self.need_compact += cmd_pos.len;
            Ok(())
        } else {
            Err(DbError::KeyNotFoundErr)
        }
    }

    fn compact(&mut self) -> Result<()> {
        let compact_gen = self.cur_gen + 1;
        self.cur_gen += 2;
        self.writer = self.new_logfile(self.cur_gen)?;

        let mut compact_writer = self.new_logfile(compact_gen)?;
        let mut new_pos = 0;

        for cmd_pos in &mut self.index.values_mut() {
            let reader = self
                .readers
                .get_mut(&cmd_pos.gen)
                .expect("get reader error");

            if reader.pos != cmd_pos.pos {
                reader.seek(SeekFrom::Start(cmd_pos.pos))?;
            }

            let mut entry_reader = reader.take(cmd_pos.len);

            let len = io::copy(&mut entry_reader, &mut compact_writer)?;
            *cmd_pos = (compact_gen, new_pos..new_pos + len).into();

            new_pos += len;
        }
        compact_writer.flush();

        let stale_file: Vec<_> = self
            .readers
            .keys()
            .filter(|&&gen| gen < compact_gen)
            .cloned()
            .collect();

        for stale_gen in stale_file {
            self.readers.remove(&stale_gen);
            fs::remove_file(log_path(&self.path, stale_gen))?;
        }
        self.need_compact = 0;
        Ok(())
    }
    fn new_logfile(&mut self, gen: u64) -> Result<LogWriter<File>> {
        new_logfile(&self.path, gen, &mut self.readers)
    }
}

fn get_gen_list(path: &Path) -> Result<Vec<u64>> {
    let mut gen_list: Vec<u64> = fs::read_dir(path)?
        .flat_map(|res| -> Result<_> { Ok(res?.path()) })
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
        .flat_map(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .map(|s| s.trim_end_matches(".log"))
                .map(str::parse::<u64>)
        })
        .flatten()
        .collect();

    gen_list.sort_unstable();
    Ok(gen_list)
}

fn load_logfile(
    gen: u64,
    reader: &mut LogReader<File>,
    index: &mut BTreeMap<String, CommandPos>,
) -> Result<u64> {
    let mut pos = reader.seek(SeekFrom::Start(0))?;
    let mut deserial_strem = Deserializer::from_reader(reader).into_iter::<KvsCommand>();
    let mut need_compact = 0;
    while let Some(cmd) = deserial_strem.next() {
        let new_pos = deserial_strem.byte_offset() as u64;
        match cmd? {
            KvsCommand::Set { key, .. } => {
                if let Some(old_cmd) = index.insert(key, (gen, pos..new_pos).into()) {
                    need_compact += old_cmd.len;
                }
            }
            KvsCommand::Remove { key } => {
                if let Some(old_cmd) = index.remove(&key) {
                    need_compact += old_cmd.len;
                }
                need_compact += new_pos - pos;
            }
        }

        pos = new_pos;
    }

    Ok(need_compact)
}

fn new_logfile(
    path: &Path,
    gen: u64,
    readers: &mut HashMap<u64, LogReader<File>>,
) -> Result<LogWriter<File>> {
    let path = log_path(&path, gen);
    let writer = LogWriter::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)?,
    )?;
    readers.insert(gen, LogReader::new(File::open(&path)?)?);
    Ok(writer)
}

fn log_path(dir: &Path, gen: u64) -> PathBuf {
    dir.join(format!("{}.log", gen))
}

#[derive(Debug, Serialize, Deserialize)]
enum KvsCommand {
    Set { key: String, value: String },
    Remove { key: String },
}

impl KvsCommand {
    fn set(key: String, value: String) -> KvsCommand {
        KvsCommand::Set { key, value }
    }
    fn remove(key: String) -> KvsCommand {
        KvsCommand::Remove { key }
    }
}

struct CommandPos {
    gen: u64,
    pos: u64,
    len: u64,
}

impl From<(u64, Range<u64>)> for CommandPos {
    fn from((gen, range): (u64, Range<u64>)) -> Self {
        CommandPos {
            gen,
            pos: range.start,
            len: range.end,
        }
    }
}

struct LogWriter<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}

impl<W: Write + Seek> LogWriter<W> {
    fn new(mut writer: W) -> Result<Self> {
        let pos = writer.seek(SeekFrom::Current(0))?;
        Ok(LogWriter {
            writer: BufWriter::new(writer),
            pos,
        })
    }
}

impl<W: Write + Seek> Write for LogWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(self.pos as usize)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()?;
        Ok(())
    }
}

impl<W: Write + Seek> Seek for LogWriter<W> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}

struct LogReader<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
}

impl<R: Read + Seek> LogReader<R> {
    fn new(mut reader: R) -> Result<Self> {
        let pos = reader.seek(SeekFrom::Current(0))?;
        Ok(LogReader {
            reader: BufReader::new(reader),
            pos,
        })
    }
}

impl<R: Read + Seek> Read for LogReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len as usize)
    }
}

impl<R: Read + Seek> Seek for LogReader<R> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}
