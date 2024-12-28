pub trait PmClient {
    async fn set_spent_time(&self, ticket_id: &str, time: &str) -> Result<(), String>;
}
