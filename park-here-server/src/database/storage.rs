use postgres_types::ToSql;
use tokio_postgres::{NoTls, Error, Row};

// create a storage structure with a new or build function which returns a mutable client and also keeps implementations
// for the most common operations

#[derive(Clone)]
pub struct Storage {
    connection_string: String,
}

impl Storage {
    pub fn new(host: String, database: String, user: String, passwd: String) -> Self {
        Self {
            connection_string: String::from(format!(
                "host={} dbname={} user={} password={}",
                host, database, user, passwd
            )),
        }
    }

    pub async fn exec(&self, cmd: String, cmd_params: &[&(dyn ToSql + Sync)]) -> bool {
        let connection_result =
        tokio_postgres::connect(&self.connection_string, NoTls).await;

        match connection_result {
            Ok((client, connection)) => {
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });

                if let Ok(_) = client.execute(&cmd, &cmd_params).await {
                    true
                } else {
                    false
                }
            },
            Err(err) => {
                print!("{}", err);
                false
            }
        }
    }

    pub async fn query(
        &self,
        cmd: String,
        query_params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, Error> {
        let connection_result =
        tokio_postgres::connect(&self.connection_string, NoTls).await;

        match connection_result {
            Ok((client, connection)) => {
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });

                client.query(&cmd, &query_params).await
            },
            Err(err) => {
                print!("{}", err);
                Result::Err(err)
            }
        }
    }
}
