use super::connection::*;
use tokio_postgres::{
    Client,
    error::Error,
    Row
};

use std::future::Future;

pub struct Service {
    pub client: Client
}

impl Service {
    pub async fn new() -> Self {
        let new_client = init().await.unwrap();

        Service {
            client: new_client
        }
    }

    // pub async fn execute(&self, stmt: String) -> impl Future<Output = Result<u64, Error>> + '_ {
    //     async move {
    //         self
    //         .client
    //         .execute(
    //             &stmt,
    //             &[]
    //         )
    //         .await
    //     }
    // }

    // pub async fn query(&self, stmt: String) -> impl Future<Output = Result<Vec<Row>, Error>> + '_ {
    //     async move {
    //         self
    //         .client
    //         .query(
    //             &stmt,
    //             &[]
    //         )
    //         .await
    //     }
    // }
}