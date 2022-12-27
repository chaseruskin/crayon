use crayon::{AsAnsi, Color, ColoredString};

fn main() -> () {
    let mut c = "blue".blue().bg_white();
    c = c.red().bg_green();

    let e = ColoredString::new("hello world");

    let e = e.blue();
    let e = e.red();

    let text = "hello world";

    let s = text.red();
    println!("{}", s);
    println!("{}", s.green());

    let d = "blue".red().bg_green();
    println!("{:?}", c.get_code());
    println!("c: {}", c);
    println!("d: {}", d);
    println!("e: {}", e);
    println!("{:?}", d.get_code());
    println!("{:?}", e.get_code());
    assert_eq!(d.get_code(), c.get_code());

    crayon::disable_color();
    let c = "coloring".green().bold().underline();
    println!("{} | size: {}", c, c.to_string().len());
    crayon::enable_color();
    println!("{} | size: {}", c, c.to_string().len());
    println!(
        "{} {} {}",
        "red".red().bold().underline(),
        "green".green().underline(),
        "blue".white().blue(),
    );
    let c = "go gators".green().magenta();
    crayon::enable_color();
    println!("{} ... {}", "go gators".yellow(), c);
}
