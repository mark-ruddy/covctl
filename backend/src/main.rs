use axum::{extract::Extension, routing::get, routing::post, Router};
use clap::Parser;
use log::{error, info};
use std::net::SocketAddr;
use std::sync::Arc;

mod routes;

/// The CLI args this server accepts
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The address to host on
    #[clap(long, default_value = "127.0.0.1:3030")]
    addr: String,

    /// The address of the MongoDB endpoint
    #[clap(long, default_value = "mongodb://localhost:27017")]
    mongo_addr: String,

    /// The name of the MongoDB Database
    #[clap(long, default_value = "klaytn_sifter")]
    mongo_name: String,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    let db_client =
        match routes::data::init_mongo_client(args.mongo_addr.as_str(), args.mongo_name.as_str())
            .await
        {
            Ok(db_client) => db_client,
            Err(e) => {
                error!("Failed to connect to database: {}", e);
                return;
            }
        };
    let handler_state = Arc::new(routes::State { db: db_client });

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/sample", post(routes::sample))
        .layer(Extension(handler_state));

    let addr: SocketAddr = match args.addr.parse() {
        Ok(addr) => addr,
        Err(e) => {
            error!("Failed to parse address to host on(flag --addr): {}", e);
            return;
        }
    };

    info!("klaytn_sifter backend running on address: {}", addr);
    match axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        Ok(_) => (),
        Err(e) => error!("Axum server has failed: {}", e),
    }
}
