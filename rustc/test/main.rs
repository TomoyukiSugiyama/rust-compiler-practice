use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::process::{Command, exit};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};

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

    let args: Vec<String> = env::args().collect();
    let parallel: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(10);

    // Build the rustc compiler once to avoid sequential cargo runs
    let build_status = Command::new("cargo")
        .arg("build")
        .arg("--bin")
        .arg("rustc")
        .status()
        .unwrap_or_else(|e| {
            eprintln!("failed to build rustc: {}", e);
            exit(1);
        });
    if !build_status.success() {
        eprintln!("cargo build --bin rustc failed");
        exit(1);
    }

    // Determine path to the compiled rustc binary (same profile as this runner)
    let mut rustc_path = env::current_exe().unwrap();
    rustc_path.pop(); // remove test-runner binary name
    rustc_path.push("rustc");
    let rustc_bin = Arc::new(rustc_path);

    let (tx, rx) = mpsc::channel();
    for &(expected, input) in tests {
        tx.send((expected, input)).unwrap_or_else(|e| {
            eprintln!("failed to send test to channel: {}", e);
            exit(1);
        });
    }
    drop(tx);
    let rx = Arc::new(Mutex::new(rx));

    // Channel for structured test results
    let (tx_res, rx_res) = mpsc::channel::<(String, i8, i8, Duration, bool)>();

    let mut handles = Vec::new();
    for _ in 0..parallel {
        let rx = Arc::clone(&rx);
        let rustc_bin = Arc::clone(&rustc_bin);
        let tx_res = tx_res.clone();
        let handle = thread::spawn(move || {
            loop {
                // receive one task under lock, then drop the lock before processing
                let (expected, input) = {
                    let guard = rx.lock().unwrap();
                    match guard.recv() {
                        Ok(pair) => pair,
                        Err(_) => break, // no more tasks
                    }
                };
                let start = Instant::now();
                let test_name = std::path::Path::new(input)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap();
                let asm_path = format!("bin/arm64-{}.s", test_name);
                let bin_path = format!("bin/arm64-{}", test_name);

                // Generate assembly
                let asm_file = File::create(&asm_path).unwrap_or_else(|e| {
                    eprintln!("failed to create asm file {}: {}", asm_path, e);
                    exit(1);
                });
                let gen_status = Command::new(&*rustc_bin)
                    .arg(input)
                    .stdout(asm_file)
                    .status()
                    .unwrap_or_else(|e| {
                        eprintln!("failed to run rustc binary {:?}: {}", rustc_bin, e);
                        exit(1);
                    });
                if !gen_status.success() {
                    // report generation failure and continue
                    let duration = start.elapsed();
                    tx_res
                        .send((input.to_string(), expected, 0, duration, false))
                        .unwrap();
                    // indicate failure
                    print!("F");
                    io::stdout().flush().unwrap();
                    continue;
                }

                // Assemble
                let assemble_status = Command::new("clang")
                    .arg("-arch")
                    .arg("arm64")
                    .arg("-x")
                    .arg("assembler")
                    .arg(&asm_path)
                    .arg("-o")
                    .arg(&bin_path)
                    .status()
                    .unwrap_or_else(|e| {
                        eprintln!("failed to run clang: {}", e);
                        exit(1);
                    });
                if !assemble_status.success() {
                    // report assembly failure and continue
                    let duration = start.elapsed();
                    tx_res
                        .send((input.to_string(), expected, 0, duration, false))
                        .unwrap();
                    // indicate failure
                    print!("F");
                    io::stdout().flush().unwrap();
                    continue;
                }

                // Run binary
                let run_status = Command::new(&bin_path).status().unwrap_or_else(|e| {
                    eprintln!("failed to run binary: {}", e);
                    exit(1);
                });
                let code = run_status.code().unwrap_or_else(|| {
                    eprintln!("{} => process terminated by signal", input);
                    exit(1);
                });
                let actual = code as u8 as i8;

                // send structured result for summary
                let success = actual == expected;
                let duration = start.elapsed();
                tx_res
                    .send((input.to_string(), expected, actual, duration, success))
                    .unwrap();
                // simple progress indicator: '.' for success, 'F' for failure
                if success {
                    print!(".");
                } else {
                    print!("F");
                }
                io::stdout().flush().unwrap();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    drop(tx_res);
    // new line before summary
    println!();
    // Collect and sort results
    let mut results: Vec<_> = rx_res.into_iter().collect();
    results.sort_by(|a, b| a.0.cmp(&b.0));
    // Print organized summary
    println!(
        "{:<30} {:<6} {:<8} {:<8} {}",
        "Test", "Result", "Expected", "Got", "Time"
    );
    for (name, expected, actual, duration, success) in &results {
        println!(
            "{:<30} {:<6} {:<8} {:<8} {:?}",
            name,
            if *success { "OK" } else { "FAIL" },
            expected,
            actual,
            duration
        );
    }
    // Exit with error if any failed
    if results.iter().any(|(_, _, _, _, success)| !success) {
        exit(1);
    } else {
        println!("OK");
    }
}
