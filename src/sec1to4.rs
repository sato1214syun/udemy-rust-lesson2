pub fn run() {
    println!("Hello, world!");
    print!("Hello, ");
    print!("Rust!");
    println!("Hello, {}", "students");

    // 定数には型宣言が必須。グローバルスコープに宣言できる
    const A: i32 = 1;

    //数値型と論理型
    // 型の変換はas f64など

    //タプル型
    let t1 = (1, true, 2.0);
    println!("{:?}", t1);  // ?はデバッグフォーマットと呼ばれる

    let u = ();  // 空のタプルユニット型。何もない値を示す。何も返さない関数の戻り値などで使われる

    // 配列。タプルに似ているが中身は全て同じ型[]でくくる
    // 要素数は固定→要素数を変更したいときはベクタ型を使用する
    // スライス &配列[0..X]

    // ベクタ型
    let v1 = vec![1, 2, 3];
    let v2 = vec![0; 1000];

    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);

    let x = v3.pop();
    println!("{:?}", x);
    println!("{:?}", v3);
    let z = v3.get(100);

    let s = &v3[0..2];
    println!("{:?}", s);

    // char型(文字型)

    // 文字列型
    // 文字列リテラル(スライス)とString型
}
