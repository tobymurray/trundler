use graphql_client::GraphQLQuery;

pub type DateTime = chrono::DateTime<chrono::Utc>;

// Has to match the generated type's name
#[allow(clippy::upper_case_acronyms)]
pub type URL = String;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "2023-04.json",
	query_path = "graphql-queries/webhooks/subscriptions.graphql",
	response_derives = "Debug"
)]
pub struct WebhookSubscriptions;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "2023-04.json",
	query_path = "graphql-queries/webhooks/get.graphql",
	response_derives = "Debug"
)]
pub struct WebhookSubscription;
