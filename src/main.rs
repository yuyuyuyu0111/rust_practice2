use std::io::stdin;

fn main() {
    println!("じゃんけん開始！");
    let mut index: i64 = 0;

    loop {
        println!("次に出す手を入力してください");
        println!("1:グー 2:チョキ 3:パー 0:終了");
        let a = input();
        if a == 0 {
            break;
        }

        let com_hand = index % 3+1;

        println!("あなたの出した手は{}", hands(a));
        println!("COMの出した手は{}", hands(com_hand));

        if com_hand == a {
            println!("あいこ");
        }

        if index == i64::MAX {
            //オーバーフロー対策　9223372036854775807回もじゃんけんするなという話だが
            index = 0;
        } else {
            index = index + 1;
        }
    }
}

fn input() -> i64 {
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}
fn hands(hand_num: i64) -> &'static str {
    match hand_num {
        1 => return "グー",
        2 => return "チョキ",
        3 => return "パー",
        _ => {
            panic!("入力してはいけない値が入力されています")
        }
    }
}
