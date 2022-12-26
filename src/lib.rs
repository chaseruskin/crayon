use std::fmt::Display;
use std::sync::atomic::{AtomicBool, Ordering};

static IS_COLOR: AtomicBool = AtomicBool::new(true);

mod color {
    pub const ON: bool = true;
    pub const OFF: bool = false;
}

/// Enables the [Color] trait to store ANSI escape sequences into a [ColoredString].
pub fn enable_color() -> () {
    IS_COLOR.store(color::ON, Ordering::SeqCst)
}

/// Gatekeeps the [Color] trait from storing ANSI escape sequences into a [ColoredString].
pub fn disable_color() -> () {
    IS_COLOR.store(color::OFF, Ordering::SeqCst)
}

/// Checks if the color escape sequences are enabled.
fn is_coloring() -> bool {
    match IS_COLOR.load(Ordering::SeqCst) {
        color::ON => true,
        color::OFF => false,
    }
}

type FgColor = Option<u8>;
type Decoration = Option<u8>;

struct Code(FgColor, Decoration);

// @idea: have some way to store codes for colored string and then either display or drop them
// upon display/to_string being called.
// - need a way to use `Color` trait on ColoredString (color can only have color and style)

#[derive(Debug, PartialEq, Clone)]
pub struct ColoredString(String);

impl Display for ColoredString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for ColoredString {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

pub trait Color<T: AsRef<str>> {
    fn red(&self) -> ColoredString;
    fn green(&self) -> ColoredString;
    fn blue(&self) -> ColoredString;

    fn yellow(&self) -> ColoredString;
    fn black(&self) -> ColoredString;
    fn magenta(&self) -> ColoredString;
    fn cyan(&self) -> ColoredString;
    fn white(&self) -> ColoredString;

    fn bold(&self) -> ColoredString;
    fn underline(&self) -> ColoredString;
}

macro_rules! coloring {
    ($a:expr, $b:expr) => {{
        match is_coloring() {
            true => {
                String::from(ESC_SEQ)
                    + "["
                    + &$b.to_string()
                    + "m"
                    + $a.as_ref()
                    + ESC_SEQ
                    + "["
                    + &palette::RESET.to_string()
                    + "m"
            }
            false => <str as AsRef<str>>::as_ref($a).to_string(),
        }
    }};
}

impl<T: AsRef<str>> Color<T> for T {
    fn red(&self) -> ColoredString {
        ColoredString(coloring!(self.as_ref(), palette::RED))
    }

    fn green(&self) -> ColoredString {
        ColoredString(coloring!(self.as_ref(), palette::GREEN))
    }

    fn blue(&self) -> ColoredString {
        ColoredString(coloring!(self.as_ref(), palette::BLUE))
    }

    fn yellow(&self) -> ColoredString {
        ColoredString(coloring!(self.as_ref(), palette::YELLOW))
    }

    fn black(&self) -> ColoredString {
        ColoredString(coloring!(self.as_ref(), palette::BLACK))
    }

    fn magenta(&self) -> ColoredString {
        ColoredString(coloring!(self.as_ref(), palette::MAGENTA))
    }

    fn cyan(&self) -> ColoredString {
        ColoredString(coloring!(self.as_ref(), palette::CYAN))
    }

    fn white(&self) -> ColoredString {
        ColoredString(coloring!(self.as_ref(), palette::WHITE))
    }

    fn bold(&self) -> ColoredString {
        ColoredString(coloring!(self.as_ref(), palette::BOLD))
    }

    fn underline(&self) -> ColoredString {
        ColoredString(coloring!(self.as_ref(), palette::UNDERLINE))
    }
}

mod palette {
    pub const RESET: u8 = 0;
    pub const BOLD: u8 = 1;
    pub const UNDERLINE: u8 = 4;

    pub const BLACK: u8 = 30;
    pub const RED: u8 = 31;
    pub const GREEN: u8 = 32;
    pub const YELLOW: u8 = 33;
    pub const BLUE: u8 = 34;
    pub const MAGENTA: u8 = 35;
    pub const CYAN: u8 = 36;
    pub const WHITE: u8 = 37;
}

const ESC_SEQ: &str = "\u{001b}";

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(true);
    }
}
