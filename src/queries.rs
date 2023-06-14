use graphql_client::GraphQLQuery;

pub type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "2023-04.json",
	query_path = "graphql-queries/webhook_subscriptions.graphql",
	response_derives = "Debug"
)]
pub struct WebhookSubscriptions;
