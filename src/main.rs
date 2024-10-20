fn main() {
    // let x: i32 = 10;
    // let y = 20;
    // let z = mul(x, y);

    // println!("z={z}");

    // println!("短絡評価");
    // println!("{}", a() || b());

    // println!("非短絡評価");
    // println!("{}", a() | b());

    // let player: u16 =
    // 1 |
    // (1 << 1) |
    // (568 << 2);

    // if player & 1 != 0 {
    //   println!("毒状態");
    // }

    // if player & (1 << 1) != 0 {
    //   println!("攻撃アップ状態");
    // }

    // let hp = (player & 0xfffc) >> 2;
    // println!("残り体力 = {hp}");

    // n を破壊的代入可能として宣言し 100 を代入
    let mut n: u64 = 100;

    // a に n の不変参照を代入
    let a: &u64 = &n;

    // 破壊的代入はできない
    // a  = 200;

    // a を参照はずした値 (n そのものの値)と、アドレスを表示
    println!("*a = {}, addr = {:p}", *a, a);

    // b に n の可変参照を代入
    let b: &mut u64 = &mut n;

    // b の指している先(つまり n )に200を破壊的代入
    *b = 200;

    println!("n = {n}");
}

// fn a() -> bool {
//   println!("call a!");
//   true
// }

// fn b() -> bool {
//   println!("call b!");
//   false
// }

// fn mul(x: i32, y: i32) -> i32 {
//   x * y
// }
