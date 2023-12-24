
use tokio_postgres::{NoTls, Error, Client, Connection};

pub struct Db {
    pub client: Client,
    pub connection: Connection<NoTls>,
}



impl Db {
    pub async fn new() -> Result<Self, Error> {
        let host = env::var("DB_HOST").expect("DB_HOST must be set");
        let user = env::var("DB_USER").expect("DB_USER must be set");
        let password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
        let dbname = env::var("DB_NAME").expect("DB_NAME must be set");

        let (client, connection) =
            tokio_postgres::connect(&format!("host={} user={} password={} dbname={}", host, user, password, dbname), NoTls).await?;

        Ok(Self { client, connection })
    }

}