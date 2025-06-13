fn displayboard() {
    // 列のヘッダーを表示
    write("  A B C D E F G H\n");

    // 1行目
    write("1 ・ ・ ・ ・ ・ ・ ・ ・ \n");
    // 2行目
    write("2 ・ ・ ・ ・ ・ ・ ・ ・ \n");
    // 3行目
    write("3 ・ ・ ・ ・ ・ ・ ・ ・ \n");
    // 4行目（初期配置）
    write("4 ・ ・ ・ ○ ● ・ ・ ・ \n");
    // 5行目（初期配置）
    write("5 ・ ・ ・ ● ○ ・ ・ ・ \n");
    // 6行目
    write("6 ・ ・ ・ ・ ・ ・ ・ ・ \n");
    // 7行目
    write("7 ・ ・ ・ ・ ・ ・ ・ ・ \n");
    // 8行目
    write("8 ・ ・ ・ ・ ・ ・ ・ ・ \n");
}

fn main() {
    displayboard();
}
