use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use serde_json::json;
use crate::auth::{Credentials, authorize_user};

use crate::repository::UserRepository;

use super::{DbConn, server_error, CacheConn};


#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(credentials: Json<Credentials>, db: DbConn, mut cache: Connection<CacheConn>) -> Result<serde_json::Value, rocket::response::status::Custom<serde_json::Value>> {
    let username = credentials.username.clone();
    let user = db.run(move |c| {
        UserRepository::find_by_username(c, &username)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => Custom(Status::Unauthorized, json!("Username is not valid")),
            _ => server_error(e.into())
        })

        // UserRepository::get_result()
    }).await?;

    let session_id = authorize_user(&user, &credentials)
        .map_err(|_| Custom(Status::Unauthorized, json!("Wrong Credentials")))?;

    cache.set_ex::<_, _, ()>(format!("sessions/{}", session_id), 
        user.id, 
        3*60*60
    ).await
    .map(|_| json!({"token": session_id}))
    .map_err(|e| server_error(e.into()))
}