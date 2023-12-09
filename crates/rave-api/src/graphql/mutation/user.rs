use crate::prelude::*;
use async_graphql::{self, Context, InputObject, Object, Result, SimpleObject};
use rave_entity::iam::user::{self, User};

use crate::services::database::Database;

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
}

impl CreateUserInput {
    fn into_model_with_arbitrary_id(self) -> User {
        User {
            entity_sid: unimplemented!(),
            name: unimplemented!(),
            email: unimplemented!(),
        }
    }
}

#[derive(SimpleObject)]
pub struct DeleteResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(
        &self,
        _ctx: &Context<'_>,
        _input: CreateUserInput,
    ) -> Result<User> {
        // let db = ctx.data::<Database>().unwrap();
        // let conn = db.get_connection();

        // Ok(Mutation::create_note(conn, input.into_model_with_arbitrary_id()).await?)
        unimplemented!()
    }

    // pub async fn delete_note(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        // let db = ctx.data::<Database>().unwrap();
        // let conn = db.get_connection();

        // let res = Mutation::delete_note(conn, id)
        // .await
        // .expect("Cannot delete note");

        // if  res.rows_affected <= 1 {
        // Ok(DeleteResult {
        // success: true,
        // rows_affected: res.rows_affected,
        // })
        // } else {
        // unimplemented!()
        // }
    // }
}