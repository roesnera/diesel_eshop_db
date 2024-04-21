use std::{io::Write, str::FromStr};

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{deserialize::{FromSql, FromSqlRow}, expression::AsExpression, pg::{Pg, PgValue}, prelude::{Associations, Identifiable}, serialize::{Output, ToSql}, sql_types::{Double, Text}, AsChangeset, Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use crate::schema::*;

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