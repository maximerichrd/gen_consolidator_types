use std::io::{Write};
use std::fs::{
  File, 
  OpenOptions, 
  read_to_string, 
  remove_file
};
use crate::gen::IO_TS_IMPORT;

pub struct WorkFile {
  pub name: String,
  pub file: File
}

pub enum WriteMode {
  Append,
  Erase
}

fn init_output(name: &str) -> File {

  let mut output = OpenOptions::new()
      .create(true)
      // each write in output file will append to it
      .append(true)
      .open(name)
      .unwrap();

  if let Err(e) = output.write_all(IO_TS_IMPORT.as_bytes()) {
      eprintln!("Couldn't write to file: {}", e);
  }

  output
}

pub fn create_files(
  output_dir: &str,
  output_name: &str,
) -> (WorkFile, WorkFile) {

  let output_name = format!("{}/{}", output_dir, output_name);
  let output = init_output(&output_name);

  let temp_name = format!("{}/{}", output_dir, &format!("{}.{}", output_name, "temp"));
  let temp = File::create(&temp_name).unwrap();

  (
    WorkFile {
    name: output_name.to_string(),
    file: output
    },
    WorkFile {
    name: temp_name.to_string(),
    file: temp
    }
  )
}

pub fn read(workfile: &WorkFile) -> String {
  read_to_string(&workfile.name).unwrap()
}

pub fn write(
  mode: WriteMode,
  workfile: &mut WorkFile, 
  data: &String,
) {

  match mode {
    WriteMode::Append => {
      if let Err(e) = workfile.file.write_all(data.as_bytes()) {
        eprintln!("Couldn't write to file: {}", e);
      }
    },
    WriteMode::Erase => {
      let mut workfile = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&workfile.name)
        .unwrap();

      if let Err(e) = workfile.write_all(data.as_bytes()) {
        eprintln!("Couldn't write to file: {}", e);
      }
    }
  }
  
}

pub fn delete(file: &WorkFile) {
  if let Err(e) = remove_file(&file.name) {
    eprintln!("Couldn't delete temp file: {}", e);
}
}
