use deadpool_diesel::Manager;
use deadpool_diesel::Pool;
use deadpool_diesel::Runtime::Tokio1;
use diesel::pg::PgConnection;
use tracing::info;

/// Connect to the database.
pub fn connect_database(database_url: &String) -> Pool<Manager<PgConnection>> {
    let address = &database_url[database_url.find('@').expect("No '@' found in the string") + 1
        ..database_url.find('?').expect("No '?' found in the string")];
    let manager = Manager::new(database_url, Tokio1);

    info!("Connecting database at {} (http)", address);
    Pool::builder(manager)
        .build()
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
