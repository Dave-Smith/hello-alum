mod routing;

use axum::Server;
use routing::create_router;

#[tokio::main]
async fn main() {
    let app = create_router();

    Server::bind(&"0.0.0.0:3333".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
    println!("Hello, world!");
}

// cargo add tokio --features full
// cargo install watch
// cargo watch -c -q -x run
