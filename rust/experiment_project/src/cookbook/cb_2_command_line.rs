use ansi_term::*;

pub fn main() {
    println!("Color and style tests");
    println!(
        "This is {} in color, {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green")
    );
    println!(
        "{} and this is not",
        Style::new().bold().paint("This is Bold")
    );
    println!(
        "{}, {} and {}",
        Colour::Yellow.paint("This is colored"),
        Style::new().bold().paint("this is bold"),
        Colour::Yellow.bold().paint("this is bold and colored")
    );
    println!(
        "How about some {} and {}?",
        Style::new().bold().paint("bold"),
        Style::new().underline().paint("underline")
    );

    println!(
        "Demonstrating {}, {} and {}!",
        Colour::Blue.bold().italic().paint("blue bold"),
        Colour::Cyan.dimmed().paint("cyan experimental"),
        Colour::Yellow.underline().paint("yellow underline")
    );

    println!(
        "Yellow on red: {}",
        Colour::Yellow.on(Colour::Red).paint("wow!")
    );
    println!("{}", Colour::Fixed(134).paint("A sort of light purple"));
    println!(
        "{}",
        Colour::Fixed(221)
            .on(Colour::Fixed(124))
            .paint("Mustard in the ketchup")
    );
    println!("{}", Colour::RGB(70, 130, 180).paint("Steel blue"));
}
