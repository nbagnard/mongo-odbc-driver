#![allow(dead_code)]
use std::env;
use std::ptr::null_mut;
use odbc_sys::{Handle, HandleType, WChar, SQLGetDiagRecW, SQLAllocHandle, SqlReturn, SQLSetEnvAttr, HEnv, EnvironmentAttribute, AttrOdbcVersion, AttrConnectionPooling};

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
pub fn verify_sql_diagnostics(
    handle_type: HandleType,
    handle: Handle,
    record_number: i16,
    expected_sql_state: &str,
    expected_message_text: &str,
    mut expected_native_err: i32,
) {
    let text_length_ptr = &mut 0;
    let actual_sql_state = &mut [0u16; 6] as *mut _;
    let actual_message_text = &mut [0u16; 512] as *mut _;
    let actual_native_error = &mut 0;
    unsafe {
        let _ = SQLGetDiagRecW(
            handle_type,
            handle as *mut _,
            record_number,
            actual_sql_state,
            actual_native_error,
            actual_message_text,
            1024,
            text_length_ptr,
        );

        print_text("error message", *text_length_ptr as usize, actual_message_text);
    };
    let mut expected_sql_state_encoded: Vec<u16> = expected_sql_state.encode_utf16().collect();
    expected_sql_state_encoded.push(0);
    let actual_message_length = *text_length_ptr as usize;
    unsafe {
        assert_eq!(
            expected_message_text,
            &(String::from_utf16_lossy(&*(actual_message_text as *const [u16; 256])))
                [0..actual_message_length],
        );
        assert_eq!(
            String::from_utf16(&*(expected_sql_state_encoded.as_ptr() as *const [u16; 6])).unwrap(),
            String::from_utf16(&*(actual_sql_state as *const [u16; 6])).unwrap()
        );
    }
    assert_eq!(&mut expected_native_err as &mut i32, actual_native_error);
}

// Verifies that the expected SQL State, message text, and native error in the handle match
// the expected input
pub fn print_sql_diagnostics(
    handle_type: HandleType,
    handle: Handle,
) {
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
    print_text("error message", *text_length_ptr as usize, actual_message_text);
}

// Verifies that the expected SQL State, message text, and native error in the handle match
// the expected input
pub fn print_outcome(
    function_name: &str,
    sql_return: SqlReturn,
) {

    let outcome = match sql_return {
        SqlReturn::SUCCESS => {"SUCCESS"},
        SqlReturn::ERROR => {"ERROR"},
        SqlReturn::SUCCESS_WITH_INFO => {"SUCCESS_WITH_INFO"},
        SqlReturn::INVALID_HANDLE => {"INVALID_HANDLE"},
        SqlReturn::NEED_DATA  => {"NEED_DATA"},
        SqlReturn::NO_DATA  => {"NO_DATA"},
        SqlReturn::PARAM_DATA_AVAILABLE => {"PARAM_DATA_AVAILABLE"},
        SqlReturn::STILL_EXECUTING => {"STILL_EXECUTING"},
        _ => {"unknown sql_return"},

    };
    println!("{} SQLReturn = {}", function_name, outcome);
}


pub fn print_text(label: &str, txt_len: usize, text: *mut WChar)
{
    unsafe {
        //println!("text_length = {}", *text_length_ptr);

        //let txt_len = *text_length_ptr as usize;
        let error_message = &(String::from_utf16_lossy( & * (text as * const [u16; 256])))[0..txt_len];
        println!("{} = {}",label, error_message);
    }
}


/// Setup flow.
/// This will allocate a new environment handle and set ODBC_VERSION and CONNECTION_POOLING environment attributes.
pub fn setup() -> odbc_sys::HEnv {
    /*
        Setup flow :
            SQLAllocHandle(SQL_HANDLE_ENV)
            SQLSetEnvAttr(SQL_ATTR_ODBC_VERSION, SQL_OV_ODBC3)
            SQLSetEnvAttr(SQL_ATTR_CONNECTION_POOLING, SQL_CP_ONE_PER_HENV)
    */

    let mut env: Handle = null_mut();

    unsafe {
        let alloc_env_handle = SQLAllocHandle(HandleType::Env, null_mut(), &mut env as *mut Handle);
        dbg!(&alloc_env_handle);
        assert_eq!(
            SqlReturn::SUCCESS,
            alloc_env_handle
        );

        assert_eq!(
            SqlReturn::SUCCESS,
            SQLSetEnvAttr(
                env as HEnv,
                EnvironmentAttribute::OdbcVersion,
                AttrOdbcVersion::Odbc3.into(),
                0,
            )
        );

        assert_eq!(
            SqlReturn::SUCCESS,
            SQLSetEnvAttr(
                env as HEnv,
                EnvironmentAttribute::ConnectionPooling,
                AttrConnectionPooling::OnePerHenv.into(),
                0,
            )
        );
    }

    env as HEnv
}
