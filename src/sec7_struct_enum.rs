// 構造体
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangleのメソッドと型関連関数を定義
// 第一引数にselfを取る
impl Rectangle {
    // メソッド
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 型関連関数(他の言語のスタティックメソッド)
    // インスタンスを生成しなくても使用可能
    // 第一引数にselfを取らない場合は型関連関数として認識される
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

// 列挙型
// 取りうる値を列挙することで独自の型を定義する
enum Shape {
    Circle,  // バリアント
    Square(u32),  // タプルで型を定義できる(タプル型バリアント)
    Triangle{base: u32, height: u32},  // 構造体の定義と同じように、キーと値(型)で定義することもできる(構造体バリアント)
}
// 構造体と同じようにメソッドを定義可能
impl Shape {
    fn simple_method(&self) {
        println!("call simple_method");
    }
}

pub fn run() {
    // let height = 5;
    // let mut rectangle = Rectangle { width: 10, height };
    // println!("width : {}", rectangle.width);
    // println!("height : {}", rectangle.height);

    // rectangle.height = 10;
    // println!("height : {}", rectangle.height);
    // // メソッドを使用
    // println!("area : {}", rectangle.area());

    // //  型関連関数でコンストラクタ(インスタンスの生成関数)を実行
    // let mut rectangle2 = Rectangle::new(10, 5);
    // println!("width : {}", rectangle2.width);
    // println!("height : {}", rectangle2.height);

    // rectangle2.height = 10;
    // println!("height : {}", rectangle2.height);
    // println!("area : {}", rectangle2.area());

    // 列挙型
    let c = Shape::Circle;
    let s = Shape::Square(1);
    let t = Shape::Triangle { base: 10, height: 5 };
    c.simple_method();
    s.simple_method();
    t.simple_method();

    /* Option型→列挙型で定義された型の一つで、値が存在しない可能性があるものに対して使用される
    Rustには、多言語では使用される値が存在しないことを表す"Null"が存在しない。Nullはバグを引き起こす。
    かわりにOption型を使用している
    値が存在する場合はSome<T>, ない場合はNoneを返す
    */
    let a = Some(1);  // Someは型推論可能
    let b = Some("str");
    let c:Option<i32> = None;  // Noneは型推論不可なので目地する必要あり
    // Someに保持された値を扱う場合は値のチェックが必要
    let v = vec![1, 2, 3];
    let val = v.get(0);
    // matchを使ったチェック
    match val {
        // Some(1) => println!("value is 1"),  // 特定の値のときの動作も指定できる
        // Some(2 | 3) => println!("value is 2 or 3"),  // |で複数の条件のマッチを使用可能
        // Some(x)  => println!("value exists {}", x),
        Some(x) if *x == 1 => println!("value is 1"),  // マッチガード: match式のパターンに更に条件を加える事ができる
        Some(x)  => println!("value exists {}", x),
        None => println!("value not exists"),
    }
    // if letを使用したチェック。matchと違ってすべてのケースを記述しなくていい
    if let Some(x) = val {
        println!("val={}", x)
    }

    // match

}
