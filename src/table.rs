use crate::config::app_config;
use anyhow::{Error, Ok, Result};
use csv::ReaderBuilder;
use log::{error, info};
use rand::{Rng, rng};
use std::fs::File;
use std::io::Read;

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
///
/// 注意: この関数は `config::app_config()` から `SAMURAI_CSV_PATH` を取得します。
/// `config::init_app_config()` が bot 起動時に呼び出されている前提で動作します。
/// この関数は静的な Lazy 初期化で使用されるため、パラメータを受け取ることができません。
///
/// # 戻り値
/// * `Ok(Vec<SamuraiEntry>)` - 読み込んだ SamuraiEntry のベクタ
/// * `Err(Error)` - エラーが発生した場合
pub fn read_samurai_csv_as_vec() -> Result<Vec<SamuraiEntry>, Error> {
    let samurai_csv_path = &app_config().samurai_csv_path;
    let file = File::open(samurai_csv_path)?;
    parse_samurai_reader(file)
}

/// ヘッダーを解析して SamuraiEntry のベクタを返す関数
/// # 引数
/// * `reader` - 読み込むリーダー
///
/// # 戻り値
/// * `Ok(Vec<SamuraiEntry>)` - 読み込んだ SamuraiEntry のベクタ
/// * `Err(Error)` - エラーが発生した場合
fn parse_samurai_reader<R: Read>(reader: R) -> Result<Vec<SamuraiEntry>, Error> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(reader);

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

    let mut samurai_entries = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let name = record.get(name_index).unwrap_or("").to_string();
        let description = record.get(description_index).unwrap_or("");

        samurai_entries.push(SamuraiEntry {
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
    use std::io::Cursor;

    const SAMPLE_CSV: &str = "S_No.,Name,Description\n1,テスト侍,テストテストテスト\n";

    fn sample_entries() -> Vec<SamuraiEntry> {
        parse_samurai_reader(Cursor::new(SAMPLE_CSV)).unwrap()
    }

    #[test]
    fn test_get_random_samurai_id() {
        let id = get_random_samurai_id(100);
        assert!(id < 100);
    }

    #[test]
    fn test_read_samurai_csv_as_vec() {
        let samurai_entries = sample_entries();
        let name = get_samurai_name(&samurai_entries).unwrap();

        assert!(name.is_some());
    }

    #[test]
    fn test_get_samurai_name() {
        let samurai_entries = sample_entries();
        let name = get_samurai_name(&samurai_entries).unwrap();

        assert!(name.is_some());
        if let Some(name) = name {
            assert!(!name.is_empty());
        } else {
            error!("Samurai name is None");
        }
    }
}
