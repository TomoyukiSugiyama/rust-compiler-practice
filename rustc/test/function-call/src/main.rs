unsafe extern "C" {
    // アセンブリ内で定義した関数を宣言
    fn test();
}

#[unsafe(no_mangle)]
pub extern "C" fn foo() {
    println!("foo");
}

fn main() {
    // unsafeブロック内で呼び出す
    unsafe {
        test();
    }
}
