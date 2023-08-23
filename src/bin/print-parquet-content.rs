fn main() {
    use polars::prelude::ParquetReader;
    use polars::prelude::SerReader;

    let df1 = ParquetReader::new(std::fs::File::open("./tmp-deltatable/part-00000-730f08c7-7089-4980-a962-2b4d8789aaa5-c000.snappy.parquet").unwrap())
        .finish()
        .unwrap();

    println!("{:?}", df1);
}
