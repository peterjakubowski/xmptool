use std::env;
use std::error::Error;
use std::process;
use xmpkit::XmpFile;
use xmptool::get_all_persons_shown;
use xmptool::models::{Action, Config, MetadataField};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = XmpFile::new();
    let _ = file.open(config.file_path);

    if matches!(config.action, Action::Read) {
        println!("Reading file!");
    } else {
        println!("Writing file!");
    }

    if matches!(config.meta_field, MetadataField::Persons) {
        println!("Field: PersonInImage");
    } else {
        println!("Field: Regions");
    }

    if let Some(meta) = file.get_xmp() {
        if let Some(persons_shown) = get_all_persons_shown(meta) {
            println!("{:#?}", persons_shown);
        } else {
            println!("No persons shown!");
        }
    } else {
        println!("No xmp here!");
    }
    Ok(())
}
