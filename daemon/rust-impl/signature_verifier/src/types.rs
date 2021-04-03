use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Payload {
	pub timestamp: f64,
	pub endpoint: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub endpoint: String,
}

#[derive(Serialize, Deserialize)]
pub struct ValidationResult<'a> {
	pub body: &'a str,
	pub status: i32,
}
