extern crate rand;

use std::io;

// extern crate rand; すると、use rand されるのでいらない
//use rand;

// gen_range を使うために Rng スコープに入ってる必要がある
// メソッドは「トレイト」と呼ばれるもので定義されていて、メソッドが動作するにはそのトレイトがスコープにある必要がある
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // let がミュータブルな束縛 guess を導入
    // 空の String を作っている
    let mut guess = String::new();

    // io の関連関数を呼び出している
    // stdin() は標準入力へのハンドルを返す
    io::stdin()
        // use std::io; してない場合は std:io::stdin() と記述する
//    std::io::stdin().read_line(&mut guess)
        // このハンドルを使って入力を取得
        // メソッドは型ではなくインスタンスに対してだけ使える
        // read_line は &mut String を受け取る
        .read_line(&mut guess)
        // read_line() が汎用の Result を返し、それをサブライブラリに特殊化したバージョンの io::Result になる
        // Result 型の目的はエラーハンドリング情報をエンコードすること
        // expect が呼び出された値が成功でなければメッセージとともに panic! する
        // panic! はメッセージを表示してプログラムをクラッシュさせる
        // expect を呼ばなかったら、Rust は警告してくる
        // 警告は io::Result が持つ特別なアノテーションに由来
        // Rustはエラーの可能性があるのに処理していないことを教えてくれる
        .expect("Failed to read line");

    // {} はプレースホルダーで、引数として guess を渡している
    // 複数 {} があれば、複数の引数を渡す
    println!("You guessed: {}", guess);
}
