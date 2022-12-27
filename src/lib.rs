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

    fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            bold: None,
            underline: None,
            reversed: None,
        }
    }
}

pub trait AsAnsi {
    /// References the ANSI terminal [Code] commands.
    ///
    /// A `None` value indicates the datatype does not support internally storing the
    /// code as a separate field.
    fn as_code(&self) -> Option<&Code>;

    /// References the original [String] contents without ANSI codes.
    fn get_data(&self) -> &str;
}

#[derive(Debug, PartialEq, Clone)]
pub struct ColoredString {
    data: String,
    code: Code,
}

impl ColoredString {
    pub fn new() -> Self {
        Self {
            data: String::new(),
            code: Code::new(),
        }
    }

    pub fn from<T>(s: T) -> Self
    where
        String: From<T>,
    {
        Self {
            data: s.into(),
            code: Code::new(),
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

impl AsAnsi for ColoredString {
    /// References the ANSI terminal [Code] commands.
    fn as_code(&self) -> Option<&Code> {
        Some(&self.code)
    }

    /// References the original [String] contents without ANSI codes.
    fn get_data(&self) -> &str {
        &self.data
    }
}

impl<T: AsRef<str>> AsAnsi for T {
    /// References the ANSI terminal [Code] commands.
    fn as_code(&self) -> Option<&Code> {
        None
    }

    /// References the original [String] contents without ANSI codes.
    fn get_data(&self) -> &str {
        self.as_ref()
    }
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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
        let code = match self.as_code() {
            Some(c) => c.clone(),
            None => Code::new(),
        };
        ColoredString {
            data: self.get_data().to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;

    const BG_CODE_LEN: usize = 5;
    const FG_CODE_LEN: usize = 5;
    const BOLD_CODE_LEN: usize = 4;
    const UNDERLINE_CODE_LEN: usize = 4;
    const REVERSED_CODE_LEN: usize = 4;
    const RESET_CODE_LEN: usize = 4;
    const NO_CODE_LEN: usize = 0;

    #[test]
    fn ut_red() {
        let text = "red".red();
        assert_ne!(text.to_string(), "red");
        assert_eq!(text.get_data(), "red");
        assert_eq!(
            text.to_string().len(),
            FG_CODE_LEN + text.get_data().len() + RESET_CODE_LEN
        );
        assert_eq!(
            text.as_code().unwrap(),
            &Code {
                bg: None,
                fg: Some(palette::Fg::Red),
                bold: None,
                underline: None,
                reversed: None
            }
        );
    }

    #[test]
    fn ut_normal_str() {
        let text = "plain";
        assert_eq!(text.to_string(), "plain");
        assert_eq!(text.get_data(), "plain");
        assert_eq!(
            text.to_string().len(),
            NO_CODE_LEN + text.get_data().len() + NO_CODE_LEN
        );
        assert_eq!(text.as_code(), None);
    }

    #[test]
    fn ut_colored_string() {
        let text = ColoredString::new();
        assert_eq!(text.to_string(), "");
        assert_eq!(text.get_data(), "");
        assert_eq!(
            text.to_string().len(),
            NO_CODE_LEN + text.get_data().len() + NO_CODE_LEN
        );
        assert_eq!(
            text.as_code().unwrap(),
            &Code {
                bg: None,
                fg: None,
                bold: None,
                underline: None,
                reversed: None
            }
        );

        let text = ColoredString::from("hello world");
        assert_eq!(text.to_string(), "hello world");
        assert_eq!(text.get_data(), "hello world");
        assert_eq!(
            text.to_string().len(),
            NO_CODE_LEN + text.get_data().len() + NO_CODE_LEN
        );
        assert_eq!(
            text.as_code().unwrap(),
            &Code {
                bg: None,
                fg: None,
                bold: None,
                underline: None,
                reversed: None
            }
        );
    }

    #[test]
    fn ut_overlap_colors() {
        let mut text = "go".green().yellow().blue();
        assert_ne!(text.to_string(), "go");
        assert_eq!(text.get_data(), "go");
        assert_eq!(
            text.to_string().len(),
            FG_CODE_LEN + text.get_data().len() + RESET_CODE_LEN
        );
        assert_eq!(
            text.as_code().unwrap(),
            &Code {
                bg: None,
                fg: Some(palette::Fg::Blue),
                bold: None,
                underline: None,
                reversed: None
            }
        );
        // swap out the foreground color
        text = text.black().bg_yellow().bg_cyan();
        assert_ne!(text.to_string(), "go");
        assert_eq!(text.get_data(), "go");
        assert_eq!(
            text.to_string().len(),
            FG_CODE_LEN + BG_CODE_LEN + text.get_data().len() + RESET_CODE_LEN
        );
        assert_eq!(
            text.as_code().unwrap(),
            &Code {
                bg: Some(palette::Bg::Cyan),
                fg: Some(palette::Fg::Black),
                bold: None,
                underline: None,
                reversed: None
            }
        );
    }

    #[test]
    fn ut_full_code() {
        let text = "Go".blue().bold().underline().reversed().bg_white();
        assert_ne!(text.to_string(), "Go");
        assert_eq!(text.get_data(), "Go");
        assert_eq!(
            text.to_string().len(),
            FG_CODE_LEN
                + BG_CODE_LEN
                + BOLD_CODE_LEN
                + UNDERLINE_CODE_LEN
                + REVERSED_CODE_LEN
                + text.get_data().len()
                + RESET_CODE_LEN
        );
        assert_eq!(
            text.as_code().unwrap(),
            &Code {
                bg: Some(palette::Bg::White),
                fg: Some(palette::Fg::Blue),
                bold: Some(palette::Bold),
                underline: Some(palette::Underline),
                reversed: Some(palette::Reversed)
            }
        );
    }
}
