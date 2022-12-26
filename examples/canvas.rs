use crayon::Color;

fn main() -> () {
    crayon::disable_color();
    println!(
        "{} {} {}",
        "red".red().bold().underline(),
        "green".green().bold(),
        "blue".blue().bold()
    );
    crayon::enable_color();
    println!("{}.. ", "go gators".yellow());
}
