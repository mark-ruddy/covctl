use mongodb::options::ClientOptions;
use std::error::Error;

/// Create and return a connection to a MongoDB database
pub async fn init_mongo_client(
    conn_url: &str,
    db_name: &str,
) -> Result<mongodb::Client, Box<dyn Error>> {
    let mut client_options = ClientOptions::parse(conn_url).await?;
    client_options.app_name = Some(db_name.to_string());
    Ok(mongodb::Client::with_options(client_options)?)
}
