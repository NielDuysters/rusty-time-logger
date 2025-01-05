use crate::config::GITHUB_PROJECT_API_URL;
use super::{graphql_client::{GraphQLClient, GraphQLQuery}, pm_client::PmClient};
use serde_json::json;
use std::collections::HashMap;

pub enum GitHubProjectOwner {
    Organization(String),
    User(String),
}

impl Default for GitHubProjectOwner {
    fn default() -> Self {
        GitHubProjectOwner::Organization("".to_string())
    }
}

pub struct GitHubClient {
    graphql_client: GraphQLClient,
    project_owner: GitHubProjectOwner,
    project_number: u8,
    spent_time_field_name: String,
}

impl GitHubClient {
    pub fn builder() -> GitHubClientBuilder {
        GitHubClientBuilder::default()
    }

    async fn get_project_id(&self) -> Result<String, String> {
        let mut variables = HashMap::new();
        let (query, owner_field) = match self.project_owner {
            GitHubProjectOwner::Organization(ref organization) => {
                variables.insert("organization".to_string(), json!(organization));
                (GraphQLQuery::GetProjectIdByOrganization, "organization")
            }
            GitHubProjectOwner::User(ref user) => {
                variables.insert("user".to_string(), json!(user));
                (GraphQLQuery::GetProjectIdByUser, "user")
            }
        };
        variables.insert("projectNumber".to_string(), json!(self.project_number));

        match self.graphql_client.execute(query, Some(variables)).await {
            Ok(response) => {
                let response_json: serde_json::Value = serde_json::from_str(&response)
                    .map_err(|e| format!("Error parsing response: {}", e))?;
                let project_id = response_json
                    .get("data")
                    .and_then(|data| data.get(owner_field))
                    .and_then(|po| po.get("projectV2"))
                    .and_then(|proj| proj.get("id"))
                    .and_then(|id| id.as_str())
                    .map(|id| id.to_string())
                    .ok_or("Project ID not found in response")?;

                Ok(project_id)
            }
            Err(err) => Err(format!("Error executing GraphQL query: {}", err)),
        }
    }

    async fn get_ticket_id(
        &self,
        project_id: &str,
        ticket_number: u64,
    ) -> Result<String, String> {
        let mut variables = HashMap::new();
        variables.insert("projectId".to_string(), json!(project_id));

        match self.graphql_client.execute(GraphQLQuery::GetTicketId, Some(variables)).await {
            Ok(response) => {
                let response_json: serde_json::Value = serde_json::from_str(&response)
                    .map_err(|e| format!("Error parsing response: {}", e))?;

                let ticket_id = response_json
                    .get("data")
                    .and_then(|data| data.get("node"))
                    .and_then(|node| node.get("items"))
                    .and_then(|items| items.get("nodes"))
                    .and_then(|nodes| {
                        nodes.as_array()?.iter().find(|ticket| {
                            ticket.get("content")
                                .and_then(|content| content.get("number"))
                                .and_then(|number| number.as_u64())
                                .map(|num| num == ticket_number)
                                .unwrap_or(false)
                        })
                    })
                    .and_then(|ticket| ticket.get("id"))
                    .and_then(|id| id.as_str())
                    .map(|id| id.to_string())
                    .ok_or("Ticket ID not found in response")?;

                Ok(ticket_id)
            }
            Err(err) => Err(format!("Error executing GraphQL query: {}", err)),
        }
    }
    
    async fn get_field_id(
        &self,
        project_id: &str,
        field_name: &str,
    ) -> Result<String, String> {
        let mut variables = HashMap::new();
        variables.insert("projectId".to_string(), json!(project_id));

        match self.graphql_client.execute(GraphQLQuery::GetFieldId, Some(variables)).await {
            Ok(response) => {
                let response_json: serde_json::Value = serde_json::from_str(&response)
                    .map_err(|e| format!("Error parsing response: {}", e))?;

                let field_id = response_json
                    .get("data")
                    .and_then(|data| data.get("node"))
                    .and_then(|node| node.get("fields"))
                    .and_then(|fields| fields.get("nodes"))
                    .and_then(|nodes| {
                        nodes.as_array()?.iter().find(|field| {
                            field.get("name")
                                .and_then(|name| name.as_str())
                                .map(|name| name.eq_ignore_ascii_case(field_name))
                                .unwrap_or(false)
                        })
                    })
                    .and_then(|field| field.get("id"))
                    .and_then(|id| id.as_str())
                    .map(|id| id.to_string())
                    .ok_or("Field ID not found in response")?;

                Ok(field_id)
            }
            Err(err) => Err(format!("Error executing GraphQL query: {}", err)),
        }
    }
}

impl PmClient for GitHubClient {
    async fn set_spent_time(&self, ticket_id: &str, time: &str) -> Result<(), String> {
        let project_id = self.get_project_id().await.unwrap();
        let mut variables = HashMap::new();
        variables.insert("projectId".to_string(), json!(project_id));
        variables.insert("itemId".to_string(), json!(self.get_ticket_id(&project_id, ticket_id.parse().unwrap()).await.unwrap()));
        variables.insert("fieldId".to_string(), json!(self.get_field_id(&project_id,  &self.spent_time_field_name).await.unwrap()));
        variables.insert("value".to_string(), json!(time));
        
        match self.graphql_client.execute(GraphQLQuery::UpdateSpentTime, Some(variables)).await {
            Ok(_) => {
                Ok(())
            }
            Err(err) => Err(format!("Error executing GraphQL query: {}", err)),
        }
    }
}

#[derive(Default)]
pub struct GitHubClientBuilder {
    project_owner: GitHubProjectOwner,
    project_number: u8,
    spent_time_field_name: String,
    auth_token: Option<String>,
}

impl GitHubClientBuilder {
    fn graphql_client(&mut self) -> GraphQLClient {
        GraphQLClient::builder()
        .endpoint(&GITHUB_PROJECT_API_URL.to_string())
        .client(
            reqwest::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(reqwest::header::USER_AGENT, "rustytimelogger".parse().unwrap());
                headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", self.auth_token.clone().unwrap()).parse().unwrap());
                headers
            })
            .build()
            .unwrap()            
        )
        .auth_token(self.auth_token.as_ref().unwrap())
        .build().unwrap()
    }

    pub fn organization(mut self, organization: &str) -> Self {
        self.project_owner = GitHubProjectOwner::Organization(organization.to_string());
        self
    }

    pub fn user(mut self, user: &str) -> Self {
        self.project_owner = GitHubProjectOwner::User(user.to_string());
        self
    }

    pub fn project_number(mut self, project_number: u8) -> Self {
        self.project_number = project_number;
        self
    }

    pub fn auth_token(mut self, auth_token: &str) -> Self {
        self.auth_token = Some(auth_token.to_string());
        self
    }

    pub fn spent_time_field_name(mut self, spent_time_field_name: &str) -> Self {
        self.spent_time_field_name = spent_time_field_name.to_string();
        self
    }

    pub fn build(mut self) -> Result<GitHubClient, &'static str> {
        Ok(GitHubClient {
            graphql_client: self.graphql_client(),
            project_owner: self.project_owner,
            project_number: self.project_number,
            spent_time_field_name: self.spent_time_field_name,
        })
    }
}

