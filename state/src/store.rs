#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Result};
use rocksdb::{BlockBasedOptions, ColumnFamilyDescriptor, Options};

use codec::{Decodable, Encodable};
use primitive_types::H256;
use smt::{SparseMerkleTree, StorageBackend};
use smt::treehasher::TreeHasher;

use crate::persistent::{default_db_opts, MemoryStore, RocksDB};

const COLUMN_TREES: &str = "t";
const COLUMN_ROOT: &str = "r";

pub fn cfs() -> Vec<ColumnFamilyDescriptor> {
    vec![
        ColumnFamilyDescriptor::new(COLUMN_TREES, default_table_options()),
        ColumnFamilyDescriptor::new(COLUMN_ROOT, default_table_options()),
    ]
}

fn default_table_options() -> Options {
    // default db options
    let mut db_opts = Options::default();

    // https://github.com/facebook/rocksdb/wiki/Setup-Options-and-Basic-Tuning#other-general-options
    db_opts.set_level_compaction_dynamic_level_bytes(false);
    db_opts.set_write_buffer_size(32 * 1024 * 1024);

    // block table options
    let mut table_options = BlockBasedOptions::default();
    // table_options.set_block_cache(&Cache::new_lru_cache(32 * 1024 * 1024).unwrap());
    // table_options.set_block_size(16 * 1024);
    // table_options.set_cache_index_and_filter_blocks(true);
    // table_options.set_pin_l0_filter_and_index_blocks_in_cache(true);

    // set format_version 4 https://rocksdb.org/blog/2019/03/08/format-version-4.html
    table_options.set_format_version(4);
    table_options.set_index_block_restart_interval(16);

    db_opts.set_block_based_table_factory(&table_options);

    db_opts
}

pub trait DatabaseBackend {
    fn put(&self, column_name: &'static str, key: &[u8], value: &[u8]) -> Result<()>;

    fn get(&self, column_name: &'static str, key: &[u8]) -> Result<Vec<u8>>;

    fn delete(&self, column_name: &'static str, key: &[u8]) -> Result<()>;

    fn checkpoint(&self, path: PathBuf) -> Result<Arc<dyn DatabaseBackend + Send + Sync>>;

    fn get_or_default(
        &self,
        column_name: &'static str,
        key: &[u8],
        default: Vec<u8>,
    ) -> Result<Vec<u8>>;
}

pub struct Database {
    pub inner: Arc<dyn DatabaseBackend + Send + Sync>,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = Arc::new(rocksdb::DB::open_cf_descriptors(
            &default_db_opts(),
            path.as_ref(),
            cfs(),
        )?);

        Ok(Self {
            inner: Arc::new(RocksDB::new(db)),
        })
    }

    pub fn open_read_only<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = Arc::new(rocksdb::DB::open_cf_for_read_only(
            &default_db_opts(),
            path,
            vec![COLUMN_ROOT, COLUMN_TREES],
            false,
        )?);
        Ok(Self {
            inner: Arc::new(RocksDB::new(db)),
        })
    }

    pub fn in_memory() -> Self {
        Self {
            inner: Arc::new(MemoryStore::new()),
        }
    }

    pub fn put<S: StorageBackend, H: TreeHasher>(&self, key: H256, value: SparseMerkleTree<S, H>) -> Result<()> {
        self.inner.put(
            COLUMN_TREES,
            &Encodable::encode(&key)?,
            &Encodable::encode(&value)?,
        )
    }

    pub fn set_root(&self, new_root: H256) -> Result<()> {
        self.inner
            .put(COLUMN_ROOT, b"root", &Encodable::encode(&new_root)?)
    }

    pub fn load_root<S: StorageBackend, H: TreeHasher>(&self) -> Result<SparseMerkleTree<S, H>> {
        let root = self.inner.get(COLUMN_ROOT, b"root")?;
        let root = <H256 as Decodable>::decode(&root)?;
        self.get(&root)
    }

    pub fn get<S: StorageBackend, H: TreeHasher>(&self, key: &H256) -> Result<SparseMerkleTree<S, H>> {
        <SparseMerkleTree<S, H> as Decodable>::decode(
            &self.inner.get(COLUMN_TREES, &Encodable::encode(key)?)?,
        )
    }

    pub fn delete(&self, key: &H256) -> Result<()> {
        self.inner.delete(COLUMN_TREES, &Encodable::encode(key)?)
    }

    pub fn checkpoint<P: AsRef<Path>>(&self, path: P) -> Result<Database> {
        Ok(Database {
            inner: self.inner.checkpoint(PathBuf::new().join(path.as_ref()))?,
        })
    }
}