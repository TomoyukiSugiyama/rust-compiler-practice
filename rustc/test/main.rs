use std::fs::{self, File};
use std::process::{Command, exit};

fn main() {
    let tests: &[(i8, &str)] = &[
        (12, "./test/assets/number.rs"),
        (3, "./test/assets/addition.rs"),
        (1, "./test/assets/subtraction.rs"),
        (48, "./test/assets/addition-and-subtraction.rs"),
        (4, "./test/assets/whitespace.rs"),
        (7, "./test/assets/multiplication.rs"),
        (4, "./test/assets/division.rs"),
        (9, "./test/assets/parentheses.rs"),
        (-3, "./test/assets/unary-minus.rs"),
        (-8, "./test/assets/unary-neg-parens.rs"),
        (-15, "./test/assets/unary-mixed.rs"),
        (1, "./test/assets/eq-true.rs"),
        (0, "./test/assets/eq-false.rs"),
        (0, "./test/assets/lt-false.rs"),
        (1, "./test/assets/lt-true.rs"),
        (0, "./test/assets/gt-false.rs"),
        (1, "./test/assets/gt-true.rs"),
        (0, "./test/assets/le-false.rs"),
        (1, "./test/assets/le-true.rs"),
        (0, "./test/assets/ge-false.rs"),
        (1, "./test/assets/ge-true.rs"),
        (3, "./test/assets/assign-simple.rs"),
        (3, "./test/assets/assign-multi.rs"),
        (3, "./test/assets/return-stmt.rs"),
        (6, "./test/assets/assign-vars.rs"),
        (3, "./test/assets/if-else-true.rs"),
        (2, "./test/assets/if-else-false.rs"),
        (10, "./test/assets/while-loop.rs"),
        (10, "./test/assets/for-loop.rs"),
        (4, "./test/assets/nested-loop.rs"),
        (5, "./test/assets/func-call.rs"),
        (55, "./test/assets/fibonacci-allow-warnings.rs"),
        (3, "./test/assets/deref.rs"),
        (10, "./test/assets/local-var.rs"),
        (10, "./test/assets/comments.rs"),
    ];

    fs::create_dir_all("bin").unwrap_or_else(|e| {
        eprintln!("failed to create bin directory: {}", e);
        exit(1);
    });

    for &(expected, input) in tests {
        let asm_path = "bin/test-arm64.s";
        let bin_path = "bin/test";

        // Generate assembly
        let asm_file = File::create(asm_path).unwrap_or_else(|e| {
            eprintln!("failed to create asm file {}: {}", asm_path, e);
            exit(1);
        });
        let gen_status = Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg("rustc")
            .arg("--")
            .arg(input)
            .stdout(asm_file)
            .status()
            .unwrap_or_else(|e| {
                eprintln!("failed to run cargo run: {}", e);
                exit(1);
            });
        if !gen_status.success() {
            println!("{} => failed to generate assembly", input);
            exit(1);
        }

        // Assemble
        let assemble_status = Command::new("clang")
            .arg("-arch")
            .arg("arm64")
            .arg("-x")
            .arg("assembler")
            .arg(asm_path)
            .arg("-o")
            .arg(bin_path)
            .status()
            .unwrap_or_else(|e| {
                eprintln!("failed to run clang: {}", e);
                exit(1);
            });
        if !assemble_status.success() {
            println!("{} => failed to assemble", input);
            exit(1);
        }

        // Run binary
        let run_status = Command::new(bin_path).status().unwrap_or_else(|e| {
            eprintln!("failed to run binary: {}", e);
            exit(1);
        });
        let code = run_status.code().unwrap_or_else(|| {
            eprintln!("{} => process terminated by signal", input);
            exit(1);
        });
        let actual = code as u8 as i8;

        if actual == expected {
            println!("{} => {}", input, actual);
        } else {
            println!("{} => {} expected, but got {}", input, expected, actual);
            exit(1);
        }
    }

    println!("OK");
}
