// Adds padding lines to start and end of println! calls
pub fn padding(lines: Vec<String>) {
    println!();

    for line in lines {
        println!("{}", line);
    }

    println!();
}