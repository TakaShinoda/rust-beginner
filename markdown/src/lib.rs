//! # 第一見出し
//!
//! テキストを書く
//!
//! ## 第二見出し
//!
//! ## 第三見出し
//!
//! - 箇条書き1
//! - 箇条書き2
//!
//! > 引用
//!
//!
//!
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod my_module {
    //! これはモジュールのドキュメント
    //!
    //! # 利用例
}

/// ```
/// use markdown::my_func;
/// let n = my_func().unwrap();
/// ```
pub fn my_func() -> Option<u32> {
    Some(100)
}

pub fn pred(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else {
        Some(n - 1)
    }
}

// テスト時のみコンパイルされるようにするマクロ
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_my_func() {
        assert_eq!(my_func(), Some(100));
    }

    #[test]
    #[should_panic] // パニックすべきというマクロ
    fn test_pred() {
        pred(0).unwrap();
    }
}
