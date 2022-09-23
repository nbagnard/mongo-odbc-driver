use crate::conn::MongoConnection;
use crate::err::Result;
use crate::stmt::{MongoStatement};
use bson::{Bson, Document};
use mongodb::sync::Cursor;
use constants::MongoColMetadata;

#[derive(Debug)]
pub struct MongoQuery {
    // The cursor on the result set.
    resultset_cursor: Cursor<Document>,
    // The result set metadata.
    resultset_metadata: Vec<MongoColMetadata>,
}

impl MongoQuery {
    // Create a new MongoStatement with StmtKind::Query on the connection currentDB.
    // Executes a $sql aggregation with the given query and initialize the Resultset cursor.
    // The query timeout comes from the statement attribute SQL_ATTR_QUERY_TIMEOUT. If there is a
    // timeout, the query must finish before the timeout or an error is returned
    pub fn execute(
        _client: &MongoConnection,
        _query_timeout: Option<i32>,
        _query: &str,
    ) -> Result<Self> {
        unimplemented!()
    }
}

impl MongoStatement for MongoQuery {
    // Move the cursor to the next document and update the current row.
    // Return true if moving was successful, false otherwise.
    fn next(&mut self) -> Result<bool> {
        unimplemented!()
    }

    // Get the BSON value for the cell at the given colIndex on the current row.
    // Fails if the first row as not been retrieved (next must be called at least once before getValue).
    fn get_value(&self, _col_index: u16) -> Result<Option<Bson>> {
        unimplemented!()
    }

    fn get_col_count(&self) -> usize {
        todo!()
    }

    fn get_col_attribute(&self, _col_index: usize) -> Result<&MongoColMetadata> {
        todo!()
    }
}
