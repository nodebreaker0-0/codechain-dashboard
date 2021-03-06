use crate::db::queries::network_usage;
use crate::db::queries::peer_count;
use r2d2_postgres::PostgresConnectionManager;
use std::{format, thread};

pub fn run(db_user: &str, db_password: &str) {
    let manager = PostgresConnectionManager::new(
        format!("postgres://{}:{}@localhost", db_user, db_password),
        r2d2_postgres::TlsMode::None,
    )
    .expect("Create connection manager");
    let pool = r2d2::Pool::new(manager).expect("Create connection pool");

    let five_minutes = std::time::Duration::from_secs(5 * 60);

    thread::Builder::new()
        .name("cron-remove-network-usage".to_string())
        .spawn(move || loop {
            let current_date = chrono::Utc::now();
            let one_week_ago = current_date - time::Duration::days(7);

            match pool.get() {
                Ok(connection) => {
                    if let Err(err) = network_usage::remove_older_logs(&connection, one_week_ago) {
                        cwarn!("Fail remove_older_logs: {:?}", err)
                    }
                    if let Err(err) = peer_count::remove_older_logs(&connection, one_week_ago) {
                        cwarn!("Fail remove_older_logs: {:?}", err)
                    }
                }
                Err(err) => cwarn!("remove_older_logs: {:?}", err),
            }

            thread::sleep(five_minutes);
        })
        .expect("Should success listening frontend");
}
