#![recursion_limit="8192"]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate chrono;

use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;
use rocket_contrib::json::JsonValue;
use rocket::request::{Form, LenientForm};


extern crate uuid;

extern crate openssl;
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

mod db;
mod schema;

mod item;
use item::{Item, QueryItem};

mod r#type;
use r#type::{Type, QueryType};

mod custom_types;

extern crate dotenv;
use dotenv::dotenv;

// Implement FromFormValue for NaiveDateTime


// Item

#[post("/", data = "<item>")]
fn item_create(item: Json<Item>, connection: db::Connection) -> Json<Item> {
    let insert = Item { id: None, ..item.into_inner() };
    Json(Item::create(insert, &connection))
}

#[get("/")]
fn item_read(connection: db::Connection) -> JsonValue {
    json!(Item::read(&connection))
}

#[get("/?<query..>")]
fn item_search(query: LenientForm<QueryItem>, connection: db::Connection) -> JsonValue {
    json!(Item::search(query.into_inner(), &connection))
}

#[put("/<id>", data = "<item>")]
fn item_update(id: Uuid, item: Json<Item>, connection: db::Connection) -> JsonValue {
    let update = Item { id: Some(id.into_inner()), ..item.into_inner() };
    json!({
        "success": Item::update(id.into_inner(), update, &connection)
    })
}

#[patch("/<id>", data = "<item>")]
fn item_patch(id: Uuid, item: Json<Item>, connection: db::Connection) -> JsonValue {
    let update = Item { id: Some(id.into_inner()), ..item.into_inner() };
    json!({
        "success": Item::update(id.into_inner(), update, &connection)
    })
}

#[delete("/<id>")]
fn item_delete(id: Uuid, connection: db::Connection) -> JsonValue {
    json!({
        "success": Item::delete(id.into_inner(), &connection)
    })
}

// Type

#[post("/", data = "<t>")]
fn type_create(t: Json<Type>, connection: db::Connection) -> Json<Type> {
    let insert = Type { id: None, ..t.into_inner() };
    Json(Type::create(insert, &connection))
}

#[get("/")]
fn type_read(connection: db::Connection) -> JsonValue {
    json!(Type::read(&connection))
}

#[get("/?<query..>")]
fn type_search(query: LenientForm<QueryType>, connection: db::Connection) -> JsonValue {
    json!(Type::search(query.into_inner(), &connection))
}

#[put("/<id>", data = "<t>")]
fn type_update(id: i16, t: Json<Type>, connection: db::Connection) -> JsonValue {
    let update = Type { id: Some(id), ..t.into_inner() };
    json!({
        "success": Type::update(id, update, &connection)
    })
}

#[patch("/<id>", data = "<t>")]
fn type_patch(id: i16, t: Json<Type>, connection: db::Connection) -> JsonValue {
    let update = Type { id: Some(id), ..t.into_inner() };
    json!({
        "success": Type::update(id, update, &connection)
    })
}

#[delete("/<id>")]
fn type_delete(id: i16, connection: db::Connection) -> JsonValue {
    json!({
        "success": Type::delete(id, &connection)
    })
}

fn main() {
    // Read .env if exists
    dotenv().ok();

    rocket::ignite()
        .manage(db::connect())
        .mount("/item", routes![item_create, item_patch, item_delete])
        .mount("/items", routes![item_read, item_search])
        .mount("/type", routes![type_create, type_patch, type_delete])
        .mount("/types", routes![type_read, type_search])
        .launch();
}