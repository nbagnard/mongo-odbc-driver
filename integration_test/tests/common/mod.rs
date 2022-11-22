use std::env;
use std::intrinsics::copy_nonoverlapping;
use std::ptr::null_mut;
use odbc_sys::{AttrConnectionPooling, AttrOdbcVersion, EnvironmentAttribute, Handle, HandleType, HEnv, SQLAllocHandle, SqlReturn, SQLSetEnvAttr, WChar};

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

/// Prints the content of a given char buffer.
pub fn print_text(label: &str, txt_len: isize, text: *mut WChar)
{
    unsafe {
        if txt_len < 0 {
            let mut dst = Vec::new();
            let mut itr = text;
            {
                while *itr != 0 {
                    dst.push(*itr);
                    itr = itr.offset(1);
                }
            }
            println!("{} = {}",label, String::from_utf16_lossy(&dst));
            return
        }

        let mut dst = Vec::with_capacity(txt_len as usize);
        dst.set_len(txt_len as usize);
        copy_nonoverlapping(text, dst.as_mut_ptr(), txt_len as usize);

        println!("{} = {}",label, String::from_utf16_lossy(&dst));
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
        let allocEnvHandle = SQLAllocHandle(HandleType::Env, null_mut(), &mut env as *mut Handle);
        dbg!(allocEnvHandle);
        assert_eq!(
            SqlReturn::SUCCESS,
            allocEnvHandle
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

