# sqlify

A CLI tool to format your SQL code.

## Features

This tool utilizes [sqlformat](https://github.com/shssoichiro/sqlformat-rs)
under the hood to format the given SQL code.

## Installation

To install `sqlify` simply run the `cargo install` command:

```sh
cargo install sqlify
```

## Usage

To use it simply pass in your SQL as the argument, or pipe it from a file, like this:

```sh
sqlify "SELECT * FROM tablename"

# or

cat query.sql | sqlify

# or

sqlify < query.sql
```

where `query.sql` looks like

```sql
SELECT * FROM tablename
```

to get the following output:

```sql
SELECT
  *
FROM
  tablename
```

There are fomrating options from [sqlformat](https://docs.rs/sqlformat/latest/sqlformat/struct.FormatOptions.html)
exposed through arguments:

- `-u` or `--uppercase` to denote the use of uppercase keywords
- `-t` or `--tabs` to denote the use of tabs
- `-i` or `--indent` to specify the number of spaces to use for indentation
  (defaults to 2), ignored if tabs are used
- `-l` or `--lines-between-queries` to specify the number of line breaks
  to use between queries (defaults to 1)
