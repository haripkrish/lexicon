use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use rustyline::Result;


pub fn start_command_loop() -> Result<()>{
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("lexicon> ");

        match readline {
            Ok(line) => {
                let args: Vec<&str> = line.trim().split_whitespace().collect();
                if args.is_empty() {
                    continue;
                }

                match args[0].to_uppercase().as_str() {
                    "HSET" => {
                        println!("Hset");
                    },
                    "HGET" => {
                        println!("Hget");
                    },
                    "EXIT" => {
                        println!("Quitting lexicon...");
                        break
                    },
                    _ => {
                        println!("Command doesn't exist");
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    Ok(())
}