use odbc_sys::{Handle, HandleType, SQLGetDiagRecW, SqlReturn, WChar};
use std::env;
use std::intrinsics::copy_nonoverlapping;

/// Generate the default connection setting defined for the tests using a connection string
/// of the form 'Driver={};PWD={};USER={};SERVER={};AUTH_SRC={}'.
/// The default driver is 'ADF_ODBC_DRIVER' if not specified.
/// The default auth db is 'admin' if not specified.
pub fn generate_default_connection_str() -> String {
    let user_name = env::var("ADF_TEST_LOCAL_USER").expect("ADF_TEST_LOCAL_USER is not set");
    let password = env::var("ADF_TEST_LOCAL_PWD").expect("ADF_TEST_LOCAL_PWD is not set");
    let host = env::var("ADF_TEST_LOCAL_HOST").expect("ADF_TEST_LOCAL_HOST is not set");

    let auth_db = match env::var("ADF_TEST_LOCAL_AUTH_DB") {
        Ok(val) => val,
        Err(_e) => "admin".to_string(), //Default auth db
    };

    let db = env::var("ADF_TEST_LOCAL_DB");
    let driver = match env::var("ADF_TEST_LOCAL_DRIVER") {
        Ok(val) => val,
        Err(_e) => "ADF_ODBC_DRIVER".to_string(), //Default driver name
    };

    let mut connection_string = format!(
        "Driver={};USER={};PWD={};SERVER={};AUTH_SRC={};",
        driver, user_name, password, host, auth_db,
    );

    // If a db is specified add it to the connection string
    match db {
        Ok(val) => connection_string.push_str(&("DATABASE=".to_owned() + &val + ";")),
        Err(_e) => (), // Do nothing
    };

    connection_string
}

// Verifies that the expected SQL State, message text, and native error in the handle match
// the expected input
pub fn print_outcome(function_name: &str, sql_return: SqlReturn) {
    let outcome = match sql_return {
        SqlReturn::SUCCESS => "SUCCESS",
        SqlReturn::ERROR => "ERROR",
        SqlReturn::SUCCESS_WITH_INFO => "SUCCESS_WITH_INFO",
        SqlReturn::INVALID_HANDLE => "INVALID_HANDLE",
        SqlReturn::NEED_DATA => "NEED_DATA",
        SqlReturn::NO_DATA => "NO_DATA",
        SqlReturn::PARAM_DATA_AVAILABLE => "PARAM_DATA_AVAILABLE",
        SqlReturn::STILL_EXECUTING => "STILL_EXECUTING",
        _ => "unknown sql_return",
    };
    println!("{} SQLReturn = {}", function_name, outcome);
}

// Verifies that the expected SQL State, message text, and native error in the handle match
// the expected input
pub fn get_sql_diagnostics(handle_type: HandleType, handle: Handle) -> String {
    let text_length_ptr = &mut 0;
    let actual_sql_state = &mut [0u16; 6] as *mut _;
    let actual_message_text = &mut [0u16; 512] as *mut _;
    let actual_native_error = &mut 0;
    unsafe {
        let _ = SQLGetDiagRecW(
            handle_type,
            handle as *mut _,
            1,
            actual_sql_state,
            actual_native_error,
            actual_message_text,
            1024,
            text_length_ptr,
        );
    };
    dbg!(*actual_native_error);
    unsafe {
        wtext_to_string(
            actual_message_text,
            *text_length_ptr as usize
        )
    }
}

pub unsafe fn wtext_to_string(text: *const WChar, len: usize) -> String {
    if (len as isize) < 0 {
        let mut dst = Vec::new();
        let mut itr = text;
        {
            while *itr != 0 {
                dst.push(*itr);
                itr = itr.offset(1);
            }
        }
        return String::from_utf16_lossy(&dst);
    }

    let mut dst = Vec::with_capacity(len);
    dst.set_len(len);
    copy_nonoverlapping(text, dst.as_mut_ptr(), len);
    String::from_utf16_lossy(&dst)
}
