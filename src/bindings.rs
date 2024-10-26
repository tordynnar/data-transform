use wasmtime::component::Linker;
use anyhow::Error;

// Need custom filesystem bindings because the wasmtime_wasi library is coupled to cap_std::fs
wasmtime::component::bindgen!({
    path: "wit",
    world: "wasi:cli/command",
    tracing: true,
    trappable_error_type: {
        "wasi:io/streams/stream-error" => wasmtime_wasi::StreamError,
        "wasi:sockets/network/error-code" => wasmtime_wasi::SocketError,
    },
    trappable_imports: true,
    with: {
        "wasi:cli": wasmtime_wasi::bindings::sync::cli,
        "wasi:clocks": wasmtime_wasi::bindings::sync::clocks,
        "wasi:io": wasmtime_wasi::bindings::sync::io,
        "wasi:random": wasmtime_wasi::bindings::sync::random,
        "wasi:sockets": wasmtime_wasi::bindings::sync::sockets,
        "wasi:filesystem/types/descriptor": crate::filesystem::Descriptor,
        "wasi:filesystem/types/directory-entry-stream": wasmtime_wasi::bindings::sync::filesystem::types::DirectoryEntryStream,
    },
    require_store_data_send: true,
});

pub fn add_to_linker_all(linker : &mut Linker<crate::Context>) -> Result<(), Error> {
    wasmtime_wasi::bindings::sync::cli::stdout::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::cli::stderr::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::cli::stdin::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::cli::exit::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::cli::environment::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::cli::terminal_input::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::cli::terminal_output::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::cli::terminal_stdin::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::cli::terminal_stdout::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::cli::terminal_stderr::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::clocks::wall_clock::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::clocks::monotonic_clock::add_to_linker(linker, |s| s)?;
    wasi::filesystem::types::add_to_linker(linker, |s| s)?;
    wasi::filesystem::preopens::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::io::error::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::io::poll::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::io::streams::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::random::random::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::random::insecure::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::random::insecure_seed::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::sockets::tcp::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::sockets::tcp_create_socket::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::sockets::udp::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::sockets::udp_create_socket::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::sockets::instance_network::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::sockets::network::add_to_linker(linker, |s| s)?;
    wasmtime_wasi::bindings::sync::sockets::ip_name_lookup::add_to_linker(linker, |s| s)?;
    Ok(())
}