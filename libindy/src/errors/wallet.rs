extern crate indy_crypto;

use std::error;
use std::error::Error;
use std::io;
use std::fmt;
use std::string::FromUtf8Error;
use libsqlite3_sys;
use std::ffi::NulError;
use std::str::Utf8Error;

use rusqlite;
use serde_json;
use base64;

use api::ErrorCode;
use errors::common::CommonError;
use errors::ToErrorCode;


#[derive(Debug)]
pub enum WalletError {
    InvalidHandle(String),
    UnknownType(String),
    TypeAlreadyRegistered(String),
    AlreadyExists(String),
    NotFound(String),
    IncorrectPool(String),
    PluggedWalletError(ErrorCode),
    AlreadyOpened(String),
    AccessFailed(String),
    CommonError(CommonError),
    InputError(String),
    EncodingError(String),
    StorageError(String),
    EncryptionError(String),
    ItemNotFound,
    ItemAlreadyExists,
    QueryError(String),
    ImportError(String),
}


impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WalletError::InvalidHandle(ref description) => write!(f, "Invalid wallet handle was passed: {}", description),
            WalletError::UnknownType(ref description) => write!(f, "Unknown wallet type: {}", description),
            WalletError::TypeAlreadyRegistered(ref description) => write!(f, "Wallet type already registered: {}", description),
            WalletError::AlreadyExists(ref description) => write!(f, "Wallet with this name already exists: {}", description),
            WalletError::NotFound(ref description) => write!(f, "Wallet not found: {}", description),
            WalletError::IncorrectPool(ref description) => write!(f, "Wallet used with different pool: {}", description),
            WalletError::PluggedWalletError(err_code) => write!(f, "Plugged wallet error: {}", err_code as i32),
            WalletError::AlreadyOpened(ref description) => write!(f, "Wallet already opened: {}", description),
            WalletError::AccessFailed(ref description) => write!(f, "Wallet security error: {}", description),
            WalletError::CommonError(ref err) => err.fmt(f),
            WalletError::InputError(ref description) => write!(f, "Wallet input error: {}", description),
            WalletError::EncodingError(ref description) => write!(f, "Wallet encoding error: {}", description),
            WalletError::StorageError(ref description) => write!(f, "Wallet storage error occurred. Description: {}", description),
            WalletError::EncryptionError(ref description) => write!(f, "Wallet encryption error occurred. Description: {}", description),
            WalletError::ItemNotFound => write!(f, "Item not found"),
            WalletError::ItemAlreadyExists => write!(f, "Item already exists"),
            WalletError::QueryError(ref description) => write!(f, "{}", description),
            WalletError::ImportError(ref description) => write!(f, "Wallet import error: {}", description),
        }
    }
}

impl error::Error for WalletError {
    fn description(&self) -> &str {
        match *self {
            WalletError::InvalidHandle(ref description) => description,
            WalletError::UnknownType(ref description) => description,
            WalletError::TypeAlreadyRegistered(ref description) => description,
            WalletError::AlreadyExists(ref description) => description,
            WalletError::NotFound(ref description) => description,
            WalletError::IncorrectPool(ref description) => description,
            WalletError::PluggedWalletError(_) => "Plugged wallet error",
            WalletError::AlreadyOpened(ref description) => description,
            WalletError::AccessFailed(ref description) => description,
            WalletError::CommonError(ref err) => err.description(),
            WalletError::InputError(ref description) => description,
            WalletError::EncodingError(ref description) => description,
            WalletError::StorageError(ref description) => description,
            WalletError::EncryptionError(ref description) => description,
            WalletError::ItemNotFound => "Item not found",
            WalletError::ItemAlreadyExists => "Item already exists",
            WalletError::QueryError(ref description) => description,
            WalletError::ImportError(ref description) => description,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WalletError::InvalidHandle(_) => None,
            WalletError::UnknownType(_) => None,
            WalletError::TypeAlreadyRegistered(_) => None,
            WalletError::AlreadyExists(_) => None,
            WalletError::NotFound(_) => None,
            WalletError::IncorrectPool(_) => None,
            WalletError::PluggedWalletError(_) => None,
            WalletError::AlreadyOpened(_) => None,
            WalletError::AccessFailed(_) => None,
            WalletError::CommonError(ref err) => Some(err),
            WalletError::InputError(_) => None,
            WalletError::EncodingError(_) => None,
            WalletError::StorageError(_) => None,
            WalletError::EncryptionError(_) => None,
            WalletError::ItemNotFound => None,
            WalletError::ItemAlreadyExists => None,
            WalletError::QueryError(_) => None,
            WalletError::ImportError(_) => None,
        }
    }
}

