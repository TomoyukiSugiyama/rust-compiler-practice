// build.rs
fn main() {
    cc::Build::new()
        .file("../../bin/test-foo.s") // アセンブリファイルのパス
        .compile("foo_lib"); // コンパイル後のライブラリ名
}
