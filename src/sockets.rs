use wasmtime_wasi::{Pollable, InputStream, OutputStream, SocketResult, SocketError};
use wasmtime::Result;
use wasmtime::component::Resource;
use anyhow::Error;
use crate::error::RunError;

use wasmtime_wasi::bindings::sync::sockets::*;

impl tcp::HostTcpSocket for crate::Context {
    fn start_bind(&mut self, _self_: Resource<tcp::TcpSocket>, _network: Resource<tcp::Network>, _local_address: tcp::IpSocketAddress) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn finish_bind(&mut self, _self_: Resource<tcp::TcpSocket>) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn start_connect(&mut self, _self_: Resource<tcp::TcpSocket>, _network: Resource<tcp::Network>, _remote_address: tcp::IpSocketAddress) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn finish_connect(&mut self, _self_: Resource<tcp::TcpSocket>) -> SocketResult<(Resource<InputStream>, Resource<OutputStream>)> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn start_listen(&mut self, _self_: Resource<tcp::TcpSocket>) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn finish_listen(&mut self, _self_: Resource<tcp::TcpSocket>) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn accept(&mut self, _self_: Resource<tcp::TcpSocket>) -> SocketResult<(Resource<tcp::TcpSocket>, Resource<InputStream>, Resource<OutputStream>)> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn local_address(&mut self, _self_: Resource<tcp::TcpSocket>) -> SocketResult<tcp::IpSocketAddress> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn remote_address(&mut self, _self_: Resource<tcp::TcpSocket>) -> SocketResult<tcp::IpSocketAddress> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn is_listening(&mut self, _self_:Resource<tcp::TcpSocket>) -> Result<bool, Error> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn address_family(&mut self, _self_: Resource<tcp::TcpSocket>) -> Result<tcp::IpAddressFamily, Error> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn set_listen_backlog_size(&mut self, _self_: Resource<tcp::TcpSocket>, _value: u64) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn keep_alive_enabled(&mut self, _self_: Resource<tcp::TcpSocket>) -> SocketResult<bool> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn set_keep_alive_enabled(&mut self, _self_:Resource<tcp::TcpSocket>, _value:bool) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn keep_alive_idle_time(&mut self, _self_: Resource<tcp::TcpSocket>,) -> SocketResult<tcp::Duration> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn set_keep_alive_idle_time(&mut self, _self_: Resource<tcp::TcpSocket>, _value: tcp::Duration) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn keep_alive_interval(&mut self, _self_: Resource<tcp::TcpSocket>) -> SocketResult<tcp::Duration> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn set_keep_alive_interval(&mut self, _self_: Resource<tcp::TcpSocket>, _value: tcp::Duration) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn keep_alive_count(&mut self, _self_: Resource<tcp::TcpSocket>) -> SocketResult<u32> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn set_keep_alive_count(&mut self, _self_: Resource<tcp::TcpSocket>, _value: u32) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn hop_limit(&mut self, _self_: Resource<tcp::TcpSocket>) -> SocketResult<u8> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn set_hop_limit(&mut self, _self_: Resource<tcp::TcpSocket>, _value:u8) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn receive_buffer_size(&mut self, _self_: Resource<tcp::TcpSocket>) -> SocketResult<u64> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn set_receive_buffer_size(&mut self, _self_: Resource<tcp::TcpSocket>, _value: u64) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn send_buffer_size(&mut self, _self_: Resource<tcp::TcpSocket>) -> SocketResult<u64> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn set_send_buffer_size(&mut self, _self_: Resource<tcp::TcpSocket>, _value:u64,) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn subscribe(&mut self, _self_: Resource<tcp::TcpSocket>) -> Result<Resource<Pollable>, Error> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn shutdown(&mut self, _self_: Resource<tcp::TcpSocket>, _shutdown_type: tcp::ShutdownType) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn drop(&mut self, _rep: Resource<tcp::TcpSocket>) -> Result<(), Error> {
        Err(RunError::SocketsNotImplemented.into())
    }
}

impl tcp::Host for crate::Context {}

impl tcp_create_socket::Host for crate::Context {
    fn create_tcp_socket(&mut self, _address_family: tcp::IpAddressFamily) -> SocketResult<Resource<tcp::TcpSocket>> {
        Err(RunError::SocketsNotImplemented.into())
    }
}

impl udp::HostOutgoingDatagramStream for crate::Context {
    fn check_send(&mut self, _self_: Resource<udp::OutgoingDatagramStream>,) -> SocketResult<u64> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn send(&mut self, _self_: Resource<udp::OutgoingDatagramStream>, _datagrams: Vec<udp::OutgoingDatagram>) -> SocketResult<u64> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn subscribe(&mut self, _self_: Resource<udp::OutgoingDatagramStream>) -> Result<Resource<Pollable>, Error> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn drop(&mut self, _rep: Resource<udp::OutgoingDatagramStream>) -> Result<(), Error> {
        Err(RunError::SocketsNotImplemented.into())
    }
}