impl ToErrorCode for WalletError {
    fn to_error_code(&self) -> ErrorCode {
        match *self {
            WalletError::InvalidHandle(_) => ErrorCode::WalletInvalidHandle,
            WalletError::UnknownType(_) => ErrorCode::WalletUnknownTypeError,
            WalletError::TypeAlreadyRegistered(_) => ErrorCode::WalletTypeAlreadyRegisteredError,
            WalletError::AlreadyExists(_) => ErrorCode::WalletAlreadyExistsError,
            WalletError::NotFound(_) => ErrorCode::WalletNotFoundError,
            WalletError::IncorrectPool(_) => ErrorCode::WalletIncompatiblePoolError,
            WalletError::PluggedWalletError(err_code) => err_code,
            WalletError::AlreadyOpened(_) => ErrorCode::WalletAlreadyOpenedError,
            WalletError::AccessFailed(_) => ErrorCode::WalletAccessFailed,
            WalletError::CommonError(ref err) => err.to_error_code(),
            WalletError::InputError(_) => ErrorCode::WalletInputError,
            WalletError::EncodingError(_) => ErrorCode::WalletDecodingError,
            WalletError::StorageError(_) => ErrorCode::WalletStorageError,
            WalletError::EncryptionError(_) => ErrorCode::WalletEncryptonError,
            WalletError::ItemNotFound => ErrorCode::WalletItemNotFound,
            WalletError::ItemAlreadyExists => ErrorCode::WalletItemAlreadyExists,
            WalletError::QueryError(_) => ErrorCode::WalletQueryError,
            WalletError::ImportError(_) => ErrorCode::WalletImportError,
        }
    }
}

impl From<io::Error> for WalletError {
    fn from(err: io::Error) -> WalletError {
        WalletError::CommonError(CommonError::IOError(err))
    }
}

impl From<indy_crypto::errors::IndyCryptoError> for WalletError {
    fn from(err: indy_crypto::errors::IndyCryptoError) -> Self {
        WalletError::CommonError(CommonError::from(err))
    }
}


impl From<WalletStorageError> for WalletError {
    fn from(err: WalletStorageError) -> Self {
        match err {
            WalletStorageError::AlreadyExists => WalletError::AlreadyExists(String::from("Storage already exists")),
            WalletStorageError::NotFound => WalletError::NotFound(String::from("Storage not found")),
            WalletStorageError::ItemNotFound => WalletError::ItemNotFound,
            WalletStorageError::ItemAlreadyExists => WalletError::ItemAlreadyExists,
            WalletStorageError::PluggedStorageError(code) => WalletError::PluggedWalletError(code),
            _ => WalletError::StorageError(err.description().to_string())
        }
    }
}

impl From<WalletQueryError> for WalletError {
    fn from(err: WalletQueryError) -> Self {
        WalletError::QueryError(format!("Invalid wallet query: {}", err.description()))
    }
}

impl From<FromUtf8Error> for WalletError {
    fn from(err: FromUtf8Error) -> Self {
        WalletError::EncodingError(format!("Failed to decode input into utf8: {}", err.description()))
    }
}

impl From<base64::DecodeError> for WalletError {
    fn from(err: base64::DecodeError) -> Self {
        WalletError::EncodingError(format!("Failed to decode input into base64: {}", err.description()))
    }
}

impl From<serde_json::Error> for WalletError {
    fn from(err: serde_json::Error) -> Self {
        WalletError::EncodingError(format!("Failed to decode input into json: {}", err.description()))
    }
}


