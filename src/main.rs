mod queries;

use graphql_client::{GraphQLQuery, Response};
use reqwest::header;
use std::env;

use crate::queries::{webhook_subscription, webhook_subscriptions, WebhookSubscription, WebhookSubscriptions};

pub const APPLICATION_GRAPHQL: &str = "application/graphql";
pub const SHOPIFY_ACCESS_TOKEN: &str = "X-Shopify-Access-Token";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
	let api_key = env::var("SHOPIFY_API_KEY").unwrap();
	let client = reqwest::Client::new();

	let graphql_endpoint = graphql_endpoint_for(
		"toby-test-store2.myshopify.com".strip_suffix(".myshopify.com").unwrap(),
		"2023-04",
	);

	let id = "gid://shopify/WebhookSubscription/1166233927878".to_string();

	let request = client
		.post(graphql_endpoint)
		.header(header::CONTENT_TYPE, APPLICATION_GRAPHQL)
		.header(SHOPIFY_ACCESS_TOKEN, api_key)
		.json(&WebhookSubscription::build_query(webhook_subscription::Variables {
			id,
		}));

	// let request = client
	// 	.post(graphql_endpoint)
	// 	.header(header::CONTENT_TYPE, APPLICATION_GRAPHQL)
	// 	.header(SHOPIFY_ACCESS_TOKEN, api_key)
	// 	.json(&WebhookSubscriptions::build_query(webhook_subscriptions::Variables {}));

	let raw_response = request.send().await?;

	let response: Response<webhook_subscription::ResponseData> = raw_response.json().await?;
	println!("{:#?}", response);
	Ok(())
}

pub fn graphql_endpoint_for(store: &str, api_version: &str) -> String {
	format!("https://{store}.myshopify.com/admin/api/{api_version}/graphql.json")
}
