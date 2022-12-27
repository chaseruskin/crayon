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

#[derive(Debug, PartialEq, Clone)]
pub struct ColoredString {
    data: String,
    code: Code,
}

impl ColoredString {
    pub fn new(s: &str) -> Self {
        Self {
            data: s.to_string(),
            code: Code {
                bg: None,
                fg: None,
                underline: None,
                bold: None,
                reversed: None,
            },
        }
    }
}

impl Display for ColoredString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match is_coloring() {
            true => {
                write!(
                    f,
                    "{}{}{}{}{}{}{}",
                    if let Some(bg) = &self.code.bg {
                        ESC_SEQ.to_owned() + "[" + &bg.to_string() + "m"
                    } else {
                        String::new()
                    },
                    if let Some(fg) = &self.code.fg {
                        ESC_SEQ.to_owned() + "[" + &fg.to_string() + "m"
                    } else {
                        String::new()
                    },
                    if let Some(dc) = &self.code.bold {
                        ESC_SEQ.to_owned() + "[" + &dc.to_string() + "m"
                    } else {
                        String::new()
                    },
                    if let Some(dc) = &self.code.underline {
                        ESC_SEQ.to_owned() + "[" + &dc.to_string() + "m"
                    } else {
                        String::new()
                    },
                    if let Some(dc) = &self.code.reversed {
                        ESC_SEQ.to_owned() + "[" + &dc.to_string() + "m"
                    } else {
                        String::new()
                    },
                    self.data,
                    if self.code.is_decorated() == true {
                        ESC_SEQ.to_owned() + "[" + RESET_CODE + "m"
                    } else {
                        String::new()
                    }
                )
            }
            false => write!(f, "{}", self.data),
        }
    }
}

impl<T: AsRef<str>> AsAnsi for T {
    fn get_data(&self) -> String {
        self.as_ref().to_owned()
    }
}

impl AsAnsi for ColoredString {
    fn get_code(&self) -> Code {
        self.code.clone()
    }

    fn get_data(&self) -> String {
        self.data.clone()
    }
}

pub trait AsAnsi {
    fn get_code(&self) -> Code {
        Code {
            fg: None,
            bg: None,
            bold: None,
            underline: None,
            reversed: None,
        }
    }

    fn get_data(&self) -> String;
}

pub trait Color<T: Display + AsAnsi> {
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

impl<T: Display + AsAnsi> Color<T> for T {
    fn bold(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: code.fg,
                bg: code.bg,
                bold: Some(Bold),
                underline: code.underline,
                reversed: code.reversed,
            },
        }
    }

    fn underline(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: code.fg,
                bg: code.bg,
                bold: code.bold,
                underline: Some(Underline),
                reversed: code.reversed,
            },
        }
    }

    fn reversed(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: code.fg,
                bg: code.bg,
                bold: code.bold,
                underline: code.underline,
                reversed: Some(Reversed),
            },
        }
    }

    fn black(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: Some(Fg::Black),
                bg: code.bg,
                bold: code.bold,
                underline: code.underline,
                reversed: code.reversed,
            },
        }
    }

    fn red(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: Some(Fg::Red),
                bg: code.bg,
                bold: code.bold,
                underline: code.underline,
                reversed: code.reversed,
            },
        }
    }

    fn green(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: Some(Fg::Green),
                bg: code.bg,
                bold: code.bold,
                underline: code.underline,
                reversed: code.reversed,
            },
        }
    }

    fn yellow(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: Some(Fg::Yellow),
                bg: code.bg,
                bold: code.bold,
                underline: code.underline,
                reversed: code.reversed,
            },
        }
    }

    fn blue(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: Some(Fg::Blue),
                bg: code.bg,
                bold: code.bold,
                underline: code.underline,
                reversed: code.reversed,
            },
        }
    }

    fn magenta(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: Some(Fg::Magenta),
                bg: code.bg,
                bold: code.bold,
                underline: code.underline,
                reversed: code.reversed,
            },
        }
    }

    fn cyan(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: Some(Fg::Cyan),
                bg: code.bg,
                bold: code.bold,
                underline: code.underline,
                reversed: code.reversed,
            },
        }
    }

    fn white(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: Some(Fg::White),
                bg: code.bg,
                bold: code.bold,
                underline: code.underline,
                reversed: code.reversed,
            },
        }
    }

    fn bg_black(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: code.fg,
                bg: Some(Bg::Black),
                bold: code.bold,
                underline: code.underline,
                reversed: code.reversed,
            },
        }
    }

    fn bg_red(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: code.fg,
                bg: Some(Bg::Red),
                bold: code.bold,
                underline: code.underline,
                reversed: code.reversed,
            },
        }
    }

    fn bg_green(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: code.fg,
                bg: Some(Bg::Green),
                bold: code.bold,
                underline: code.underline,
                reversed: code.reversed,
            },
        }
    }

    fn bg_yellow(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: code.fg,
                bg: Some(Bg::Yellow),
                bold: code.bold,
                underline: code.underline,
                reversed: code.reversed,
            },
        }
    }

    fn bg_blue(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: code.fg,
                bg: Some(Bg::Blue),
                bold: code.bold,
                underline: code.underline,
                reversed: code.reversed,
            },
        }
    }

    fn bg_magenta(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: code.fg,
                bg: Some(Bg::Magenta),
                bold: code.bold,
                underline: code.underline,
                reversed: code.reversed,
            },
        }
    }

    fn bg_cyan(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: code.fg,
                bg: Some(Bg::Cyan),
                bold: code.bold,
                underline: code.underline,
                reversed: code.reversed,
            },
        }
    }

    fn bg_white(&self) -> ColoredString {
        let code = self.get_code();
        ColoredString {
            data: self.get_data(),
            code: Code {
                fg: code.fg,
                bg: Some(Bg::White),
                bold: code.bold,
                underline: code.underline,
                reversed: code.reversed,
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
        White,
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
        write!(
            f,
            "{}",
            match self {
                Self::Black => "30",
                Self::Red => "31",
                Self::Green => "32",
                Self::Yellow => "33",
                Self::Blue => "34",
                Self::Magenta => "35",
                Self::Cyan => "36",
                Self::White => "37",
            }
        )
    }
}

impl Display for palette::Bg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Black => "40",
                Self::Red => "41",
                Self::Green => "42",
                Self::Yellow => "43",
                Self::Blue => "44",
                Self::Magenta => "45",
                Self::Cyan => "46",
                Self::White => "47",
            }
        )
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
