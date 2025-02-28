use clap::Parser as _;

mod cli;
mod plumbing;

fn main() {
    let args = cli::Command::parse();

    if let Err(err) = args.exec() {
        eprintln!("Error: {err:?}");

        std::process::exit(1);
    }
}
