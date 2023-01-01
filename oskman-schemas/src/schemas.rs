#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct CustomResponse {
  pub message: String,
  pub other_val: usize,
}
