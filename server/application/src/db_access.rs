use diesel::pg::PgConnection;
use diesel::prelude::*;

struct DBConnector {
    db_name: &'static str,
    db_host: &'static str,
    db_user: &'static str,
    db_password: &'static str,
}

#[allow(dead_code)]
impl DBConnector {
    /// DB access constructor
    pub fn new(
        database: &'static str,
        host: &'static str,
        user: &'static str,
        password: &'static str,
    ) -> DBConnector {
        DBConnector {
            db_name: database,
            db_host: host,
            db_user: user,
            db_password: password,
        }
    }

    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.db_user, self.db_password, self.db_host, self.db_name
        )
    }

    pub fn establixh_connection(&self) -> PgConnection {
        // DBへ接続する
        let database_url = self.url();

        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_database_url() {
        let postgres = DBConnector::new("diesel_demo", "localhost", "username", "password");
        let db_url = postgres.url();
        assert_eq!("postgres://username:password@localhost/diesel_demo", db_url);
    }

    #[test]
    fn connect_to_database() {
        let postgres = DBConnector::new("test", "localhost:5433", "postgres", "password");

        postgres.establixh_connection();
    }
}
