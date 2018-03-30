use super::error::Error;

#[derive(Serialize)]
pub struct Meta{
    pub success: bool,
    pub error: Option<Error>
}