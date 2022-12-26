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

macro_rules! coloring {
    ($word:expr, $code:expr) => {{
        match is_coloring() {
            true => {
                format!(
                    "{}{}{}{}{}{}{}",
                    if let Some(bg) = &$code.bg {
                        ESC_SEQ.to_owned() + "[" + &bg.to_string() + "m"
                    } else {
                        String::new()
                    },
                    if let Some(fg) = &$code.fg {
                        println!("{:?}", fg.to_string());
                        ESC_SEQ.to_owned() + "[" + &fg.to_string() + "m"
                    } else {
                        String::new()
                    },
                    if let Some(dc) = &$code.bold {
                        ESC_SEQ.to_owned() + "[" + &dc.to_string() + "m"
                    } else {
                        String::new()
                    },
                    if let Some(dc) = &$code.underline {
                        ESC_SEQ.to_owned() + "[" + &dc.to_string() + "m"
                    } else {
                        String::new()
                    },
                    if let Some(dc) = &$code.reversed {
                        ESC_SEQ.to_owned() + "[" + &dc.to_string() + "m"
                    } else {
                        String::new()
                    },
                    $word.to_string(),
                    if $code.is_decorated() == true {
                        ESC_SEQ.to_owned() + "[" + RESET_CODE + "m"
                    } else {
                        String::new()
                    }
                )
            }
            false => <str as AsRef<str>>::as_ref($word).to_string(),
        }
    }};
}

use palette::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Code {
    fg: Option<Fg>,
    bg: Option<Bg>,
    bold: Option<Bold>,
    underline: Option<Underline>,
    reversed: Option<Reversed>,
}

impl Code {
    fn is_decorated(&self) -> bool {
        self.fg.is_some()
            || self.bg.is_some()
            || self.bold.is_some()
            || self.underline.is_some()
            || self.reversed.is_some()
    }
}

// @idea: have some way to store codes for colored string and then either display or drop them
// upon display/to_string being called.
// - need a way to use `Color` trait on ColoredString (color can only have color and style)

#[derive(Debug, PartialEq, Clone)]
pub struct ColoredString {
    data: String,
    code: Code,
}

impl Display for ColoredString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", coloring!(&self.data, &self.code))
    }
}

impl<T: AsRef<str>> AccessCode for T {}

impl AccessCode for ColoredString {
    fn get_code(&self) -> Code {
        dbg!(&self.code);
        self.code.clone()
    }
}

pub trait AccessCode {
    fn get_code(&self) -> Code {
        Code {
            fg: None,
            bg: None,
            bold: None,
            underline: None,
            reversed: None,
        }
    }
}

pub trait Color<T: Display + AccessCode> {
    fn bold(&self) -> ColoredString;
    fn underline(&self) -> ColoredString;
    fn reversed(&self) -> ColoredString;

    fn black(&self) -> ColoredString;
    fn red(&self) -> ColoredString;
    fn green(&self) -> ColoredString;
    fn yellow(&self) -> ColoredString;
    fn blue(&self) -> ColoredString;
    fn magenta(&self) -> ColoredString;
    fn cyan(&self) -> ColoredString;
    fn white(&self) -> ColoredString;

    fn bg_black(&self) -> ColoredString;
    fn bg_red(&self) -> ColoredString;
    fn bg_green(&self) -> ColoredString;
    fn bg_yellow(&self) -> ColoredString;
    fn bg_blue(&self) -> ColoredString;
    fn bg_magenta(&self) -> ColoredString;
    fn bg_cyan(&self) -> ColoredString;
    fn bg_white(&self) -> ColoredString;
}

impl<T: Display + AccessCode> Color<T> for T {
    fn bold(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: self.get_code().fg,
                bg: self.get_code().bg,
                bold: Some(Bold),
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }

    fn underline(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: self.get_code().fg,
                bg: self.get_code().bg,
                bold: self.get_code().bold,
                underline: Some(Underline),
                reversed: self.get_code().reversed,
            },
        }
    }

    fn reversed(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: self.get_code().fg,
                bg: self.get_code().bg,
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: Some(Reversed),
            },
        }
    }

    fn black(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: Some(Fg::Black),
                bg: self.get_code().bg,
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }

    fn red(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: Some(Fg::Red),
                bg: self.get_code().bg,
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }

    fn green(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: Some(Fg::Green),
                bg: self.get_code().bg,
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }

    fn yellow(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: Some(Fg::Yellow),
                bg: self.get_code().bg,
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }

    fn blue(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: Some(Fg::Blue),
                bg: self.get_code().bg,
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }

    fn magenta(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: Some(Fg::Magenta),
                bg: self.get_code().bg,
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }

    fn cyan(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: Some(Fg::Cyan),
                bg: self.get_code().bg,
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }

    fn white(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: Some(Fg::White),
                bg: self.get_code().bg,
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }

    fn bg_black(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: self.get_code().fg,
                bg: Some(Bg::Black),
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }

    fn bg_red(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: self.get_code().fg,
                bg: Some(Bg::Red),
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }

    fn bg_green(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: self.get_code().fg,
                bg: Some(Bg::Green),
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }

    fn bg_yellow(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: self.get_code().fg,
                bg: Some(Bg::Yellow),
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }

    fn bg_blue(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: self.get_code().fg,
                bg: Some(Bg::Blue),
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }

    fn bg_magenta(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: self.get_code().fg,
                bg: Some(Bg::Magenta),
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }

    fn bg_cyan(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: self.get_code().fg,
                bg: Some(Bg::Cyan),
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }

    fn bg_white(&self) -> ColoredString {
        ColoredString {
            data: self.to_string(),
            code: Code {
                fg: self.get_code().fg,
                bg: Some(Bg::White),
                bold: self.get_code().bold,
                underline: self.get_code().underline,
                reversed: self.get_code().reversed,
            },
        }
    }
}

mod palette {
    // standard text decorators
    #[derive(Debug, PartialEq, Clone)]
    pub struct Bold;

    #[derive(Debug, PartialEq, Clone)]
    pub struct Underline;

    #[derive(Debug, PartialEq, Clone)]
    pub struct Reversed;

    // standard 8 ANSI foreground colors
    #[derive(Debug, PartialEq, Clone)]
    pub enum Fg {
        Black,
        Red,
        Green,
        Yellow,
        Blue,
        Magenta,
        Cyan,
        White,
    }

    // standard 8 ANSI background colors
    #[derive(Debug, PartialEq, Clone)]
    pub enum Bg {
        Black,
        Red,
        Green,
        Yellow,
        Blue,
        Magenta,
        Cyan,
        White
    }
}

impl Display for palette::Bold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "1")
    }
}

impl Display for palette::Underline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "4")
    }
}

impl Display for palette::Reversed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "7")
    }
}

impl Display for palette::Fg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Black => "30",
            Self::Red => "31",
            Self::Green => "32", 
            Self::Yellow => "33",
            Self::Blue => "34",
            Self::Magenta => "35",
            Self::Cyan => "36",
            Self::White => "37",
        })
    }
}

impl Display for palette::Bg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Black => "40",
            Self::Red => "41",
            Self::Green => "42", 
            Self::Yellow => "43",
            Self::Blue => "44",
            Self::Magenta => "45",
            Self::Cyan => "46",
            Self::White => "47",
        })
    }
}

const ESC_SEQ: &str = "\u{001b}";
const RESET_CODE: &str = "0";

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(true);
    }
}
