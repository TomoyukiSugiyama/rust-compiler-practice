use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::process::{Command, exit};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let tests: &[(i8, &str, Option<&str>)] = &[
        (12, "./test/assets/number.rs", None),
        (3, "./test/assets/addition.rs", None),
        (1, "./test/assets/subtraction.rs", None),
        (48, "./test/assets/addition-and-subtraction.rs", None),
        (4, "./test/assets/whitespace.rs", None),
        (7, "./test/assets/multiplication.rs", None),
        (4, "./test/assets/division.rs", None),
        (9, "./test/assets/parentheses.rs", None),
        (-3, "./test/assets/unary-minus.rs", None),
        (-8, "./test/assets/unary-neg-parens.rs", None),
        (-15, "./test/assets/unary-mixed.rs", None),
        (1, "./test/assets/eq-true.rs", None),
        (0, "./test/assets/eq-false.rs", None),
        (1, "./test/assets/ne-true.rs", None),
        (0, "./test/assets/ne-false.rs", None),
        (1, "./test/assets/lt-true.rs", None),
        (0, "./test/assets/lt-false.rs", None),
        (1, "./test/assets/gt-true.rs", None),
        (0, "./test/assets/gt-false.rs", None),
        (1, "./test/assets/le-true.rs", None),
        (0, "./test/assets/le-false.rs", None),
        (1, "./test/assets/ge-true.rs", None),
        (0, "./test/assets/ge-false.rs", None),
        (3, "./test/assets/assign-simple.rs", None),
        (3, "./test/assets/assign-multi.rs", None),
        (3, "./test/assets/return-stmt.rs", None),
        (6, "./test/assets/assign-vars.rs", None),
        (3, "./test/assets/if-else-true.rs", None),
        (2, "./test/assets/if-else-false.rs", None),
        (10, "./test/assets/while-loop.rs", None),
        (10, "./test/assets/for-loop.rs", None),
        (4, "./test/assets/nested-loop.rs", None),
        (5, "./test/assets/func-call.rs", None),
        (55, "./test/assets/fibonacci-allow-warnings.rs", None),
        (3, "./test/assets/reference-and-dereference.rs", None),
        (10, "./test/assets/local-var.rs", None),
        (10, "./test/assets/comments.rs", None),
        (0, "./test/assets/string.rs", None),
        (
            0,
            "./test/assets/systemcall-write.rs",
            Some("Hello, \nworld!\n"),
        ),
        (3, "./test/assets/array.rs", None),
        (15, "./test/assets/array-sum.rs", None),
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
    for &(expected, input, expected_output) in tests {
        tx.send((expected, input, expected_output))
            .unwrap_or_else(|e| {
                eprintln!("failed to send test to channel: {}", e);
                exit(1);
            });
    }
    drop(tx);
    let rx = Arc::new(Mutex::new(rx));

    // Channel for structured test results
    let (tx_res, rx_res) = mpsc::channel::<(String, i8, i8, Duration, bool, Option<String>)>();

    let mut handles = Vec::new();
    for _ in 0..parallel {
        let rx = Arc::clone(&rx);
        let rustc_bin = Arc::clone(&rustc_bin);
        let tx_res = tx_res.clone();
        let handle = thread::spawn(move || {
            loop {
                // receive one task under lock, then drop the lock before processing
                let (expected, input, expected_output) = {
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
                        .send((input.to_string(), expected, 0, duration, false, None))
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
                        .send((input.to_string(), expected, 0, duration, false, None))
                        .unwrap();
                    // indicate failure
                    print!("F");
                    io::stdout().flush().unwrap();
                    continue;
                }

                // Run binary
                let output = Command::new(&bin_path).output().unwrap_or_else(|e| {
                    eprintln!("failed to run binary: {}", e);
                    exit(1);
                });

                let (success, failure_info) = if let Some(expected_output) = expected_output {
                    // Check stdout output
                    let actual_output = String::from_utf8_lossy(&output.stdout);
                    let success = actual_output == expected_output;
                    let failure_info = if !success {
                        Some(format!(
                            "Test failed: {}\nExpected output: {:?}\nActual output:   {:?}",
                            input, expected_output, actual_output
                        ))
                    } else {
                        None
                    };
                    (success, failure_info)
                } else {
                    // Check exit code
                    let code = output.status.code().unwrap_or_else(|| {
                        eprintln!("{} => process terminated by signal", input);
                        exit(1);
                    });
                    let actual = code as u8 as i8;
                    let success = actual == expected;
                    let failure_info = if !success {
                        Some(format!(
                            "Test failed: {}\nExpected return code: {}\nActual return code: {}",
                            input, expected, actual
                        ))
                    } else {
                        None
                    };
                    (success, failure_info)
                };

                // send structured result for summary
                let duration = start.elapsed();
                tx_res
                    .send((
                        input.to_string(),
                        expected,
                        0,
                        duration,
                        success,
                        failure_info,
                    ))
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
    for (name, expected, actual, duration, success, _) in &results {
        println!(
            "{:<30} {:<6} {:<8} {:<8} {:?}",
            name,
            if *success { "OK" } else { "FAIL" },
            expected,
            actual,
            duration
        );
    }

    // Print failure details at the end
    let failures: Vec<_> = results
        .iter()
        .filter_map(|(_, _, _, _, _, failure_info)| failure_info.as_ref())
        .collect();
    if !failures.is_empty() {
        println!("\nFailure details:");
        for failure in failures {
            println!("{}", failure);
        }
    }

    // Exit with error if any failed
    if results.iter().any(|(_, _, _, _, success, _)| !success) {
        exit(1);
    } else {
        println!("OK");
    }
}
