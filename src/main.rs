mod enums;
mod error_handling;
mod generics;
mod lifetime;
mod ownership;
mod stack_heap;
mod structs;
mod traits;
mod unit_test;
mod vars;
extern crate lib_demo; // 自作ライブラリのインポート
mod debug;
fn main() {
    //     println!("Hello, world!");
    // vars::run();
    // stack_heap::run();
    // ownership::run();
    // generics::run();
    // lifetime::run();
    // structs::run();
    // enums::run();
    // traits::run();
    // error_handling::run();
    // lib_demo::print_random_number();
    debug::run();
    //     vars::sub_a::func_a();
    //     vars::sub_b::func_b();

    //     // a, b はスコープを抜けるとメモリ解放(drop)される
    //     let a = 10; // 型推論によって型が決まるのでスタックに積まれる
    //     let b = 20; // スタックに積まれる
    //     println!("{} {}", a, b);
}
