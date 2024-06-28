use crate::server::schema::user_schema::UserMutation;
use crate::server::schema::user_schema::UserQuery;
use async_graphql::EmptySubscription;
use async_graphql::MergedObject;
use async_graphql::Schema;
use deadpool_diesel::postgres::Pool;

pub mod user_schema;

/// The GraphQL schema type.
pub type GraphSchema = Schema<Query, Mutation, EmptySubscription>;

/// The parent query object, merged from child modules.
#[derive(MergedObject, Default)]
pub struct Query(UserQuery);

/// The parent mutation object, merged from child modules.
#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation);

/// Create a GraphQL schema.
pub fn create_schema(database: Pool) -> GraphSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(database)
        .finish()
}
