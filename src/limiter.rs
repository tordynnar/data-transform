#![allow(dead_code)]

use wasmtime::ResourceLimiter;
use crate::error::WasiError;

pub struct Limiter {
    pub memory: usize,
    pub table: u32,
}

impl ResourceLimiter for Limiter {
    fn memory_growing(&mut self, _: usize, desired: usize, _: Option<usize>) -> Result<bool, anyhow::Error> {
        if desired > self.memory {
            return Err(WasiError::MemoryExceeded.into());
        }
        Ok(true)
    }

    fn table_growing(&mut self, _: u32, desired: u32, _: std::option::Option<u32>) -> Result<bool, anyhow::Error> {
        if desired > self.table {
            return Err(WasiError::TableExceeded.into());
        }
        Ok(true)
    }
}
