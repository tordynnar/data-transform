use wasmtime_wasi::{HostInputStream, HostOutputStream, InputStream, OutputStream, StreamError, StreamResult, Subscribe};
use wasmtime::Result;
use wasmtime::component::Resource;
use std::sync::{Arc, Mutex};
use bytes::Bytes;
use anyhow::Error;
use wasmtime_wasi::bindings::sync::cli::*;
use crate::error::RunError;

pub struct ClosedInputStream;

impl HostInputStream for ClosedInputStream {
    fn read(&mut self, _size: usize) -> StreamResult<Bytes> {
        Err(StreamError::Closed)
    }
}

#[async_trait::async_trait]
impl Subscribe for ClosedInputStream {
    async fn ready(&mut self) {}
}


#[derive(Clone)]
pub struct CircularOutputStream {
    max_size: usize,
    data: Arc<Mutex<Vec<u8>>>
}

impl CircularOutputStream {
    pub fn new(max_size: usize) -> CircularOutputStream {
        CircularOutputStream { max_size, data: Arc::new(Mutex::new(Vec::<u8>::new())) }
    }

    pub fn get_data(&self) -> Vec<u8> {
        if let Ok(data) = self.data.lock() {
            data.clone()
        } else {
            vec![]
        }
    }
}

#[async_trait::async_trait]
impl Subscribe for CircularOutputStream {
    async fn ready(&mut self) {}
}

impl HostOutputStream for CircularOutputStream {
    fn write(&mut self, bytes: Bytes) -> Result<(), StreamError> {
        if let Ok(mut data) = self.data.lock() {
            data.extend(bytes);
            let data_len = data.len();
            if data_len > self.max_size {
                data.drain(..(data_len - self.max_size));
            }
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), StreamError> {
        Ok(())
    }

    fn check_write(&mut self) -> Result<usize, StreamError> {
        Ok(1024 * 1024)
    }
}

impl stdin::Host for crate::Context {
    fn get_stdin(&mut self) -> Result<Resource<InputStream>>  {
        let stream : InputStream = Box::new(ClosedInputStream);
        Ok(self.table.push(stream)?)
    }
}

impl stdout::Host for crate::Context {
    fn get_stdout(&mut self) -> Result<Resource<OutputStream>, Error>  {
        let stream : OutputStream = Box::new(self.debug.clone()) as OutputStream;
        Ok(self.table.push(stream)?)
    }
}

impl stderr::Host for crate::Context {
    fn get_stderr(&mut self) -> Result<Resource<OutputStream>, Error>  {
        let stream : OutputStream = Box::new(self.debug.clone());
        Ok(self.table.push(stream)?)
    }
}

impl exit::Host for crate::Context {
    fn exit(&mut self, _status: Result<(),()>) -> Result<(), Error>  {
        Err(RunError::ExitCalled.into())
    }
}

impl environment::Host for crate::Context {
    fn get_environment(&mut self) -> Result<Vec<(String,String)>, Error>  {
        Ok(Vec::new())
    }

    fn get_arguments(&mut self) -> Result<Vec<String>, Error>  {
        Ok(Vec::new())
    }

    fn initial_cwd(&mut self) -> Result<Option<String>, Error>  {
        Ok(None)
    }
}

impl terminal_input::Host for crate::Context {
}

impl terminal_output::Host for crate::Context {
}

impl terminal_stdin::Host for crate::Context {
    fn get_terminal_stdin(&mut self) -> Result<Option<Resource<terminal_input::TerminalInput>>, Error>  {
        Ok(None)
    }
}

impl terminal_stdout::Host for crate::Context {
    fn get_terminal_stdout(&mut self) -> Result<Option<Resource<terminal_output::TerminalOutput>>, Error>  {
        Ok(None)
    }
}

impl terminal_stderr::Host for crate::Context {
    fn get_terminal_stderr(&mut self) -> Result<Option<Resource<terminal_output::TerminalOutput>>, Error>  {
        Ok(None)
    }
}

impl terminal_input::HostTerminalInput for crate::Context {
    fn drop(&mut self, r: Resource<terminal_input::TerminalInput>) -> Result<(), Error> {
        self.table.delete(r)?;
        Ok(())
    }
}

impl terminal_output::HostTerminalOutput for crate::Context {
    fn drop(&mut self, r: Resource<terminal_output::TerminalOutput>) -> Result<(), Error>  {
        self.table.delete(r)?;
        Ok(())
    }
}