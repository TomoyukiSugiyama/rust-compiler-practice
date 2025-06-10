unsafe extern "C" {
    // アセンブリ内で定義した関数を宣言
    fn test();
}

#[unsafe(no_mangle)]
pub extern "C" fn debug1(x: i32) {
    println!("x = {}", x);
}

#[unsafe(no_mangle)]
pub extern "C" fn debug2(x: i32, y: i32) {
    println!("x = {}", x);
    println!("y = {}", y);
}

#[unsafe(no_mangle)]
pub extern "C" fn debug3(x: i32, y: i32, z: i32) {
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
}

#[unsafe(no_mangle)]
pub extern "C" fn debug4(x: i32, y: i32, z: i32, w: i32) {
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    println!("w = {}", w);
}

#[unsafe(no_mangle)]
pub extern "C" fn debug5(x: i32, y: i32, z: i32, w: i32, v: i32) {
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    println!("w = {}", w);
    println!("v = {}", v);
}

fn main() {
    // unsafeブロック内で呼び出す
    unsafe {
        test();
    }
}
