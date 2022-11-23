extern crate core;

mod common;

use odbc_sys::*;
use std::ptr::null_mut;

/// Setup flow.
/// This will allocate a new environment handle and set ODBC_VERSION and CONNECTION_POOLING environment attributes.
fn setup() -> odbc_sys::HEnv {
    /*
        Setup flow :
            SQLAllocHandle(SQL_HANDLE_ENV)
            SQLSetEnvAttr(SQL_ATTR_ODBC_VERSION, SQL_OV_ODBC3)
            SQLSetEnvAttr(SQL_ATTR_CONNECTION_POOLING, SQL_CP_ONE_PER_HENV)
    */

    let mut env: Handle = null_mut();

    unsafe {
        assert_eq!(
            SqlReturn::SUCCESS,
            SQLAllocHandle(HandleType::Env, null_mut(), &mut env as *mut Handle)
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

mod integration {
    use super::*;
    use crate::common::{get_sql_diagnostics};
    use odbc::ffi::SQL_NTS;
    /// Test PowerBI Setup flow
    #[test]
    fn test_setup() {
        setup();
    }

    /// Test PowerBi environment clean-up
    #[test]
    fn test_env_cleanup() {
        // We need a handle to be able to test that freeing the handle work
        let env_handle: HEnv = setup();

        unsafe {
            // Verify that freeing the handle is working as expected
            assert_eq!(
                SqlReturn::SUCCESS,
                SQLFreeHandle(HandleType::Env, env_handle as Handle)
            );
        }
    }

    /// Emulate PowerBi connection flow :
    ///         SQLAllocHandle(SQL_HANDLE_DBC)
    ///         SQLSetConnectAttrW(SQL_ATTR_LOGIN_TIMEOUT)
    ///         SQLDriverConnectW({NullTerminatedInConnectionString}, SQL_NTS, {NullTerminatedOutConnectionString}, SQL_NTS, SQL_DRIVER_NOPROMPT)
    ///         SQLGetInfoW(SQL_DRIVER_NAME)
    ///         SQLGetInfoW(SQL_DRIVER_VER)
    ///         SQLGetInfoW(SQL_DBMS_NAME)
    ///         SQLGetInfoW(SQL_DBMS_VER)
    #[test]
    fn test_connection() {
        // First setup the environment handle
        let env_handle: HEnv = setup();

        unsafe {
            // ---- Allocate the DB handle --- //
            let mut dbc: Handle = null_mut();
            // SQLAllocHandle(SQL_HANDLE_DBC)
            assert_eq!(
                SqlReturn::SUCCESS,
                SQLAllocHandle(
                    HandleType::Dbc,
                    env_handle as *mut _,
                    &mut dbc as *mut Handle
                )
            );

            // ---- Set the login timeout --- //
            let login_timeout_value: UInteger = 900;
            // SQLSetConnectAttrW(SQL_ATTR_LOGIN_TIMEOUT)
            assert_eq!(
                SqlReturn::SUCCESS,
                SQLSetConnectAttrW(
                    dbc as HDbc,
                    ConnectionAttribute::LoginTimeout,
                    login_timeout_value as Pointer,
                    0,
                )
            );

            // ---- Connect --- //
            let in_connection_string = common::generate_default_connection_str();
            let mut in_connection_string_encoded: Vec<u16> =
                in_connection_string.encode_utf16().collect();
            in_connection_string_encoded.push(0);

            let driver_completion = DriverConnectOption::NoPrompt;
            let out_string_length = &mut 0;
            const BUFFER_LENGTH: SmallInt = 300;
            let out_connection_string = &mut [0u16; (BUFFER_LENGTH as usize - 1)] as *mut _;

            // SQLDriverConnectW({NullTerminatedInConnectionString}, SQL_NTS, {NullTerminatedOutConnectionString}, SQL_NTS, SQL_DRIVER_NOPROMPT)
            match SQLDriverConnectW(
                dbc as HDbc,
                null_mut(),
                in_connection_string_encoded.as_ptr(),
                SQL_NTS,
                out_connection_string,
                BUFFER_LENGTH,
                out_string_length,
                driver_completion,
            ) {
                SqlReturn::ERROR => {
                    let error_msg = get_sql_diagnostics(HandleType::Dbc, dbc);
                    panic!("SQLDriverConnectW failed {}", error_msg);
                }
                SqlReturn::SUCCESS_WITH_INFO => {
                    let error_msg = get_sql_diagnostics(HandleType::Dbc, dbc);
                    println!("SUCCESS_WITH_INFO : {}", error_msg );
                }
                _ => {}
            };

            // ---- Get driver info --- //
            // SQLGetInfoW(SQL_DRIVER_NAME)
            /*
            assert_eq!(
                SqlReturn::SUCCESS,
                SQLGetInfoW(dbc as HDbc,
                            InfoType::DriverName,
                            info_value_ptr: Pointer,
                            buffer_length: SmallInt,
                            string_length_ptr));
             */

            // SQLGetInfoW(SQL_DRIVER_VER)
            /*
            assert_eq!(
                SqlReturn::SUCCESS,
                SQLGetInfoW(dbc as HDbc,
                            InfoType::DriverVersion,
                            info_value_ptr: Pointer,
                            buffer_length: SmallInt,
                            string_length_ptr));
             */
            // SQLGetInfoW(SQL_DBMS_NAME)
            let out_dbms_name = &mut [0u16; (BUFFER_LENGTH as usize - 1)] as *mut _;
            let out_string_length = &mut 0;
            assert_eq!(
                SqlReturn::SUCCESS,
                SQLGetInfoW(
                    dbc as HDbc,
                    InfoType::DbmsName,
                    out_dbms_name as Pointer,
                    BUFFER_LENGTH,
                    out_string_length
                )
            );

            let actual_message_length = *out_string_length as usize;
            println!("DBMS name : {}", &(String::from_utf16_lossy(&*(out_dbms_name as *const [u16; BUFFER_LENGTH as usize])))
                [0..actual_message_length], );

            // SQLGetInfoW(SQL_DBMS_VER)
            let out_dbms_version = &mut [0u16; (BUFFER_LENGTH as usize - 1)] as *mut _;
            let out_string_length = &mut 0;
            assert_eq!(
                SqlReturn::SUCCESS,
                SQLGetInfoW(
                    dbc as HDbc,
                    InfoType::DbmsVer,
                    out_dbms_version as Pointer,
                    BUFFER_LENGTH,
                    out_string_length
                )
            );

            let actual_message_length = *out_string_length as usize;
            println!("DBMS ver : {}", &(String::from_utf16_lossy(&*(out_dbms_name as *const [u16; BUFFER_LENGTH as usize])))
                [0..actual_message_length], );
        }
    }
}
