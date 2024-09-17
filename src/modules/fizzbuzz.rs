use crate::modules::input::input;

pub fn fizzbuzz() {
    println!("FizzBuzzを眺めよ");
    println!("とりあえず、上限を入力（0以下の値は適切な値に変換されます。）");
    let input_num = input();

    println!("-----START-----");

    let limit = if input_num == 0 { 1 } else { input_num.abs() };
    for i in 1..=limit {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}
