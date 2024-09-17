use janken::modules::fizzbuzz::fizzbuzz;
use janken::modules::input::input;
use janken::modules::janken::janken;
use janken::modules::kazuate::kazuate;

fn main() {
    println!("初心に帰ってコーディングをやるリポジトリ");

    println!("起動したいものを以下から選択して数値を入力");
    println!("1:じゃんけん 2:数当て 3:FizzBuzz");
    let choice = input();
    match choice {
        1 => janken(),
        2 => kazuate(),
        3 => fizzbuzz(),
        _ => panic!("正しい数値を入力してください。入力された値：{}", choice),
    }
}
