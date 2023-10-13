use std::net::SocketAddr;

use axum::{
    routing::{post,get},
    response::Html,
    Router,
};

use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate{
}

#[tokio::main]
async fn main (){
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/clicked", post(clicked));

    let addr = SocketAddr::from(([127,0,0,1],3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<String> {
    let test = HomeTemplate {};
    Html(test.render().unwrap().to_string())
}

async fn clicked() -> Html<String>{
    Html("Teste".to_string())
}
