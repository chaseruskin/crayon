use crayon::Color;

fn main() -> () {
    let text = vec![
        "c".red().bg_white().bold(),
        "r".green().bg_white().bold(),
        "a".yellow().bg_white().bold(),
        "y".blue().bg_white().bold(),
        "o".magenta().bg_white().bold(),
        "n".cyan().bg_white().bold(),
    ];
    println!(
        "{0}   {0}",
        text.iter()
            .fold(String::new(), |acc, x| { acc + &x.to_string() })
    );
    crayon::disable_color();
    println!(
        "{0}   {0}",
        text.iter()
            .fold(String::new(), |acc, x| { acc + &x.to_string() })
    );
    crayon::enable_color();
    println!("");
    print!("{} ", "1".black());
    print!("{} ", "2".red());
    print!("{} ", "3".green());
    print!("{} ", "4".yellow());
    print!("{} ", "5".blue());
    print!("{} ", "6".magenta());
    print!("{} ", "7".cyan());
    print!("{} ", "8".white());
    println!("");
    print!("{} ", "1".bg_black());
    print!("{} ", "2".bg_red());
    print!("{} ", "3".bg_green());
    print!("{} ", "4".bg_yellow());
    print!("{} ", "5".bg_blue());
    print!("{} ", "6".bg_magenta());
    print!("{} ", "7".bg_cyan());
    print!("{} ", "8".bg_white());
    println!("");
    print!("{} ", "1".black().underline());
    print!("{} ", "2".red().underline());
    print!("{} ", "3".green().underline());
    print!("{} ", "4".yellow().underline());
    print!("{} ", "5".blue().underline());
    print!("{} ", "6".magenta().underline());
    print!("{} ", "7".cyan().underline());
    print!("{} ", "8".white().underline());
    println!("");
    print!("{} ", "1".black().bold());
    print!("{} ", "2".red().bold());
    print!("{} ", "3".green().bold());
    print!("{} ", "4".yellow().bold());
    print!("{} ", "5".blue().bold());
    print!("{} ", "6".magenta().bold());
    print!("{} ", "7".cyan().bold());
    print!("{} ", "8".white().bold());
    println!("");
    print!("{} ", "1".black().reversed());
    print!("{} ", "2".red().reversed());
    print!("{} ", "3".green().reversed());
    print!("{} ", "4".yellow().reversed());
    print!("{} ", "5".blue().reversed());
    print!("{} ", "6".magenta().reversed());
    print!("{} ", "7".cyan().reversed());
    print!("{} ", "8".white().reversed());
    println!("");
    print!("{} ", "1".black().bold().underline());
    print!("{} ", "2".red().bold().underline());
    print!("{} ", "3".green().bold().underline());
    print!("{} ", "4".yellow().bold().underline());
    print!("{} ", "5".blue().bold().underline());
    print!("{} ", "6".magenta().bold().underline());
    print!("{} ", "7".cyan().bold().underline());
    print!("{} ", "8".white().bold().underline());
    println!("");
    println!("");

    let mut i = 0;
    for r in 0..6 {
        for g in 0..6 {
            for b in 0..6 {
                print!("{} ", (i + 16).to_string().rgb(46 + (40*r), 46 + (40*g), 46 + (40*b)));
                i += 1;
            }
        }
    }
    for i in 0..24 {
        let p = 8 + (i*10);
        print!("{} ", (i + 232).to_string().rgb(p, p, p));
    }
    println!("");
    println!("");

    for i in 0..16 {
        print!("{} ", i.to_string().wheel(i));
    }
    println!("");

    for i in 16..=255 {
        print!("{} ", i.to_string().wheel(i));
    }
    println!("");
    println!("");

    let mut i = 0;
    for r in 0..6 {
        for g in 0..6 {
            for b in 0..6 {
                print!("{}", format!("{} ", i+16).bg_rgb(46 + (40*r), 46 + (40*g), 46 + (40*b)));
                i += 1;
            }
        }
    }
    for i in 0..24 {
        let p = 8 + (i*10);
        print!("{}", format!("{} ", i+232).bg_rgb(p, p, p));
    }
    println!("");

    println!("");
    for i in 0..16 {
        print!("{}", format!("{} ", i).bg_wheel(i));
    }
    println!("");
    println!("");

    for i in 16..=255 {
        print!("{}", format!("{} ", i).bg_wheel(i));
    }
    println!("");

}
