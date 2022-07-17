use openssl::error::ErrorStack;
use openssl::ssl::{SslConnector, SslMethod};
use tokio_postgres::{ Client, Connection, Socket, error::Error, connect };
use postgres_openssl::{ MakeTlsConnector, TlsStream };

pub async fn init() -> Result<Client, Error> {
    let tls_connector = ssl_config().unwrap();
    let url = connection_string();

    let (client, connection) = connect(&url, tls_connector).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection Error: {}", e);
        }
    });

    Ok(client)
}

fn ssl_config() -> Result<MakeTlsConnector, ErrorStack> {
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_ca_file("root.crt")?;

    Ok(MakeTlsConnector::new(builder.build()))
}

fn connection_string() -> String {
    dotenv::dotenv().ok();

    let user = std::env::var("USERNAME").expect("USER not found");
    let password = std::env::var("PASSWORD").expect("PASSWORD not found");
    let host = std::env::var("HOST").expect("HOST not found");
    let port = std::env::var("PORT").expect("PORT not found");
    let cluster = std::env::var("CLUSTER").expect("CLUSTER not found");
    let database = std::env::var("DATABASE").expect("DATABASE not found");

    format!(
        "postgresql://{}:{}@{}:{}/{}?options=--cluster%3D{}",
        user,
        password,
        host,
        port,
        database,
        cluster
    )
}