#[derive(Debug)]
pub enum WalletStorageError {
    AlreadyExists,
    NotFound,
    ConfigError,
    ItemNotFound,
    ItemAlreadyExists,
    IOError(String),
    PluggedStorageError(ErrorCode),
    CommonError(CommonError)
}


impl From<rusqlite::Error> for WalletStorageError {
    fn from(err: rusqlite::Error) -> WalletStorageError {
        match &err {
            &rusqlite::Error::SqliteFailure(libsqlite3_sys::Error{code: libsqlite3_sys::ErrorCode::ConstraintViolation, extended_code: _}, _) => WalletStorageError::ItemAlreadyExists,
            _ => WalletStorageError::IOError(format!("IO error during storage operation: {}", err.description()))
        }
    }
}

impl From<io::Error> for WalletStorageError {
    fn from(err: io::Error) -> WalletStorageError {
        WalletStorageError::IOError(err.description().to_string())
    }
}

impl From<serde_json::Error> for WalletStorageError {
    fn from(_err: serde_json::Error) -> WalletStorageError {
        WalletStorageError::ConfigError
    }
}

impl From<NulError> for WalletStorageError {
    fn from(err: NulError) -> WalletStorageError { WalletStorageError::IOError(err.description().to_owned()) }
}

impl From<Utf8Error> for WalletStorageError {
    fn from(err: Utf8Error) -> WalletStorageError { WalletStorageError::IOError(err.description().to_owned()) }
}

impl From<base64::DecodeError> for WalletStorageError {
    fn from(err: base64::DecodeError) -> WalletStorageError { WalletStorageError::IOError(err.description().to_owned()) }
}

impl From<CommonError> for WalletStorageError {
    fn from(err: CommonError) -> WalletStorageError {WalletStorageError::CommonError(err)}
}

impl error::Error for WalletStorageError {
    fn description(&self) -> &str {
        match *self {
            WalletStorageError::AlreadyExists => "Storage already created",
            WalletStorageError::NotFound => "Storage not found",
            WalletStorageError::ConfigError => "Storage configuration is invalid",
            WalletStorageError::ItemNotFound => "Item not found",
            WalletStorageError::ItemAlreadyExists => "Item already exists",
            WalletStorageError::PluggedStorageError(err_code) => "Plugged storage error",
            WalletStorageError::IOError(ref s) => s,
            WalletStorageError::CommonError(ref e) => e.description(),
        }
    }
}


impl fmt::Display for WalletStorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WalletStorageError::AlreadyExists => write!(f, "Storage already created"),
            WalletStorageError::NotFound => write!(f, "Storage not found"),
            WalletStorageError::ConfigError => write!(f, "Storage configuration is invalid"),
            WalletStorageError::ItemNotFound => write!(f, "Item not found"),
            WalletStorageError::ItemAlreadyExists => write!(f, "Item already exists"),
            WalletStorageError::IOError(ref s) => write!(f, "IO error occurred during storage operation: {}", s),
            WalletStorageError::PluggedStorageError(err_code) => write!(f, "Plugged storage error: {}", err_code as i32),
            WalletStorageError::CommonError(ref e) => write!(f, "Common error: {}", e.description()),
        }
    }
}


#[derive(Debug)]
pub enum WalletQueryError {
    ParsingErr(String),
    StructureErr(String),
    ValueErr(String),
}

impl fmt::Display for WalletQueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WalletQueryError::ParsingErr(ref s) | WalletQueryError::StructureErr(ref s) | WalletQueryError::ValueErr(ref s) => f.write_str(s)
        }
    }
}
impl error::Error for WalletQueryError {
    fn description(&self) -> &str {
        match *self {
            WalletQueryError::ParsingErr(ref s) | WalletQueryError::StructureErr(ref s) | WalletQueryError::ValueErr(ref s) => s,
        }
    }
}

impl From<serde_json::Error> for WalletQueryError {
    fn from(err: serde_json::Error) -> WalletQueryError {
        WalletQueryError::ParsingErr(err.to_string())
    }
}

impl From<CommonError> for WalletError {
    fn from(err: CommonError) -> WalletError {
        WalletError::CommonError(err)
    }
}
