#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
//#![allow(non_snake_case)]
//#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const FIDO_DEBUG: i32 = 0x01;
pub const FIDO_DISABLE_U2F_FALLBACK: i32 = 0x02;

pub const FIDO_ERR_SUCCESS: i32 = 0x00;
pub const FIDO_ERR_INVALID_COMMAND: i32 = 0x01;
pub const FIDO_ERR_INVALID_PARAMETER: i32 = 0x02;
pub const FIDO_ERR_INVALID_LENGTH: i32 = 0x03;
pub const FIDO_ERR_INVALID_SEQ: i32 = 0x04;
pub const FIDO_ERR_TIMEOUT: i32 = 0x05;
pub const FIDO_ERR_CHANNEL_BUSY: i32 = 0x06;
pub const FIDO_ERR_LOCK_REQUIRED: i32 = 0x0a;
pub const FIDO_ERR_INVALID_CHANNEL: i32 = 0x0b;
pub const FIDO_ERR_CBOR_UNEXPECTED_TYPE: i32 = 0x11;
pub const FIDO_ERR_INVALID_CBOR: i32 = 0x12;
pub const FIDO_ERR_MISSING_PARAMETER: i32 = 0x14;
pub const FIDO_ERR_LIMIT_EXCEEDED: i32 = 0x15;
pub const FIDO_ERR_UNSUPPORTED_EXTENSION: i32 = 0x16;
pub const FIDO_ERR_FP_DATABASE_FULL: i32 = 0x17;
pub const FIDO_ERR_LARGEBLOB_STORAGE_FULL: i32 = 0x18;
pub const FIDO_ERR_CREDENTIAL_EXCLUDED: i32 = 0x19;
pub const FIDO_ERR_PROCESSING: i32 = 0x21;
pub const FIDO_ERR_INVALID_CREDENTIAL: i32 = 0x22;
pub const FIDO_ERR_USER_ACTION_PENDING: i32 = 0x23;
pub const FIDO_ERR_OPERATION_PENDING: i32 = 0x24;
pub const FIDO_ERR_NO_OPERATIONS: i32 = 0x25;
pub const FIDO_ERR_UNSUPPORTED_ALGORITHM: i32 = 0x26;
pub const FIDO_ERR_OPERATION_DENIED: i32 = 0x27;
pub const FIDO_ERR_KEY_STORE_FULL: i32 = 0x28;
pub const FIDO_ERR_NOT_BUSY: i32 = 0x29;
pub const FIDO_ERR_NO_OPERATION_PENDING: i32 = 0x2a;
pub const FIDO_ERR_UNSUPPORTED_OPTION: i32 = 0x2b;
pub const FIDO_ERR_INVALID_OPTION: i32 = 0x2c;
pub const FIDO_ERR_KEEPALIVE_CANCEL: i32 = 0x2d;
pub const FIDO_ERR_NO_CREDENTIALS: i32 = 0x2e;
pub const FIDO_ERR_USER_ACTION_TIMEOUT: i32 = 0x2f;
pub const FIDO_ERR_NOT_ALLOWED: i32 = 0x30;
pub const FIDO_ERR_PIN_INVALID: i32 = 0x31;
pub const FIDO_ERR_PIN_BLOCKED: i32 = 0x32;
pub const FIDO_ERR_PIN_AUTH_INVALID: i32 = 0x33;
pub const FIDO_ERR_PIN_AUTH_BLOCKED: i32 = 0x34;
pub const FIDO_ERR_PIN_NOT_SET: i32 = 0x35;
pub const FIDO_ERR_PIN_REQUIRED: i32 = 0x36;
pub const FIDO_ERR_PIN_POLICY_VIOLATION: i32 = 0x37;
pub const FIDO_ERR_PIN_TOKEN_EXPIRED: i32 = 0x38;
pub const FIDO_ERR_REQUEST_TOO_LARGE: i32 = 0x39;
pub const FIDO_ERR_ACTION_TIMEOUT: i32 = 0x3a;
pub const FIDO_ERR_UP_REQUIRED: i32 = 0x3b;
pub const FIDO_ERR_UV_BLOCKED: i32 = 0x3c;
pub const FIDO_ERR_UV_INVALID: i32 = 0x3f;
pub const FIDO_ERR_UNAUTHORIZED_PERM: i32 = 0x40;
pub const FIDO_ERR_ERR_OTHER: i32 = 0x7f;
pub const FIDO_ERR_SPEC_LAST: i32 = 0xdf;

pub const FIDO_OK: i32 = FIDO_ERR_SUCCESS;
pub const FIDO_ERR_TX: i32 = -1;
pub const FIDO_ERR_RX: i32 = -2;
pub const FIDO_ERR_RX_NOT_CBOR: i32 = -3;
pub const FIDO_ERR_RX_INVALID_CBOR: i32 = -4;
pub const FIDO_ERR_INVALID_PARAM: i32 = -5;
pub const FIDO_ERR_INVALID_SIG: i32 = -6;
pub const FIDO_ERR_INVALID_ARGUMENT: i32 = -7;
pub const FIDO_ERR_USER_PRESENCE_REQUIRED: i32 = -8;
pub const FIDO_ERR_INTERNAL: i32 = -9;
pub const FIDO_ERR_NOTFOUND: i32 = -10;
pub const FIDO_ERR_COMPRESS: i32 = -11;
