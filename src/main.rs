mod utils;
use clap::{Subcommand, Parser};

#[derive(Parser)]
#[command(name="git404")]
#[command(about="A Version Control Tool Created By 404", long_about = None)]

struct CLI {
   #[command(subcommand)]
   command: Commands,
}

#[derive(Subcommand)]
enum Commands{
    Init,
}

fn main() {
    let cli = CLI::parse();

    match cli.command{
        Commands::Init => {
            utils::mygit_init();
        }
    }
    
}
