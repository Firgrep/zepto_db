use std::path::Path;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

use std::fs;
use std::io::BufReader;

use std::cmp;

#[derive(Debug)]
pub struct Table {
  name: String,
  n_rows: usize,
  n_cols: usize,
  schema: Vec<String>,
  contents: Vec<String>
}

/*
 Make it possible to iterate over rows.
 See: https://dev.to/wrongbyte/implementing-iterator-and-intoiterator-in-rust-3nio
*/
pub struct RowIterator<'a> {
    table: &'a Table,
    index: usize,
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Vec<String>> {
        if self.index < self.table.n_rows {
            let mut row : Vec<String> = Vec::new();
            for j in 0..self.table.n_cols {
              row.push(self.table.contents[self.index * self.table.n_cols + j].clone());
            }
            let result = Some(row);
            self.index += 1;
            result
        } else {
            None
        }
    }
}


pub fn create(name: &str, schema: &str) -> std::io::Result<()> {
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

fn parse_schema_string(schema_string: &str) -> Vec<&str> {
  return schema_string.split(",").map(|x| x.trim()).collect();
}

pub fn load(name: &str) -> Table {
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

// fn get_table_schema(name &str) -> Vec<&str> {
fn get_schema(name: &str) {
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

impl Table {
    pub fn iter_rows(&self) -> RowIterator {
        RowIterator {
            table: self,
            index: 0,
        }
    }

  pub fn display(&self) {
    // Print name
    println!("Table name: {}", self.name);

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
