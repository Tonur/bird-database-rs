// use crate::bird::Bird;
// use crate::database::Db;
// use crate::error;

// use rocket::serde::json::Json;

// use sea_orm_rocket::Connection;

// use rocket_okapi::okapi::openapi3::OpenApi;

// use rocket_okapi::settings::OpenApiSettings;

// use rocket_okapi::{openapi, openapi_get_routes_spec};


// // pub fn stage() -> AdHoc {
// //     AdHoc::on_ignite("Bird database", |rocket| async {
// //         rocket.attach(Db::init())
// //             .mount("/birds", routes![list, read, create, update, delete])
// //     })
// // }

// pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
//     openapi_get_routes_spec![settings: list, read, create, update, delete]
// }

// pub type R<T> = std::result::Result<rocket::serde::json::Json<T>, error::Error>;
// pub type DataResult<'a, T> =
//     std::result::Result<rocket::serde::json::Json<T>, rocket::serde::json::Error<'a>>;

// #[openapi(tag = "GET")]
// #[get("/")]
// async fn list(
//     conn: Connection<'_, Db>,
// ) -> R<Json<Vec<Bird>>> {
// }

// #[openapi(tag = "READ")]
// #[get("/<id>")]
// async fn read(
//     conn: Connection<'_, Db>, 
//     id: i32
// ) -> R<Option<Bird>> {
// }

// #[openapi(tag = "LIST")]
// #[post("/", data = "<post_data>")]
// async fn create(
//     conn: Connection<'_, Db>, 
//     post_data: DataResult<'_, Bird>,
// ) -> R<Option<String>> {
//     let db = conn.into_inner();
//     let form = post_data?.into_inner();
//     let cmd = Mutation::create_post(db, form);
//     match cmd.await {
//         Ok(_) => Ok(Json(Some("Post successfully added.".to_string()))),
//         Err(e) => {
//             let m = error::Error {
//                 err: "Could not insert post".to_string(),
//                 msg: Some(e.to_string()),
//                 http_status_code: 400,
//             };
//             Err(m)
//         }
//     }
// }

// #[openapi(tag = "POST")]
// #[post("/<id>", data = "<post_data>")]
// async fn update(
//     conn: Connection<'_, Db>,
//     id: i32, 
//     post_data: DataResult<'_, Bird>,
// ) -> R<Option<String>> {
//     let db = conn.into_inner();

//     let form = post_data?.into_inner();

//     let cmd = Mutation::update_post_by_id(db, id, form);
//     match cmd.await {
//         Ok(_) => Ok(Json(Some("Post successfully updated.".to_string()))),
//         Err(e) => {
//             let m = error::Error {
//                 err: "Could not update post".to_string(),
//                 msg: Some(e.to_string()),
//                 http_status_code: 400,
//             };
//             Err(m)
//         }
//     }
// }

// #[openapi(tag = "DELETE")]
// #[delete("/<id>")]
// async fn delete(
//     conn: Connection<'_, Db>,
//     id: i32,
// ) -> R<Option<()>> {
// }
