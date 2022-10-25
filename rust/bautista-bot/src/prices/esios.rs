use thiserror::Error;

const GEOID: i64 = 8741;
const URL: &str = "https://api.esios.ree.es/indicators/1001";

#[derive(Debug, Error)]
pub enum EsiosError {
    #[error("E·sios API call failed: {0}")]
    CallFailed(String),
}
