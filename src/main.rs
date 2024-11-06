use std::collections::BTreeMap;
use std::collections::LinkedList;

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
        storage: Storage::SSD(1024),
    };

    println!("{}", spec.cpus);

    match &spec {
        PCSpec {
            storage: Storage::SSD(512), // 512GiB SSD
            .. // 他のフィールドは無視
        } => {
            println!("512");
        }
        PCSpec {
            cpus: 4 | 8, // 4 か 8
            memory: m, // m にメモリサイズが代入される
            storage: _, // storage の値は無視
        } => {
            println!("4 or 8 CPUs");
            println!("{}GiB memory", *m);
        }
        PCSpec { memory: m, .. } if *m < 4 => {  // if 以降はガードと呼ばれる、この条件に合致したもののみがマッチ
            println!("4 GiB より小さいメモリ");
        }
        // 全パターン
        _ => (),
    }

    hello();

    println!("{}", sumup(10));
    println!("{}", sumup_loop(10));
    println!("{}", sumup_while(10));
    println!("{}", sumup_for(10));

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

    let arr: &[f32] = &[3.5, 5.2];
    let total = average(arr);
    println!("{:?}", total);

    let s = Storage::HDD {
        size: 2048,
        rpm: 7200,
    };
    println!("{:?}", s);
    println!("{:#?}", s);

    // クロージャ
    // | 引数| 式
    let f = |a, b| a + b;
    let n = f(5, 10);
    println!("{n}");

    let mut s = Storage::SSD(512);
    let mut f = || match &mut s {
        // s キャプチャ
        Storage::HDD { size: s, .. } => *s += 64,
        _ => (),
    };
    f();

    // move
    let mut g = move || match &mut s {
        Storage::HDD { size: s, .. } => *s += 64,
        _ => (),
    };
    g();

    // メソッド呼び出し
    let mut s = Storage::SSD(512);
    let size = s.get_size();

    // 型関連関数の呼び出し
    let s = Storage::SSD(512);
    let spec = PCSpec::new(8, 32, s);

    // コンパイル時定数
    const PI: f64 = 3.14159265358979323846264338327950288f64;
    println!("{PI}");
    println!("{}", std::f64::consts::PI);

    // 静的変数
    static A: u32 = 100;
    static mut B: u32 = 200;

    let mut list1 = LinkedList::new();
    list1.push_back(0);
    list1.push_back(1);
    list1.push_back(2);

    let mut list2 = LinkedList::new();
    list2.push_back(100);
    list2.push_back(200);
    list2.push_back(300);

    list1.append(&mut list2); // list1 == [0, 1, 2, 100, 200, 300]、list2 == []

    list1.push_front(-10);

    let mut m = BTreeMap::new();
    m.insert(1, "apple"); // キーとバリューのペアを挿入
    m.insert(2, "orange");
    m.insert(3, "banana");

    // 2 に対応する値を削除
    if let Some(old) = m.remove(&2) {
        println!("{old}");
    }

    // 3 に対応する値への不変参照を取得
    if let Some(vale) = m.get(&3) {
        println!("{vale}");
    }
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
fn do_if(f: fn(u32, u32) -> u32, a: u32, b: u32) {
    println!("{}", f(a, b));
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn mul(a: u32, b: u32) -> u32 {
    a * b
}

#[derive(Debug)]
enum Storage {
    HDD { size: u32, rpm: u32 },
    SSD(u32),
}

struct PCSpec {
    cpus: u16,
    memory: u32,
    storage: Storage,
}

fn hello() {
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

fn sumup_for(n: u64) -> u64 {
    let mut total = 0;
    // 0 から n までの値が x に代入される
    for x in 0..=n {
        total += x;
    }
    total
}

fn average(v: &[f32]) -> Option<f32> {
    if v.is_empty() {
        return None;
    }
    let mut total = 0.0;
    for n in v {
        total += n;
    }

    Some(total / v.len() as f32)
}

// 型に特化した関数は impl を使って定義する
// Storage 型の実装を定義
// メソッド
impl Storage {
    // &self は self: &Storage と同じ
    fn get_size(&self) -> u32 {
        match self {
            Storage::HDD { size: s, .. } => *s,
            Self::SSD(s) => *s,
        }
    }
}

// impl 内の第一引数が self ではない関数は 型関連関数と呼ぶ
impl PCSpec {
    fn new(cpus: u16, memory: u32, storage: Storage) -> PCSpec {
        PCSpec {
            cpus,
            memory,
            storage,
        }
    }
}
