use crayon::{Color, ColoredString, AsAnsi};

fn main() -> () {
    let mut c = "blue".blue().bg_white();
    c = c.red().bg_green();

    let e = ColoredString::new("hello world");

    let e = e.blue().get_data();
    let text = "hello world";

    let s = text.red();
    println!("{}", s);
    println!("{}", s.green());

    let d = "blue".red().bg_green();
    println!("c: {}", c);
    println!("d: {}", d);
    println!("e: {}", e);

    crayon::disable_color();
    let c = "coloring".green().bold().underline();
    println!("{} | size: {}", c, c.to_string().len());
    crayon::enable_color();
    println!("{} | size: {}", c, c.to_string().len());

    println!(
        "{} {} {}",
        "red".red().bold(),
        "green".green().underline(),
        "blue".blue(),
    );

    let message = "go".green().to_string() + &" gators".blue().to_string();
    println!("{} ... {}", "go gators".cyan().bold(), message);
}
