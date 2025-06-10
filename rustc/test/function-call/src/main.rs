unsafe extern "C" {
    // アセンブリ内で定義した関数を宣言
    fn test();
}

#[unsafe(no_mangle)]
pub extern "C" fn foo() {
    println!("foo");
}

#[unsafe(no_mangle)]
pub extern "C" fn foowithargs(x: i32, y: i32) {
    println!("x = {}", x);
    println!("y = {}", y);
    println!("sum = {}", x + y);
}

fn main() {
    // unsafeブロック内で呼び出す
    unsafe {
        test();
    }
}
