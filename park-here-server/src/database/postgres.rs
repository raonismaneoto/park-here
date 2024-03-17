use postgres::{Client, NoTls, Error};

// create a storage structure with a new or build function which returns a mutable client and also keeps implementations
// for the most common operations

pub struct Storage {
    connectionString: String
}

impl Storage {
    pub fn new(host: String, database: String, user: String, passwd: String) -> Self {
        Self {
            connectionString: String::from(format!("host={} database={} user={} password={}", host, database, user, passwd))
        }
    }

    pub fn exec(&self, cmd: String) -> bool {
        let get_conn_result = self.get_conn();
        let mut conn;
        match get_conn_result {
            Ok(connection) => conn = connection,
            Err(e) => panic!("{}", e)
        };

        conn.execute(&cmd, params)
    }

    pub fn query<T>(&self, cmd: String) -> Vec<T> {

    }

    fn get_conn(&self) ->  Result<Client, Error>{
        Client::connect::<NoTls>(&self.connectionString, NoTls)
    }
}