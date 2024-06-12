// needed for benchmarking tests I think
extern crate test;

use std::path::Path;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

use std::fs;
use std::io::BufReader;

use std::cmp;

use std::collections::HashMap; // for left_join_fast


#[derive(Debug)]
pub struct Table {
  name: String,
  n_rows: usize,
  n_cols: usize,
  schema: Vec<String>,
  contents: Vec<String>
}

pub fn new(name: &str, schema_string: &str) -> Table  {
  let schema : Vec<String> = parse_schema_string(schema_string).into_iter().map(|x| x.to_string()).collect();
  return Table {
    name: name.to_string(),
    n_rows: 0,
    n_cols: schema.len(),
    schema: schema,
    contents: Vec::new()
  }
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
    pub fn left_join_fast(&self, other: &Table, column: &str) -> Table {
    // More efficient O(n log n) implementation of left join
    /*
      Do the same as the naive implementation, but replace the lookup
      operation to check if a key is in other table by a more efficient
      operation.
    */
      // figure out the index of the column in each table
      let col_indx_a = self.schema.iter().position(|r| r == column).unwrap();
      let col_indx_b = other.schema.iter().position(|r| r == column).unwrap();

      // Define the schema of the new table
      let mut schema = self.schema.clone();
      for j in 0..other.n_cols {
        if j == col_indx_b {
          continue
        } else {
          schema.push(other.schema[j].clone());
        }
      }

      // the new table 
      let mut joined = Table {
        name: format!("{}_{}",self.name, other.name),
        n_rows: 0,
        n_cols: schema.len(),
        schema: schema,
        contents: Vec::new()
      };

      let mut index_map : HashMap<String, Vec<usize>> = HashMap::new();

      // Construct a hashmap mapping each value of other[column] to
      // the vec of indices of rows with this value.
      for (i, row) in other.iter_rows().enumerate() {
        let value = &row[col_indx_b];
          index_map.entry(value.clone()).and_modify(|x| x.push(i)).or_insert(vec![i]);
      }

      for row_a in self.iter_rows() {
        let col_val = &row_a[col_indx_a];
        match index_map.get(col_val) {
          Some(indices) => {
            for i in indices.into_iter() {
              // insert a new row into the joined table
              let mut new_row = row_a.clone(); // this isn't correct I think
              // loop over the row in other
              for j in 0..other.n_cols {
                if j != col_indx_b {
                    new_row.push(other.contents[other.n_cols * i + j].clone());
                }
              }

              joined.insert(new_row.clone());
            }
          }
        None => ()
        }
     }
      
      return joined
    }

    pub fn left_join(&self, other: &Table, column: &str) -> Table {
      // Naive implementation of join
      /*
        for each row_a in self:
          for each row_b in other:
            if row_a[column] == row_b[column]:
              new_row = row_a | row_b
              add new_row to new_table 
      */

      // figure out the index of the column in each table
      let col_indx_a = self.schema.iter().position(|r| r == column).unwrap();
      let col_indx_b = other.schema.iter().position(|r| r == column).unwrap();

      let mut schema = self.schema.clone();
      for j in 0..other.n_cols {
        if j == col_indx_b {
          continue
        } else {
          schema.push(other.schema[j].clone());
        }
      }
      

      let mut joined = Table {
        name: format!("{}_{}",self.name, other.name),
        n_rows: 0,
        n_cols: schema.len(),
        schema: schema,
        contents: Vec::new()
      };


      for row_a in self.iter_rows() {
        for row_b in other.iter_rows() {
          if row_a[col_indx_a] == row_b[col_indx_b] {
            // include this row
            let mut new_row = row_a.clone(); // this isn't correct I think
            for j in 0..row_b.len() {
              if j == col_indx_b {
                continue
              } else {
                new_row.push(row_b[j].clone());
              }
            }

            joined.insert(new_row.clone());
          }
        }
      }

      return joined
    }

    pub fn insert(&mut self, mut row: Vec<String>) {
      // Insert a new row
      if row.len() != self.n_cols {
        panic!("row does not match schema");
      }

      self.contents.append(&mut row);
      self.n_rows += 1;
    }

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

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn bench_join(b: &mut Bencher) {
    let mut my_table = new("furniture", "location,value");
    let n = 300;
    for i in 0..n{
      my_table.insert(vec![format!("{i}"), format!("{i}")]);
    }
    my_table.display();
    let mut other_table = new("houses", "number,value");
    for i in 0..n {
      other_table.insert(vec![format!("{i}"), format!("{}", n - i)]);
    }
    b.iter(|| my_table.left_join(&other_table, "value"))
  }

  #[bench]
  fn bench_join_fast(b: &mut Bencher) {
    let mut my_table = new("furniture", "location,value");
    let n = 300;
    for i in 0..n{
      my_table.insert(vec![format!("{i}"), format!("{i}")]);
    }
    my_table.display();
    let mut other_table = new("houses", "number,value");
    for i in 0..n {
      other_table.insert(vec![format!("{i}"), format!("{}", n - i)]);
    }
    b.iter(|| my_table.left_join_fast(&other_table, "value"))
  }
}
