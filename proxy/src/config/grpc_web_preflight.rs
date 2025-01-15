use async_trait::async_trait;
use http::Response;
use pingora::{
    apps::http_app::{HttpServer, ServeHttp},
    modules::http::compression::ResponseCompressionBuilder,
    protocols::http::ServerSession,
    services::listening::Service,
    upstreams::peer::HttpPeer,
};

pub struct GrpcWebPreflightHttpApp;

pub const IPV4_GRPC_WEB_PREFLIGHT_URL: &str = "0.0.0.0:8080";

#[async_trait]
impl ServeHttp for GrpcWebPreflightHttpApp {
    async fn response(&self, _http_session: &mut ServerSession) -> Response<Vec<u8>> {
        let buffer = "Grpc Web OK".as_bytes().to_vec();

        Response::builder()
            .status(200)
            .header(
                http::header::ACCESS_CONTROL_ALLOW_HEADERS,
                "content-type,x-grpc-web,x-user-agent",
            )
            .header(http::header::ACCESS_CONTROL_ALLOW_METHODS, "POST")
            .header(http::header::CONTENT_LENGTH, buffer.len())
            .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "null")
            .header(http::header::ACCESS_CONTROL_EXPOSE_HEADERS, "*")
            .header(http::header::ACCESS_CONTROL_MAX_AGE, 1728000)
            .body(buffer)
            .unwrap()
    }
}

type GrpcWebPreflightServer = HttpServer<GrpcWebPreflightHttpApp>;

fn create_grpc_web_preflight_server() -> GrpcWebPreflightServer {
    let mut server = HttpServer::new_app(GrpcWebPreflightHttpApp);
    server.add_module(ResponseCompressionBuilder::enable(7));
    server
}

pub type GrpcWebPreflightService = Service<GrpcWebPreflightServer>;

pub fn create_grpc_web_preflight_service() -> GrpcWebPreflightService {
    let mut service = Service::new(
        "Grpc Web Preflight".to_string(),
        create_grpc_web_preflight_server(),
    );
    service.add_tcp(IPV4_GRPC_WEB_PREFLIGHT_URL);
    service
}

pub fn create_grpc_web_preflight_peer() -> Box<HttpPeer> {
    Box::new(HttpPeer::new(
        IPV4_GRPC_WEB_PREFLIGHT_URL,
        false,
        "".to_string(),
    ))
}
