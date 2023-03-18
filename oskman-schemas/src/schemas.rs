#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Schemas {
  pub fido_reset_command: FidoResetCommand,
  pub fido_reset_response: FidoResetResponse,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct FidoResetCommand {
  pub dev: String,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct FidoResetResponse {
  pub ret: i32,
  pub message: String,
}
