use anyhow::{Error, Ok, Result};
use csv::ReaderBuilder;
use log::{error, info};
use rand::{Rng, rng};
use std::fs::File;
use std::{env, fmt::Debug};

// データを保持するための構造体
#[derive(Debug, Clone)]
pub struct SamuraiEntry {
    pub name: String,
    pub description: String,
}

/// csvのデータ数からランダムな Samurai ID を取得する関数
/// # 引数
/// * `idlength` - CSVファイルのデータ数
///
/// # 戻り値
/// * `u32` - ランダムな Samurai ID
pub fn get_random_samurai_id(idlength: u32) -> u32 {
    // ランダムな Samurai ID を生成する関数
    // 0 から idlength - 1 の範囲でランダムな整数を生成
    let mut rng = rng();
    let number = rng.random_range(0..idlength) as u32;
    // Samurai ID を返す
    number as u32
}

/// CSVファイルを読み込んで SamuraiEntry のベクタを返す関数
/// # 戻り値
/// * `Ok(Vec<SamuraiEntry>)` - 読み込んだ SamuraiEntry のベクタ
/// * `Err(Error)` - エラーが発生した場合
pub fn read_samurai_csv_as_vec() -> Result<Vec<SamuraiEntry>, Error> {
    // csvファイルのパスを取得
    dotenv::dotenv().ok();
    let _samurai_csv_path =
        env::var("SAMURAI_CSV_PATH").expect("Expected a CSV path in the environment");

    let file = File::open(&_samurai_csv_path)?;

    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut samurai_entries = Vec::new();

    // ヘッダーを読み飛ばして、"Name"と"Description"のインデックスを取得
    let headers = rdr.headers()?;
    let name_index = headers.iter().position(|h| h == "Name").ok_or_else(|| {
        Error::from(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to find 'Name' column",
        ))
    })?;
    let description_index = headers
        .iter()
        .position(|h| h == "Description")
        .ok_or_else(|| {
            Error::from(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to find 'Description' column",
            ))
        })?;

    for result in rdr.records() {
        let record = result?;
        let name = record.get(name_index).unwrap_or("").to_string();
        let description = record.get(description_index).unwrap_or("");

        samurai_entries.push(SamuraiEntry {
            // s_no: record.get(0).unwrap_or("").to_string(), // S_No.が必要な場合はコメントを外す
            name: name.to_string(),
            description: description.to_string(),
        });
    }
    Ok(samurai_entries)
}

/// Samurai ID に基づいて名前を取得する関数
/// # 引数
/// * `df` - 読み込んだデータフレーム
///
/// # 戻り値
/// * `Ok(Some(name))` - Samurai ID に基づいて取得した名前と説明
/// * `Ok(None)` - Samurai ID が見つからなかった場合
/// * `Err(e)` - エラーが発生した場合
pub fn get_samurai_name(samurai_entries: &[SamuraiEntry]) -> Result<Option<String>> {
    // samurai_entries が空でないことを確認
    if samurai_entries.is_empty() {
        error!("Samurai entries are empty");
        return Ok(None);
    }

    // ランダムな Samurai ID を生成
    let id = get_random_samurai_id(samurai_entries.len() as u32);

    // "Name"列を取得
    if let Some(entry) = samurai_entries.get(id as usize) {
        let name = &entry.name;
        let description = &entry.description;

        // name と description を改行コードで結合して返す
        info!(
            "Samurai ID: {}, Name: {}, Description: {}",
            id, name, description
        );
        Ok(Some(format!("{}: {}\n{}", id, name, description)))
    } else {
        error!("Samurai ID {} not found", id);
        Ok(None)
    }
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
    fn test_read_samurai_csv_as_vec() {
        // CSVファイルを読み込む
        let samurai_entries = read_samurai_csv_as_vec().unwrap();
        // Samurai ID に基づいて名前を取得
        let name = get_samurai_name(&samurai_entries).unwrap();

        // 名前が取得できたことを確認
        assert!(name.is_some());
    }

    #[test]
    fn test_get_samurai_name() {
        // CSVファイルを読み込む
        let samurai_entries = read_samurai_csv_as_vec().unwrap();
        // Samurai ID に基づいて名前を取得
        let name = get_samurai_name(&samurai_entries).unwrap();

        // 名前が取得できたことを確認
        assert!(name.is_some());
        if let Some(name) = name {
            assert!(!name.is_empty());
        } else {
            error!("Samurai name is None");
        }
    }
}
