// テストコードを実際のコードと同じファイルに書くことができる
// テストコードは、テスト対象のコードの上に書く

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 実データを渡してしまうと所有権がメソッドに移ってしまうので、参照を渡す
    fn compare_area(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }
}

fn double_value(x: i32) -> i32 {
    x * 2
}

// &str 文字列スライス
fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// cargo test でテスト実行
// cfgアトリビュート テスト実行したときのみコンパイルされるようになる
// (通常のコンパイルではコンパイルされない)
#[cfg(test)]
// ファイルの中にmodを書くと、
// ファイル内にサブモジュールを記述することができる
mod tests {
    // use で外部のモジュールをインポートできる
    // super:: で親モジュールの要素を参照できる
    // * で全ての要素をインポートできる
    use super::*;

    // #[test] でテスト関数を定義する
    #[test]
    fn test_a_is_larget() {
        let a = Rectangle {
            width: 5,
            height: 5,
        };
        let b = Rectangle {
            width: 3,
            height: 3,
        };
        assert!(a.compare_area(&b));
    }

    #[test]
    fn test_a_is_smaller() {
        let a = Rectangle {
            width: 3,
            height: 3,
        };
        let b = Rectangle {
            width: 5,
            height: 5,
        };
        assert!(!(a.compare_area(&b)));
    }

    #[test]
    fn test_double() {
        assert_eq!(6, double_value(3));
    }

    #[test]
    fn test_contains_name() {
        let res = greeting("rust");
        assert!(res.contains("rust"));
    }
}

// struct Complex {
//     real: f64,
//     imag: f64,
// }

// impl Complex {
//     fn new(real: f64, imag: f64) -> Self {
//         Self { real, imag }
//     }

//     fn abs(&self) -> f64 {
//         (self.real * self.real + self.imag * self.imag).sqrt()
//     }
// }

// struct Quaternion {
//     real: f64,
//     imag: f64,
//     jmag: f64,
//     kmag: f64,
// }

// impl Quaternion {
//     fn new(real: f64, imag: f64, jmag: f64, kmag: f64) -> Self {
//         Self {
//             real,
//             imag,
//             jmag,
//             kmag,
//         }
//     }

//     fn abs(&self) -> f64 {
//         (self.real * self.real
//             + self.imag * self.imag
//             + self.jmag * self.jmag
//             + self.kmag * self.kmag)
//             .sqrt()
//     }

//     fn mul(&self, other: &Self) -> Self {
//         Self {
//             real: self.real * other.real
//                 - self.imag * other.imag
//                 - self.jmag * other.jmag
//                 - self.kmag * other.kmag,
//             imag: self.real * other.imag + self.imag * other.real + self.jmag * other.kmag
//                 - self.kmag * other.jmag,
//             jmag: self.real * other.jmag - self.imag * other.kmag
//                 + self.jmag * other.real
//                 + self.kmag * other.imag,
//             kmag: self.real * other.kmag + self.imag * other.jmag - self.jmag * other.imag
//                 + self.kmag * other.real,
//         }
//     }
// }

// fn affinetransform(x: f64, y: f64, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> (f64, f64) {
//     (a * x + c * y + e, b * x + d * y + f)
// }

// fn get_inverse(a: f64, b: f64, c: f64, d: f64) -> (f64, f64, f64, f64) {
//     let det = a * d - b * c;
//     (d / det, -b / det, -c / det, a / det)
// }
