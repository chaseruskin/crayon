# Changelog

## 0.1.0

### Features
- supports 8 standard ANSI colors for foreground and background: black, red, green, yellow, blue, magenta, cyan, white
- supports styles for underlining, bold, and reversed
- uses global boolean to enable/disable converting `ColoredString` structs into ANSI-encoded strings (enabled by default)
- any struct that implements the `AsRef<str>` automatically implements the `AsAnsi` trait
- any struct that implements the `Display` and `AsAnsi` traits automatically has the ability to use the `Color` trait