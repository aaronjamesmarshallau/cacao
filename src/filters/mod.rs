use rusoto_core::Region;
use rusoto_credential::AwsCredentials;
use warp::Filter;
use crate::handlers;

pub fn signed_urls(bucket_name: String, region: Region, credentials: AwsCredentials)
	-> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("api" / "files" / "signed_urls")
		.and(warp::post())
		.and_then(move || handlers::create_signed_url(bucket_name.to_owned(), region.clone(), credentials.clone()))
}
