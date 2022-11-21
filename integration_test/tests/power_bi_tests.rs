mod common;

mod integration {
    use std::ffi::c_void;
    use std::ptr::null_mut;
    use odbc::ffi::SQL_NTS;
    use odbc_sys::{Handle, HandleType, HEnv, SqlReturn, SQLFreeHandle, EnvironmentAttribute, SQLAllocHandle, SQLSetConnectAttrW, HDbc, ConnectionAttribute, DriverConnectOption, SmallInt, SQLDriverConnectW, SQLGetDiagRecW, SQLGetEnvAttr};
    use crate::common;
    use crate::common::{print_outcome, print_sql_diagnostics, print_text, setup};

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
            print_outcome("SQLGetEnvAttr",
                          SQLGetEnvAttr(
                env_handle,
                EnvironmentAttribute::OdbcVersion,
                p_odbc_version,
                0,
                &mut 0));

            println!("odbcVersion = {}", odbc_version);

            let mut dbc: Handle = null_mut();

            // Verify that freeing the handle is working as expected
            assert_eq!(
                SqlReturn::SUCCESS,
                SQLAllocHandle(HandleType::Dbc, env_handle as *mut _, &mut dbc as *mut Handle));

            let mut login_timeout = 0;
            let p_login_timeout = &mut login_timeout as *mut i32 as *mut c_void;

            assert_eq!(
                SqlReturn::SUCCESS,
                SQLSetConnectAttrW(
                    dbc as HDbc,
                    ConnectionAttribute::LoginTimeout,
                    p_login_timeout,
                    0,
                ));

            let in_connection_string = common::generate_default_connection_str();
            let mut in_connection_string_encoded: Vec<u16> =
                in_connection_string.encode_utf16().collect();
            in_connection_string_encoded.push(0);

            let driver_completion = DriverConnectOption::NoPrompt;
            let string_length_2 = &mut 0;
            const BUFFER_LENGTH: SmallInt = 300;
            let out_connection_string = &mut [0u16; (BUFFER_LENGTH as usize - 1)] as *mut _;

            /*
            odbct32w        1c74-18a8   ENTER SQLDriverConnectW
            HDBC                0x0000000000488FB0
            HWND                0x0000000000260548
            WCHAR *             0x00007FFA9C7CCF80 [      -3] "******\ 0"
            SWORD                       -3
            WCHAR *             0x00007FFA9C7CCF80
            SWORD                       -3
            SWORD *             0x0000000000000000
            UWORD                        0 <SQL_DRIVER_NOPROMPT>
             */
            dbg!(">>>> pbi test - SQLDriverConnectW");
            let driver_connect_outcome = SQLDriverConnectW(dbc as HDbc,
                                            null_mut(),
                                            in_connection_string_encoded.as_ptr(),
                                            BUFFER_LENGTH,
                                            out_connection_string,
                                            BUFFER_LENGTH,
                                            string_length_2,
                                            driver_completion);
            dbg!("<<<< pbi test - SQLDriverConnectW");

            print_outcome( "SQLDriverConnectW",
                           driver_connect_outcome);

            dbg!(*string_length_2);
            print_text("out_connection_string", *string_length_2 as usize, out_connection_string);

            if driver_connect_outcome == SqlReturn::ERROR
            {
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
}
