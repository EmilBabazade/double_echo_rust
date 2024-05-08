use clap::{App, Arg};

fn main() {
    let matches = App::new("double_echo_rust")
        .about("double prints the inputs")
        .author("Emil Babazade")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("text to double print")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("seperator")
                .short("s")
                .required(false)
                .help("seperator between double printed lines")
                .number_of_values(1),
        )
        .get_matches();

    let line_seperator = match matches.value_of("seperator") {
        Some(str) => "\n".to_owned() + str + "\n",
        _ => "\n".to_owned(),
    };

    let texts = matches.values_of_lossy("text").unwrap();
    let text = texts.join(" ");
    print!("{}{}{}", text, line_seperator, text);
}