impl udp::HostIncomingDatagramStream for crate::Context {
    fn receive(&mut self, _self_: Resource<udp::IncomingDatagramStream>, _max_results: u64) -> SocketResult<Vec<udp::IncomingDatagram>> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn subscribe(&mut self, _self_: Resource<udp::IncomingDatagramStream>) -> Result<Resource<Pollable>, Error> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn drop(&mut self, _rep: Resource<udp::IncomingDatagramStream>) -> Result<(), Error> {
        Err(RunError::SocketsNotImplemented.into())
    }
}

impl udp::HostUdpSocket for crate::Context {
    fn start_bind(&mut self, _self_: Resource<udp::UdpSocket>, _network: Resource<udp::Network>, _local_address: udp::IpSocketAddress) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn finish_bind(&mut self, _self_: Resource<udp::UdpSocket>) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn stream(&mut self, _self_: Resource<udp::UdpSocket>, _remote_address: Option<udp::IpSocketAddress>) -> SocketResult<(Resource<udp::IncomingDatagramStream>,Resource<udp::OutgoingDatagramStream>)> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn local_address(&mut self, _self_: Resource<udp::UdpSocket>) -> SocketResult<udp::IpSocketAddress> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn remote_address(&mut self, _self_: Resource<udp::UdpSocket>) -> SocketResult<udp::IpSocketAddress> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn address_family(&mut self, _self_: Resource<udp::UdpSocket>) -> Result<udp::IpAddressFamily, Error> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn unicast_hop_limit(&mut self, _self_: Resource<udp::UdpSocket>) -> SocketResult<u8> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn set_unicast_hop_limit(&mut self, _self_: Resource<udp::UdpSocket>, _value: u8) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn receive_buffer_size(&mut self, _self_: Resource<udp::UdpSocket>) -> SocketResult<u64> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn set_receive_buffer_size(&mut self, _self_: Resource<udp::UdpSocket>, _value: u64) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn send_buffer_size(&mut self, _self_: Resource<udp::UdpSocket>) -> SocketResult<u64> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn set_send_buffer_size(&mut self, _self_: Resource<udp::UdpSocket>, _value: u64) -> SocketResult<()> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn subscribe(&mut self, _self_: Resource<udp::UdpSocket>) -> Result<Resource<Pollable>, Error> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn drop(&mut self, _rep: Resource<udp::UdpSocket>) -> Result<(), Error> {
        Err(RunError::SocketsNotImplemented.into())
    }
}

impl udp::Host for crate::Context {}

impl udp_create_socket::Host for crate::Context {
    fn create_udp_socket(&mut self, _address_family: udp_create_socket::IpAddressFamily) -> SocketResult<Resource<udp_create_socket::UdpSocket>> {
        Err(RunError::SocketsNotImplemented.into())
    }
}

impl instance_network::Host for crate::Context {
    fn instance_network(&mut self) -> Result<Resource<instance_network::Network>, Error> {
        Err(RunError::SocketsNotImplemented.into())
    }
}

impl network::HostNetwork for crate::Context {
    fn drop(&mut self, _rep: Resource<network::Network>) -> Result<(), Error>  {
        Err(RunError::SocketsNotImplemented.into())
    }
}

impl network::Host for crate::Context {
    fn convert_error_code(&mut self, error: SocketError) -> Result<network::ErrorCode, Error>  {
        error.downcast()
    }
}

impl ip_name_lookup::HostResolveAddressStream for crate::Context {
    fn resolve_next_address(&mut self, _self_: Resource<ip_name_lookup::ResolveAddressStream>) -> SocketResult<Option<ip_name_lookup::IpAddress>> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn subscribe(&mut self, _self_: Resource<ip_name_lookup::ResolveAddressStream>) -> Result<Resource<Pollable>, Error> {
        Err(RunError::SocketsNotImplemented.into())
    }

    fn drop(&mut self, _rep: Resource<ip_name_lookup::ResolveAddressStream>) -> Result<(), Error> {
        Err(RunError::SocketsNotImplemented.into())
    }
}

impl ip_name_lookup::Host for crate::Context {
    fn resolve_addresses(&mut self, _network: Resource<ip_name_lookup::Network>, _name: String) -> SocketResult<Resource<ip_name_lookup::ResolveAddressStream>> {
        Err(RunError::SocketsNotImplemented.into())
    }
}