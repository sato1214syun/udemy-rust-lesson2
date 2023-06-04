pub fn exercise2(end: i32) {
    let mut i = 0;
    let mut message: String;
    while i < end {
        i += 1;

        if (i % 3 == 0) && (i % 5 == 0) {
            message = String::from("FizzBuzz");
        } else if i % 3 == 0 {
            message = String::from("Fizz");
        } else if i % 5 == 0 {
            message = String::from("Buzz");
        } else {
            message = i.to_string();
        }
        println!("{}", message);
    }
}

pub fn exercise3(end: i32) {
    let mut message;
    for i in 1..=end {
        match i % 15 {
            0 => message = String::from("FizzBuzz"),
            3 | 6 | 9 | 12 => message = String::from("Fizz"),
            5 | 10 => message = String::from("Buzz"),
            _ => message = i.to_string(),
        }
        println!("{}", message);
    }
}

pub fn exercise4(end: i32) {
    let mut message;
    for i in 1..=end {
        match (i % 3, i % 5) {
            (0, 0) => message = String::from("FizzBuzz"),
            (0, _) => message = String::from("Fizz"),
            (_, 0) => message = String::from("Buzz"),
            _ => message = i.to_string(),
        }
        println!("{}", message);
    }
}
