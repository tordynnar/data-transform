use wasmtime_wasi::Pollable;
use wasmtime::Result;
use wasmtime::component::Resource;
use anyhow::Error;
use wasmtime_wasi::bindings::sync::clocks::*;
use crate::error::RunError;

impl wall_clock::Host for crate::Context {
    fn now(&mut self) -> Result<wall_clock::Datetime, Error>  {
        Ok(wall_clock::Datetime {
            seconds: 0,
            nanoseconds: 0,
        })
    }

    fn resolution(&mut self) -> Result<wall_clock::Datetime, Error>  {
        Ok(wall_clock::Datetime {
            seconds: 0,
            nanoseconds: 0,
        })
    }
}

impl monotonic_clock::Host for crate::Context {
    fn now(&mut self) -> Result<monotonic_clock::Instant, Error>  {
        Ok(0)
    }

    fn resolution(&mut self) -> Result<monotonic_clock::Duration, Error>  {
        Ok(0)
    }

    fn subscribe_instant(&mut self, _when: monotonic_clock::Instant) -> Result<Resource<Pollable>, Error>  {
        Err(RunError::SleepNotImplemented.into())
    }

    fn subscribe_duration(&mut self, _when: monotonic_clock::Duration) -> Result<Resource<Pollable>, Error>  {
        Err(RunError::SleepNotImplemented.into())
    }
}
