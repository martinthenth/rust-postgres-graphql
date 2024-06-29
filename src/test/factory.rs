use crate::core::models::User;
use crate::{config, core::users};
use diesel::Connection;
use diesel::PgConnection;

pub fn create_user() -> User {
    let config = config::get_config();
    let mut conn = PgConnection::establish(&config.database_url).unwrap();

    users::create_user(
        &mut conn,
        users::CreateUserAttrs {
            first_name: String::from("Jane"),
            last_name: String::from("Doe"),
            email_address: String::from("jane@doe.com"),
        },
    )
    .unwrap()
    .unwrap()
}
