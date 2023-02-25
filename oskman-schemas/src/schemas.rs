#[derive(serde::Serialize, serde::Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct CustomResponse {
  pub message: String,
  pub other_val: usize,
}
