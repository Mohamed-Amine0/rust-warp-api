use crate::any_values;
use polars::prelude::PolarsError;
use polars::prelude::*;
use std::fs::File;

#[macro_export]
macro_rules! any_values {
    ( $( $val:expr ),* ) => {
        vec![
            $(
                crate::ToAnyValue::to_any_value(&$val),
            )*
        ]
    };
}

pub trait ToAnyValue {
    fn to_any_value(&self) -> AnyValue;
}

impl ToAnyValue for i32 {
    fn to_any_value(&self) -> AnyValue {
        AnyValue::Int32(*self)
    }
}

impl ToAnyValue for i64 {
    fn to_any_value(&self) -> AnyValue {
        AnyValue::Int64(*self)
    }
}

impl<'a> ToAnyValue for &'a str {
    fn to_any_value(&self) -> AnyValue {
        AnyValue::Utf8(self)
    }
}

impl ToAnyValue for f64 {
    fn to_any_value(&self) -> AnyValue {
        AnyValue::Float64(*self)
    }
}

impl ToAnyValue for bool {
    fn to_any_value(&self) -> AnyValue {
        AnyValue::Boolean(*self)
    }
}

impl ToAnyValue for f32 {
    fn to_any_value(&self) -> AnyValue {
        AnyValue::Float32(*self)
    }
}

impl ToAnyValue for u8 {
    fn to_any_value(&self) -> AnyValue {
        AnyValue::UInt8(*self)
    }
}

impl ToAnyValue for u16 {
    fn to_any_value(&self) -> AnyValue {
        AnyValue::UInt16(*self)
    }
}

impl ToAnyValue for u32 {
    fn to_any_value(&self) -> AnyValue {
        AnyValue::UInt32(*self)
    }
}

impl ToAnyValue for u64 {
    fn to_any_value(&self) -> AnyValue {
        AnyValue::UInt64(*self)
    }
}

impl ToAnyValue for i8 {
    fn to_any_value(&self) -> AnyValue {
        AnyValue::Int8(*self)
    }
}

impl ToAnyValue for i16 {
    fn to_any_value(&self) -> AnyValue {
        AnyValue::Int16(*self)
    }
}

impl ToAnyValue for String {
    fn to_any_value(&self) -> AnyValue {
        AnyValue::Utf8(self)
    }
}
//CSV support functions could replace parquet functions with no issues
/*pub async fn read_csv(csv_path: &str) -> Result<DataFrame, PolarsError> {
    let df = CsvReader::from_path(csv_path)?;
    Ok(df.finish()?)
}

pub async fn write_csv(mut df: DataFrame, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create(output_path)?;
    CsvWriter::new(&mut file).finish(&mut df)?;
    Ok(())
}*/

pub async fn read_parquet(parquet_path: &str) -> Result<DataFrame, PolarsError> {
    let r = File::open(parquet_path)?;
    let reader = ParquetReader::new(r);
    reader.finish()
}

pub async fn write_parquet(
    mut df: DataFrame,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(output_path)?;
    ParquetWriter::new(file).finish(&mut df)?;
    Ok(())
}

pub async fn delete_column(df: DataFrame, column_name: &str) -> PolarsResult<DataFrame> {
    df.drop(column_name)
}

pub async fn rename_column(
    mut df: DataFrame,
    column_name: &str,
    new_column_name: &str,
) -> PolarsResult<DataFrame> {
    df.rename(column_name, new_column_name)?;
    Ok(df)
}

pub async fn add_data_to_column(
    mut df: DataFrame,
    column_name: &str,
    data: AnyValue<'_>,
) -> PolarsResult<DataFrame> {
    if !df.get_column_names().contains(&column_name) {
        return Err(PolarsError::ColumnNotFound(column_name.to_string().into()));
    } else {
        let mut df1 = df! (
            column_name => &vec![data],
        )?;

        for i in df.get_column_names() {
            if i != column_name {
                let column_type = df.column(i)?.dtype();
                let new_series = match column_type {
                    DataType::Int32 => Series::new(i, vec![None::<i32>; df1.height()]),
                    DataType::Int64 => Series::new(i, vec![None::<i64>; df1.height()]),
                    DataType::Boolean => Series::new(i, vec![None::<bool>; df1.height()]),
                    DataType::Float32 => Series::new(i, vec![None::<f32>; df1.height()]),
                    DataType::Float64 => Series::new(i, vec![None::<f64>; df1.height()]),
                    DataType::Utf8 => Series::new(i, vec![None::<String>; df1.height()]),
                    _ => {
                        return Err(PolarsError::SchemaMismatch(
                            format!("Type mismatch for column '{}'", i).into(),
                        ));
                    }
                };
                df1.with_column(new_series)?;
            }
        }

        df1 = df1.select(&df.get_column_names())?;
        df = df.vstack(&df1)?;
        Ok(df)
    }
}

