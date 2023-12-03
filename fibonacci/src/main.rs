use std::time::Instant;

fn fibonacci(n: i64) -> i64 {
    if n == 0 || n == 1 { 
        return 1 
    };

    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn main() {
    let program_start = Instant::now();
    println!("Starting");
    for i in 1..50 {
        let start = Instant::now();
        let result = fibonacci(i);
        let duration = start.elapsed();
        println!("fibonacci({}): {} in {:?}.", i, result, duration);
    }
    let program_duration = program_start.elapsed();
    println!("Ending in {:?}", program_duration)
}
