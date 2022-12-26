use crayon::{Color, AccessCode};

fn main() -> () {

    let mut c = "blue".red().bg_blue();
    c = c.red().underline().bg_green();
    println!("{:?}", c.get_code());
    println!("{}", c);
    // r
    // crayon::disable_color();
    // let c = "coloring".green().bold().underline();
    // println!("{} | size: {}", c, c.to_string().len());
    // crayon::enable_color();
    // println!("{} | size: {}", c, c.to_string().len());
    // println!(
    //     "{} {} {}",
    //     "red".red().bold().underline(),
    //     "green".green().underline(),
    //     "blue".white().blue(),
    // );
    // let c = "go gators".green().reversed();
    // crayon::enable_color();
    // println!("{} ... {}", "go gators".yellow(), c);
}
