// use rand::Rng;

pub mod sub_module1;
pub mod sub_module2;

// pub mod sub_module1 {
//     pub fn test_fn1() {
//         println!("hello world1 self!")
//     }
//     fn test_fn2() {
//         println!("hello Rust1 self!");
//     }
// }

// mod sub_module2 {
//     pub fn test_fn1() {
//         println!("hello world2!")
//     }
//     fn test_fn2() {
//         println!("hello Rust2!");
//     }
// }

// mod test_module {
//     pub mod sub_module1 {
//         pub fn test_fn1() {
//             println!("hello world1 self!")
//         }
//         fn test_fn2() {
//             println!("hello Rust1 self!");
//         }
//     }

//     mod sub_module2 {
//         pub fn test_fn1() {
//             println!("hello world2!")
//         }
//         fn test_fn2() {
//             println!("hello Rust2!");
//         }
//     }
// }

// use crate::test_module::sub_module1;
// use test_module::sub_module1;
// use sec8_test_module::{sub_module1, sub_module2};

pub fn run() {
    // let random_number = rand::thread_rng().gen_range(1..10);
    // println!("{}", random_number)

    // crate::test_module::sub_module1::test_fn1();  // main.rs内のモジュールの絶対インポート
    // self::test_module::sub_module1::test_fn1();  // このファイル内に記述されたモジュールを相対インポート
    // test_module::sub_module1::test_fn1();  // 相対インポート selfは省略可能
    // // test_module::sub_module2::test_fn1();  // 相対インポート sub_moduleはpubではないのでエラーが発生
    sub_module1::test_fn1();  // use test_module::sub_module1; でインポートしたモジュールを使用
    sub_module2::test_fn1();
}