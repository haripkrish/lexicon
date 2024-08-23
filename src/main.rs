use rustyline::Result;

mod command_handler;

fn main() -> Result<()> {
    println!(
        "Connected to Lexicon instance."
    );
    command_handler::start_command_loop()?;
    Ok(())
}
