use rocket_okapi::{rapidoc::{make_rapidoc, RapiDocConfig, GeneralConfig, HideShowConfig}, settings::UrlObject};

#[macro_use] 
extern crate rocket;

mod bird_endpoints;
mod database;
mod error;

#[launch]
pub async fn rocket() -> _ {
    rocket::build()
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("Rocket/SeaOrm - RapiDoc documentation | RapiDoc".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../v1/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
}