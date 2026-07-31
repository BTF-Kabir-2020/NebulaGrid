use std::net::SocketAddr;
use std::pin::Pin;
use tonic::{Request, Response, Status};
use futures_core::Stream;

pub mod proto {
    tonic::include_proto!("nebula.agent");
}

use proto::agent_service_server::{AgentService, AgentServiceServer};
use proto::{
    CommandRequest, CommandResponse, FileRequest, FileResponse, LogEntry, LogRequest,
    MetricsResponse, RegisterRequest, RegisterResponse, SystemMetrics,
};

type FileStream = Pin<Box<dyn Stream<Item = Result<FileResponse, Status>> + Send>>;
type LogStream = Pin<Box<dyn Stream<Item = Result<LogEntry, Status>> + Send>>;

#[derive(Debug, Default)]
pub struct AgentServiceImpl;

#[tonic::async_trait]
impl AgentService for AgentServiceImpl {
    type TransferFileStream = FileStream;
    type StreamLogsStream = LogStream;

    async fn register_agent(
        &self,
        req: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let inner = req.into_inner();
        tracing::info!("gRPC register from {} ({})", inner.hostname, inner.os_name);
        Ok(Response::new(RegisterResponse {
            node_id: uuid::Uuid::new_v4().to_string(),
            control_plane_version: env!("CARGO_PKG_VERSION").into(),
            metrics_interval_seconds: 10,
        }))
    }

    async fn report_metrics(
        &self,
        _req: Request<tonic::Streaming<SystemMetrics>>,
    ) -> Result<Response<MetricsResponse>, Status> {
        tracing::info!("gRPC metrics stream received");
        Ok(Response::new(MetricsResponse {
            accepted: true,
            message: "Metrics received".into(),
        }))
    }

    async fn execute_command(
        &self,
        req: Request<CommandRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        let inner = req.into_inner();
        tracing::info!("gRPC command: {} {}", inner.command, inner.args.join(" "));
        Ok(Response::new(CommandResponse {
            command_id: inner.command_id,
            success: false,
            stdout: format!("Executing: {} {}", inner.command, inner.args.join(" ")),
            stderr: String::new(),
            exit_code: 0,
            duration_ms: 5,
        }))
    }

    async fn transfer_file(
        &self,
        _req: Request<tonic::Streaming<FileRequest>>,
    ) -> Result<Response<Self::TransferFileStream>, Status> {
        Err(Status::unavailable("File transfer service not available on this agent"))
    }

    async fn stream_logs(
        &self,
        _req: Request<LogRequest>,
    ) -> Result<Response<Self::StreamLogsStream>, Status> {
        Err(Status::unavailable("Log streaming service not available on this agent"))
    }
}

pub async fn start_server(addr: SocketAddr) -> Result<(), tonic::transport::Error> {
    tracing::info!("Starting gRPC server on {addr}");
    tonic::transport::Server::builder()
        .add_service(AgentServiceServer::new(AgentServiceImpl))
        .serve(addr)
        .await
}
