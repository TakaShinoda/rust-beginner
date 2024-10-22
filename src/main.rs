use core::num;

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

    // 配列の定義
    let arr: [u32; 4] = [1, 2, 3, 4];
    println!("{}, {}, {}, {}", arr[0], arr[1], arr[2], arr[3]);

    // [1..3] -> 1以上、3未満の範囲
    let s: &[u32] = &arr[1..3]; // スライスを取得
    println!("{:?}", s);


    let a: &str = "     Hello";
    // コンパイルエラー
    // a += ", world!";

    let mut b: String = a.to_string();
    b += ", world!";

    let c: &str = b.trim();
    println!("{c}");

    let d = r##"これは
    複数行の
    文字列です"##;
    println!("{d}");

    do_if(add, 10, 2);
    do_if(mul, 10, 2);

    // let hdd = Storage::HDD { size: 512, rpm: 7200 };
    // let ssd = Storage::SSD(512);

    let spec = PCSpec {
      cpus: 8,
      memory: 16,
      storage: Storage::SSD(1024)
    };

    println!("{}", spec.cpus);

    hello();

    println!("{}",sumup(10));
    println!("{}",sumup_loop(10));
    println!("{}",sumup_while(10));
    println!("{}",sumup_for(10));

    let v = [3, 8, 11, 15];
    let mut result = 0;
    // iter() 配列の要素への不変参照が先頭から順に x に代入される
    for x in v.iter() {
      if *x % 2 == 0 {
        continue;
      }
      result += *x;
    }
    println!("{}", result);

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

// fn(u32, u32) -> u32 関数ポインタ型
fn do_if(f: fn(u32, u32) -> u32, a: u32, b:u32) {
  println!("{}", f(a, b));
}

fn add(a:u32, b:u32) -> u32 {
  a + b
}

fn mul(a:u32, b:u32) -> u32 {
  a * b
}

enum Storage {
  HDD { size: u32, rpm: u32 },
  SSD(u32)
}

struct PCSpec {
  cpus: u16,
  memory: u32,
  storage: Storage
}

fn hello () {
  struct Msg {
    msg1: &'static str, // ライフタイム指定子
    msg2: &'static str,
  }

  fn print_msg(msg: &Msg) {
    println!("{}{}", msg.msg1, msg.msg2);
  }

  let msg = Msg {
    msg1: "Hello, ",
    msg2: "world!",
  };
  print_msg(&msg);
}

fn sumup(n: u64) -> u64 {
  if n == 0 {
    0
  } else {
    n + sumup(n - 1)
  }
}

fn sumup_loop(mut n: u64) -> u64 {
  let mut total = 0;
  loop {
    if n == 0 {
      break;
    }
    total += n;
    n -= 1;
  }
  total
}

fn sumup_while(mut n: u64) -> u64 {
  let mut total = 0;
  while n > 0 {
    total += n;
    n -= 1;
  }
  total
}

fn sumup_for(mut n: u64) -> u64 {
  let mut total = 0;
  // 0 から n までの値が x に代入される
  for x in 0..=n {
    total += x;
  }
  total
}
