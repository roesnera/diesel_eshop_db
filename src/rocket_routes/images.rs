use std::fs;

use rocket::{data::Data, http::ContentType, serde::json::{Json, Value, serde_json::json}, response::status::{Custom, NoContent}, http::Status};
use rocket_multipart_form_data::{FileField, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions};

use crate::{models::{Image, NewImage}, repository::ImageRepository, rocket_routes::AdminUser};
use crate::rocket_routes::DbConn;

use super::server_error;

#[rocket::post("/images/new/<item_id>", data = "<data>")]
pub async fn upload_image(content_type: &ContentType, data: Data<'_>, db: DbConn, _user: AdminUser, item_id: i32) -> Result<Json<Value>, Custom<Value>> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("media"),
    ]);

    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();

    let photo = multipart_form_data.raw.remove("media");

    if let Some(mut file_fields) = photo {
        let raw_field = file_fields.remove(0);
        let file_name = match &raw_field.file_name {
            Some(file_name) => file_name,
            None => return Err(Custom(Status::BadRequest, "No file name provided".into()))
        };
        let content_type = &raw_field.content_type;
        let raw_file = &raw_field.raw;

        let image_db_entry = NewImage {
            url: file_name.to_string(),
        };

        fs::write(format!("images/{}", &image_db_entry.url), raw_file).map_err(|e| server_error(e.into()))?; // Save the file to the images folder

        return db.run(move |c| ImageRepository::create_with_item(c, image_db_entry, item_id))
            .await
            .map(|image| Json(json!(image)))
            .map_err(|e| server_error(e.into()))
    }

    Err(Custom(Status::BadRequest, "No file provided".into()))
}