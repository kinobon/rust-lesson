struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct PointAnother<T, U> {
    x: T,
    y: U,
}

// 構造体にimplementationでメソッドを追加できる //クラスにメソッド追加するのに似ている
impl<T, U> PointAnother<T, U> {
    // 構造体のメソッドの第1引数にはselfを指定
    // １つの構造体に対してインスタンスは複数生成できるので
    // mixupメソッド実行時にどのインスタンスから実行しているか判別するために必要
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}

// impl PointAnother<T, U> {
//     fn Display(self) {

//     }
// }

pub fn run() {
    // vec![] はベクターを作るマクロ
    let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest is {}", largest);
    // println!("{}", largest_i32(number_list)); // コメントアウトしないとlargest_i32に所有権が移動、関数実行後にnubmer_listがdropされるので下のlargestで引数で渡せない

    // "" で囲うと文字列リテラル、 '' で囲うとchar型
    let char_list = vec!['a', 'b', 'c', 'd']; // char 4byte
    println!("{}", largest(char_list));
    println!("{}", largest(number_list));

    // 構造体
    let p1 = Point { x: 1, y: 2 }; // 上記で定義した構造体のインスタンスを作成
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = PointAnother { x: 5, y: 10.4 };
    let p4 = PointAnother { x: "Rust", y: 'a' };
    let p5 = p3.mixup(p4);
    println!("{} {}", p5.x, p5.y);
    println!("{:?}", p5); // structの上に#[derive(Debug)]で実装される ? Debug用
}

fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// generics
// genericsとして許容する型をトレイト境界を指定することで絞り込むことができる
// 比較が可能な型をgenericsとして許容する trait bound トレイト境界 と呼ぶ
// : <指定したいトレイト境界>で指定できる
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
