use rocket::http::Status;
use rocket::{Request, request::{FromRequest, Outcome}};
use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Value};
use rocket_db_pools::{deadpool_redis::{self, redis::AsyncCommands}, Database, Connection};
use diesel::PgConnection;
use rocket_sync_db_pools::database;

pub mod items;
pub mod authorization;
pub mod images;

use crate::models::{RoleCode, User};
use crate::repository::{RoleRepository, UserRepository};

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);

pub fn server_error(e: Box< dyn std::error::Error>) -> Custom<Value> {
  log::error!("{}", e);
  Custom(Status::InternalServerError, json!({ "error": e.to_string() }))
}

pub fn not_found_error(e: Box< dyn std::error::Error>) -> Custom<Value> {
  log::error!("{}", e);
  Custom(Status::NotFound, json!({ "error": e.to_string() }))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
  type Error = ();
  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    // Authorization: Bearer SESSION_ID_128_CHARS_LONG
    // Get the Authorization header from the request
    let session_header = request.headers().get_one("Authorization")
      // Split the header value by whitespace to get a vec with "Bearer" and the session id
      .map(|v| v.split_whitespace().collect::<Vec<_>>())
      // Filter the vec to only get the values that have a length of 2 and the first value is "Bearer", which will filter out formally invalid auth headers
      .filter(|v| v.len() == 2 && v[0] == "Bearer");

    // If the session header is present and has the format we expect
    if let Some(session_value) = session_header {
      // Get the connection to the redis cache
      let mut cache: Connection<CacheConn> = request.guard::<Connection<CacheConn>>().await
        .expect("Cannot connect to redis in request guard");

      // Get the connection to the postgres database
      let db: DbConn = request.guard::<DbConn>().await
        .expect("Cannot connect to postgres in request guard");

      // Get the user id from the cache using the session id
      let result = cache.get::<_, i32>(format!("session:{}", session_value[1])).await;

      // If the session id is present in the cache
      if let Ok(user_id) = result {
        return match db.run(move |c| UserRepository::find(c, user_id)).await {
          Ok(user) => Outcome::Success(user),
          Err(_) => Outcome::Error((Status::Unauthorized, ()))
        }
      }
    }
    Outcome::Error((Status::Unauthorized, ()))
  }
}

pub struct AdminUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
  type Error = ();

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let user = request.guard::<User>().await.expect("Cannot retrieve logged in user in request guard");

    // Get the connection to the postgres database
    let db: DbConn = request.guard::<DbConn>().await
      .expect("Cannot connect to postgres in request guard");

    let admin_option = db.run(move |c| {
      match RoleRepository::find_by_user(c, &user) {
        Ok(roles) => {
          log::info!("Assigned roles: {:?}", roles);
          let is_admin = roles.iter().any(|r| r.code == RoleCode::Admin.to_string());
          log::info!("is_admin: {:?}", is_admin);
          is_admin.then_some(AdminUser(user))
        },
        _ => None
      }
    }).await;

    match admin_option {
      Some(admin) => Outcome::Success(admin),
      None => Outcome::Error((Status::Unauthorized, ()))
    }

  }
}