use rusoto_credential::{StaticProvider, ProvideAwsCredentials};
use warp::Filter;

pub mod filters;
pub mod handlers;
pub mod responses;

#[tokio::main(max_threads = 10_000)]
async fn main() {
	let aws_region = dotenv::var("AWS_REGION").expect("AWS_REGION must be set");
	let aws_access_key = dotenv::var("AWS_ACCESS_KEY").expect("AWS_ACCESS_KEY must be set");
	let aws_secret_key = dotenv::var("AWS_SECRET_KEY").expect("AWS_SECRET_KEY must be set");
	let bucket_name = dotenv::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME must be set");

	let credentials = StaticProvider::new_minimal(aws_access_key, aws_secret_key).credentials().await.unwrap();
	let region = aws_region.parse().expect("AWS_REGION must be a valid AWS region");

	let api = filters::signed_urls(bucket_name, region, credentials);

	let routes = api.with(warp::log("signed_urls"));

	warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
