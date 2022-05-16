use crate::utils::{read_lines};
use std::error::Error;
use std::path::Path;

pub struct Config {
  pub db_user: String,
  pub db_password: String,
  pub db_host: String,
  pub db_name: String,
  pub db_port: i32,
  pub output_dir: String,
  pub output_name: String,
  pub sql_iots_types: Vec<(String,String)>,
  pub extra_ts_types_filepath: Option<String>,
}

pub fn get_from_file() -> Result<Config, Box<dyn Error>> {

  let mut db_user: Option<String> = None;
  let mut db_password: Option<String> = None;
  let mut db_host: Option<String> = None;
  let mut db_name: Option<String> = None;
  let mut db_port: Option<i32> = None;
  let mut output_dir: Option<String> = None;
  let mut output_name: Option<String> = None;
  let mut sql_iots_types: Vec<(String,String)> = Vec::new();
  let mut extra_ts_types_filepath: Option<String> = None;

  let path = Path::new("./config");
  
  if let Ok(lines) = read_lines(path) {

      for line in lines {

          if let Ok(s) = line {
              
              // Ignore empty lines and '#' commented lines
              if s.trim() == "" { continue; }
              if s.starts_with("#") { continue; }

              
              // All other lines must have the key=value syntax
              let (key, value): (String, String) = match s.find("=") {
                  Some(i) => (s[..i].trim().to_lowercase(), s[(i+1)..].trim().to_string()),
                  None => panic!("Error in config file syntax: key=value")
              };

              match key.as_str() {
                  "mysql_db_name" => {
                      db_name = Some(value);
                  },
                  "mysql_db_user" => {
                      db_user = Some(value);
                  },
                  "mysql_db_password" => {
                      db_password = Some(value);
                  },
                  "mysql_db_host" => {
                      db_host = Some(value);
                  },
                  "mysql_db_port" => {
                      db_port = Some(value.parse::<i32>().unwrap());
                  },
                  "output_dir" => {
                      output_dir = Some(value);
                  },
                  "output_name" => {
                      output_name = Some(value);
                  },
                  "sql_iots_types" => {
                    let (sql, iots) = match value.find(",") {
                        Some(i) => (value.as_str()[..i].trim().to_lowercase(), value.as_str()[(i+1)..].trim().to_lowercase()),
                        None => panic!("Error in config file, couldn't parse sql_iots_types pair: sqltype,iotstype")
                    };
                    if sql.len() < 1 || iots.len() < 1 {
                        panic!("Error in config file, couldn't parse sql_iots_types pair: sqltype,iotstype");
                    }

                    sql_iots_types.push((sql,iots));
                  },
                  "extra_ts_types_filepath" => {
                    extra_ts_types_filepath = Some(value);    
                  },
                  _ => ()
              }
          }
      }
  }

  Ok(Config {
      db_name: get_or_panic(db_name, "MYSQL_DB_NAME"),
      db_user: get_or_panic(db_user, "MYSQL_DB_USER"),
      db_password: get_or_panic(db_password, "MYSQL_DB_PASSWORD"),
      db_host: get_or_panic(db_host, "MYSQL_DB_HOST"),
      db_port: get_or_panic(db_port, "MYSQL_DB_PORT"),
      output_dir: get_or_panic(output_dir, "OUTPUT_DIR"),
      output_name: get_or_panic(output_name, "OUTPUT_NAME"),
      sql_iots_types: panic_if_empty_vec(sql_iots_types, "SQL_IOTS_TYPES"),
      extra_ts_types_filepath,
  })
}

fn get_or_panic<T>(value: Option<T>, name: &str) -> T {
    match value {
        Some(v) => v,
        None => panic!("missing config variable {}", name)
    }
}

fn panic_if_empty_vec<T>(vector: Vec<T>, name: &str) -> Vec<T> {
    if vector.len() == 0 {
        panic!("missing config variable {}", name) 
    }
    vector
}