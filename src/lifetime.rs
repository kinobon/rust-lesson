// 参照(reference)と借用(borrowing)
// 借用: 所有権を移動させずに参照する権利だけを貸し出す
// ・参照(&によるアドレス)自体は、所有権が適用される型・プリミティブ型(Copy traitを実装している)両方に存在
// ・プリミティブ型は所有権の概念がないので借用という概念はない
// 所有権が適用される型のデータに対しての参照を作成->借用みたいな感じ
// 関数の引数に所有権が適用される型のデータの参照を渡す->参照を引数に貸し出す=その関数は所有権を借用しているみたいな感じ？

// Rustのメモリ安全性
// ・二重開放エラー -> 所有権システム の概念よりコンパイラが指摘
// ・メモリ開放忘れ -> RAII で解決
// ・ダングリングポインター -> ライフタイム の概念によりコンパイラが指摘

// RAII (Resource Acquisition Is Initialization) c++でもオプションであったが、rustは言語仕様として適用されている
// 変数初期化時にリソース(メモリ)が確保される、変数のメモリだけでなくネットワークやファイルを開いた時も
// スコープを抜けるとリソースが解放(drop)される

// 20m 21:07~

pub fn run() {
    let st1 = String::from("x");
    let st2 = String::from("y");
    let res1 = get_longest(&st1, &st2);
    println!("{}", res1);

    // let st: &str = &st2;

    // 引数のライフタイムが違う例
    let st3 = String::from("x");
    {
        let st4 = String::from("y");

        // res2のライフタイムはst4と同じになりダングリングポインタは発生しない
        // 仮にst3と同じライフタイムだとダングリングポインタが発生する
        let res2 = get_longest(&st3, &st4);
        println!("{}", res2);
    } // スコープを抜けるとst4とres2はdropされる

    // // 下記はダングリングポインタ(参照が実体より長生きしている)が発生
    // 発生しているというエラーをコンパイラが指摘してくれるのでそれを修正していく
    // runtime errorでダングリングポインタが発生することはない
    // let st3 = String::from("x");
    // let res2; // ホバーすると &strと出るがコンパイルが行われて型が決定し逆算されて推論される
    // {
    //     let st4 = String::from("y");
    //     res2 = get_longest(&st3, &st4);
    // }
    // println!("{}", res2);
}

// referenceを引数として受け取る、&str
// 下の記述だとどちらの引数のライフタイムを優先したらいいかわからない
// fn get_longest<T>(x: &str, y: &str) -> &str {
// generic lifetime annotation
// 引数で受け取ったreferenceで最小のlifetimeを返り値のreferenceのlifetimeと見なす
// これにより違うlifetimeを持つ複数の値の参照を渡してもdangling pointerを検知できる
// 'a とか 'b とか付けたりする
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// generic lifetime annotation
// どのような検討を経てこのような仕様が策定されたかは分からないが
// 必要なケースとしては
// 2つの参照を引数として借用し、その内のどちらかの参照を返す関数があるとして
// 2つの参照の実データのlifetimeが違う場合に、返り値の参照のlifetimeを最小(実データの内の最小)のlifetimeとみなすことで
// 関数実行後にdangling pointerが発生するのを防ぐために使われたりする。

// fn dummy1<'a>() -> &'a str {
//     let s = String::from("demo");
//     &s // エラー 関数のスコープを抜けると実体であるsがdropされてしまうのでdangling pointerが発生
// }

// fn dummy2<'a>() -> &'a i32 {
//     let x = 10;
//     &x // 実体のxがdropされているのに参照が返されるのでdangling pointer が発生
// }

fn dummy3() -> String {
    let s = String::from("demo");
    s // 参照が一人歩きしないのでエラーは出ない
}

// pub fn run() {
//     // let a = 5;
//     // let r = &a;
//     // println!("{}", r);
//     // println!("{}", a);

//     // dangling pointer
//     let r;
//     {
//         let x = 5;
//         r = &x; // x's ptr is bind to r

//         // x dropped here
//     }
//     // println!("r: {}", r); // 参照が実体より長生きしてはいけない

//     // rustはコンパイラが通る限りdangling pointer発生しない
// }
