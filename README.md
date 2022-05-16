# gen_consolidator_types

- fetches all tables and columns details from a MySQL DB
- builds all io-ts types matching these tables and columns
- writes these types to a file
- optionally appends other types read from a given file

## Usage
- all setup is done via a file named **config**, like the one at the root of this repo
- then simply execute the rust binary <code>./gen_consolidator_types</code>