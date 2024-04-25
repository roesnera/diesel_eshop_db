extern crate diesel_eshop_db;

use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
  let _ = rocket::build()
  .mount("/", rocket::routes![
    diesel_eshop_db::rocket_routes::items::get_items
    ])
    .attach(diesel_eshop_db::rocket_routes::DbConn::fairing())
    .attach(diesel_eshop_db::rocket_routes::CacheConn::init())
    .launch();
}