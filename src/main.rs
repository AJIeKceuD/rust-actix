use actix_web::{web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use env_logger::Env;
#[allow(unused_imports)]
use log::{debug, error, log_enabled, info, Level};

use std::env;
use dotenv::dotenv;

// use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

mod routes;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    info!("using sqlite database at: {}", &database_url);
    let db_pool =  PgPoolOptions::new()
        .max_connections(5)
        // .connect("host=localhost user=postgres password=postgres port=5433 dbname=rust").await?;
        .connect(database_url.as_str()).await.expect("Cant connect to DB");;

    info!("using sqlite database at: {}", &database_url);
    // Make a simple query to return the given parameter (TODO delete)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&db_pool).await.expect("Cant make first request to DB");
    assert_eq!(row.0, 150);

    HttpServer::new(move || {
        App::new()
            // pass database pool to application so we can access it inside handlers
            .app_data(Data::new(db_pool.clone()))
            // middleware
            .wrap(middleware::sayhi::default())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            // routes
            .configure(routes::config)
            .service(web::scope("/api").configure(routes::scoped_config))
            .service(routes::hello)
            .service(routes::echo)
            .route("/hey", web::get().to(routes::manual_hello))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

// Stop on middleware, cant make it right
