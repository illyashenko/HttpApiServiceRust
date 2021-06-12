use tokio_postgres::{Client, NoTls, Connection};
use std::fmt::Error;

pub struct Context{
   pub client: Client
}

pub struct ContextBuilder {}

impl ContextBuilder{
   pub async fn new() -> Result<Context, Error> {
       let (client_, connection) = tokio_postgres::connect("postgresql://postgres:17101030@localhost/STORE", NoTls).await.unwrap();
       tokio::spawn(async move {
           if let Err(err) = connection.await {
               eprintln!("connection error: {}", err);
           }
       });
        Ok(Context {client: client_})
    }
}
