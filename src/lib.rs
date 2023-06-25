// // section 8 ライブラリ、ドキュメント

// //! ライブラ入りクレートのドキュメント

// /// say_hello関数のドキュメント
// pub fn say_hello() {
//     println!("Hello!");
// }

// /// say_good_by関数のドキュメント
// /// ### 使用例
// /// ```
// /// fn main() {
// ///     udemy_rust_lesson::say_good_by();
// /// }
// /// ```
// pub fn say_good_bye() {
//     println!("Good gye!");
// }


// // section 9 トレイト
// pub mod sample_trait{
//     use std::f64::consts::PI;

//     pub trait Shape {
//         fn calc_area(&self) -> f64;
//         fn calc_perimeter(&self) -> f64;
//         fn default_something(&self) -> &str {
//             "This is default method!"
//         }
//         fn do_something();

//     }

//     pub struct Rectangle {
//         pub width: f64,
//         pub height: f64,
//     }

//     impl Shape for Rectangle {
//         fn calc_area(&self) -> f64 {
//             self.width * self.height
//         }
//         fn calc_perimeter(&self) -> f64 {
//             self.width * 2.0 + self.height * 2.0
//         }

//         fn default_something(&self) -> &str {
//             "This is Rectangle default!"
//         }

//         fn do_something() {
//             println!("This is Rectangle function");
//         }
//     }

//     pub struct Circle {
//         pub radius: f64,
//     }

//     impl Shape for Circle {
//         fn calc_area(&self) -> f64 {
//             self.radius.powf(2.0) * PI
//         }
//         fn calc_perimeter(&self) -> f64 {
//             self.radius * 2.0 * PI
//         }

//         fn do_something() {
//             println!("This is Circle function");
//         }
//     }

//     // 引数の型としてのトレイト
//     // &impl Shapeで　Shape型と同じ型を受け付けるように共有参照している
//     pub fn double_area(shape: &impl Shape) -> f64 {
//         shape.calc_area() * 2.0
//     }


// }

pub mod services;
pub mod models;
