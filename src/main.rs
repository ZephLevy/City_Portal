use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> &str {
        "Hello, world!"
    }
    async fn testing(&self) -> &str {
        "Testing"
    }
    async fn user(&self, id: i32) -> User {
        User {
            id,
            name: String::from("Zeph"),
        }
    }
}

#[derive(SimpleObject)]
struct User {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

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
