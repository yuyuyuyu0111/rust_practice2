use rand::Rng;

use crate::modules::input::input;

pub fn kazuate() {
    println!("数当て開始\n （範囲は1から10まで）");

    let mut rng = rand::thread_rng();
    let num: i64 = rng.gen_range(1..10);
    loop {
        let inputted_number = input();

        if inputted_number == num {
            break;
        }
        if inputted_number > num {
            println!("入力された値は大きすぎます");
        } else {
            println!("入力された値は小さすぎます");
        }
    }

    println!("正解は{}でした！", num);
}
