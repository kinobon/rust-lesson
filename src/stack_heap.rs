// 15章 smart pointer
// enum List {
//     // 列挙型はそれぞれのデータ型のサイズで容量を確保する
//     Node(i32, List), // i32 4byte List ... サイズが決まらない box pointer化するとポインタは8byte(32bit osだと4byte arc)なので8byteで確定する
//     Nil,             // データがないので0byte
// }

enum List {
    // 列挙型はそれぞれのデータ型のサイズで容量を確保する
    Node(i32, Box<List>), // i32 4byte List ... サイズが決まらない box pointer化するとポインタは8byte(32bit osだと4byte arc)なので8byteで確定する
    Nil,                  // データがないので0byte
}

pub fn run() {
    // stack overflowを発生させる
    // 8mbyteを超えると発生
    // let a1: [u8; 7_000_000] = [1; 7_000_000]; // stackの中で7Mbyte
    // let a1: [u8; 9_000_000] = [1; 9_000_000];// 上限を超えるのでエラー

    // stackの中に過度な大きさのデータは格納できない

    // 実行時に配列の要素を動的に変更するための Vector型
    // 生成のためにはvec!マクロを使う
    let mut v1 = vec![1, 2, 3, 4]; // mut で値を更新可に
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    println!("Stack address of v1: {:p}", &v1);
    println!("Stack address of v2: {:p}", &v2);
    println!("Stack address of v3: {:p}", &v3);

    /*
        メモリ構造

        String型とほぼ同じ

        Vector型の実データはHeapに確保される
        Heapにはallocatorがありメモリ確保を動的に行う
        stackに24byte分領域が確保
        ptr(8 byte) ... Heapに格納されている実データの先頭アドレスを<所有>
        len(8 byte) ... 実データの要素数 ※バイト数ではない
        cap(8 byte) ... Heapの中で実データが使用できる最大要素数

        v1の１つの要素はi32で4byteなので4byte毎にHeapに要素が格納される
        1行の長さは64bit os なら 8byte
    */

    println!("Heap memory address of v1 is: {:?}", &v1.as_ptr());
    println!("Len of v1 is: {}", &v1.len());
    println!("Capacity of v1 is: {}", &v1.capacity());
    v1.insert(1, 10);
    println!("{:?}", v1);
    v1.remove(0);
    println!("{:?}", v1);
    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3); // 連結後は空になる

    // ポインタは先頭アドレスの番地とそこにあるデータの型の情報も保持している
    // 要素数が分かればそのバイト分ずつ飛んで辿れる

    // Boxポインタ
    // stackにあるデータをheapに移動させて、移動させたデータに対するポインタを
    // Boxポインタに保有する
    // 使い道,,, コンパイル時にサイズが決まらないデータをheapに移動させて
    // そのデータのポインタを保有する、ポインタ自体は8byteなのでコンパイルが通る
    // stackのデータをheapに移してそのデータの所有権を持ったbox pointerをstackに新たに作成
    // stackの中に所有権を持ったポインタがある場合、それはheapに移っても保持される

    let t1: (i64, String) = (10, String::from("hello"));
    println!("Stack address of tuple data is: {:p}", &t1); // tupleの先頭アドレス
    println!("Heap memory address of t1.1: {:?}", t1.1.as_ptr()); // heapに格納されているhelloの先頭アドレス
    println!("Len of t1.1 is: {}", t1.1.len());
    println!("Capacity of t1.1 is: {}", t1.1.capacity());

    // Boxポインタ
    let mut b1 = Box::new(t1); // Boxポインタを通してtupleの値を操作したいのでmut
    (*b1).1 += "world";
    println!("{} {}", b1.0, b1.1);
    println!("Stack address of box pointer is: {:p}", &b1); // b1のアドレス
    println!("Heap address of tuple data is: {:p}", b1); // b1の値はHeapに移したデータの先頭アドレス

    // boxポインタ使い道 上に記述した列挙型
    // the book 15章 smart pointer
}
