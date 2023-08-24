use arrow::array::Array;
use arrow::datatypes::Schema;
use arrow::{
    array::{ArrayRef, Float64Array, Int32Array, StringArray},
    datatypes::{Field, Schema as ArrowSchema},
    record_batch::RecordBatch,
};
use deltalake::builder::DeltaTableBuilder;
use deltalake::operations::create::CreateBuilder;
use deltalake::operations::DeltaOps;
use deltalake::{DeltaTable, SchemaDataType};
use polars::prelude::*;
use polars::prelude::{DataFrame, Series};
use std::sync::Arc;

fn col_to_vec(column: &Series) -> Vec<String> {
    let col_vec: Vec<String> = column
        .utf8()
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap().to_string())
        .collect();

    // Rename the Vec<String> to match the input column name
    let mut result = Vec::new();
    result.extend(col_vec);

    result
}

fn create_primitive_array(column: &Series) -> Arc<dyn Array> {
    match column.dtype() {
        polars::prelude::DataType::Int32 => {
            let array = Int32Array::from(column.i32().unwrap().to_vec());
            Arc::new(array) as Arc<dyn Array>
        }
        polars::prelude::DataType::Float64 => {
            let array = Float64Array::from(column.f64().unwrap().to_vec());
            Arc::new(array) as Arc<dyn Array>
        }
        polars::prelude::DataType::Utf8 => {
            let array = StringArray::from(col_to_vec(column));
            Arc::new(array) as Arc<dyn Array>
        }
        _ => unimplemented!("Data type not supported"),
    }
}

fn generate_schema_from_dataframe(df: &DataFrame) -> Schema {
    let mut fields = Vec::new();

    for col in df.get_columns() {
        let name = col.name();
        let arrow_data_type = match col.dtype() {
            polars::datatypes::DataType::Int32 => arrow::datatypes::DataType::Int32,
            polars::datatypes::DataType::Float64 => arrow::datatypes::DataType::Float64,
            polars::datatypes::DataType::Utf8 => arrow::datatypes::DataType::Utf8,
            _ => unimplemented!("Data type not supported"),
        };

        let field = Field::new(name, arrow_data_type, false);
        fields.push(field);
    }

    ArrowSchema::new(fields)
}

// Function to create a RecordBatch from a DataFrame
fn generate_recordbatch_from_dataframe(df: &DataFrame) -> Option<RecordBatch> {
    let schema = generate_schema_from_dataframe(df);
    let mut arrays: Vec<ArrayRef> = Vec::new();

    for col in df.get_columns() {
        let array = create_primitive_array(col);
        arrays.push(array.into());
    }

    let recordbatch = RecordBatch::try_new(Arc::new(schema), arrays).unwrap();
    Some(recordbatch)
}

async fn create_deltatable_from_recordbatch(path: &str, batch: RecordBatch) -> DeltaTable {
    // First operation: Add columns to the table
    let mut builder = CreateBuilder::new();
    builder = builder.with_location(path);

    for field in batch.schema().fields() {
        let schema_data_type = match field.data_type() {
            arrow::datatypes::DataType::Int32 => "integer",
            arrow::datatypes::DataType::Int64 => "long",
            arrow::datatypes::DataType::Int16 => "short",
            arrow::datatypes::DataType::Int8 => "byte",
            arrow::datatypes::DataType::Float32 => "float",
            arrow::datatypes::DataType::Float64 => "double",// here it was float and it caused a panic error saying that the schema of the recordbatch and the schema of the table are not the same
            arrow::datatypes::DataType::Boolean => "boolean",
            arrow::datatypes::DataType::Utf8 => "string",
            arrow::datatypes::DataType::Binary => "binary",
            arrow::datatypes::DataType::Date32 => "date",
                _ => unimplemented!("Data type not supported"),
        };

        builder = builder.with_column(
            field.name().to_string(),
            SchemaDataType::primitive(String::from(schema_data_type)),
            // field.is_nullable().clone(),
            false,
            Default::default(),
        );
    }

    let ops = DeltaOps::from(builder.await.unwrap());

    let commit_result = ops.write(vec![batch.clone()]).await.unwrap();
    commit_result
}

async fn append_to_table(path: String, batch: RecordBatch) -> DeltaTable {
    let table = DeltaTableBuilder::from_uri(path).build().unwrap();

    let ops = DeltaOps::from(table);

    let commit_result = ops.write(vec![batch.clone()]).await.unwrap();
    commit_result
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let countries: DataFrame = df!("Rank (2021)" => &[105, 106, 107, 108, 109],
        "Apple Price (€/kg)" => &[0.75, 0.70, 0.70, 0.65, 0.52],
        "Country" => &["Kosovo", "Moldova", "North Macedonia", "Syria", "Turkey"])
    .unwrap();

    let recordbatch = generate_recordbatch_from_dataframe(&countries).unwrap();
    let recordbatch2 = recordbatch.clone();
    // println!("\n\n\nThis is the record batch\n{:#?}", recordbatch);

    // // loop to print all recordbatch.schema().fields()
    // for field in recordbatch.schema().fields() {
    //     println!("\n\n\nThis is the field\n{:#?}", field.data_type().to_string());
    // }

    let table = create_deltatable_from_recordbatch("./tmp-deltatable", recordbatch).await;

    //print the schema of the DeltaTable
    println!(
        "\n\n\nThis is the schema of the DeltaTable\n{:#?}",
        table.get_schema()
    );
    println!(
        "\n\n\nThis is the record batch schema\n{:#?}",
        recordbatch2.schema()
    );
    println!(
        "\n\n\nThis is the version of the DeltaTable\n{:#?}",
        table.version()
    );

    // let appended_counties: DataFrame = df!("Rank (2021)" => &[105, 106, 107, 108, 109, 110, 111],
    //     "Apple Price (€/kg)" => &[0.75, 0.70, 0.70, 0.65, 0.52, 0.50, 0.45],
    //     "Country" => &["Kosovo", "Moldova", "North Macedonia", "Syria", "Turkey", "Algeria", "Morocco"])
    // .unwrap();

    // let recordbatch3 = generate_recordbatch_from_dataframe(&appended_counties).unwrap();

    // let table2 = append_to_table(
    //     "./tmp-deltatable".to_string(),
    //     recordbatch3,
    // )
    // .await;

    // println!(
    //     "\n\n\nThis is the schema of the DeltaTable\n{:#?}",
    //     table2.get_schema()
    // );
    // println!(
    //     "\n\n\nThis is the version of the DeltaTable\n{:#?}",
    //     table2.version()
    // );

    // let table2 = append_to_table(
    //     "./tmp-deltatable".to_string(),
    //     generate_recordbatch_from_dataframe(&countries).unwrap(),
    // )
    // .await;

    // println!(
    //     "\n\n\nThis is the schema of the DeltaTable\n{:#?}",
    //     table2.get_schema()
    // );
    // println!(
    //     "\n\n\nThis is the version of the DeltaTable\n{:#?}",
    //     table2.version()
    // );

    // let table = append_to_table("./tmp-deltatable".to_string(), recordbatch).await;
    // println!("Data inserted with version : {}", table.version());

    // let table = deltalake::open_table("./tmp-deltatable").await.unwrap();

    // println!("\n\n\nThis is the record batch schema\n{:#?}", recordbatch.schema());
    // println!("\n\n\nThis is the Table schema\n{:#?}", table.get_schema());

    // println!("Table version : {:#?}", table.version());
    // println!("Table files : {:#?}", table.get_files());
}
