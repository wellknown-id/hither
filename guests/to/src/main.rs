fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("What would you like to encant? Tell me your wishes.");
        println!("Do you wish for pictures of cats? Then say it, and I shall make it so!");
        println!();
        println!("Run `hither to pictures of cats` and your wish is my command!");
        println!();
        println!("Run `hither to financial news today` and I will do as you bid!");
        return;
    }
    let wish = args.join(" ");
    println!("Your wish is my command! '{}'", wish);
}
