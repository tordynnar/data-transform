use wasmtime_wasi::{StreamError, StreamResult, Pollable, InputStream, OutputStream, subscribe};
use wasmtime::Result;
use wasmtime::component::Resource;
use anyhow::Error;
use wasmtime_wasi::bindings::sync::io::*;
use bytes::Bytes;
use crate::error::RunError;

impl error::HostError for crate::Context {
    fn to_debug_string(&mut self, err: Resource<Error>) -> Result<String, Error>  {
        Ok(format!("{:?}", self.table.get(&err)?))
    }

    fn drop(&mut self, err: Resource<Error>) -> Result<(), Error>  {
        self.table.delete(err)?;
        Ok(())
    }
}

// All pollables are always ready because the implementation is to/from static memory
impl poll::HostPollable for crate::Context {
    fn ready(&mut self, _pollable: Resource<Pollable>) -> Result<bool, Error> {
        Ok(true)
    }

    fn block(&mut self, _pollable: Resource<Pollable>) -> Result<(), Error> {
        Ok(())
    }

    fn drop(&mut self, pollable: Resource<Pollable>) -> Result<(), Error> {
        self.table.delete(pollable)?;
        Ok(())
    }
}

impl error::Host for crate::Context {}

impl poll::Host for crate::Context {
    fn poll(&mut self, pollables: Vec<Resource<Pollable>>) -> Result<Vec<u32>, Error> {
        let len : u32 = pollables.len().try_into()?;
        Ok((0..len).collect())
    }
}

impl streams::HostOutputStream for crate::Context {
    fn check_write(&mut self, _stream: Resource<OutputStream>) -> StreamResult<u64> {
        Ok(1024 * 1024)
    }

    fn write(&mut self, stream: Resource<OutputStream>, bytes: Vec<u8>) -> StreamResult<()> {
        self.table.get_mut(&stream)?.write(bytes.into())

    }

    fn blocking_write_and_flush(&mut self, stream: Resource<OutputStream>, bytes: Vec<u8>) -> StreamResult<()> {
        self.write(stream, bytes)
    }

    fn flush(&mut self, _stream: Resource<OutputStream>) -> StreamResult<()> {
        Ok(())
    }

    fn blocking_flush(&mut self, _stream: Resource<OutputStream>) -> StreamResult<()> {
        Ok(())
    }

    fn subscribe(&mut self, stream: Resource<OutputStream>) -> Result<Resource<Pollable>, Error> {
        subscribe(&mut self.table, stream)
    }

    fn write_zeroes(&mut self, stream: Resource<OutputStream>, nelem: u64) -> StreamResult<()> {
        let bs = Bytes::from_iter(core::iter::repeat(0).take(nelem as usize));
        self.table.get_mut(&stream)?.write(bs)
    }

    fn blocking_write_zeroes_and_flush(&mut self, stream: Resource<OutputStream>, nelem: u64) -> StreamResult<()> {
        self.write_zeroes(stream, nelem)
    }

    fn splice(&mut self, dest: Resource<OutputStream>, src: Resource<InputStream>, len: u64) -> StreamResult<u64> {
        let contents = self.table.get_mut(&src)?.read(len as usize)?;
        let reslen = contents.len();
        self.table.get_mut(&dest)?.write(contents)?;
        Ok(reslen as u64)
    }

    fn blocking_splice(&mut self, dest: Resource<OutputStream>, src: Resource<InputStream>, len: u64) -> StreamResult<u64> {
        self.splice(dest, src, len)
    }

    fn drop(&mut self, stream: Resource<OutputStream>) -> Result<(), Error> {
        self.table.delete(stream)?;
        Ok(())
    }
}

impl streams::HostInputStream for crate::Context {
    fn read(&mut self, stream: Resource<InputStream>, len: u64) -> StreamResult<Vec<u8>> {
        Ok(self.table.get_mut(&stream)?.read(len as usize)?.into())
    }

    fn blocking_read(&mut self, stream: Resource<InputStream>, len: u64) -> StreamResult<Vec<u8>> {
        self.read(stream, len)
    }

    fn skip(&mut self, stream: Resource<InputStream>, len: u64) -> StreamResult<u64> {
        let len = len.try_into().unwrap_or(usize::MAX);
        let written = self.table.get_mut(&stream)?.skip(len)?;
        Ok(written.try_into().map_err(|_| RunError::Unexpected)?)
    }

    fn blocking_skip(&mut self, stream: Resource<InputStream>, len: u64) -> StreamResult<u64> {
        self.skip(stream, len)
    }

    fn subscribe(&mut self, stream: Resource<InputStream>) -> Result<Resource<Pollable>, Error> {
        subscribe(&mut self.table, stream)
    }

    fn drop(&mut self, stream: Resource<InputStream>) -> Result<(), Error> {
        self.table.delete(stream)?;
        Ok(())
    }
}

impl streams::Host for crate::Context {
    fn convert_stream_error(&mut self, err: StreamError) -> Result<streams::StreamError, Error> {
        match err {
            StreamError::Closed => Ok(streams::StreamError::Closed),
            StreamError::LastOperationFailed(e) => Ok(streams::StreamError::LastOperationFailed(
                self.table.push(e)?,
            )),
            StreamError::Trap(e) => Err(e),
        }
    }
}