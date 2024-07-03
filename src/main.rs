use std::io::stdin;

use rand::Rng;

fn main() {
    println!("初心に帰ってコーディングをやるリポジトリ");

    println!("起動したいものを以下から選択して数値を入力");
    println!("1:じゃんけん 2:数当て");
    let choice = input();
    match choice {
        1 => janken(),
        2 => kazuate(),
        _ => panic!("正しい数値を入力してください。入力された値：{}", choice),
    }
}
fn kazuate() {
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

fn janken() {
    //乱数初期化
    let mut rng = rand::thread_rng();

    println!("じゃんけん開始！");

    loop {
        println!("次に出す手を入力してください");
        println!("1:グー 2:チョキ 3:パー 0:終了");
        let player_hand = input();
        if player_hand == 0 {
            println!("じゃんけんを終了します");
            break;
        }

        let rand_num: i64 = rng.gen();
        let com_hand = rand_num.abs() % 3 + 1;

        println!("あなたの出した手は{}", hands(player_hand));
        println!("COMの出した手は{}", hands(com_hand));

        if com_hand == player_hand {
            println!("あいこ");
        } else if (player_hand == 1 && com_hand == 2)
            || (player_hand == 2 && com_hand == 3)
            || (player_hand == 3 && com_hand == 1)
        {
            println!("かち！");
        } else {
            println!("まけ...");
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
            panic!(
                "入力してはいけない値が入力されています。\n入力された値は{}です。",
                hand_num
            );
        }
    }
}
