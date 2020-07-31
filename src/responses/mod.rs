use serde::Serialize;

#[derive(Serialize)]
pub struct PreSignedUrlResponse {
	pub url: String,
}
