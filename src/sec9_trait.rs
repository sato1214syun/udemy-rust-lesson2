// traitを使用するときは、そのtrait(ここではShape)もインポートする必要がある
use udemy_rust_lesson2::sample_trait::{double_area, Circle, Rectangle, Shape};
use std::fmt::Debug;

pub fn run() {
    let rect = Rectangle {
        width: 4.0,
        height: 5.0,
    };

    let circle = Circle { radius: 2.0 };

    // println!("Rectangle area is: {}", rect.calc_area());
    // println!("Rectangle perimeter is: {}", rect.calc_perimeter());
    // Rectangle::do_something();
    // println!("Circle area is: {}", circle.calc_area());
    // println!("Circle perimeter is: {}", circle.calc_perimeter());
    // Circle::do_something();
    // println!("Rectangle default is {}", rect.default_something());
    // println!("Rectangle default is {}", circle.default_something());
    // println!("Rectangle double area is {}", double_area(&rect));
    // println!("Rectangle double area is {}", double_area(&circle));

    // derive属性
    /* Rustでは属性はメタデータのこと、#[属性名]と書くことで使える */
    // println!("{:?}", (1, 2, 3));

    // #[derive(Debug, PartialEq)]  // deriveにDebugトレイトが渡され、Sに適用されている
    // struct S {
    //     val1: i32,
    //     val2: i32,
    // }

    // println!("{:?}", S {val1: 1, val2: 2});  // Sに#[derive(Debug)]をつけるとエラーが消える

    // let s1 = S {
    //     val1: 1,
    //     val2: 3,
    // };

    // let s2 = S {
    //     val1: 1,
    //     val2: 3,
    // };

    // println!("{}", s1 == s2);  // PartialEqで次のエラ〜を解決。binary operation `==` cannot be applied to type `S`

    println!("{}", max(1, 2));
    println!("{}", max(1.1, 2.1));
    println!("{}", max("x", "a"));
}

// section 9 ジェネリクス 関数の引数に複数の型を指定する(任意の型を受け取る方法 →ジェネリクス)
// <T>だけだと比較演算子が使える型が引数に渡れるか不明なのでエラーになる
// ここで<T: PartialOPrd>とすることで、比較演算子が使える型を指定する。これをトレイト境界やジェネリック境界という
// また where T: PartialOrdも使える
// 制約がシンプルが前者を使ったほうがいいだろう

fn max<T>(a: T, b: T) -> T
where T: PartialOrd + Debug
{
    if a >= b {
        a
    } else {
        b
    }
}