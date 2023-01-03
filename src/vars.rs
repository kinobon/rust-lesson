// varsから普通に呼び出せるが
// 親モジュールから呼び出すにはpubをつける必要がある
pub mod sub_a;
pub mod sub_b;

const MAX_POINTS: u32 = 100_000; // 型を明示する

// const はスコープ関係なく定義できる
// let は関数のスコープ外で定義できない

// デフォルトはprivate
pub fn run() {
    println!("Here is vars module");
    // sub_a::func_a();
    // sub_b::func_b();

    // mutable immutable
    let mut x = 5; // 5 をバインド(代入)
    println!("The value of x is: {}", x);
    x = 6; // デフォルトではimmutableなのでエラーでるのmutつける
    println!("The value of x is: {}", x);

    // 型推論
    // let i1 = 4294967295; 範囲外のエラーおそらく
    // 未使用の変数に対するコンパイラの警告を無効化
    let _i1: u32 = 4294967295;
    let _f1 = 0.1;

    println!("{}", usize::BITS); // 64bit os なら64が出る

    // アドレスの取得 :pでポインタ表記で出力
    println!("Memory address of const is: {:p}", &MAX_POINTS); // constは静的領域に確保されている

    let i2: i64 = 1; // 8byte
    let i3: i64 = 2;

    println!("Stack address of i2 is: {:p}", &i2);
    println!("Stack address of i3 is: {:p}", &i3);

    // アドレスが1増えると1byte分ずれる

    // シャドーイング（再宣言）
    let y = 5;
    println!("Stack address of y is: {:p}", &y);
    let y = y + 1; // yを再宣言 5 + 1 がyにバインド、それ以前のyは隠れる
    println!("Stack address of y is: {:p}", &y);
    let y = y * 2;
    println!("Stack address of y is: {:p}", &y);
    println!("The value of y is: {}", y);

    // バインドを行うと新しい領域に格納される

    {
        let y = 0;
        println!("The value of y is: {}", y); // 0
    }

    println!("The value of y is: {}", y); // 12

    // tuple
    let t1 = (500, 6.4, "dummy"); // 違うデータ型持てる "dummy"文字列スライス
    let (_x, _y, _z) = t1;
    println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));

    // refキーワードでtupleの要素のポインタを分割して取得できる
    // i32 4バイトデータが格納されている領域の先頭アドレスをそれぞれ取得している
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2; // 2つめの値は取得しない

    /*
        参照から実データにアクセスするためには*をつける
        参照に*をつけて実データにアクセスすることを参照外し(dereference)という
        *は参照外し演算子(dereference operator)と呼ばれる
    */
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("{:?}", t2); // プリミティブ型 {} でも出せる。タプルとか複雑になると{:?}をつける

    // 配列 コンパイル時にサイズが決まっている必要がある => サイズがきまっているのでstackに積まれる
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);

    // let mut d = 5;
    // let ref mut dp = d;
    // println!("{}", *dp);
    // *dp = 8;
    // println!("{}", d);

    // 文字列スライス型
    let s1 = "helloこんにちは挨拶"; // 実データ 26byte  utf-8 1byte*5 + 3byte*7
    let s2 = "hello";
    println!("Stack address of s1 is: {:p}", &s1); // 16byte分しか確保されてなかった？
    println!("Stack address of s2 is: {:p}", &s2);

    /*
        メモリ構造
        文字列スライスの実データは静的領域に確保される
        stackに16byte分領域が確保
        ptr(8 byte) ... 静的領域に格納されている実データの先頭アドレスの<参照> 例) hello なら hのアドレス
        len(8 byte) ... 実データのバイト数 例) hello なら 5バイト
    */

    println!("Stack memory address of s1 is: {:?}", s1.as_ptr());
    println!("Stack memory address of s2 is: {:?}", s2.as_ptr());
    println!("Stack memory address of s1 is: {}", s1.len());
    println!("Stack memory address of s2 is: {}", s2.len());

    /*
        文字列スライス 実データが静的領域に確保されるため、アプリケーション実行時に文字数を変更したりできない
        そのため、動的に文字数を増やすためにString型を使用する、heapに格納される
        heap プライオリティキュー...追加自由、、、取り出しは最小値から
    */
    // String変更するためにmut付ける
    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");
    println!("Stack address of s1 is: {:p}", &s1); // アドレス24byte分確保されている
    println!("Stack address of s2 is: {:p}", &s2);

    /*
        メモリ構造
        文字列スライスの実データはHeapに確保される
        Heapにはallocatorがありメモリ確保を動的に行う
        stackに24byte分領域が確保
        ptr(8 byte) ... Heapに格納されている実データの先頭アドレスを<所有> 例) hello なら hのアドレス
        len(8 byte) ... 実データのバイト数 例) hello なら 5バイト
        cap(8 byte) ... Heapの中で実データが使用できる最大容量(値はrustがアプリケーション内での最大のlenより少し余裕を持つように割り当て、見た感じだと最大のlenと同じっぽいけど)
    */

    println!("Heap memory address of s1 is: {:?}", &s1.as_ptr());
    println!("Heap memory address of s2 is: {:?}", &s2.as_ptr());
    println!("Len of s1 is: {}", &s1.len());
    println!("Len of s2 is: {}", &s2.len());
    println!("Capacity of s1 is: {}", &s1.capacity());
    println!("Capacity of s2 is: {}", &s2.capacity());

    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("{} {}", s1, s2);

    // let s3: &str = s1.as_str();

    // println!("Len of s1 is: {}", &s1.len());
    // println!("Len of s2 is: {}", &s2.len());
    // println!("Capacity of s1 is: {}", &s1.capacity());
    // println!("Capacity of s2 is: {}", &s2.capacity());

    // s1.clear();
    // s2.clear();

    // clear後もcapは0にならない
    // println!("Len of s1 is: {}", &s1.len());
    // println!("Len of s2 is: {}", &s2.len());
    // println!("Capacity of s1 is: {}", &s1.capacity());
    // println!("Capacity of s2 is: {}", &s2.capacity());

    // 文字列スライスは実データが格納される静的領域への参照、
    // String型は実データが格納されるHeapへの所有

    /*
        所有権による二重解放エラー回避

        所有権者のみヒープのメモリを解放出来る
        所有の関係があるデータ型にString型やVector型BoxPointer型がある

        ・所有権は、データに対して必ず一人
        ・所有権者がメモリを解放する->解放(drop)はrustによって自動的に行われる
    */

    // 文字列スライスはなぜデータを所有しない
    // ・文字列リテラルから作る場合は静的領域に実体があるので解放の必要がない
    // ・String型から文字列スライスを作った場合は、参照する権利を借用する形になる

    // 参照と借用
    // 所有権を移動させずに参照する権利だけを貸し出す。
}
