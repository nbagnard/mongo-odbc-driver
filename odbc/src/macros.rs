#[cfg(debug_assertions)]
use std::{fs::File, sync::Mutex};

#[cfg(debug_assertions)]
use lazy_static::lazy_static;

#[cfg(all(target_os = "windows", debug_assertions))]
lazy_static! {
    pub(crate) static ref LOGGER_FILE: Mutex<File> =
        match File::create("C:\\cygwin\\home\\Administrator\\logs\\atlas_odbc_log") {
            Err(why) => panic!("couldn't open {}: {}", "atlas_odbc_log", why),
            Ok(file) => Mutex::new(file),
        };
}

#[cfg(all(target_os = "macos", debug_assertions))]
lazy_static! {
    pub(crate) static ref LOGGER_FILE: Mutex<File> =
        match File::create("~/mongo-odbc-driver-myfork/atlas_odbc_log") {
            Err(why) => panic!("couldn't open {}: {}", "adl_odbc_log", why),
            Ok(file) => Mutex::new(file),
        };
}

#[cfg(all(target_os = "linux", debug_assertions))]
lazy_static! {
    pub(crate) static ref LOGGER_FILE: Mutex<File> =
        match File::create("~/logs/atlas_odbc_log") {
            Err(why) => panic!("couldn't open {}: {}", "adl_odbc_log", why),
            Ok(file) => Mutex::new(file),
        };
}

#[macro_export]
macro_rules! file_dbg {
    () => {
        #[cfg(debug_assertions)]
        {
            use crate::macros::LOGGER_FILE;
            use std::io::Write;

            let mut logger_file = LOGGER_FILE.lock();
            while logger_file.is_err() {
                logger_file = LOGGER_FILE.lock();
            }
            let mut logger_file = logger_file.unwrap();
            match (*logger_file).write_all(format!("{}:{}\n", file!(), line!()).as_bytes()) {
                Err(why) => panic!("couldn't write to adl_odbc_log: {}", why),
                Ok(_) => (),
            };
            match (*logger_file).flush() {
                Err(why) => panic!("couldn't flush adl_odbc_log: {}", why),
                Ok(_) => (),
            }
        }
    };
    ( $val:expr ) => {
        #[cfg(debug_assertions)]
        {
            use crate::macros::LOGGER_FILE;
            use std::io::Write;

            let mut logger_file = LOGGER_FILE.lock();
            while logger_file.is_err() {
                logger_file = LOGGER_FILE.lock();
            }
            let mut logger_file = logger_file.unwrap();
            match (*logger_file)
                .write_all(format!("{}:{} {:?}\n", file!(), line!(), $val).as_bytes())
            {
                Err(why) => panic!("couldn't write to adl_odbc_log: {}", why),
                Ok(_) => (),
            };
            match (*logger_file).flush() {
                Err(why) => panic!("couldn't flush adl_odbc_log: {}", why),
                Ok(_) => (),
            }
        }
    };
}