use polars::prelude::*;
use rand::{rng, Rng};
use anyhow::{Result, bail, anyhow};
use std::env;
use log::info;

pub fn get_random_samurai_id(idlength: u32) -> u32 {
    // ランダムな Samurai ID を生成する関数
    // 0 から idlength - 1 の範囲でランダムな整数を生成
    let mut rng = rng();
    let number = rng.random_range(0..idlength) as u32;
    // Samurai ID を返す
    number as u32
}

pub fn read_samurai_csv() -> Result<DataFrame, PolarsError> {
    dotenv::dotenv().ok();
    let _samurai_csv_path = env::var("SAMURAI_CSV_PATH").expect("Expected a CSV path in the environment");

    // CSVファイルを読み込み、データフレームへ変換する関数
    // LazyCsvReaderに渡す際に所有権が必要なので、clone()を使用
    let q = LazyCsvReader::new(_samurai_csv_path.clone())
        .with_has_header(true)
        .finish()?; // `?`演算子でエラー処理を追加

    let df = q
        .with_columns(vec![
            col("S_No."),
            col("Name"),
            col("Description"),
        ])
        .collect()?; // `?`演算子でエラー処理を追加


    // DataFrameを返す
    info!("csv:{} read success", _samurai_csv_path);
    Ok(df)
}


pub fn get_samurai_name(df: &DataFrame) -> Result<Option<String>> {
    // "S_No."列を取得
    let s_no_column = df.column("S_No.")
        .map_err(|e| anyhow!("Failed to get 'S_No.' column: {}", e))?;

    // "S_No."列の長さを取得
    let s_no_length = s_no_column.len();

    // ランダムな Samurai ID を生成
    let id = get_random_samurai_id(s_no_length as u32);

    // "Name"列を取得
    let name_series = df.column("Name")
        .map_err(|e| anyhow!("Failed to get 'Name' column from filtered data: {}", e))?;

    // "Description"列を取得
    let description_series = df.column("Description")
        .map_err(|e| anyhow!("Failed to get 'Description' column from filtered data: {}", e))?;

    // "Name"列から最初の値 (0番目の行) を取得
    let name = match name_series.get(id.try_into().unwrap()) {
        Ok(AnyValue::String(name_str)) => name_str.to_string(),
        Ok(AnyValue::Null) => String::from(""),
        other_value => {
            bail!("'Name' column contains unexpected data, value: {:?}", other_value)
        }
    };

    // "Description"列から最初の値 (0番目の行) を取得
    let description = match description_series.get(id.try_into().unwrap()) {
        Ok(AnyValue::String(desc_str)) => desc_str.to_string(),
        Ok(AnyValue::Null) => String::from(""),
        other_value => {
            bail!("'Description' column contains unexpected data, value: {:?}", other_value)
        }
    };

    // name と description を改行コードで結合して返す
    info!("Samurai ID: {}, Name: {}, Description: {}", id, name, description);
    Ok(Some(format!("{}\n{}", name, description)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::error;

    #[test]
    fn test_get_random_samurai_id() {
        let id = get_random_samurai_id(100);
        assert!(id < 100);
    }

    #[test]
    fn test_read_samurai_csv() {
        // CSVファイルを読み込む
        let res_df = read_samurai_csv();

        let df = match res_df {
            Ok(data) => data,
            Err(e) => {
                error!("Error reading CSV: {}", e);
                panic!("Failed to read CSV");
            }
        };

        //dfの型にS_No., Name, Descriptionの列があることを確認
        assert!(df.try_get_column_index("S_No.").is_ok());
        assert!(df.try_get_column_index("Name").is_ok());
        assert!(df.try_get_column_index("Description").is_ok());
    }

    #[test]
    fn test_get_samurai_name() {
        // CSVファイルを読み込む
        let df = read_samurai_csv().unwrap();
        // Samurai ID に基づいて名前を取得
        let name = get_samurai_name(&df).unwrap();

        // 名前が取得できたことを確認
        assert!(name.is_some());
    }
}