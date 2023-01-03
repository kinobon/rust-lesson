#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 大文字始まりの Self は Rectangle という自身のデータ型を指す
    // クラスの静的メソッドっぽい
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    // self をつけると対象のインスタンスに対するメソッドとして呼び出すことができる
    // メソッドが参照ではなく値として引数を受け取った場合、所有権はメソッドに移るため、
    // その変数はインスタンスにアクセスすることが不可能になります。
    fn area(&self) {
        println!("{}", self.width * self.height);
    }

    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }
}

pub fn run() {
    let user1 = User {
        username: String::from("someone@example.com"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };

    // 構造体も所有権の概念がある

    // mutableな構造体
    let mut user1 = User {
        username: String::from("someone@example.com"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };
    // emailフィールドを変更
    user1.email = String::from("anotheremail@example.com");
    // println!("{:?}", user1); // deriveでDebug traitを実装後、出力できる
    println!("{:#?}", user1); // フィールドごとに改行されて出力される(見やすい)

    let user2 = build_user(String::from("user2@xxx.com"), String::from("user2"));
    println!("{:#?}", user2);

    let rect = Rectangle::create(20, 20);
    println!("{:#?}", rect);
    rect.area();
    println!("{:#?}", rect); // 参照をareaに渡しているので所有権のエラーは起きない
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
