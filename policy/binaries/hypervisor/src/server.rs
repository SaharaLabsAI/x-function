use axum::http::HeaderValue;
use axum::{http::Method, Router};
use tower_http::cors::CorsLayer;

use crate::api::{self, RouterRegister};
use crate::types::{HypervisorState, ServerContext};
use crate::Config;

pub struct Server {
    app: Router,
    ctx: ServerContext,
}

impl Server {
    pub fn build(config: Config) -> anyhow::Result<Self> {
        let state = HypervisorState::new(config);

        let ctx = ServerContext {
            state: state.clone(),
        };

        let app = Router::new()
            .register_api(api::ping::api_register)
            .register_api(api::encrypt::api_register)
            .register_api(api::openai::api_register)
            .register_api(api::agent::api_register)
            .with_state(state)
            .layer(
                CorsLayer::new()
                    .allow_origin("*".parse::<HeaderValue>()?)
                    .allow_methods([Method::GET, Method::POST]),
            );

        Ok(Server { app, ctx })
    }

    pub async fn start(self) -> anyhow::Result<()> {
        let config = &self.ctx.state.config;

        let listener = tokio::net::TcpListener::bind(config.listening).await?;
        tracing::info!("listening on {}", config.listening);

        axum::serve(listener, self.app).await?;

        Ok(())
    }
}
