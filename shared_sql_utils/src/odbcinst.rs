use constants::DRIVER_NAME;
use cstr::{input_text_to_string_w, to_widechar_ptr, Char, WideChar};
use thiserror::Error;

const ODBCINSTINI: &str = "ODBCINST.INI";

// The maximum length of a registry value is 16383 characters, but this seems like an overly large max.
// iODBC and Postgres largest buffer for retrieving registry values is 4096. This seems realistically enough.
pub(crate) const MAX_VALUE_LENGTH: usize = 4096;

#[derive(Error, Debug, Clone)]
pub enum SettingError {
    #[error("Invalid DSN: {}\nDSN may not be longer than 32 characters, and may not contain any of the following characters: [ ] {{ }} ( ) , ; ? * = ! @ \\", .0)]
    Dsn(String),
    #[error(
        "The maximum length of an allowed registry value is {} characters.",
        MAX_VALUE_LENGTH
    )]
    Value,
    #[error("{}", .0)]
    Generic(String),
}

// The setting used to set the driver log level
pub(crate) const LOGLEVEL: &str = "LogLevel";
// The setting used to set the driver path
const DRIVER: &str = "Driver";

#[cfg_attr(target_os = "linux", link(name = "odbcinst", kind = "dylib"))]
#[cfg_attr(target_os = "macos", link(name = "iodbcinst", kind = "dylib"))]
#[cfg_attr(target_os = "windows", link(name = "odbccp32", kind = "raw-dylib"))]
extern "C" {
    pub fn SQLValidDSNW(dsn: *const WideChar) -> bool;
    pub fn SQLWriteDSNToIniW(dsn: *const WideChar, driver: *const WideChar) -> bool;
    pub fn SQLWritePrivateProfileStringW(
        section: *const WideChar,
        entry: *const WideChar,
        string: *const WideChar,
        filename: *const WideChar,
    ) -> bool;
    pub fn SQLRemoveDSNFromIniW(dsn: *const WideChar) -> bool;
    pub fn SQLGetPrivateProfileStringW(
        section: *const WideChar,
        entry: *const WideChar,
        default: *const WideChar,
        buffer: *mut WideChar,
        buffer_size: i32,
        filename: *const WideChar,
    ) -> i32;
    pub fn SQLGetPrivateProfileString(
        section: *const Char,
        entry: *const Char,
        default: *const Char,
        buffer: *mut Char,
        buffer_size: i32,
        filename: *const Char,
    ) -> i32;
    pub fn SQLGetConfigMode(buffer: *mut u32) -> i32;
}

///
/// Reads the value of the specified key under the specified section from the ini file.
/// Note : On Windows INI files are actually registry entries.
/// Return the value for the key or an error if it is too long.
pub(crate) fn read_value_for_key(
    section: &str,
    key: &str,
    ini_file_name: &str,
) -> Result<Option<String>, SettingError> {
    let mut buffer: Vec<WideChar> = Vec::with_capacity(MAX_VALUE_LENGTH);
    let len = unsafe {
        SQLGetPrivateProfileStringW(
            to_widechar_ptr(section).0,
            to_widechar_ptr(key).0,
            to_widechar_ptr("").0,
            buffer.as_mut_ptr(),
            buffer.len() as i32,
            to_widechar_ptr(ini_file_name).0,
        )
    };

    if len > MAX_VALUE_LENGTH as i32 {
        return Err(SettingError::Generic(format!("If you see this error, please report it. Attempted to read a value from registry that was over {MAX_VALUE_LENGTH} characters for key: `{key}`.")));
    } else if len < 1 {
        // Key not found
        eprintln!("-- Key -- {}.{} not found", section, key);

        return Ok(None);
    }
    let val =
    unsafe {
        input_text_to_string_w(buffer.as_mut_ptr(), len as usize)
    };
    eprintln!("-- Looking for key -- {}.{} = {}", section, key, val);
    Ok(Some(val))
}

/// Get the driver path defined at driver level in ODBCINST.INI.
/// If there is no log level defined, return None.
pub fn get_driver_path() -> Result<Option<String>, SettingError> {
    read_value_for_key(DRIVER_NAME, DRIVER, ODBCINSTINI)
}

/// Get the log level defined at driver level in ODBCINST.INI.
/// If there is no log level defined, return None.
pub fn get_driver_log_level() -> Result<Option<String>, SettingError> {
    read_value_for_key(DRIVER_NAME, LOGLEVEL, ODBCINSTINI)
}
