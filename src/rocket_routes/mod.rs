use reqwest::Request;
use rocket::http::hyper::request;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Value};
use rocket_db_pools::{deadpool_redis::{self, redis::AsyncCommands}, Database, Connection};
use diesel::PgConnection;
use rocket_sync_db_pools::database;

pub mod items;
pub mod authorization;

use crate::repository::RoleRepository;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Connection);

pub fn server_error(e: Box< dyn std::error::Error>) -> Custom<Value> {
  log::error!("{}", e);
  Custom(Status::InternalServerError, json!({ "error": e.to_string() }))
}

pub fn not_found_error(e: Box< dyn std::error::Error>) -> Custom<Value> {
  log::error!("{}", e);
  Custom(Status::NotFound, json!({ "error": e.to_string() }))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> {
  type Error = ();
  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let session_header = request.headers().get_one("Authorization")
      .map(|v| v.split_whitespace().collect::<Vec<_>>())
      .filter(|v| v.len == 2 && v[0] == "Bearer");

    if let Some(session_value) = session_header {
      let mut cache: Connection<CacheConn> = request.guard::<Connection<CacheConn>>().await
        .expect("Cannot connect to redis in request guard");

      let db: DbConn = request.guard::<DbConn>().await
        .expect("Cannot connect to postgres in request guard");

      let result = cache.get::<_, i32>(format!("session:{}", session_value[1])).await;

      if let Ok(user_id) = result {
        return match db.run(move |c| UserRepository::find(c, user_id)).await {
          Ok(user) => Outcome::Success(user),
          Err(e) => Outcome::Failure((Status::Unauthorized, e))
        }
      }
    }
  }
}