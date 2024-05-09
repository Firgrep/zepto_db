use clap::{Args, Parser, Subcommand};

/// # `zepto_db` A Tiny DB built with Rust
///
/// ⚠️ Work in progress 🏗️
///
/// Usage:
/// ```rs
/// zepto_db create MyNewTable
/// ```
///
/// ## Resources
/// - [`Clap` derive tutorial](https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html)
/// - [`Clap` examples from the cookbook](https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html)
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create(name) => {
            println!("'Create' was used, name is: {:?}", name.name)
        }
    }
}

/// zepto_db: A Tiny DB built with Rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Creates a new table
    Create(CreateArgs),
}

#[derive(Args, Debug)]
struct CreateArgs {
    name: String,
}
