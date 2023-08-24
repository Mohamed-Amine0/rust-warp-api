use polars::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RenameColumnRequestBody {
    pub new_column_name: String,
}

#[derive(Deserialize)]
pub struct AddDataRequestBody {
    pub data: AnyValue<'static>,
}
