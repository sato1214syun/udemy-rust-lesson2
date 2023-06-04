use std::rc::Rc;

pub fn run() {
    // let mut v1 = vec![1, 2, 3];
    // println!("v1 ptr: {:?}", v1.as_ptr());
    // println!("v1[0]: {:p}", &v1[0]);

    // println!("v1 len: {}", v1.len());
    // println!("v1 caapcity: {}", v1.capacity());

    // v1.push(4);
    // println!("v1 ptr: {:?}", v1.as_ptr());
    // println!("v1 len: {}", v1.len());
    // println!("v1 caapcity: {}", v1.capacity());

    // 所有権の移動の確認
    // println!("va ptr: {:?}", v1.as_ptr());
    // let v2 = v1;
    // let v3 = v2.clone();
    // println!("va ptr: {:?}", v1.as_ptr()); v1はすでに所有権を持っていないため
    // println!("va ptr: {:?}", v2.as_ptr());
    // println!("va ptr: {:?}", v3.as_ptr());

    // 関数の引数への所有権の移動
    // let s1 = String::from("Hello");
    // let s2 = String::from("Rust");
    // let (s, a, b) = concat(s1, s2);
    // println!("{}", s);
    // println!("{}", a);
    // println!("{}", b);
    // println!("{}", s1); // すでに所有権がないためエラー
    // println!("{}", s2); // すでに所有権がないためエラー

    // 参照
    // let s1 = String::from("Hello?");
    // let s2 = String::from("Rust?");
    // let s = concat2(&s1, &s2);
    // println!("{}", s);
    // println!("{}", s1);
    // println!("{}", s2);

    /* ライフタイム。
    ダングリングポインタを防ぐ仕組み。
    通常は推論によって決まるが、明示するケースが稀にある。
    明示する場合は、'a (tick Aと発音)
    fn f<'a>(p: &'a i320) {...}などと表記
    */

    /*スマートポインタ*/
    // Box: 値をヒープ領域に格納できる。
    // スタックに格納されるデータはサイズが確定している必要があるが、
    // Box型でヒープ領域に移動することで、サイズを確定しなくてもエラーが出なくなる
    let x = Box::new(1);
    println!("{:p}", x); // {:p} 変数をポインタとして表示できる
    println!("*x + 2 = {}", *x + 2);
    // Rc: Rcを使うと値に対し、例外的に複数の変数に所有権をもたせることができる
    let a = Rc::new("hello".to_string());
    println!("count1: {}", Rc::strong_count(&a));  // 所有者の数を数える→ aが所有権を持つため1
    {
        let b = Rc::clone(&a);  // 所有権をbにコピー
        println!("a: {:p}", a);  // a, bで同じポインタを持つ
        println!("a: {:p}", b);
        println!("count2: {}", Rc::strong_count(&a));  // 2になる(所有権が1,b２つ)ここでbのスコープが終わるため所有権は消滅
    }
    println!("count3: {}", Rc::strong_count(&a));  // aのみが所有権を持つので1になる
}

// fn concat(a: String, b: String) -> (String, String, String) {
//     let c = format!("{} {}", a, b);
//     (c, a, b)
// }

// fn concat2(a: &String, b: &String) -> (String) {
//     let c = format!("{} {}", a, b);
//     c
// }
