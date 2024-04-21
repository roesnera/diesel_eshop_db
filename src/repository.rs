use std::borrow::Borrow;

use diesel::dsl::{IntervalDsl, now};
use diesel::{PgConnection, QueryResult};
use diesel::prelude::*;

use crate::schema::*;
use crate::models::{self, Item, NewItem};

pub struct ItemRepository;

impl ItemRepository {
  pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Item> {
    items::table.find(id).get_result(c)
  }

  pub fn find_by_name(c: &mut PgConnection, name: &str) -> QueryResult<Item> {
    items::table.filter(items::name.eq(name)).first(c)
  }

  pub fn create(c: &mut PgConnection, new_item: NewItem) -> QueryResult<Item> {
    diesel::insert_into(items::table)
      .values(new_item)
      .get_result(c)
  }

  pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(items::table.find(id)).execute(c)
  }

  pub fn update(c: &mut PgConnection, id: i32, item: Item) -> QueryResult<Item> {
    diesel::update(items::table.find(id))
      .set((
        items::name.eq(item.name),
        items::description.eq(item.description),
        items::price.eq(item.price),
        items::quantity.eq(item.quantity),
        items::created_at.eq(item.created_at),
      ))
      .get_result(c)
  }
}