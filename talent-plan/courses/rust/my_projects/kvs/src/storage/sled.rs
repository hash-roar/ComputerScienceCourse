use sled::{Db, Tree};

use super::DbError;
use super::kvsEngine;

pub struct SledEngine(Db);

impl SledEngine {
    pub fn new(db: Db) -> Self {
        SledEngine(db)
    }
}

impl kvsEngine for SledEngine {
    fn set(&mut self, key: String, value: String) -> crate::Result<()> {
        let tree: &Tree = &self.0;
        tree.insert(key, value.into_bytes())?;
        tree.flush()?;
        Ok(())
    }

    fn get(&mut self, key: String) -> crate::Result<Option<String>> {
        let tree: &Tree = &self.0;
        Ok(tree
            .get(key)?
            .map(|ivec| AsRef::<[u8]>::as_ref(&ivec).to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }

    fn remove(&mut self, key: String) -> crate::Result<()> {
        let tree: &Tree = &self.0;
        tree.remove(key)?.ok_or(DbError::KeyNotFoundErr)?;
        tree.flush()?;
        Ok(())
    }
}
