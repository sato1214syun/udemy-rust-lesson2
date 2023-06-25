pub fn test_fn1() {
    println!("hello world1 self!")
}
fn test_fn2() {
    println!("hello Rust1 self!");
}

// struct宣言の先頭に pubをつけただけではフィールドはpublicにならない
// フィールド全てをpublicにするか、
// インスタンスを返すpublicな型関連関数newを定義する
pub struct TestStruct {
    val1: i32,
    val2: i32,

    // pub val1: i32,
    // pub val2: i32,
}

impl TestStruct {
    pub fn new(val1: i32, val2: i32) -> TestStruct {
        TestStruct { val1, val2 }
    }
}