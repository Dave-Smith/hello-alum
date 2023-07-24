mod api;
mod state;

use axum::Router;
use axum::routing::get;
use axum::extract::Path;
use crate::routing::state::AppState;

async fn greet() -> String {
    "Hello world!".to_string()
}

async fn greet_name(Path(name): Path<String>) -> String {
    format!("Hello {}!", name)
}

pub fn create_router() ->Router {
    let todo_api_route = api::create_router();
    let app_state = AppState::new();
    let app = Router::new()
        .route("/", get(greet))
        .route("/:name", get(greet_name))
        .nest("api/v1/todos", todo_api_route)
        .with_state(app_state);
    return app;
}

#[cfg(test)]
mod tests {
    use axum::http::Request;
    use axum::body::Body;
    use tower::ServiceExt;
    use super::*;

    #[tokio::test]
    async fn test_greet() {
        assert_eq!(super::greet().await, "Hello world!".to_string())
    }

    #[tokio::test]
    async fn test_create_router() {
        let router = create_router();
        let _resp = router.oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await.unwrap();
        //assert_eq!(resp.into_body(), "hello, world".to_string())
        assert_eq!("".to_string(), "hello, world".to_string())
    }
}