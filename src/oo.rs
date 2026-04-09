use tracing::info;

/// 2文字が「おお」系かどうか判定
pub fn is_oo(a: char, b: char) -> bool {
    (a == 'お' && b == 'お')
        || (a == 'オ' && b == 'オ')
        || (a.eq_ignore_ascii_case(&'o') && b.eq_ignore_ascii_case(&'o'))
}

/// 「おお」が最初に出てくる文字インデックスを返す
pub fn find_oo_col(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len().saturating_sub(1) {
        if is_oo(chars[i], chars[i + 1]) {
            return i;
        }
    }
    0
}

/// 「おお」→「2」に置換した提案文字列を作る
pub fn suggest(s: &str) -> String {
    s.replace("おお", "2")
        .replace("オオ", "2")
        .replace("oo", "2")
        .replace("OO", "2")
        .replace("Oo", "2")
        .replace("oO", "2")
}

/// コンパイルエラー風メッセージを生成する
pub fn build_error_msg(content: &str) -> String {
    let col = find_oo_col(content);
    let spaces = " ".repeat(col);
    let suggestion = suggest(content);
    info!(
        "「おお」が見つかりました: col={}, content=\"{}\", suggestion=\"{}\"",
        col, content, suggestion
    );

    format!(
        "```\nError: Unexpected 「おお」\n1 | {content}\n  | {spaces}^^\n  Note: \"{content}\" includes 「おお」. use \"{suggestion}\"\n```"
    )
}
