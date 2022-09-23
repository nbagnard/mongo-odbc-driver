pub const VENDOR_IDENTIFIER: &str = "MongoDB";

// SQL states
pub const NOT_IMPLEMENTED: &str = "HYC00";
pub const TIMEOUT_EXPIRED: &str = "HYT00";
pub const GENERAL_ERROR: &str = "HY000";
pub const INVALID_ATTR_VALUE: &str = "HY024";
pub const NO_DSN_OR_DRIVER: &str = "IM007";
pub const RIGHT_TRUNCATED: &str = "01004";
pub const OPTION_CHANGED: &str = "01S02";
pub const UNABLE_TO_CONNECT: &str = "08001";

pub const SQL_TABLES_METADATA: [MongoColMetadata; 5] = [
    MongoColMetadata {
        base_col_name: "TABLE_CAT",
        base_table_name: "",
        catalog_name: "",
        display_size: usize::MAX,
        fixed_prec_scale: false,
        label: "TABLE_CAT",
        length: usize::MAX,
        col_name: "TABLE_CAT",
        is_nullable: false,
        octet_length: usize::MAX,
        precision: 0,
        scale: 0,
        is_searchable: false,
        table_name: "",
        type_name: "string",
        is_unsigned: false,
        is_updatable: false
    },
    MongoColMetadata {
        base_col_name: "TABLE_SCHEM",
        base_table_name: "",
        catalog_name: "",
        display_size: usize::MAX,
        fixed_prec_scale: false,
        label: "TABLE_SCHEM",
        length: usize::MAX,
        col_name: "TABLE_CAT",
        is_nullable: true,
        octet_length: usize::MAX,
        precision: 0,
        scale: 0,
        is_searchable: false,
        table_name: "",
        type_name: "string",
        is_unsigned: false,
        is_updatable: false
    },
    MongoColMetadata {
        base_col_name: "TABLE_NAME",
        base_table_name: "",
        catalog_name: "",
        display_size: usize::MAX,
        fixed_prec_scale: false,
        label: "TABLE_NAME",
        length: usize::MAX,
        col_name: "TABLE_NAME",
        is_nullable: false,
        octet_length: usize::MAX,
        precision: 0,
        scale: 0,
        is_searchable: false,
        table_name: "",
        type_name: "string",
        is_unsigned: false,
        is_updatable: false
    },
    MongoColMetadata {
        base_col_name: "TABLE_TYPE",
        base_table_name: "",
        catalog_name: "",
        display_size: usize::MAX,
        fixed_prec_scale: false,
        label: "TABLE_TYPE",
        length: usize::MAX,
        col_name: "TABLE_TYPE",
        is_nullable: false,
        octet_length: usize::MAX,
        precision: 0,
        scale: 0,
        is_searchable: false,
        table_name: "",
        type_name: "string",
        is_unsigned: false,
        is_updatable: false
    },
    MongoColMetadata {
        base_col_name: "REMARKS",
        base_table_name: "",
        catalog_name: "",
        display_size: usize::MAX,
        fixed_prec_scale: false,
        label: "REMARKS",
        length: usize::MAX,
        col_name: "REMARKS",
        is_nullable: false,
        octet_length: usize::MAX,
        precision: 0,
        scale: 0,
        is_searchable: false,
        table_name: "",
        type_name: "string",
        is_unsigned: false,
        is_updatable: false
    },
];

// Metadata information for a column of the result set.
// The information is to be used when reporting columns information from
// SQLColAttribute or SQLDescribeCol and when converting the data to the targeted C type.
#[derive(Debug)]
pub struct MongoColMetadata {
    pub base_col_name: &'static str,
    pub base_table_name: &'static str,
    pub catalog_name: &'static str,
    pub display_size: usize,
    pub fixed_prec_scale: bool,
    pub label: &'static str,
    pub length: usize,
    pub col_name: &'static str,
    pub is_nullable: bool,
    pub octet_length: usize,
    pub precision: u16,
    pub scale: u16,
    pub is_searchable: bool,
    pub table_name: &'static str,
    // BSON type name
    pub type_name: &'static str,
    pub is_unsigned: bool,
    pub is_updatable: bool,
}