pub async fn add_null_column(
    mut df: DataFrame,
    column_name: &str,
    column_type: &str,
) -> PolarsResult<DataFrame> {
    let lower_column_type = column_type.to_lowercase();
    let new_column: Series = match &lower_column_type[..] {
        "string" => Series::new(column_name, vec![None::<String>; df.height()]),
        "integer" => Series::new(column_name, vec![None::<i32>; df.height()]),
        "integer64" => Series::new(column_name, vec![None::<i64>; df.height()]),
        "float" => Series::new(column_name, vec![None::<f32>; df.height()]),
        "float64" => Series::new(column_name, vec![None::<f64>; df.height()]),
        "boolean" => Series::new(column_name, vec![None::<bool>; df.height()]),
        _ => Series::new(column_name, vec![None::<String>; df.height()]),
    };
    df.with_column(new_column)?;
    Ok(df)
}

pub async fn cast_column(
    mut df: DataFrame,
    column_name: &str,
    new_type: &str,
) -> PolarsResult<DataFrame> {
    let lower_column_type = new_type.to_lowercase();
    let new_column: Series = match &lower_column_type[..] {
        "string" => df
            .column(column_name)
            .unwrap()
            .clone()
            .cast(&DataType::Utf8)
            .expect("Error"),
        "integer" => df
            .column(column_name)
            .unwrap()
            .clone()
            .cast(&DataType::Int32)
            .expect("Error"),
        "integer64" => df
            .column(column_name)
            .unwrap()
            .clone()
            .cast(&DataType::Int64)
            .expect("Error"),
        "float" => df
            .column(column_name)
            .unwrap()
            .clone()
            .cast(&DataType::Float32)
            .expect("Error"),
        "float64" => df
            .column(column_name)
            .unwrap()
            .clone()
            .cast(&DataType::Float64)
            .expect("Error"),
        "boolean" => df
            .column(column_name)
            .unwrap()
            .clone()
            .cast(&DataType::Boolean)
            .expect("Error"),
        _ => df.column(column_name).unwrap().clone(),
    };
    df.with_column(new_column)?;
    Ok(df)
}

pub async fn add_column_with_values(
    mut df: DataFrame,
    column_name: &str,
    values: Vec<AnyValue<'_>>,
) -> PolarsResult<DataFrame> {
    let mut vec = values;
    if vec.len() < df.height() {
        while vec.len() < df.height() {
            vec.push(AnyValue::Null);
        }
    }
    let new_column = Series::new(column_name, vec);
    df.with_column(new_column)?;
    Ok(df)
}

pub async fn add_row(
    mut df: DataFrame,
    new_row_values: Vec<crate::AnyValue<'_>>,
) -> Result<DataFrame, PolarsError> {
    if new_row_values.len() != df.width() {
        return Err(PolarsError::ShapeMismatch(
            "Number of values in new row doesn't match the number of columns"
                .to_string()
                .into(),
        ));
    }
    let mut df1: DataFrame = DataFrame::default();
    let mut j = 0;
    for i in df.get_column_names() {
        let column_type = df.column(i)?.dtype();
        let new_series = match column_type {
            DataType::Int32 => Series::new(i, vec![new_row_values[j].clone(); 1]),
            DataType::Int64 => Series::new(i, vec![new_row_values[j].clone(); 1]),
            DataType::Boolean => Series::new(i, vec![new_row_values[j].clone(); 1]),
            DataType::Float32 => Series::new(i, vec![new_row_values[j].clone(); 1]),
            DataType::Float64 => Series::new(i, vec![new_row_values[j].clone(); 1]),
            DataType::Utf8 => Series::new(i, vec![new_row_values[j].clone(); 1]),
            DataType::UInt32 => Series::new(i, vec![new_row_values[j].clone(); 1]),

            _ => todo!(),
        };
        df1.with_column(new_series)?;
        j += 1;
    }
    df1 = df1.select(&df.get_column_names())?;
    df = df.vstack(&df1)?;
    Ok(df)
}

pub async fn delete_row(mut df: DataFrame, row_index: u32) -> PolarsResult<DataFrame> {
    df = df.with_row_count("row_nr", None)?;
    let mask = df.column("row_nr")?.not_equal(row_index)?;
    df = df.filter(&mask)?;
    df = df.drop("row_nr")?;
    Ok(df)
}

pub async fn update_row(
    mut df: DataFrame,
    row_index: u32,
    mut new_row_values: Vec<AnyValue<'_>>,
) -> PolarsResult<DataFrame> {
    df = df.with_row_count("row_nr", None)?;
    let mask = df.column("row_nr")?.not_equal(row_index)?;
    df = df.filter(&mask)?;
    let mut vec = any_values![row_index];
    vec.append(&mut new_row_values);
    df = add_row(df, vec).await?;
    df = df.sort(["row_nr"], false, false)?;
    df = df.drop("row_nr")?;
    Ok(df)
}

pub async fn update_row_counter(mut df: DataFrame) -> PolarsResult<DataFrame> {
    if df.get_column_names().contains(&"row_index") {
        df = df.drop("row_index")?;
    }
    df = df.with_row_count("row_index", None)?;
    df = cast_column(df, "row_index", "integer").await?;
    Ok(df)
}
