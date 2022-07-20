mod error;

use error::{DbError, Result};

use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    ops::Range,
    path::PathBuf,
};

const COMPACT_WATERFLOW: u64 = 1024 * 1024;

pub struct KvStore {
    path: PathBuf,
    writer: LogWriter<File>,
    cur_gen: u64,
    need_compact: u64,
    readers: HashMap<u64, LogReader<File>>,
    index: BTreeMap<String, CommandPos>,
    pub data: HashMap<String, String>,
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        fs::create_dir_all(path)?;

        let mut readers = HashMap::new();
        let mut index = BTreeMap::new();
        // load files from specific directory and build index
        //
        
    }
    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        let cmd = KvsCommand::set(key.clone(), val);
        let pos = self.writer.pos;
        serde_json::to_writer(self.writer, &cmd)?;
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
            serde_json::to_writer(self.writer, &cmd);
            self.writer.flush()?;

            self.need_compact += cmd_pos.len;
            Ok(())
        } else {
            Err(DbError::KeyNotFoundErr)
        }
    }

    fn compact(&mut self) -> Result<()> {
        Ok(())
    }
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
    fn new(reader: R) -> Result<Self> {
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
