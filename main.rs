extern crate clap;

use clap::{Arg, App};
use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::fs::File;

fn main() {
  let arguments = App::new("PostgreSQL Database Extractor")
    .version("0.0.1")
    .author("Paul Colusso")
    .about("Restores a specific database from a PostgreSQL SQL dump made by pg_dumpall. Will latch onto the first instance of the search term, if you have multiple databases matching this pattern only the first will be extracted.")
    .arg(Arg::with_name("database")
      .value_name("DATABASE")
      .required(true)
      .index(1)
      .help("Name of the database to be restored."))
    .arg(Arg::with_name("inputfile")
      .value_name("SQL DUMP")
      .required(false)
      .short("i")
      .long("input-file")
      .help("File to process."))
    .arg(Arg::with_name("outfile")
      .value_name("Output File")
      .short("o")
      .required(false)
      .index(2)
      .help("Saves output to a file."))
    .arg(Arg::with_name("rename")
      .value_name("New Database Name")
      .short("r")
      .long("rename-db-to")
      .required(false)
      .help("Generate SQL to restore to a different database name."))
    .get_matches();
 
  let reader: Box<BufRead> = match arguments.value_of("inputfile") {
    None => Box::new(BufReader::new(io::stdin())),
    Some(file_name) => Box::new(BufReader::new(File::open(file_name).expect("Unable to open the source file.")))
  };

  let mut writer: Box<Write> = match arguments.value_of("outfile") {
    None => Box::new(BufWriter::new(io::stdout())),
    Some(file_name) => Box::new(BufWriter::new(File::create(file_name).expect("Cannot open file for writing.")))
  };

  let db_name = arguments.value_of("database").expect("No database name specified.");

  let mut found_db = false;
  let mut data_read: usize = 0;

  'outer: for line in reader.lines() {
    let line = line.unwrap();

    data_read  += line.len();

    if !found_db {
      if line.contains("\\connect") && line.contains(db_name) {
        found_db = true;
        let new_name = arguments.value_of("rename").unwrap_or(db_name);
        let connect_line = format!("\\connect {}", new_name);
        writer.write_all(connect_line.as_bytes()).expect("Could not write a line to the file.");
        writer.write_all("\n".as_bytes()).expect("Could not write a line to the file.");
      }
    } else {
      if line.contains("\\connect") {
        break 'outer;
      } else {
        writer.write_all(line.as_bytes()).expect("Could not write a line to the file.");
        writer.write_all("\n".as_bytes()).expect("Could not write a line to the file.");
      }
    }
  }

  if !found_db {
    panic!("Was unable to find database {}", db_name);
  }
}