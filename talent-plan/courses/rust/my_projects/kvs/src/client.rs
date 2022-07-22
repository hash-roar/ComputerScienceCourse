use crate::error::*;
use crate::protocol::*;
use std::{
    io::{BufReader, BufWriter, Write},
    net::{TcpStream, ToSocketAddrs},
};

use serde::Deserialize;
use serde_json::{de::IoRead, Deserializer};

pub struct KvsClient {
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>,
}

impl KvsClient {
    pub fn conncet<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let reader = TcpStream::connect(addr)?;
        let writer = reader.try_clone()?;
        Ok(KvsClient {
            reader: Deserializer::from_reader(BufReader::new(reader)),
            writer: BufWriter::new(writer),
        })
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        serde_json::to_writer(&mut self.writer, &Request::Get { key })?;
        self.writer.flush()?;
        let response = GetResponse::deserialize(&mut self.reader)?;
        match response {
            GetResponse::Ok(value) => Ok(value),
            GetResponse::Err(err) => Err(DbError::StrErr(err)),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Set { key, value })?;
        self.writer.flush()?;
        let response = SetResponse::deserialize(&mut self.reader)?;
        match response {
            SetResponse::Ok(_) => Ok(()),
            SetResponse::Err(err) => Err(DbError::StrErr(err)),
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Remove { key })?;
        self.writer.flush()?;
        let response = RemoveResponse::deserialize(&mut self.reader)?;
        match response {
            RemoveResponse::Ok(_) => Ok(()),
            RemoveResponse::Err(err) => Err(DbError::StrErr(err)),
        }
    }
}
