use diesel::{PgConnection, QueryResult};
use diesel::prelude::*;

use crate::schema::*;
use crate::models::{Item, NewItem, NewRole, Role, RoleCode, User, NewUser, UserRole, Image, NewImage, ItemsImage, NewItemsImage};

use self::items_images::image_id;

pub struct ItemRepository;

impl ItemRepository {
  pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Item> {
    items::table.find(id).get_result(c)
  }

  pub fn find_all(c: &mut PgConnection) -> QueryResult<Vec<Item>> {
    items::table.load(c)
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

pub struct RoleRepository;

impl RoleRepository {
  pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Role> {
    roles::table.find(id).get_result(c)
  }

  pub fn find_by_code(c: &mut PgConnection, code: RoleCode) -> QueryResult<Role> {
    roles::table.filter(roles::code.eq(code)).first(c)
  }

  pub fn find_by_user(c: &mut PgConnection, user: &User) -> QueryResult<Vec<Role>> {
    let user_roles = UserRole::belonging_to(&user).get_results(c)?;

    let role_ids = user_roles.iter().map(|ur: &UserRole| ur.role_id).collect();
    Self::find_by_ids(c, role_ids)
  }

  pub fn find_by_ids(c: &mut PgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
      roles::table.filter(roles::id.eq_any(ids)).get_results(c)
  }

  pub fn create(c: &mut PgConnection, new_role: NewRole) -> QueryResult<Role> {
    diesel::insert_into(roles::table)
      .values(new_role)
      .get_result(c)
  }

  pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(roles::table.find(id)).execute(c)
  }

  pub fn update(c: &mut PgConnection, id: i32, role: Role) -> QueryResult<Role> {
    diesel::update(roles::table.find(id))
      .set((
        roles::code.eq(role.code),
        roles::name.eq(role.name),
        roles::created_at.eq(role.created_at),
      ))
      .get_result(c)
  }
}

pub struct UserRepository;

impl UserRepository {
  pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<User> {
    users::table.find(id).get_result(c)
  }

  pub fn find_by_username(c: &mut PgConnection, username: &str) -> QueryResult<User> {
    users::table.filter(users::username.eq(username)).first(c)
  }

  pub fn create(c: &mut PgConnection, new_user: NewUser) -> QueryResult<User> {
    diesel::insert_into(users::table)
      .values(new_user)
      .get_result(c)
  }

  pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(users::table.find(id)).execute(c)
  }

  pub fn update(c: &mut PgConnection, id: i32, user: User) -> QueryResult<User> {
    diesel::update(users::table.find(id))
      .set((
        users::username.eq(user.username),
        users::password.eq(user.password),
        users::email.eq(user.email),
        users::created_at.eq(user.created_at),
      ))
      .get_result(c)
  }
}

pub struct ImageRepository;

/**
 * ImageRepository is a struct that contains methods for interacting with the images table in the database.
 * Should be able to find
 *  by id
 *  by url
 *  by item association
 *  all
 * create
 *  a new image with an item association
 * delete
 *  by id both the image and it's relations in the items_images table
 */
impl ImageRepository {
  pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Image> {
    images::table.find(id).get_result(c)
  }

  pub fn find_by_url(c: &mut PgConnection, url: &str) -> QueryResult<Image> {
    images::table.filter(images::url.eq(url)).first(c)
  }

  pub fn find_all(c: &mut PgConnection) -> QueryResult<Vec<Image>> {
    images::table.load(c)
  }

  pub fn create_with_item(c: &mut PgConnection, new_image: NewImage, item_id: i32) -> QueryResult<Image> {

    let new_img_entry: Image = diesel::insert_into(images::table)
      .values(new_image)
      .get_result(c).unwrap();


    diesel::insert_into(items_images::table)
      .values(NewItemsImage {
        item_id,
        image_id: new_img_entry.id,
      })
      .execute(c)?;

    Ok(new_img_entry)
  }

  pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(items_images::table.filter(image_id.eq(id))).execute(c)?;
    diesel::delete(images::table.find(id)).execute(c)
  }
}