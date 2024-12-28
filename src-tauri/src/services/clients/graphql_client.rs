use std::collections::HashMap;
use serde_json::json;

#[derive(strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum GraphQLQuery {
    GetProjectId,
    GetTicketId,
    GetFieldId,
    UpdateSpentTime,
}


#[derive(Default)]
pub struct GraphQLClient {
    endpoint: String,
    client: reqwest::Client,
}

impl GraphQLClient {
    pub fn builder() -> GraphQLClientBuilder {
        GraphQLClientBuilder::default()
    }

    pub async fn execute(
        &self,
        query: GraphQLQuery,
        variables: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<String, reqwest::Error> {
        let request_body = json!({
            "query": Self::load_query(&query.to_string()).unwrap(),
            "variables": variables,
        });

        let response = self
            .client
            .post(&self.endpoint)
            .json(&request_body)
            .send()
            .await?;

        response.text().await
    }
    
    fn load_query(query_name: &str) -> Result<String, String> {
        std::fs::read_to_string(format!("src/graphql_queries/{}.graphql", query_name))
            .map_err(|e| format!("Error reading query file {}: {}", query_name, e))
    }
}

#[derive(Default)]
pub struct GraphQLClientBuilder {
    endpoint: String,
    client: reqwest::Client,
    auth_token: Option<String>,
}

impl GraphQLClientBuilder {
    pub fn endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = endpoint.to_string();
        self
    }

    pub fn client(mut self, client: reqwest::Client) -> Self {
        self.client = client;
        self
    }

    pub fn auth_token(mut self, auth_token: &str) -> Self {
        self.auth_token = Some(auth_token.to_string());
        self
    }

    pub fn build(self) -> Result<GraphQLClient, &'static str> {
        Ok(GraphQLClient {
            endpoint: self.endpoint,
            client: self.client,
        })
    }
}
