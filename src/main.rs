use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Player {
    },
    Team {
    },
    Fixture {
    }
}

fn main() {
    let args = Args::parse();

    match args.commands {
        Commands::Player {} => {
            println!("show players");
        }
        Commands::Team {} => {
            println!("show teams");
        }
        Commands::Fixture {} => {
            println!("show fixtures");
        }
    }
 }
