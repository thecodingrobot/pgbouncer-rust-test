use tokio_postgres::{NoTls, Error};
use env_logger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let (client, connection) =
        tokio_postgres::connect("host=localhost dbname=mydb user=myuser password=mypwd", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT $1::TEXT", &[&"hello world"])
        .await?;

    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");

    Ok(())
}