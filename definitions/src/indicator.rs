//! Special indicator values
use crate::Len;

/// Indicates `NULL` values.
pub const SQL_NULL_DATA: Len = -1;

/// Indicates that the size of the value is not known. ODBC returns this value in indicator buffers
/// for truncated values of unknown size.
pub const SQL_NO_TOTAL: i32 = -4;

/// Use this as the indicator argument to `SQLBindParameter` in order to indicate that the data is
/// send at statement execution time.
pub const SQL_DATA_AT_EXEC: Len = -2;

/// Use result as the indicator argument to `SQLBindParameter` in order to indicate that the data is
/// send at statement execution time. In contrast to `DATA_AT_EXEC` the total size is passed to the
/// driver manager.
pub fn len_data_at_exec(length: Len) -> Len {
    const SQL_LEN_DATA_AT_EXEC_OFFSET: Len = -100;

    (-length).checked_add(SQL_LEN_DATA_AT_EXEC_OFFSET).unwrap()
}
