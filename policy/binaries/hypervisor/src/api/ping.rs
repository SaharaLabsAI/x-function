use axum::{routing::get, Router};

use crate::api::ServerState;

pub fn api_register<S: ServerState>(router: Router<S>) -> Router<S> {
    router.route("/ping", get(pong))
}

async fn pong() -> &'static str {
    "pong"
}

#[cfg(test)]
mod tests {
    use crate::api::RouterRegister;

    use super::*;

    #[tokio::test]
    async fn test_api_ping() {
        let server = axum_test::TestServer::new(Router::new().register_api(api_register)).unwrap();

        let response = server.get("/ping").await;

        response.assert_status_ok();
        response.assert_text("pong");
    }
}
