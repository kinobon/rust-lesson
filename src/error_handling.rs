// rustにおけるエラーハンドリング
// 標準でOption型とResult型が用意されている
// これらは列挙型である
// Option型は値が存在しない場合に対して例外処理を簡単に書ける,None, Some<T>
// Result型はエラーが発生する可能性がある値に対して例外処理を簡単に書ける,OK<T>, Err<E>

// 実行時例外が起きるとパニックが起きて処理が停止するがエラーハンドリングをすることで
// 処理が止まらずに済む

// 例外処理の流れ(by Copilot)
// 1. panic!マクロを呼び出す
// 2. panic!マクロはエラーメッセージを出力し、スタックをアンロールして、プログラムを終了する
// 3. panic!マクロはデフォルトでスタックをアンロールして、プログラムを終了する
// 4. スタックをアンロールすると、スタックに積まれていた関数の呼び出しを順番に取り出す
// 5. 関数の呼び出しを取り出すと、その関数のローカル変数をすべて破棄する
// 6. これを繰り返して、プログラムの実行を終了する

pub fn run() {
    let res1 = division_option(5.0, 0.0f64);
    // match式でOption型を処理する
    match res1 {
        Some(x) => println!("Result: {:.3}", x), // 小数第3位まで表示
        None => println!("Not allowed !!"),
    }

    // division(5, 9f64);
    let res2 = division_result(5.0, 0.0);
    match res2 {
        Ok(x) => println!("Result: {:.3}", x),
        Err(e) => println!("Error: {}", e),
    }

    let a = [0, 1, 2];
    let res3 = sum(&a);
    match res3 {
        Some(x) => println!("Toatal is: {}", x),
        None => println!("Out of index !!"),
    }
}

fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None // 数字を0で割るのは無理なのでNoneを返す
    } else {
        Some(x / y)
    }
}

fn division_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("Not allowed !!"))
    } else {
        Ok(x / y)
    }
}

// 配列の存在しないインデックスにアクセスしたときにエラーハンドリング
// i32型の配列の先頭アドレスを引数に受け取る
fn sum(a: &[i32]) -> Option<i32> {
    // get で配列の要素を取得する、a[<index>]で取得しないのは?を使いたいからだと思う
    // ?をメソッドの後ろにつけているので
    // インデックスが存在しない場合はその時点で処理を抜けてNoneを返す
    // 例外をcatchして処理を抜けることができる
    let a0 = a.get(0)?;
    let a1 = a.get(1)?;
    let a2 = a.get(2)?;
    Some(a0 + a1 + a2)
}

// // 引数に数値型のジェネリック型をとり、返り値はResult型
// fn division<T: std::ops::Div<Output = T> + Copy>(x: T, y: T) -> Result<T, String> {
//     if y == 0.0 {
//         Err(String::from("Not allowed !!"))
//     } else {
//         Ok(x / y)
//     }
// }
