use once_cell::sync::Lazy;
use regex::Regex;

// 正規表現オブジェクトを初回起動時にコンパイルして静的に保持
static RE_SAMURAI_PHRASE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[\s\S]+?侍").expect("Failed to compile samurai regex")
});

/// 文章内に「〇〇侍」という形式のフレーズ（改行を含む可能性あり）が含まれているかどうかを判定する関数
/// # 引数
/// * `text` - 判定対象の文章 (文字列スライス)
///
/// # 戻り値
/// * `true` - フレーズが1つ以上含まれている場合
/// * `false` - フレーズが含まれていない場合
pub fn contains_samurai_phrase(text: &str) -> bool {
    RE_SAMURAI_PHRASE.is_match(text)
}

#[cfg(test)]
mod tests {
    // 親モジュール(detect)の contains_samurai_phrase をインポート
    use super::*;

    #[test]
    fn test_samurai_detection() {
        assert!(contains_samurai_phrase("ピタッとハウス侍"));
        assert!(contains_samurai_phrase("こんにちは\nゲームしたい侍"));
        assert!(contains_samurai_phrase("ただの侍"));
        assert!(!contains_samurai_phrase("普通の文章"));
        assert!(!contains_samurai_phrase(""));
    }
}