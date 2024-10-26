mod cli;
mod clocks;
mod filesystem;
mod random;
mod io;
mod sockets;
mod error;
mod bindings;
mod context;

use wasmtime::{Config, Engine, Store};
use wasmtime::component::{Linker, Component};
use wasmtime_wasi::bindings::sync::Command;
use anyhow::Error;
use std::sync::Arc;
use filesystem::OutputFile;
use context::{Context, remove_mutex};

#[derive(Debug)]
pub struct RunResult {
    pub memory_used: usize,
    pub table_used: u32,
    pub fuel_used: u64,
    pub debug: String,
    pub metadata: Vec<Arc<Vec<u8>>>,
    pub content: Vec<Arc<Vec<u8>>>,
}

pub fn run(engine: &Engine, linker: &Linker<Context>, component: &Component, context: Context) -> Result<RunResult, Error> {
    let (memory_used, table_used, fuel_used, debug, metadata, content) = {
        let mut store = Store::new(engine, context);
        store.set_fuel(store.data().fuel_limit)?;
        store.limiter(|state| state);
        
        let command = Command::instantiate(&mut store, component, linker)?;
        let _ = command.wasi_cli_run().call_run(&mut store)?;

        let fuel = store.get_fuel()?;
        let fuel_used = store.data().fuel_limit - fuel;

        let debug = String::from_utf8(store.data().debug.get_data())?;

        (store.data().memory_used, store.data().table_used, fuel_used, debug, store.data().metadata.clone(), store.data().content.clone())
    };

    Ok(RunResult {
        memory_used,
        table_used,
        fuel_used,
        debug,
        metadata: remove_mutex(metadata)?,
        content: remove_mutex(content)?,
    })
}

pub fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.consume_fuel(true);

    let engine = Engine::new(&config)?;

    let mut linker: Linker<Context> = Linker::new(&engine);
    bindings::add_to_linker_all(&mut linker)?;

    let component = Component::from_file(&engine, "../wasi-command-module/target/wasm32-wasip2/release/wasi-command-module.wasm")?;

    let context = Context::new(1_000_000_000, 10_000, 1_000_000_000, 1000, Arc::new("hello world".to_owned().into_bytes()));
    let result = run(&engine, &linker, &component, context);

    println!("{:?}", result);

    Ok(())
}
