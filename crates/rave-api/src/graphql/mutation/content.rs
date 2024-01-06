use crate::prelude::*;
use async_graphql::{self, Context, InputObject, Object, Result, SimpleObject};
use rave_entity::{tables::ContentFieldJson, async_graphql::Json};
use crate::services::database::Database;

#[derive(InputObject)]
pub struct CreateContentInput {
    content: Json<ContentFieldJson>,
}

#[derive(InputObject)]
pub struct CreateContentAttachmentInput {
    content_uid: String,
}

#[derive(Default)]
pub struct ContentMutation;

#[Object]
impl ContentMutation {
    pub async fn create_content_attachment(
        &self,
        _ctx: &Context<'_>,
        _input: CreateContentAttachmentInput,
    ) -> Result<String> {
        unimplemented!()
    }
    
    pub async fn create_content(
        &self,
        _ctx: &Context<'_>,
        _input: CreateContentInput,
    ) -> Result<String> {
        // let db = ctx.data::<Database>().unwrap();
        // let conn = db.get_connection();
        
        unimplemented!()
    }
}