# Deeb CLI Starter

A starter repo to create a CLI with built in data persistence.

## Getting Started

1. Create the `.env` file by copying the `example.env` into a `.env` and setting the environment variables.

2. Start the CLI

```bash
# Run the CLI
cargo run
```

```bash
# Expected Help Screen
My awesome CLI with easy data persistence

Usage: deeb-cli-starter <COMMAND>

Commands:
  todo  Manage Todos
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
``````

## Features and Starter Concepts

1. Edit the `cli` module to easily adjust or add commands. This project uses Clap under the hood. 
2. Modify the `models` module to persist structures of data with deeb! It's quick as adding a struct and calling a method such as `insert_one` or `find_many`.
3. Environment variables are checked before runtime, ensuring all options are set beforehand.
4. Use the `log` crate, which is already configured to log runtime debugs or infos.

