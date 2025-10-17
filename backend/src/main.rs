use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpServer, guard, web};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use std::env;

mod ai;
mod db;
mod graphql;
mod models;

use db::create_pool;
use graphql::schema::{AppSchema, MutationRoot, QueryRoot};

async fn index(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open_async("./src/graphql/graphiql.html").await?)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Failed to read .env file");

    let pool = create_pool().await.expect("Failed to create DB pool");

    let allowed_origin = env::var("ALLOWED_ORIGIN").expect("Allowed Origin not set");

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool.clone())
        .finish();

    println!("🚀 Server starting on http://127.0.0.1:8080");
    println!("🚀 GraphiQL IDE available at http://127.0.0.1:8080/graphql");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&allowed_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(schema.clone()))
            .wrap(cors)
            .service(web::resource("/graphql").guard(guard::Post()).to(index)) // Main GraphQL endpoint
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(index_graphiql),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
