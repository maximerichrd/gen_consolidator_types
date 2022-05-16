use indoc::indoc;
use crate::utils::read_lines;

pub const IO_TS_IMPORT: &str = "import * as t from 'io-ts'\n\n";

pub fn iots_types(
  col_array: &Vec::<(String,String)>,
  table_name: &str
) -> String {

  let mut types_data = String::new();
  for (col, typ) in col_array.iter() {
      let field = format!("\t{}: t.{},\n", col, typ);
      types_data.push_str(&field);
  }

  format!(indoc! {
    r#"export const {} = t.type({{
    {}
    }})

    "#},
    &table_name, types_data)
}

pub fn get_iots_from_sql_type(sql_type: &str , types_map: &Vec<(String,String)>) -> String {
  match types_map.iter().find(|p| p.0 == sql_type) {
    None => {
      eprintln!("Couldn't find matching iots type for sql type {}", sql_type);
      "/!\\ TO_DO /!\\".to_string()
    },
    Some((_, io_ts)) => io_ts.to_string()
  }
}

pub fn ts_types(table_names: &Vec<String>) -> String {
  let mut types = String::new();

  for name in table_names {

    let code = format!(indoc! {
      r#"export type {} = t.TypeOf<typeof {}>

      export function is{}(
        ev: SupportedEvents
      ): ev is {} {{
        return ev.sourceReference?.table === "{}" || ev.table === "{}"
      }}

      "#},
      name, name, name, name, name, name);

    types.push_str(&code);
  }
  
  types
}

pub fn global_event_type(table_names: &Vec<String>) -> String {
  let mut glob_type = String::new();

  glob_type.push_str("export const SupportedDbEvents = t.union([\n");

  for name in table_names {
    glob_type.push_str(&format!("\t{},\n", name));
  }

  glob_type.push_str("])\n\n");

  glob_type.push_str("export type SupportedDbEvents = t.TypeOf<typeof SupportedDbEvents>\n\n");
  
  glob_type
}

pub fn extra_types(
  source_filename: &Option<String>
) -> String {
  if let Some(source_filename) = source_filename {
    if !source_filename.ends_with(".ts") {
      println!("Extra-types file is missing a .ts extension")
    }
    // read ts file
    let source_data = read_lines(source_filename)
      .expect(&format!("Couldn't read extra types from {}", source_filename));
  
    // remove ts imports
    let mut extra_types = String::new();
    for line in source_data {
      if let Ok(s) = line {
        if !s.trim().starts_with("import") {

          extra_types.push_str(&format!("{}\n", &s));
        }
      }
    }

    extra_types 
  }else {
    "".to_string()
  }
  
}

