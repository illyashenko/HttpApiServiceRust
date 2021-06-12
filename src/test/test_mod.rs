use crate::models::User;
use crate::sql_mapper::*;
use tokio_postgres::{Error, NoTls};

pub async fn get_data(email: String) -> Result<User, Error>{

    let (client, connection) = tokio_postgres::connect("postgresql://postgres:17101030@localhost/STORE", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let row = client.query("SELECT * FROM guids_users WHERE email = $1",
                           &[&email.as_str()]).await?;

    let user = user_mapper(&row[0]);

    Ok(user)
}