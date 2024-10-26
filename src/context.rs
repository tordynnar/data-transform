use std::sync::Arc;
use crate::cli::CircularOutputStream;
use crate::filesystem::OutputFile;
use crate::error::RunError;
use wasmtime::ResourceLimiter;
use wasmtime_wasi::ResourceTable;
use anyhow::Error;

pub struct Context {
    pub table: ResourceTable,
    pub memory_limit: usize,
    pub memory_used: usize,
    pub table_limit: u32,
    pub table_used: u32,
    pub fuel_limit: u64,
    pub debug: CircularOutputStream,
    pub input: Arc<Vec<u8>>,
    pub metadata: Vec<OutputFile>,
    pub content: Vec<OutputFile>,
}

impl ResourceLimiter for Context {
    fn memory_growing(&mut self, _: usize, desired: usize, _: Option<usize>) -> Result<bool, Error> {
        let result = desired <= self.memory_limit;
        if result { self.memory_used = desired }
        Ok(result)
    }

    fn table_growing(&mut self, _: u32, desired: u32, _: Option<u32>) -> Result<bool, Error> {
        let result = desired <= self.table_limit;
        if result { self.table_used = desired }
        Ok(result)
    }
}

impl Context {
    pub fn new(memory_limit: usize, table_limit: u32, fuel_limit: u64, debug_limit: usize, input: Arc<Vec<u8>>) -> Context {
        Context {
            table: ResourceTable::new(),
            memory_limit,
            memory_used: 0,
            table_limit,
            table_used: 0,
            fuel_limit,
            debug: CircularOutputStream::new(debug_limit),
            input,
            metadata: Vec::new(),
            content: Vec::new(),
        }
    }
}

pub fn remove_mutex_item(v: OutputFile) -> Result<Arc<Vec<u8>>, Error> {
    let o2 = Arc::into_inner(v.data).ok_or(RunError::UnexpectedMutexError)?;
    let o3 = o2.into_inner().map_err(|_| RunError::UnexpectedMutexError)?;
    Ok(Arc::new(o3))
}

pub fn remove_mutex(v: Vec<OutputFile>) -> Result<Vec<Arc<Vec<u8>>>, Error> {
    v.into_iter().map(remove_mutex_item).collect()
}