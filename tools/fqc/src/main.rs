use fqc::Cli;

fn main() {
    println!("Starting fqc CLI...");
    let cli = Cli::new();
    if let Err(e) = cli.run() {
        handle_error(e);
    }
}
fn handle_error(error: String) {
    eprintln!("Error: {}", error);
    std::process::exit(1);
}
