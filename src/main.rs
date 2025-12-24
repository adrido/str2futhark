use clap::Parser;
mod translit;
use translit::transliterate;

#[derive(Parser, Debug)]
#[command(author, version, about = "Transliterate text into Elder Futhark runes", long_about = None)]
struct Args {
    /// Text to transliterate as positional words. If omitted, use --input FILE.
    input: Vec<String>,

    /// Read input from file instead of positional text.
    #[arg(short, long, value_name = "FILE")]
    input_file: Option<String>,

    /// Write output to FILE instead of stdout.
    #[arg(short, long, value_name = "FILE")]
    output_file: Option<String>,
}


fn main() {
    let args = Args::parse();

    // Determine input source: positional text or input_file.
    let input = if !args.input.is_empty() {
        args.input.join(" ")
    } else if let Some(path) = args.input_file.as_deref() {
        std::fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("Fehler beim Lesen von '{}': {}", path, e);
            std::process::exit(2);
        })
    } else {
        // No stdin reading per request — print help and exit.
        use clap::CommandFactory;
        Args::command().print_help().ok();
        println!();
        return;
    };

    let result = transliterate(&input);

    if let Some(out_path) = args.output_file.as_deref() {
        if let Err(e) = std::fs::write(out_path, result) {
            eprintln!("Fehler beim Schreiben nach '{}': {}", out_path, e);
            std::process::exit(3);
        }
    } else {
        println!("{}", result);
    }
}
