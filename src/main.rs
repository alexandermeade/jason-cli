use clap::{Parser, Subcommand};
use jason_rs::jason_to_json;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "jason")]
#[command(author = "Alexander Meade")]
#[command(version = "0.1.0")]
#[command(about = "A simple CLI wrapper for the Jason-rs library", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compile {
        #[arg(value_name = "FILE")]
        input: PathBuf,

        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        #[arg(short, long)]
        pretty: bool,
    },
    Watch {
        #[arg(value_name = "FILE")]
        input: PathBuf,

        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        #[arg(short, long)]
        pretty: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { input, output, pretty } => {
            if let Err(e) = compile_file(&input, output.as_ref(), pretty) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Watch { input, output, pretty } => {
            println!("watching {} for changes...", input.display());
            println!("press Ctrl+C to stop.");
            
            // Initial compile
            if let Err(e) = compile_file(&input, output.as_ref(), pretty) {
                eprintln!("error: {}", e);
            } else {
                println!("✓ Compiled successfully");
            }

            // Watch for changes (simple polling implementation)
            let mut last_modified = fs::metadata(&input)
                .and_then(|m| m.modified())
                .ok();

            loop {
                std::thread::sleep(std::time::Duration::from_millis(500));

                if let Ok(metadata) = fs::metadata(&input) {
                    if let Ok(modified) = metadata.modified() {
                        if Some(modified) != last_modified {
                            last_modified = Some(modified);
                            println!("\nChange detected, recompiling...");
                            
                            if let Err(e) = compile_file(&input, output.as_ref(), pretty) {
                                eprintln!("Error: {}", e);
                            } else {
                                println!("✓ Compiled successfully");
                            }
                        }
                    }
                }
            }
        }
    }
}

fn compile_file(input: &PathBuf, output: Option<&PathBuf>, pretty: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Check if input file exists
    if !input.exists() {
        return Err(format!("Input file '{}' does not exist", input.display()).into());
    }

    // Compile the Jason file
    let json_str = jason_to_json(input.to_str().unwrap())?;

    // Parse and optionally pretty print
    let output_str = if pretty {
        let json_value: serde_json::Value = serde_json::from_str(&json_str.to_string())?;
        serde_json::to_string_pretty(&json_value)?
    } else {
        json_str.to_string()
    };

    // Write to output or stdout
    if let Some(output_path) = output {
        fs::write(output_path, output_str)?;
        println!("Compiled {} -> {}", input.display(), output_path.display());
    } else {
        println!("{}", output_str);
    }

    Ok(())
}
