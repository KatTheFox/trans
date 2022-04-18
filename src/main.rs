use std::io::{self, Write};
use strip_ansi_escapes;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    loop {
        match stdin.read_line(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                buf = std::str::from_utf8(
                    &strip_ansi_escapes::strip(&buf.as_bytes())
                        .expect("error stripping ansi escapes"),
                )
                .expect("invalid utf8")
                .to_string();
                print_trans(&buf);
                buf = String::new();
            }
            Err(err) => println!("{}", err.to_string()),
        }
    }
}
fn print_trans(input: &str) {
    if input.len() == 0 {
        return;
    }
    let size = input.len() - 1;
    let blue_pink_size = (size as f64 / 5f64).floor() as usize;
    let white_size = (size - (4 * blue_pink_size) as usize) as usize;
    let blue1 = &input[0..blue_pink_size];
    let pink1 = &input[blue_pink_size..blue_pink_size * 2];
    let white = &input[blue_pink_size * 2..blue_pink_size * 2 + white_size];
    let pink2 = &input[blue_pink_size * 2 + white_size..blue_pink_size * 3 + white_size];
    let blue2 = &input[blue_pink_size * 3 + white_size..blue_pink_size * 4 + white_size];
    let blue = termcolor::Color::Rgb(85, 205, 252);
    let pink = termcolor::Color::Rgb(247, 168, 184);
    let mut wtr = StandardStream::stdout(ColorChoice::AlwaysAnsi);
    wtr.set_color(ColorSpec::new().set_fg(Some(blue)))
        .expect("failed to set color");
    write!(&mut wtr, "{}", blue1).expect("failed to print");
    wtr.set_color(ColorSpec::new().set_fg(Some(pink)))
        .expect("failed to set color");
    write!(&mut wtr, "{}", pink1).expect("failed to print");
    wtr.set_color(ColorSpec::new().set_fg(Some(Color::White)))
        .expect("failed to set color");

    write!(&mut wtr, "{}", white).expect("failed to print");
    wtr.set_color(ColorSpec::new().set_fg(Some(pink)))
        .expect("failed to set color");
    write!(&mut wtr, "{}", pink2).expect("failed to print");
    wtr.set_color(ColorSpec::new().set_fg(Some(blue)))
        .expect("failed to set color");
    write!(&mut wtr, "{}", blue2).expect("failed to print");
    wtr.set_color(ColorSpec::new().set_fg(Some(Color::White)))
        .expect("failed to set color");
    writeln!(&mut wtr, " ").expect("failed to write")
}
