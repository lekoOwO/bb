use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use schema::types;

extern crate serde;
use self::serde::{Serialize, Deserialize};

use custom_types::{NaiveDateTimeForm};

use rocket::request::FromForm;

use chrono::{NaiveDateTime};

#[table_name = "types"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Type {
    pub id: Option<i16>,
    pub name: String,
    pub create_at: Option<NaiveDateTime>,
    pub comment: Option<String>
}

#[derive(FromForm)]
pub struct QueryType {
    pub id: Option<i16>,
    pub name: Option<String>,
    pub create_at: Option<NaiveDateTimeForm>,
    pub comment: Option<String>
}

fn filt<'a>(query: QueryType) -> types::BoxedQuery<'a, diesel::pg::Pg> {
    let mut boxed = types::table.into_boxed();
    if let Some(condition) = query.id {
        boxed = boxed.filter(types::id.eq(condition));
    }
    if let Some(condition) = query.name {
        boxed = boxed.filter(types::name.like(condition));
    }
    if let Some(condition) = query.create_at {
        boxed = boxed.filter(types::create_at.eq(*condition));
    }
    if let Some(condition) = query.comment {
        boxed = boxed.filter(types::comment.like(condition));
    }
    boxed
}

impl Type {
    pub fn create(r#type: Type, connection: &PgConnection) -> Type {
        diesel::insert_into(types::table)
            .values(&r#type)
            .execute(connection)
            .expect("Error creating new type");

        types::table.order(types::create_at.desc()).first(connection).unwrap()
    }

    pub fn search(query: QueryType, connection: &PgConnection) -> Vec<Type> {
        let boxed = filt(query);
        boxed.order(types::create_at.asc()).load::<Type>(connection).unwrap()
    }

    pub fn read(connection: &PgConnection) -> Vec<Type> {
        types::table.order(types::create_at.asc()).load::<Type>(connection).unwrap()
    }

    pub fn update(id: i16, r#type: Type, connection: &PgConnection) -> bool {
        diesel::update(types::table.find(id)).set(&r#type).execute(connection).is_ok()
    }

    pub fn delete(id: i16, connection: &PgConnection) -> bool {
        diesel::delete(types::table.find(id)).execute(connection).is_ok()
    }
}