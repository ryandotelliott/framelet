mod cli;
mod screen_recorder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::run_cli()
}
