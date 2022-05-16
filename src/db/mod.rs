use mysql::*;
use mysql::prelude::*;
use crate::config::Config;

pub struct ColumnInfo {
  pub name: String,
  pub table_name: String,
  pub data_type: String,
  pub is_nullable: String
}

pub fn get_connection(conf: &Config) -> mysql::Conn {
  let db_url = Opts::from_url(
    &format!(
        "mysql://{}:{}@{}:{}/{}",
        conf.db_user,
        conf.db_password,
        conf.db_host,
        conf.db_port,
        conf.db_name
    )
  ).unwrap();

  println!("Try connecting to database {} on port {}...", conf.db_name, conf.db_port);
  let connection = Conn::new(db_url).unwrap();
  println!("Connected to database !");

  connection
}

pub fn fetch_information_schema_columns<'a>(
  connection: &'a mut Conn, 
  db_name: &'a str
) -> impl Iterator<Item = ColumnInfo> + 'a {

  let get_tables_info_query = format!(r#" 
  SELECT 
  COLUMN_NAME,
  TABLE_NAME,
  DATA_TYPE,
  IS_NULLABLE
  FROM INFORMATION_SCHEMA.COLUMNS
  WHERE
  TABLE_SCHEMA = '{}';
  "#,
  db_name
  );

  connection
    .query_iter(get_tables_info_query)
    .unwrap()
    .map(|r| {
        let row = from_row::<(String, String, String, String)>(r.unwrap());
        ColumnInfo {
            name: row.0,
            table_name: row.1,
            data_type: row.2,
            is_nullable: row.3
        }
    })
}