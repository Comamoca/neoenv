use crate::Skim;
use inquire::Confirm;
use skim::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;

pub const NVIM_ENV_PREFIX: &str = "nvim-";

pub fn create_config() {
    let ans = Confirm::new("Notthing neoenv config file. Create config now?")
        .with_default(false)
        .prompt();

    match ans {
        Ok(true) => {
            let xdg_dirs = xdg::BaseDirectories::with_prefix("neoenv").unwrap();
            let path = xdg_dirs.get_config_file("neoenv");

            let _ = File::create(path);
            println!(
                "🚀 Create neoenv config at {}\nPlease restart neoenv.",
                env_path().unwrap().to_string_lossy()
            );
            std::process::exit(0);
        }
        Ok(false) => {
            eprintln!("Notthing config file. aborted.");
            std::process::exit(1)
        }
        Err(_) => {
            eprintln!("Cant get your input at create_config().");
            std::process::exit(1)
        }
    }
}

pub fn env_path() -> std::result::Result<std::path::PathBuf, std::io::Error> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("neoenv").unwrap();
    // let path = xdg_dirs.find_config_file("neoenv");
    let path = xdg_dirs.place_config_file("neoenv");

    // match path {
    //     Ok(path) => {
    //         if path.exists() {
    //         } else {
    //             create_config();
    //             return path;
    //         }
    //     }
    //     Err(err) => {
    //         eprintln!("An error occurred at env_path()\n{}", err);
    //         std::process::exit(1);
    //     }
    // }

    return path;
}

pub fn load_env_file() -> Vec<String> {
    let path = env_path();

    match path {
        Ok(path) => {
            let file = File::open(&path);

            match file {
                Ok(mut file) => {
                    let mut buf = Vec::new();
                    let _ = file.read_to_end(&mut buf).unwrap();

                    let buf_string = String::from_utf8(buf).unwrap();
                    let buf_splited: Vec<String> =
                        buf_string.lines().into_iter().map(String::from).collect();

                    let uniq: HashSet<String> = buf_splited.into_iter().collect();
                    let uniq_string: Vec<String> = uniq.into_iter().map(String::from).collect();

                    return uniq_string;
                }
                Err(_) => {
                    // create new config
                    create_config();
                    return vec![];
                }
            }
        }
        Err(_err) => {
            create_config();
            let _ = env_path();
            load_env_file()
        }
    }
}

pub fn finder(nvim_envs: Vec<String>) -> String {
    // when input vector has no attributes.
    if nvim_envs.len() == 0 {
        eprintln!(
            "Notthing neovim enviroments.\nPlease run `neoenv add ENV_NAME` to add enviroment."
        );
        std::process::exit(1);
    }

    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(false)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(nvim_envs.join("\n")));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    return selected_items[0].output().to_string();
}

pub fn save(envs: Vec<String>) {
    let mut file = File::create(env_path().unwrap()).unwrap();
    file.write_all(envs.join("\n").as_bytes()).unwrap();
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn gen_script(app_name: String) -> Result<String, String> {
    match std::env::var("SHELL") {
        Ok(shell) => {
            let with_prefix = NVIM_ENV_PREFIX.to_string() + &app_name;

            if shell.contains("fish") {
                return Ok(format!("set -x NVIM_APPNAME {}", with_prefix));
            } else if shell.contains("bash") || shell.contains("zsh") {
                return Ok(format!("export NVIM_APPNAME={}", with_prefix));
            } else if shell.contains("pwsh") {
                return Ok(format!("$env:NVIM_APPNAME  = {}", with_prefix));
            } else {
                Err(format!(
                    "Sorry! {} is not supported.\n
Are you angry at me?
You can add new shell support when you move **your hands**.
Pull Request is you are always welcome here:)",
                    shell
                ))
            }
        }
        Err(_) => {
            return Err("$SHELL is empty. Please specificate enviroment variable. ".to_string())
        }
    }
}

// TODO: add windows support
// #[cfg(target_os = "windows")]
// pub fn gen_script(app_name: String) -> String {
//     let with_prefix = NVIM_ENV_PREFIX.to_string() + app_name;
//     return format!("$env:NVIM_APPNAME  = {}", with_prefix);
// }
