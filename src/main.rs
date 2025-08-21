use clap::{Parser, Subcommand};
use countvoncount::{run_cli, run_cli_no_tools, run_benchmarks};
use dotenvy::dotenv;

#[derive(Parser)]
#[command(name = "cvc")]
#[command(about = "Count von Count - Adding Basic Tools to Agents")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    // must be in quotes | Note: using trailing_var_arg or allow_hypen_values didn't fix glob operator 
    prompt: Option<String>,
    
    #[arg(short, long)]
    verbose: bool,

    #[arg(long)]
    no_tools: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(long_flag = "combine")]
    Combine,
}

#[tokio::main]
async fn main() {
    dotenv().ok(); 
    let args = Args::parse();
    
    match args.command {
        Some(Commands::Combine) => {
            match run_benchmarks().await {
                Ok(output) => println!("{}", output),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        },
        None => {
            match args.prompt {
                Some(prompt) => {
                    let result = if args.no_tools {
                        run_cli_no_tools(prompt).await
                    } else {
                        run_cli(prompt, args.verbose).await
                    };
                    
                    match result {
                        Ok(res) => println!("{}", res),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                },
                None => {
                    eprintln!("No prompt provided");
                    std::process::exit(1);
                }
            }
        }
    }
}