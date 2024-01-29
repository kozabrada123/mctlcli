use color_print::cprintln;
use types::Config;

mod routes;
mod types;

// Constants are used for the command line arguments, so they can be refrenced multiple times in
// different places
pub const ADD_KEYWORD: &str = "add";
pub const ADD_KEYWORD_SHORT: &str = "a";

pub const UPDATE_KEYWORD: &str = "update";
pub const UPDATE_KEYWORD_SHORT: &str = "u";

pub const WITH_BRANCH_KEYWORD: &str = "branch";
pub const WITH_BRANCH_KEYWORD_SHORT: &str = "b";

#[tokio::main]
pub async fn main() {
    // Try to load the config to see any errors immediently
    Config::load();

    let mut args = std::env::args()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    if args.len() == 1 {
        cprintln!("<yellow>No subcommand specified, printing help</yellow>");
        println!("");
        print_help();
        std::process::exit(2);
    }

    let verb = args[1].clone();

    let mut branch = None;

    for i in 0..args.len() {
        let arg = args[i].clone();
        let argument_lower = arg.to_lowercase();

        if argument_lower == format!("--{WITH_BRANCH_KEYWORD}")
            || argument_lower == format!("-{WITH_BRANCH_KEYWORD_SHORT}")
        {
            branch = Some(args.remove(i + 1));
            args.remove(i);
            cprintln!("<blue>Using branch {:?}</blue>", branch.clone());
            break;
        }
    }

    match verb.as_str() {
        ADD_KEYWORD | ADD_KEYWORD_SHORT => {
            let url = args[2].clone();

            let name = args.get(3).clone();

            let schema = types::AddDatapackSchema {
                url,
                name: name.cloned(),
                branch,
            };

            println!("Debug: created schema {:?}", schema.clone());

            routes::datapacks::add::add(schema).await;
        }
        UPDATE_KEYWORD | UPDATE_KEYWORD_SHORT => {
            let name = args.get(2).unwrap();

            let schema = types::UpdateDatapackSchema {
                pack: name.clone(),
                branch,
            };

            println!("Debug: created schema {:?}", schema.clone());

            routes::datapacks::update::update(schema).await;
        }
        _ => {
            cprintln!("<red>Invalid operation {}, exiting.</red>", verb);
            std::process::exit(3);
        }
    }
}

pub fn print_help() {
    println!("mctlcli");
    println!("");
    println!("A cli for the mctl api");
    println!("");
    println!("Usage:");
    println!("");
    println!("'mctlcli {ADD_KEYWORD} <git url>' - add a datapack, using the git repo name as the datapack name");
    println!("'mctlcli {ADD_KEYWORD} <git url> <name>' - add a datapack, specifying its name");
    println!("'mctlcli {UPDATE_KEYWORD} <name>'- update a datapack");
    println!("");
    println!("Optional arguments:");
    println!("");
    println!("'--{WITH_BRANCH_KEYWORD}' ('-{WITH_BRANCH_KEYWORD_SHORT}') - specify a git branch to use (default is main)");
}
