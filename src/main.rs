extern crate skim;
use clap::{Parser, Subcommand};
use skim::prelude::*;
mod utils;
use utils::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct App {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { app_name: String },
    Remove {},
    Switch {},
}

pub fn main() {
    let app = App::parse();

    // checking config file
    match env_path() {
        Ok(_file) => {}
        Err(_err) => create_config(),
    }

    match &app.command {
        Commands::Add { app_name } => {
            let mut envs = load_env_file();
            envs.push(app_name.to_string());
            save(envs);
        }
        Commands::Remove {} => {
            let mut env_file = load_env_file();
            let selected = finder(env_file.clone());

            env_file.retain(|item| {
                if item.to_string() == selected {
                    false
                } else {
                    true
                }
            });

            println!("Removed {}", selected);
            save(env_file);
        }
        Commands::Switch {} => {
            let selected = finder(load_env_file());
            let script = gen_script(selected);

            match script {
                Ok(script) => {
                    println!("{}", script)
                }
                Err(err) => {
                    eprintln!("{}", err)
                }
            }
        }
    }
}
