/// 偶数の乱数を返す
pub fn rand_even() {
    // !1 1のビット反転
    rand::random::<u32>() & !1
}
