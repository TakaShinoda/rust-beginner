use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::LinkedList;
use std::fmt::{Display, Formatter};
use std::iter::Iterator;
use std::ops::Mul;
use std::sync::{Arc, Mutex};
use std::{fs::File, io::prelude::*, path::Path};

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

    let mut s = BTreeSet::new();
    s.insert(100);
    s.insert(400);
    s.insert(6);
    s.insert(1);

    // セットに対して、iter() を実行し、イテレーターを返す
    // そのイテレーターに対して for を実行する
    for n in s.iter() {
        println!("{n}"); // BTreeSet は要素をソートして保持する
    }

    let mut v = Vec::new();
    v.push(10);
    v.push(5);

    let mut s = BTreeSet::new();
    s.insert(100);
    s.insert(400);

    let it = v.iter().chain(s.iter()); // イテレーターを連結
    for n in it.clone().map(|n| n * 2) {
        println!("{n}");
    }

    let total = it.clone().fold(0, |acc, x| acc + x);
    println!("{total}");

    let list: LinkedList<_> = it.filter(|n| *n % 2 == 0).collect();

    for (n, m) in v.iter().zip(s.iter()) {
        println!("({n}, {m})");
    }

    // スレッドの生成
    let handler = std::thread::spawn(worker);

    // join でスレッドの終了を待ち合わせ
    match handler.join() {
        Ok(n) => println!("{n}"),
        Err(e) => println!("{:?}", e),
    }

    // チャネルを生成
    let (tx, rx) = std::sync::mpsc::sync_channel(64);

    // スレッドを生成し rx から受信
    let handler = std::thread::spawn(move || match rx.recv() {
        Ok((x, y)) => println!("{}, {}", x, y),
        Err(e) => eprintln!("{e}"),
    });

    // チャネルに送信
    if let Err(e) = tx.send((10, 20)) {
        eprint!("{e}");
    }

    // 待ち合わせ
    if let Err(e) = handler.join() {
        eprint!("{:?}", e);
    }

    // single_threaded();
    // multi_threaded();

    let h2_1 = H2 {};
    let h2_2 = H2 {};
    let o2 = O2 {};

    // 関数呼び出しや、match式、代入などで変数を使うと、その変数は消費される
    // 消費された変数は、関数内や代入先の変数に所有権が移動する
    let (h2o_1, h2o_2) = burn(h2_1, h2_2, o2);
    // let (h2o_1, h2o_2) = burn(h2_1, h2_2, o2); // すでに消費されたのでコンパイルエラー

    let a = Coin {};
    let b = a;
    let c = b;

    // move semantics によりコンパイルエラー
    // let d = a;

    // プリミティブ型は消費されない
    // let a = 10;
    // let b = 20;
    // let c = a + b;
    // let d = a * b;

    let a: i32 = 10;
    let b: &i32 = &a;

    square(b);

    Foo { x: &a };

    let mut x = 10;
    {
        let y = 20;
        add2(&mut x, &y);
    }
    println!("{x}");

    let v = Arc::new(vec![1, 2, 3]); // 参照カウント1
    let w = v.clone(); // 参照カウント2
    let z = v.clone(); // 参照カウント3

    let x = Arc::new(Mutex::new(100_000)); // Mutex 型の値を生成、引数には共有リソース
    let x2 = x.clone(); // 参照カウンタをインクリメント

    let h1 = std::thread::spawn(move || {
        // スレッド生成
        let mut guard = x.lock().unwrap();
        *guard -= 20_000; // ガードを参照して保護対象データにアクセス
    });

    let h2 = std::thread::spawn(move || {
        let mut guard = x2.lock().unwrap();
        *guard -= 30_000;
    });

    h1.join().unwrap();
    h2.join().unwrap();

    let n = ImaginaryNumber {
        real: 3.0,
        img: 4.0,
    };
    println!("{n}");

    // [2, 1, 0] というリストを生成
    // let list = List::new().cons(0).cons(1).cons(2);

    // for x in list.iter() {
    //     println!("{x}");
    // }

    // println!();

    // let mut it = list.iter();
    // println!("{:?}", it.next().unwrap());
    // println!("{:?}", it.next().unwrap());
    // println!("{:?}", it.next().unwrap());

    // リストを作成
    let list = List::new().cons(0).cons(1).cons(2);

    // JSON にシリアライズ
    let js = serde_json::to_string(&list).unwrap();
    println!("JSON: {} bytes", js.len());
    println!("{js}");

    // YAML にシリアライズ
    let yml = serde_yml::to_string(&list).unwrap();
    println!("YAML: {} bytes", yml.len());
    println!("{yml}");

    // MessagePack にシリアライズ
    let msgpack = rmp_serde::to_vec(&list).unwrap();
    println!("MessagePack: {} bytes", msgpack.len());

    // JSON からデシリアライズ
    let list = serde_json::from_str::<List<i32>>(&js).unwrap();
    println!("{:?}", list);

    // YAML からデシリアライズ
    let list = serde_yml::from_str::<List<i32>>(&yml).unwrap();
    println!("{:?}", list);

    // MessagePack からデシリアライズ
    let list = rmp_serde::from_slice::<List<i32>>(&msgpack).unwrap();
    println!("{:?}", list);

    // ファイルへ書き出し
    let path = Path::new("test.yml");

    // 新規ファイルうを生成
    let mut f = File::create(path).unwrap();
    f.write_all(yml.as_bytes()).unwrap();

    // ファイルから YAML 読み出し
    let path = Path::new("test.yml");

    // 既存のファイルをオープン
    let mut f = File::open(path).unwrap();
    let mut yml = String::new();
    f.read_to_string(&mut yml).unwrap();

    // YAML からデシリアライズ
    let list = serde_yml::from_str::<List<i32>>(&yml).unwrap();
    println!("{:?}", list);

    let n = square2(4);
    println!("{n}");

    let m = square3(5);
    println!("{m}");
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

