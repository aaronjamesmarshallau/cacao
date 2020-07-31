use serde::Serialize;

/// A response used for transmitting pre-signed URLs.
#[derive(Serialize)]
pub struct PreSignedUrlResponse {
	/// The pre-signed URL to return.
	pub url: String,
}
