use axum::Server;
use axum::Router;
use axum::routing::get;

async fn greet() -> String {
    "Hello world!".to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(greet));

    Server::bind(&"0.0.0.0:3333".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
    println!("Hello, world!");
}

// cargo add tokio --features full
// cargo install watch
// cargo watch -c -q -x run