fn worker() -> u32 {
    println!("worker!");
    100
}

struct XOR64 {
    x: u64,
}

impl XOR64 {
    fn new(seed: u64) -> XOR64 {
        XOR64 {
            x: seed ^ 88172645463325252,
        }
    }

    // 擬似乱数生成
    fn next(&mut self) -> u64 {
        let x: u64 = self.x;
        let x = x ^ (x << 13);
        let x = x ^ (x >> 7);
        let x = x ^ (x << 17);
        self.x = x;
        x
    }
}

const NUM: usize = 200000000;

fn randomized_vec() -> (Vec<u64>, Vec<u64>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    let mut generator = XOR64::new(1234);

    // 乱数生成
    for _ in 0..NUM {
        let x1 = generator.next();
        let x2 = generator.next();
        v1.push(x1);
        v2.push(x2);
    }
    (v1, v2)
}

fn single_threaded() {
    let (mut v1, mut v2) = randomized_vec();

    let start = std::time::Instant::now(); // 現在時刻

    v1.sort();
    v2.sort();

    let end = start.elapsed(); // 経過時間

    println!(
        "single_threaded: {}.{:03}秒",
        end.as_secs(),
        end.subsec_nanos()
    )
}

fn multi_threaded() {
    let (mut v1, mut v2) = randomized_vec();

    let start = std::time::Instant::now(); // 現在時刻

    let handler1 = std::thread::spawn(move || {
        v1.sort();
        v1
    });

    let handler2 = std::thread::spawn(move || {
        v2.sort();
        v2
    });

    let _v1 = handler1.join().unwrap();
    let _v2 = handler2.join().unwrap();

    let end = start.elapsed();

    println!(
        "multi_threaded: {}.{:03}秒",
        end.as_secs(),
        end.subsec_nanos()
    )
}

// スタックメモリ
fn m() {
    let a = 10;
    let b = 20; // 地点1

    {
        let c = 30;
        let d = 40; // 地点2
        n(); // 地点4
    }
    // 地点5
}

fn n() {
    let e = 50;
    let f = 60; // 地点3
}

struct H2O {}
struct O2 {}
struct H2 {}

fn burn(_h2_1: H2, _h2_2: H2, _o2: O2) -> (H2O, H2O) {
    (H2O {}, H2O {})
}

struct Coin {}

// ライフタイムを取る関数
fn square<'a>(x: &'a i32) -> i32 {
    x * x
}

// 参照を持つ構造体
struct Foo<'a> {
    x: &'a i32,
}

fn add2<'a>(x: &'a mut i32, y: &'a i32) {
    *x += *y
}

// 虚数を表す型
struct ImaginaryNumber {
    real: f64,
    img: f64,
}

// 虚数を表示するため、Display トレイトを実装
// ImaginaryNumber 型に対して Display トレイトを実装することを示す
impl Display for ImaginaryNumber {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} + {}i", self.real, self.img)
    }
}

// 不変イテレーターを返す型
struct ListIter<'a, T> {
    elm: &'a List<T>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.elm {
            List::Node { data, next } => {
                self.elm = next;
                Some(data)
            }
            List::Nil => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum List<T> {
    Node { data: T, next: Box<List<T>> },
    Nil,
}

// impl<T> List<T> : ジェネリクス型の impl
impl<T> List<T> {
    fn new() -> List<T> {
        List::Nil
    }

    // リストを消費して、そのリストの先頭に引数のdataを追加したリストを返す
    fn cons(self, data: T) -> List<T> {
        List::Node {
            data,
            next: Box::new(self),
        }
    }

    // 不変イテレーターを返す
    fn iter<'a>(&'a self) -> ListIter<'a, T> {
        ListIter { elm: self }
    }
}

// トレイト制約
// T は std::ops::Mul トレイトと、Copy トレイトを実装していなければならない
// 複数のトレイト制約はプラス演算子を用いる
fn square2<T>(x: T) -> T
where
    T: Mul<Output = T> + Copy,
{
    x * x
}

// トレイト制約
// where を用いない用いない定義
fn square3<T: Mul<Output = T> + Copy>(x: T) -> T {
    x * x
}
