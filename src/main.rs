mod models;
mod resolvers;
mod schema;

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    routing::{get, post},
};
use schema::build_schema;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let schema = build_schema();

    let app = Router::new()
        .route(
            "/graphql",
            post({
                let schema = schema.clone();
                move |req: GraphQLRequest| async move {
                    GraphQLResponse::from(schema.execute(req.into_inner()).await)
                }
            }),
        )
        .route(
            "/graphql",
            get({
                let schema = schema.clone();
                move || async move { axum::Json(schema.sdl()) }
            }),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}
