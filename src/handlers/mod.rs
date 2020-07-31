use crate::responses::PreSignedUrlResponse;
use rusoto_core::Region;
use rusoto_credential::{AwsCredentials};
use rusoto_s3::PutObjectRequest;
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use std::convert::Infallible;
use std::time::Duration;

const URL_EXPIRATION_MINUTES: u64 = 5;

pub async fn create_signed_url(bucket_name: String, region: Region, credentials: AwsCredentials) -> Result<impl warp::Reply, Infallible> {
	let req = PutObjectRequest {
		bucket: bucket_name.to_owned(),
		key: "testfile".to_owned(),
		..Default::default()
	};

	let presign_options = PreSignedRequestOption {
		expires_in: Duration::from_secs(URL_EXPIRATION_MINUTES * 60),
	};

	let presigned_url = req.get_presigned_url(&region, &credentials, &presign_options);
	let json_reply = warp::reply::json(&PreSignedUrlResponse {
		url: presigned_url,
	});

	let result = warp::reply::with_status(json_reply, warp::http::StatusCode::OK);

	Ok(result)
}
