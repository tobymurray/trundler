pub mod queries;

pub const APPLICATION_GRAPHQL: &str = "application/graphql";
pub const SHOPIFY_ACCESS_TOKEN: &str = "X-Shopify-Access-Token";

pub fn graphql_endpoint_for(store: &str, api_version: &str) -> String {
	format!("https://{store}.myshopify.com/admin/api/{api_version}/graphql.json")
}

#[cfg(test)]
mod tests {
	use std::env;

	use crate::queries::{webhook_subscription, WebhookSubscription};

	use super::*;
	use graphql_client::{GraphQLQuery, Response};
	use reqwest::header;
	use tokio::test;

	#[test]
	async fn it_works() {
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

		let raw_response = request.send().await.unwrap();

		let response: Response<webhook_subscription::ResponseData> = raw_response.json().await.unwrap();
		println!("{:#?}", response);
	}
}
