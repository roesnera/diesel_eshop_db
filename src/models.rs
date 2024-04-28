use std::{io::Write, str::FromStr};

use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;
use diesel::{expression::AsExpression, prelude::*, sql_types::Text, pg::{Pg, PgValue}, serialize::{Output, ToSql}, deserialize::{FromSql, FromSqlRow}};
use rocket::serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
pub struct Image {
    pub id: i32,
    pub url: String,
    #[serde(skip_deserializing)]
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name=images)]
pub struct NewImage {
    pub url: String,
}

#[derive(Queryable, Associations, Identifiable, Debug)]
#[belongs_to(Item)]
#[belongs_to(Image)]
#[diesel(table_name=items_images)]
pub struct ItemsImage {
    pub id: i32,
    pub item_id: i32,
    pub image_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name=items_images)]
pub struct NewItemsImage {
    pub item_id: i32,
    pub image_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
pub struct Item {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    #[serde(skip_deserializing)]
    pub created_at: Option<NaiveDateTime>,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name=items)]
pub struct NewItem {
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, AsChangeset, Debug)]
pub struct Role {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name=roles)]
pub struct NewRole {
    pub code: String,
    pub name: String,
}

#[derive(Queryable, Associations, Identifiable, Debug)]
#[belongs_to(User)]
#[belongs_to(Role)]
#[diesel(table_name=users_roles)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name=users_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(AsExpression, FromSqlRow, Serialize, Deserialize, Debug, Clone)]
#[sql_type="Text"]
pub enum RoleCode {
    Admin,
    User,
}

// Implement the `ToSql` trait for `RoleCode`.
impl FromStr for RoleCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes() {
            b"admin" => Ok(RoleCode::Admin),
            b"user" => Ok(RoleCode::User),
            _ => Err(()),
        }
    }
}

// Implement the `FromSql` trait for `RoleCode`.
// We don't need an Error case because there are only two possible states
// And we know them in advance
impl ToString for RoleCode {
    fn to_string(&self) -> String {
        match self {
            RoleCode::Admin => "admin".to_string(),
            RoleCode::User => "user".to_string(),
        }
    }
}

// Translates a `Text` SQL type to our `RoleCode` enum.
impl FromSql<Text, Pg> for RoleCode {
    fn from_sql(value: PgValue) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"admin" => Ok(RoleCode::Admin),
            b"user" => Ok(RoleCode::User),
            _ => Err("Unrecognized role code".into()),
        }
    }
}

// Translates our `RoleCode` enum to a `Text` SQL type.
impl ToSql<Text, Pg> for RoleCode {
    fn to_sql<'b>(&self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match self {
            RoleCode::Admin => out.write_all(b"admin")?,
            RoleCode::User => out.write_all(b"user")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}