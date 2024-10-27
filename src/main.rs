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
use anyhow::{Error, anyhow};
use std::sync::Arc;
use filesystem::OutputFile;
use context::{Context, remove_mutex};
use clap::Parser;
use std::path::PathBuf;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use tracing::info;

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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    components: PathBuf,

    #[arg(long)]
    input: PathBuf,

    #[arg(long)]
    output: PathBuf,
}

pub fn validate_directory(path: &PathBuf) -> Result<(), Error> {
    if !path.is_dir() {
        Err(anyhow!("{} must be a directory", path.display()))
    } else {
        Ok(())
    }
}

pub fn load_component(engine: &Engine, path: &PathBuf) -> Result<Component, Error> {
    let mut hasher = DefaultHasher::new();
    engine.precompile_compatibility_hash().hash(&mut hasher);
    let hash = hasher.finish();

    let mut precompiled = path.to_owned().into_os_string();
    precompiled.push(format!(".precompiled_{:x}", hash));
    let precompiled : PathBuf = precompiled.into();
    
    let component = if precompiled.is_file() {
        info!("Found precompiled component at {}", precompiled.display());
        let component = unsafe { Component::deserialize_file(engine, precompiled)? };
        component
    } else {
        info!("Compiling {}", path.display());
        let component = Component::from_file(&engine, path)?;
        fs::write(precompiled, component.serialize()?)?;
        component
    };

    Ok(component)
}

pub fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();

    let args = Cli::parse();
    validate_directory(&args.components)?;
    validate_directory(&args.input)?;
    validate_directory(&args.output)?;
    
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.consume_fuel(true);

    let engine = Engine::new(&config)?;

    let mut linker: Linker<Context> = Linker::new(&engine);
    bindings::add_to_linker_all(&mut linker)?;

    let component_paths = fs::read_dir(args.components)?
        .filter_map(|p| p.ok() )
        .map(|p| p.path())
        .filter(|p| p.extension().map(|s| s == "wasm").unwrap_or(false))
        .collect::<Vec<_>>();

    let components = component_paths.iter()
        .map(|p| load_component(&engine, p))
        .collect::<Result<Vec<_>,_>>()?;

    let input_paths = fs::read_dir(args.input)?
        .filter_map(|p| p.ok() )
        .map(|p| p.path())
        .collect::<Vec<_>>();

    for input_path in input_paths {
        // TODO: Check file size, loopback new content
        let data = Arc::new(fs::read(input_path)?);

        for component in &components {
            let context = Context::new(1_000_000_000, 10_000, 1_000_000_000, 1000, data.clone());
            let result = run(&engine, &linker, component, context);
            println!("{:?}", result);
        }
    }

    Ok(())
}
