use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::{self, JsonSchema};

#[derive(
  Clone, Debug, PartialEq, Eq, Deserialize, Serialize, FromForm, JsonSchema,
)]
#[serde(crate = "rocket::serde")]
pub struct Bird {
  pub id: i32,
  pub name: String,
  pub image_id: i32,
}