mod config;

use http::header::CONTENT_TYPE;
use pingora::{
    apps::HttpServerOptions,
    http::RequestHeader,
    modules::http::{
        grpc_web::{GrpcWeb, GrpcWebBridge},
        HttpModules,
    },
    prelude::HttpPeer,
    protocols::ALPN,
    proxy::{http_proxy_service, ProxyHttp, Session},
    server::{configuration::Opt, Server},
    Error,
};

use config::grpc_web_preflight::{
    create_grpc_web_preflight_peer, create_grpc_web_preflight_service,
};

struct MyProxy;

#[async_trait::async_trait]
impl ProxyHttp for MyProxy {
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX {}

    fn init_downstream_modules(&self, modules: &mut HttpModules) {
        // Add the gRPC web module
        modules.add_module(Box::new(GrpcWeb))
    }

    async fn early_request_filter(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<(), Box<Error>> {
        let grpc = session
            .downstream_modules_ctx
            .get_mut::<GrpcWebBridge>()
            .expect("GrpcWebBridge module added");

        // initialize gRPC module for this request
        grpc.init();

        Ok(())
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>, Box<Error>> {
        let method = &session.req_header().method;

        if method == "OPTIONS" {
            return Ok(create_grpc_web_preflight_peer());
        }

        // gRPC server
        let mut peer = Box::new(HttpPeer::new(
            String::from("0.0.0.0:50051"),
            false,
            String::from(""),
        ));

        peer.options.alpn = ALPN::H2;

        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<(), Box<Error>> {
        let content_type = upstream_request
            .headers
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default();

        println!("content-type: {}", content_type);

        Ok(())
    }
}

fn main() {
    let opt = Opt::default();
    let mut server = Server::new(opt).unwrap();
    server.bootstrap();

    let mut proxy = http_proxy_service(&server.configuration, MyProxy);
    let grpc_web_preflight = create_grpc_web_preflight_service();

    let app_logic = proxy.app_logic_mut().unwrap();
    let mut http_server_options = HttpServerOptions::default();
    http_server_options.h2c = true;
    app_logic.server_options = Some(http_server_options);

    proxy.add_tcp("0.0.0.0:6193");

    server.add_service(grpc_web_preflight);
    server.add_service(proxy);

    println!("proxy running");
    server.run_forever();
}
