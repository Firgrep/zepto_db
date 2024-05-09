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
        Commands::Create(args) => {
            println!("==>'Create' new table: {:?}", args.table_name)
        }
        Commands::Insert(args) => {
            println!(
                "==>'Insert' into table: {:?} with payload {:?}",
                args.table_name, args.payload
            )
        }
        Commands::Select(args) => {
            println!(
                "==>'Select' from table: {:?} with filter {:?}",
                args.table_name, args.filter
            )
        }
        Commands::Update(args) => {
            println!(
                "==>'Update' table: {:?} with payload {:?} and filter {:?}",
                args.table_name, args.payload, args.filter
            )
        }
        Commands::Delete(args) => {
            println!(
                "==>'Delete' from table: {:?} with filter {:?}",
                args.table_name, args.filter
            )
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
    #[command(arg_required_else_help = true)]
    Create(CreateArgs),
    /// Inserts entry into a table
    #[command(arg_required_else_help = true)]
    Insert(InsertArgs),
    /// Selects entry from a table based on filter
    #[command(arg_required_else_help = true)]
    Select(SelectArgs),
    /// Updates entry from a table based on filter
    #[command(arg_required_else_help = true)]
    Update(UpdateArgs),
    /// Deletes entry from a table based on filter
    #[command(arg_required_else_help = true)]
    Delete(DeleteArgs),
}

#[derive(Args, Debug)]
struct CreateArgs {
    #[arg(short, long)]
    table_name: String,
}

#[derive(Args, Debug)]
struct InsertArgs {
    #[arg(short, long)]
    table_name: String,
    #[arg(short, long)]
    payload: String,
}

#[derive(Args, Debug)]
struct SelectArgs {
    #[arg(short, long)]
    table_name: String,
    #[arg(short, long)]
    filter: String,
}

#[derive(Args, Debug)]
struct UpdateArgs {
    #[arg(short, long)]
    table_name: String,
    #[arg(short, long)]
    filter: String,
    #[arg(short, long)]
    payload: String,
}

#[derive(Args, Debug)]
struct DeleteArgs {
    #[arg(short, long)]
    table_name: String,
    #[arg(short, long)]
    filter: String,
}

// #[derive(Clone, Debug)]
// enum InsertPayload {
//     String(String),
//     Number(i32),
// }
