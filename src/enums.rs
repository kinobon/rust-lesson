// 列挙型
// enumの要素はデータを(型, 型, ....)で保有できる
enum OS {
    Windows(u32, String),
    Mac(u32, String),
    Linux(u32, String),
}

pub fn run() {
    // enumの実体を作成
    let linux = OS::Linux(1991, String::from("Linus"));
    print_os_info(linux);
    let windows = OS::Windows(1985, String::from("Microsoft"));
    print_os_info(windows);
    let mac = OS::Mac(2001, String::from("Apple"));
    print_os_info(mac);

    // matching pattern <= 通常のswitch文にあたる
}

fn print_os_info(os: OS) {
    // パターンマッチングにはmatchを使用する
    // match <判定したい変数>
    // caseは <パターン> => <処理>
    match os {
        OS::Windows(year, who) => {
            println!("Windows : First release in {} by {}", year, who);
        }
        OS::Mac(year, who) => {
            println!("MacOS : First release in {} by {}", year, who);
        }
        OS::Linux(year, who) => {
            println!("Linux : First release in {} by {}", year, who);
        }
    }
}
