use crate::responses::PreSignedUrlResponse;
use rusoto_core::Region;
use rusoto_credential::{AwsCredentials};
use rusoto_s3::PutObjectRequest;
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use std::convert::Infallible;
use std::time::Duration;

/// The expiration of our pre-signed urls in minutes.
/// This is used in conjunction with Duration::from_secs(u64).
const URL_EXPIRATION_MINUTES: u64 = 5;

/// Creates a signed URL from the given `bucket_name`, `region`, and `credentials`.
/// Returns a `Result` of `warp::Reply`.
///
/// # Examples
///
/// ```
/// let region = "ap-southeast-2".parse().expect("Region must be valid.");
/// let creds = StaticProvider::new_minimial("test", "test").credentials().await.unwrap();
/// let filter = warp::path!("/my/path")
/// 	.and(warp::post())
/// 	.and_then(handlers::create_signed_url("my_bucket_name", region, creds));
///
/// warp::serve(filter).run(([0.0.0.0], 3000)).await;
/// ```
///
/// # Remarks
/// This method merely leverages rusoto_s3 to generate a URL, and no
/// external requests are made. It's not feasible that this method could
/// error, thus declared as infallible.
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
