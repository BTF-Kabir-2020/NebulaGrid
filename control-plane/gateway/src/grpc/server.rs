use std::net::SocketAddr;
use tracing::info;

pub async fn start_grpc_server(addr: SocketAddr) {
    info!("gRPC server configured for {addr}");
    info!("gRPC server will accept agent connections on port {}", addr.port());
}
