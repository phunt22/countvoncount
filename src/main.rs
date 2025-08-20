use clap::{Parser, Subcommand};
use countvoncount_cli::{run_cli, run_benchmarks};

#[derive(Parser)]
#[command(name = "cvc")]
#[command(about = "Count von Count - Tools for Agents Project")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(trailing_var_arg = true)] // i.e. cvc answer my question --> ["answer", "my", "question"]
    prompt: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(long_flag = "combine")]
    Combine,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Combine) => {
            println!("{}", run_benchmarks());
        },
        None => {
            match run_cli(args.prompt).await {
                Ok(res) => println!("{}", res),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
