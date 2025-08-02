use diesel::result::Error;
use rocket::{serde::json::{Json, Value, serde_json::json}, response::status::{Custom, NoContent}, http::Status};

use crate::{models::{Item, NewItem, User}, repository::ItemRepository, rocket_routes::AdminUser};
use crate::rocket_routes::DbConn;

use super::{server_error, not_found_error};

#[rocket::get("/items")]
pub async fn get_items(db: DbConn, _user: User) -> Result<Json<Value>, Custom<Value>> {
    db.run(|c| ItemRepository::find_all(c))
        .await
        .map(|items| Json(json!(items)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/items/<id>")]
pub async fn get_item(id: i32, db: DbConn, _user: User) -> Result<Json<Value>, Custom<Value>> {
    db.run(move |c| ItemRepository::find(c, id))
        .await
        .map(|item| Json(json!(item)))
        .map_err(|e| match e {
            Error::NotFound => not_found_error(e.into()),
            _ => server_error(e.into())
        })
}

#[rocket::post("/items", format = "json", data = "<new_item>")]
pub async fn create_item(new_item: Json<NewItem>, db: DbConn, _user: AdminUser) -> Result<Json<Value>, Custom<Value>> {
    db.run(move |c| ItemRepository::create(c, new_item.into_inner()))
        .await
        .map(|item| Json(json!(item)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/items/<id>")]
pub async fn delete_item(id: i32, db: DbConn, _user: AdminUser) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| ItemRepository::delete(c, id))
        .await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/items/<id>", format = "json", data = "<item>")]
pub async fn update_item(id: i32, item: Json<Item>, db: DbConn, _user: AdminUser) -> Result<Json<Value>, Custom<Value>> {
    db.run(move |c| ItemRepository::update(c, id, item.into_inner()))
        .await
        .map(|item| Json(json!(item)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/items/<name>")]
pub async fn get_item_by_name(name: String, db: DbConn, _user: User) -> Result<Json<Value>, Custom<Value>> {
    db.run(move |c| ItemRepository::find_by_name(c, &name))
        .await
        .map(|item| Json(json!(item)))
        .map_err(|e| match e {
            Error::NotFound => not_found_error(e.into()),
            _ => server_error(e.into())
        })
}