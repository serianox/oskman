#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Schemas {
    pub fido_device_list: FidoDeviceList,
    pub fido_get_info_command: FidoGetInfoCommand,
    pub fido_get_info_response: FidoGetInfoResponse,
    pub fido_set_pin_command: FidoSetPinCommand,
    pub fido_set_pin_response: FidoSetPinResponse,
    pub fido_change_pin_command: FidoChangePinCommand,
    pub fido_change_pin_response: FidoChangePinResponse,
    pub fido_reset_command: FidoResetCommand,
    pub fido_reset_response: FidoResetResponse,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct FidoDeviceList {
    pub dev: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct FidoGetInfoCommand {
    pub dev: String,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct FidoGetInfoResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,
    pub aaguid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<FidoGetInfoOptions>,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FidoGetInfoOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rk: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_pin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uv: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FidoSetPinCommand {
    pub dev: String,
    pub new_pin: String,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct FidoSetPinResponse {
    pub result: bool,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FidoChangePinCommand {
    pub dev: String,
    pub new_pin: String,
    pub old_pin: String,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct FidoChangePinResponse {
    pub result: bool,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct FidoResetCommand {
    pub dev: String,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct FidoResetResponse {
    pub result: bool,
}
