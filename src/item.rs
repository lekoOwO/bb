use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use schema::items;

extern crate serde;
use self::serde::{Serialize, Deserialize};

use chrono::{NaiveDateTime};
use uuid::Uuid;

use custom_types::{NaiveDateTimeForm, UuidForm};

use rocket::request::FromForm;


#[table_name = "items"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Item {
    pub id: Option<Uuid>,
    pub name: String,
    pub buy_time: Option<NaiveDateTime>,
    pub owner: Option<String>,
    pub create_at: Option<NaiveDateTime>,
    pub type_id: Option<i16>,
    pub comment: Option<String>
}

#[derive(FromForm)]
pub struct QueryItem {
    pub id: Option<UuidForm>,
    pub name: Option<String>,
    pub buy_time: Option<NaiveDateTimeForm>,
    pub owner: Option<String>,
    pub create_at: Option<NaiveDateTimeForm>,
    pub type_id: Option<i16>,
    pub comment: Option<String>
}

fn filt<'a>(query: QueryItem) -> items::BoxedQuery<'a, diesel::pg::Pg> {
    let mut boxed = items::table.into_boxed();
    if let Some(condition) = query.id {
        boxed = boxed.filter(items::id.eq(*condition));
    }
    if let Some(condition) = query.name {
        boxed = boxed.filter(items::name.like(condition));
    }
    if let Some(condition) = query.buy_time {
        boxed = boxed.filter(items::buy_time.eq(*condition));
    }
    if let Some(condition) = query.owner {
        boxed = boxed.filter(items::owner.like(condition));
    }
    if let Some(condition) = query.create_at {
        boxed = boxed.filter(items::create_at.eq(*condition));
    }
    if let Some(condition) = query.type_id {
        boxed = boxed.filter(items::type_id.eq(condition));
    }
    if let Some(condition) = query.comment {
        boxed = boxed.filter(items::comment.like(condition));
    }
    boxed
}

impl Item {
    pub fn create(item: Item, connection: &PgConnection) -> Item {
        diesel::insert_into(items::table)
            .values(&item)
            .execute(connection)
            .expect("Error creating new item");

        items::table.order(items::create_at.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &PgConnection) -> Vec<Item> {
        items::table.order(items::create_at.asc()).load::<Item>(connection).unwrap()
    }

    pub fn search(query: QueryItem, connection: &PgConnection) -> Vec<Item> {
        let boxed = filt(query);
        boxed.order(items::create_at.asc()).load::<Item>(connection).unwrap()
    }

    pub fn update(id: Uuid, item: Item, connection: &PgConnection) -> bool {
        diesel::update(items::table.find(id)).set(&item).execute(connection).is_ok()
    }

    pub fn delete(id: Uuid, connection: &PgConnection) -> bool {
        diesel::delete(items::table.find(id)).execute(connection).is_ok()
    }
}