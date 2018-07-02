extern crate clap;
extern crate indicatif;

use clap::{Arg, App};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{BufReader, BufRead, BufWriter, Write};
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
      .required(true)
      .index(2)
      .help("File to process."))
    .arg(Arg::with_name("outfile")
      .value_name("Output File")
      .short("o")
      .required(true)
      .long("output")
      .help("Saves output to a file."))
    .get_matches();

  let in_file = File::open(arguments.value_of("inputfile").unwrap()).expect("Unable to open file for reading.");
  let out_file = File::create(arguments.value_of("outfile").unwrap()).expect("Unable to open file for writing.");
  let db_name = arguments.value_of("database").unwrap_or("");

  let f_size = in_file.metadata().unwrap().len();

  let reader = BufReader::new(in_file);
  let mut writer = BufWriter::new(out_file);
  let pb = ProgressBar::new(f_size);
  pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

  let mut found_db = false;
  let mut data_read: usize = 0;

  'outer: for line in reader.lines() {
    let line = line.unwrap();

    data_read  += line.len();
    pb.set_position(data_read as u64);

    if !found_db {
      if line.contains("\\connect") && line.contains(db_name) {
        found_db = true;
        println!("Database founnd. Commencing extraction...");
        writer.write_all(line.as_bytes()).expect("Could not write a line to the file.");
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

  pb.finish_with_message("Done!");

  if !found_db {
    panic!("Was unable to find database {}", db_name);
  }
}