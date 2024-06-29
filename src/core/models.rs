use crate::core::models::schema::users;
use chrono::NaiveDateTime;
use diesel::pg::Pg;
use diesel::prelude::Insertable;
use diesel::prelude::Queryable;
use diesel::prelude::Selectable;
use uuid::Uuid;

pub mod schema;

#[derive(Debug, Insertable, PartialEq, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
