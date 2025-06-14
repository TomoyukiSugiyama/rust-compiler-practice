// This program is a sample code that can be compiled by a custom compiler.
// Displays the initial Othello board.
// A sample program to display an 8x8 board.
// Implements the following features:
// - Support for array literals
// - Support for string literals
// - For loops with initialization, condition, and increment expressions
// - Arithmetic operations and index access
// - Function definitions and return values
// - Writing to standard output (write function)
//

fn displayboard() -> i32 {
    let board = [
        0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,
        0,0,0,1,2,0,0,0,
        0,0,0,2,1,0,0,0,
        0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0
    ];
    let indexlist = [
        "1", "2", "3", "4", "5", "6", "7", "8"
    ];

    write("  A  B  C  D  E  F  G  H\n");
    let idx = 0;
    let cell = 0;
    for ( i=0; i<8; i=i+1 ) {
        write(indexlist[i]);
        write(" ");
        for ( j=0; j<8; j=j+1 ) {
            idx = i*8+j;
            cell = board[idx];

            if ( cell==0 ) {
                write("・");
            }
            if ( cell==1 ) {
                write("○ ");
            }
            if ( cell==2 ) {
                write("● ");
            }
            if ( j<7 ) {
                write(" ");
            }
        }
        write("\n");
    }
    return 0;
}

fn main() {

    displayboard();

    return 0;
}
