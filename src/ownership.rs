pub fn run() {
    let s1 = String::from("hello"); // String型 所有権が適用
    let s2 = s1; // 所有権がs1からs2に移動

    // Copy trait が実装されている型 主にstackで完結するデータ型 (整数型、少数型、配列)はmoveが発生せず、単純にstack内でコピーが行われる

    // String, Vector, Box の値をコピーしようとすると所有権の移動が発生する
    // なぜか
    // shallow copy の場合、片方の値が使用しなくなり解放されると、そのポインタの参照している実データも解放される
    // その状態でheapに新たなデータが格納されて、もう片方の値も解放されると参照しているアドレスのデータが解放されておかしくなる
    // このような事態を防ぐために所有権の概念があり、所有権の移動が発生しない = 安全なコードということになる

    // println!("{} {}", s1, s2); // s1からs2に所有権がMoveしておりs1へのアクセスはエラーになる
    println!("{}", s2); // 現在アドレスへの所有権をもつs2のみアクセス可

    // 値がコピーされる場合
    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2); // 整数型はCopy traitを実装しているので単純にstack内に値がコピーされる(アドレスが別になる)

    println!("Stack address of i1 is: {:p}", &i1);
    println!("Stack address of i2 is: {:p}", &i2);

    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);

    // 文字列スライスの場合は、たぶん実データが不変なのでそのままコピーするだけでいい
    println!("Stack address of sl1 is: {:p}", &sl1);
    println!("Stack address of sl2 is: {:p}", &sl2);

    // deep copy ... 実データもコピーしてそのデータに対して参照（所有）する heapのメモリ容量が圧迫される
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("Stack address of s3 is: {:p}", &s3);
    println!("Stack address of s4 is: {:p}", &s4);
    println!("Heap memory address of s3 is: {:p}", s3.as_ptr());
    println!("Heap memory address of s4 is: {:p}", s4.as_ptr());
    println!("{} {}", s3, s4);

    // moveは関数の引数、戻り値に与えた場合も起こる
    let s5 = String::from("hello");
    // 所有権が移動すると、例えばStringだとptr,len,capの値が移動先に移る
    println!("Stack address of s5 is: {:p}", &s5);
    println!("Heap memory address of s5 is: {:?}", &s5.as_ptr());
    println!("Len of s5 is: {}", &s5.len());
    println!("Capacity of s5 is: {}", &s5.capacity());
    take_ownership(s5);
    // println!("{}", s5); // take_ownershipへ所有権が移動したので以降はアクセスできない
    // 関数実行後 引数の値が解放、その後heapの実データ(s5のhello)も解放(drop)される

    // moveは関数の引数、戻り値に与えた場合も起こる
    let s6 = String::from("hello");
    println!("Stack address of s6 is: {:p}", &s6);
    println!("Heap memory address of s6 is: {:?}", &s6.as_ptr());
    println!("Len of s6 is: {}", &s6.len());
    let s7 = take_giveback_ownership(s6); // s6はもうアクセスできない
    println!("Stack address of s7 is: {:p}", &s7);
    println!("Heap memory address of s7 is: {:?}", &s7.as_ptr());
    println!("Len of s7 is: {}", &s7.len());
    // 所有権 s6 -> take_giveback_ownership -> s7
    // s7がheapのメモリを解放する責任を負う
    // それぞれの変数はstackの違うアドレス上に生成されるが所有権が移動していく

    let s8 = String::from("hello");
    let _len = calculate_length(&s8); // 参照(reference)を使用、s8のスタック内の先頭アドレスを渡している

    // 参照から変更するのでs9もmutつける
    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}", s9);

    // immutable な reference は複数作ることができる
    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2); // Stringも

    // mutable な reference と immutable な reference は共存できない
    // データの整合性を保つため
    // 片方がimmutableでももう片方がmutableで変更出来たら意味ないですね
    // let mut s10 = String::from("hello");
    // let r1 = &s10;
    // let r2 = &mut s10;
    // println!("{} {} {}", s10, r1, r2);

    let mut s11 = String::from("hello");

    let r1 = &mut s11; // 以下 immutable な reference が有効な領域
                       // println!("{}", s11); // immutable な reference が有効な領域では所有権者ですら値を読めない
    println!("{}", r1); // immutable な reference が有効な領域 ここまで(r1が最後に使われる場所)
    println!("{}", s11);

    // 有効範囲の例 2
    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{} and {}", r1, r2); // println!実行した以降、r1,r2を使用しなければ mutableなreferenceを作成できる
    let r3 = &mut s12;
    *r3 = String::from("hello_update");
    println!("{}", r3); // println!("{}", *r3); で参照外ししないと出力できないと思ったがそうでもない
                        // life time

    // 参照と借用 References and Borrowing
    // 借用: 所有権を移動させずに参照する権利だけを貸し出す
    // 所有権があるデータに対して参照を使用することを借用と言うのか？

    // 参照のメモリ表現
    // let s1 = String::from("hello");
    // let sr1 = &s1;
    // let sr2 = &s1;
    // s1はStringでheapに格納されている実データ"hello"の先頭アドレスへのポインタptrとlen,capが入っている
    // sr1,sr2はポインタでstackに格納されている変数s1の先頭アドレスが入っている
    // println!("{}", *sr1);
}

fn take_ownership(s: String) {
    println!("Stack address of s is: {:p}", &s);
    println!("Heap memory address of s is: {:?}", &s.as_ptr());
    println!("Len of s is: {}", &s.len());
    println!("Capacity of s is: {}", &s.capacity());
    println!("{}", s);
}
// 関数実行後 sの値が解放、その後heapの実データ(s5のhello)も解放(drop)される

// rustではreturn文を明示的に書かず、
// 最後の行でセミコロンがないものが返り値として判定される　<= エラー出たよ、明示してたらセミコロン書かなくてもエラー出なかった
// 型を指定する場合は -> 型で指定する
// ドキュメント見た感じ最後のreturnは省略できそうだが戻り値は明示しないとダメっぽい
fn take_giveback_ownership(s: String) -> String {
    s
}

// 所有権をmoveさせるのではなく参照を使用する
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 参照から値を変更 mutableな参照を受け取る
fn change(s: &mut String) {
    s.push_str("_world");
}
