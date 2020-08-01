use rusoto_core::Region;
use rusoto_credential::AwsCredentials;
use warp::Filter;
use crate::handlers;

/// Returns the filter for the `signed_urls` creation endpoint.
///
/// # Arguments
///
/// * `bucket_name` - The name of the S3 bucket to use as a `String`.
/// * `region` - The `rusoto_core::Region` to use.
/// * `credentials` - The `rusoto_credential::AwsCredentials` to use for signing.
///
/// # Examples
///
/// ```
/// let region = "ap-southeast-2".parse().expect("Region must be valid.");
/// let creds = StaticProvider::new_minimal("test", "test").credentials().await.unwrap();
/// let filter = crate::filters::signed_urls("test", region, credentials);
///
/// warp::serve(filter).run(([0, 0, 0, 0], 8000)).await;
/// ```
///
/// # Remarks
///
/// Path is defined as `/api/files/signed_urls`.
/// Method = "POST"
pub fn signed_urls(bucket_name: String, region: Region, credentials: AwsCredentials)
	-> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("api" / "files" / "signed_urls")
		.and(warp::post())
		.and_then(move || handlers::create_signed_url(bucket_name.to_owned(), region.clone(), credentials.clone()))
}
