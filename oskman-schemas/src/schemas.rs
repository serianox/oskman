#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Schemas {
    pub fido_device_list: FidoDeviceList,
    pub fido_get_info_command: FidoGetInfoCommand,
    pub fido_get_info_response: FidoGetInfoResponse,
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
    pub aaguid: String,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct FidoResetCommand {
    pub dev: String,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct FidoResetResponse {
    pub result: bool,
}
