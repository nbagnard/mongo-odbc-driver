use crate::err::Result;
use bson::Bson;
use std::fmt::Debug;
use constants::MongoColMetadata;

pub trait MongoStatement: Debug {
    // Move the cursor to the next item.
    // Return true if moving was successful, false otherwise.
    fn next(&mut self) -> Result<bool>;
    // Get the BSON value for the cell at the given colIndex on the current row.
    // Fails if the first row has not been retrieved (next must be called at least once before getValue).
    fn get_value(&self, col_index: u16) -> Result<Option<Bson>>;

    // Returns the number of columns for the Resultset.
    fn get_col_count(&self) -> usize;

    // Returns the metadata for the given column.
    fn get_col_attribute(&self, col_index: usize) -> Result<&MongoColMetadata>;
}