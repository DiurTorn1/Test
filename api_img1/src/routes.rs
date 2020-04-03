use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{Image, NewImage};
use serde_json::Value;
use rocket::{get, post, put, delete};

#[get("/images", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let images = Image::all(&conn);

    Json(json!({
        "status": 200,
        "result": images,
    }))
}
#[post("/images", format = "application/json", data = "<new_image>")]
pub fn new(conn: DbConn, new_image: Json<NewImage>) -> Json<Value> {
    Json(json!({
        "status": Image::insert(new_image.into_inner(), &conn),
        "result": Image::all(&conn).first(),
    }))
}

#[get("/images/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result = Image::show(id, &conn);
    let status = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[put("/images/<id>", format = "application/json", data = "<image>")]
pub fn update(conn: DbConn, id: i32, image: Json<NewImage>) -> Json<Value> {
    let status = if Image::update_by_id(id, &conn, image.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/images/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
     let status = if Image::delete_by_id(id, &conn) {
         200
     } else {
         404
     };
     Json(json!({
         "status": status,
         "result": null,
     }))
}

#[get("/images/nameimg/<nameimg>", format = "application/json")]
pub fn nameimg(nameimg: String, conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": "200",
        "result": Image::all_by_nameimg(nameimg, &conn),
    }))
}

#[catch(404)]
pub fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found"
    }))
}