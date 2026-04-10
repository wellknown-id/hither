fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("No hither modules found.");
        return;
    }
    println!("Available hither modules:");
    for name in &args {
        println!("  {}", name);
    }
}
