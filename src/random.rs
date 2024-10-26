use wasmtime::Result;
use anyhow::Error;
use wasmtime_wasi::bindings::sync::random::*;

impl random::Host for crate::Context {
    fn get_random_bytes(&mut self, len: u64) -> Result<Vec<u8>, Error>  {
        Ok(vec![0; len as usize])
    }

    fn get_random_u64(&mut self) -> Result<u64, Error>  {
        Ok(0)
    }
}

impl insecure::Host for crate::Context {
    fn get_insecure_random_bytes(&mut self, len: u64) -> Result<Vec<u8>, Error>  {
        Ok(vec![0; len as usize])
    }

    fn get_insecure_random_u64(&mut self,) -> Result<u64, Error>  {
        Ok(0)
    }
}

impl insecure_seed::Host for crate::Context {
    fn insecure_seed(&mut self) -> Result<(u64,u64), Error>  {
        Ok((0, 0))
    }
}