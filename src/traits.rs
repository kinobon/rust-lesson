// trait
// 複数のデータ型(この場合構造体か？)に対して共通する機能を持たせたいときに使う
// インタフェースっぽい
trait Fruits {
    fn price(&self) -> u32;
}

// この宣言ではなにも書いていない
struct Apple;
// Fruits trait を struct Apple に実装
impl Fruits for Apple {
    fn price(&self) -> u32 {
        10 // 最終行セミコロンなしでreturn扱い
    }
}

struct Banana;
impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}

trait Summary {
    // 第１引数に&selfがあるやつはメンバメソッドみたいな感じ
    // fn summarize(&self) -> String;

    // トレイトのデフォルト実装
    // trait に具体的な処理を記述すると、それがデフォルトの実装になる
    // 実装するときにそちらに処理を記述すると、デフォルトの実装は上書き(オーバーライド)される
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

trait Message {
    fn message(&self) -> String {
        String::from("Message")
    }
}
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        // format!マクロで変数が埋め込まれた文字列を返す
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
impl Message for NewsArticle {}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn run() {
    let apple = Apple {};
    let banana = Banana {};
    get_price(apple);
    get_price(banana);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably alreaday know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pitsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pitsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };
    println!("{}", article.summarize());
    notify(&article);
    notify_another(&article);
    // notify_another(&tweet); // Tweetは Message trait を実装していないのでエラー
}

// generics T はFruits トレイトを実装する型のみを許容する
// トレイト境界でFruitsのみを許容
fn get_price<T: Fruits>(fruits: T) {
    println!("price is: {}", fruits.price());
}

// 引数の &impl Summaryは
// Summary trait を実装しているデータ型を引数にとるという意味
// さらに、&がついているので参照を引数に受け取る
// つまり、
// Summary trait を実装しているデータ型の参照を引数にとる関数
fn notify(item: &impl Summary) {
    // Summary trait を実装していれば summarizeメソッドを使用できる
    // 参照からメソッドを呼び出す
    println!("Breaking news! {}", item.summarize());
}

// Summary trait と　Message trait 両方を実装しているデータ型の参照を引数にとる関数
fn notify_another(item: &(impl Summary + Message)) {
    println!("Breaking news! {}", item.summarize());
    println!("Message! {}", item.message());
}
