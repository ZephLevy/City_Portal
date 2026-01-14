use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    response::Html,
    routing::{get, post},
};
use std::net::SocketAddr;

mod models;
mod resolvers;
mod schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let schema = schema::build_schema().await?;

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
        )
        .route(
            "/graphiql",
            get(Html(GraphiQLSource::build().endpoint("/graphql").finish())),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
    Ok(())
}
