use clap::{Args, Parser, Subcommand};

use std::fs;
use std::path::PathBuf;

use std::fs::File;
use std::io::prelude::*;

use std::path::Path;

use std::io::Error;

use std::io::BufReader;

use std::cmp;

const DATA_DIR : &str = "data";

#[derive(Debug)]
struct Table {
  name: String,
  n_rows: usize,
  n_cols: usize,
  schema: Vec<String>,
  contents: Vec<String>
}

fn load_table(name: &str) -> Table {
  let table_filename = format!("data/{}.csv", name);
  let table_file = fs::File::open(table_filename).expect("Table not found");
  let table_file_reader = BufReader::new(table_file);
  let mut lines = table_file_reader.lines();

  let schema_string = lines.next().unwrap().unwrap();
  let schema : Vec<String> = parse_schema_string(&schema_string).into_iter().map(|x| x.to_string()).collect();

  let n_cols = schema.len();
  let mut contents: Vec<String> = Vec::new();

  for line in lines {
    let line_str = line.unwrap();

    // Last line is always empty
    if line_str == "" {
      continue;
    }


    let line_vals: Vec<&str> = line_str.split(",").collect();

    if line_vals.len() != n_cols {
      panic!("Error: wrong number of columns")
    }

    let mut line_vals_2: Vec<String> = line_vals.into_iter().map(|x| x.trim().to_string()).collect();

    contents.append(&mut line_vals_2);
  }

  let n_rows = (contents.len() / n_cols) as usize;

  return Table {
    name : name.to_string(),
    n_rows : n_rows,
    n_cols : n_cols,
    schema : schema,
    contents : contents
  }
}

fn create_table(name: &str, schema: &str) -> std::io::Result<()> {
  println!("Creating table ``{}'' with schema ``{}''", name, schema);
  
  /*
    1. Check if a file with "name.csv" exists in data folder.
      - if yes return error
      - if no continue
    2. Create a file "name.csv" with a single line specifying the schema.
  */

    let filename = format!("data/{}.csv", name);
    // check if the file exists
    if Path::new(&filename).exists() {
      return Err(Error::other("Table already exists"))
    }
    let mut file = File::create(filename)?;
    file.write_all(schema.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(())
}

impl Table {
  fn display(&self) {
    // First calculate the column widths
    let mut x: Vec<usize> = Vec::new();
    for i in 0..self.n_cols {
      let mut col_width = self.schema[i].chars().count();
      for j in 0..self.n_rows {
        let val = &self.contents[i + j * self.n_cols];
        col_width = cmp::max(col_width, val.chars().count());
      }
      col_width = col_width + 2;
      x.push(col_width);
    }
    let col_widths = x;

    // Print headers
    for i in 0..self.n_cols {
      print!("{:^width$}", self.schema[i], width=col_widths[i]);
      if (i < self.n_cols-1) & (self.n_cols != 1) {
        print!("|");
      }
    }

    println!();

    // Next print a horisontal bar separating headers from content
    let mut total_width = 0;
    for width in &col_widths {
        total_width += width
    }
    total_width += self.n_cols - 1;

    // Print a bar
    for _ in 1..total_width{
        print!("-");
    }
    println!();

    // Print all of the rows
    for i in 0..self.n_rows {
        let start = i * self.n_cols;
        let end = (i+1) * self.n_cols;
        let row = &self.contents[start..end];

        for j in 0..self.n_cols {
            print!("{:^width$}", row[j], width=col_widths[j]);
            if (j < self.n_cols-1) & (self.n_cols != 1) {
                print!("|");
            } 
        }
        println!()
    }
    
  }
}

fn parse_schema_string(schema_string: &str) -> Vec<&str> {
  return schema_string.split(",").map(|x| x.trim()).collect();
}

// fn get_table_schema(name &str) -> Vec<&str> {
fn get_table_schema(name: &str) {
  /*
    Read the first line from the file.
    Split by ","
    Ignore leading/trailing whitespace 
  */
  // read first line only
  
  let table_filename = format!("data/{}.csv", name);
  let table_file = fs::File::open(table_filename).expect("Table not found");
  let table_file_reader = BufReader::new(table_file);
  let schema_string = table_file_reader.lines().next().unwrap().unwrap();
  let schema = parse_schema_string(&schema_string);

  println!("{:?}", schema);
}

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
    let datadir= PathBuf::from(DATA_DIR);
    match fs::canonicalize(&datadir) {
      Ok(path) => println!("Data directory: {}", path.display()),
      Err(_error) => {
        println!("Error: Data directory not found. Exiting.");
        return
      }
    }

    let cli = Cli::parse();

    match &cli.command {
        Commands::Create(args) => {
            println!("==>'Create' new table: {:?}", args.table_name);
            println!("{:?}", create_table(&args.table_name, &args.schema))
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
        Commands::Display(args) => {
                load_table(&args.table_name).display()
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
    /// Loads a table from file and displays in terminal
    #[command(arg_required_else_help = true)]
    Display(DisplayArgs),
}

#[derive(Args, Debug)]
struct CreateArgs {
    #[arg(short, long)]
    table_name: String,
    #[arg(short, long)]
    schema: String
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

#[derive(Args, Debug)]
struct DisplayArgs {
    #[arg(short, long)]
    table_name: String,
}



// #[derive(Clone, Debug)]
// enum InsertPayload {
//     String(String),
//     Number(i32),
// }
