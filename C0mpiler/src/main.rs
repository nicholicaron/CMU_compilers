use clap::{App, Arg};
pub mod parser;
pub mod scanner;

fn main() {
    let matches = App::new("C0mpiler")
        .version("0.1.0")
        .author("Nicholi Caron <nmcaron@protonmail.ch>")
        .about("A compiler for the C0 programming language")
        .arg(
            Arg::new("filename")
                .short('f')
                .takes_value(true)
                .value_name("FILENAME")
                .help("Name of file to be compiled")
                .required(false)
                .max_values(1),
        )
        .get_matches();

    if let Some(f) = matches.get_one::<String>("filename") {
        // Impl error handling here if file is unavailable
        let _scan = scanner::run_file(f.clone());
    } else {
        let _scan = scanner::run_prompt();
    }
}
