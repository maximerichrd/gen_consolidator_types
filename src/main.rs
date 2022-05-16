mod config;
mod utils;
mod db;
mod fs;
mod gen;

use fs::WriteMode::{Append, Erase};

fn main() {
    let conf = config::get_from_file().unwrap();
    let mut connection = db::get_connection(&conf);
    let mut iterator = db::fetch_information_schema_columns(&mut connection, &conf.db_name);


    // To build our output
    // while fetching rows from DB one by one (for low memory usage)
    // We need 2 files
    let (mut output, mut temp) = fs::create_files(&conf.output_dir, &conf.output_name);
    // and a bunch of variables
    let mut latest_table = String::new();
    let mut table_names = Vec::<String>::new();
    let mut col_array = Vec::<(String, String)>::new();
    let mut first_iteration = true;

    while let Some(col) = iterator.next() {

        if latest_table != col.table_name 
        && !first_iteration {

            // latest table done, update output file
            let complete_table_types = fs::read(&temp);
            fs::write(
                Append, 
                &mut output, 
                &complete_table_types
            );

            // continue for this new table
            latest_table = col.table_name.to_string();
            table_names.push(col.table_name.to_string());

            col_array = Vec::<(String, String)>::new();
            col_array.push((
                col.name.to_string(), 
                gen::get_iots_from_sql_type(&col.data_type, &conf.sql_iots_types)
            ));
            fs::write(
                Erase, 
                &mut temp,
                &gen::iots_types(&col_array, &col.table_name)
            );

        } else {
            if first_iteration {
                latest_table = col.table_name.to_string();
                table_names.push(col.table_name.to_string());
                first_iteration = false;
            }

            col_array.push((
                col.name.to_string(), 
                gen::get_iots_from_sql_type(&col.data_type, &conf.sql_iots_types)
            ));
            fs::write(
                Erase, 
                &mut temp, 
                &gen::iots_types(&col_array, &col.table_name)
            );
        }
    }

    // our last table is now completed: update output file here again,
    // because it was not done after the last iteration above.
    let complete_table_types = fs::read(&temp);
    fs::write(Append, &mut output, &complete_table_types);

    // add other types
    fs::write(Append, &mut output, &gen::ts_types(&table_names));
    fs::write(Append, &mut output, &gen::global_event_type(&table_names));
    fs::write(Append, &mut output, &gen::extra_types(&conf.extra_ts_types_filepath));

    // clean up
    fs::delete(&temp);
    println!("Successfully generated file: {}/{}", &conf.output_dir, &conf.output_name);
}
