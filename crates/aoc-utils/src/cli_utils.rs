pub fn solve_part<F>(part: u8, solver: F)
where
    F: FnOnce() -> String,
{
    println!("Solving Part {part}...");
    let start_time = std::time::Instant::now();
    let result = solver();
    let elapsed_time = start_time.elapsed();
    println!(
        "Result: {}",
        if !result.is_empty() {
            result
        } else {
            "No solution found".to_string()
        }
    );
    println!(
        "Solving Part {part} took {} microseconds.",
        elapsed_time.as_micros()
    );
}
