mod common;

mod integration {
    use crate::common;
    use crate::common::{get_sql_diagnostics, print_outcome, print_sql_diagnostics, print_text, setup, wtext_to_string};
    use odbc::ffi::SQL_NTS;
    use odbc_sys::{ConnectionAttribute, DriverConnectOption, EnvironmentAttribute, HDbc, HEnv, Handle, HandleType, SQLAllocHandle, SQLDriverConnectW, SQLFreeHandle, SQLGetDiagRecW, SQLGetEnvAttr, SQLSetConnectAttrW, SmallInt, SqlReturn, WChar, SQLSetEnvAttr, AttrOdbcVersion};
    use std::ffi::c_void;
    use std::ptr::null_mut;

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

    #[test]
    fn test_connection() {
        /*

        SQLAllocHandle(SQL_HANDLE_DBC)
        SQLSetConnectAttrW(SQL_ATTR_LOGIN_TIMEOUT)
        SQLDriverConnectW({NullTerminatedInConnectionString}, SQL_NTS, {NullTerminatedOutConnectionString}, SQL_NTS, SQL_DRIVER_NOPROMPT)
        SQLGetInfoW(SQL_DRIVER_NAME)
        SQLGetInfoW(SQL_DRIVER_VER)
        SQLGetInfoW(SQL_DBMS_NAME)
        SQLGetInfoW(SQL_DBMS_VER)

        */
        // We need a handle to be able to test that freeing the handle work
        let env_handle: HEnv = setup();
        unsafe {
            let mut odbc_version = 0;
            println!("odbcVersion = {}", odbc_version);
            let p_odbc_version = &mut odbc_version as *mut i32 as *mut c_void;
            print_outcome(
                "SQLGetEnvAttr",
                SQLGetEnvAttr(
                    env_handle,
                    EnvironmentAttribute::OdbcVersion,
                    p_odbc_version,
                    0,
                    &mut 0,
                ),
            );

            println!("odbcVersion = {}", odbc_version);

            let mut dbc: Handle = null_mut();

            assert_eq!(
                SqlReturn::SUCCESS,
                SQLAllocHandle(
                    HandleType::Dbc,
                    env_handle as *mut _,
                    &mut dbc as *mut Handle
                )
            );

            let mut login_timeout = 0;
            let p_login_timeout = &mut login_timeout as *mut i32 as *mut c_void;

            assert_eq!(
                SqlReturn::SUCCESS,
                SQLSetConnectAttrW(
                    dbc as HDbc,
                    ConnectionAttribute::LoginTimeout,
                    p_login_timeout,
                    0,
                )
            );

            let in_connection_string = common::generate_default_connection_str();
            let mut in_connection_string_encoded: Vec<u16> =
                in_connection_string.encode_utf16().collect();
            in_connection_string_encoded.push(0);

            let driver_completion = DriverConnectOption::NoPrompt;
            let string_length_2 = &mut 0;
            const BUFFER_LENGTH: SmallInt = 300;
            let out_connection_string = &mut [0u16; (BUFFER_LENGTH as usize - 1)] as *mut _;

            let in_conn_ptr: *mut WChar = in_connection_string_encoded.as_mut_ptr();
            print_text("in_connection_string = ", -3, in_conn_ptr);

            dbg!(">>>> pbi test - SQLDriverConnectW");
            let driver_connect_outcome = SQLDriverConnectW(
                dbc as HDbc,
                null_mut(),
                in_connection_string_encoded.as_ptr(),
                SQL_NTS,
                out_connection_string,
                BUFFER_LENGTH,
                string_length_2,
                driver_completion,
            );
            dbg!("<<<< pbi test - SQLDriverConnectW");

            print_outcome("SQLDriverConnectW", driver_connect_outcome);

            dbg!(*string_length_2);
            print_text(
                "out_connection_string = ",
                *string_length_2 as isize,
                out_connection_string,
            );

            if driver_connect_outcome == SqlReturn::ERROR {
                print_sql_diagnostics(HandleType::Dbc, dbc);
            }

            /*
            let text_length_ptr = &mut 0;
            let actual_sql_state = &mut [0u16; 6] as *mut _;
            let actual_message_text = &mut [0u16; 512] as *mut _;
            let actual_native_error = &mut 0;

            let _ = SQLGetDiagRecW(
                HandleType::Dbc,
                dbc as *mut _,
                1,
                actual_sql_state,
                actual_native_error,
                actual_message_text,
                1024,
                text_length_ptr,
            );

            print_text("error", *text_length_ptr as usize, actual_message_text);
             */
        }
    }

    #[test]
    fn rep_crashing_odbc_test() {
        unsafe {
            // First setup the environment handle
            let mut env_handle: Handle = null_mut();
            assert_eq!(
                SqlReturn::SUCCESS,
                SQLAllocHandle(HandleType::Env, null_mut(), &mut env_handle as *mut Handle)
            );

            assert_eq!(
                SqlReturn::SUCCESS,
                SQLSetEnvAttr(
                    env_handle as HEnv,
                    EnvironmentAttribute::OdbcVersion,
                    AttrOdbcVersion::Odbc3.into(),
                    0,
                )
            );

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
                    println!("SUCCESS_WITH_INFO : {}", error_msg);
                }
                _ => {}
            };

            println!(
                "Ouput connection string : {}",
                wtext_to_string(
                    out_connection_string as *const WChar,
                    *out_string_length as usize
                )
            );
        }
    }
}
