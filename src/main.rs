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
        .get_matches();

    let texts = matches.values_of_lossy("text").unwrap();
    let text = texts.join(" ");
    print!("{}\n{}", text, text);
}
