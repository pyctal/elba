use data_service::database::database_client::establish_connection;
use diesel::{Connection, PgConnection};

pub fn get_test_connection() -> PgConnection {
    let mut pg_connection = establish_connection();
    pg_connection.begin_test_transaction().unwrap();
    pg_connection
}
