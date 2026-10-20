use std::env;
use std::error::Error;
use std::process;
use xmpkit::XmpFile;
use xmptool::models::{Action, Config};
use xmptool::{read_xmp_meta, write_xmp_meta};

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
    if matches!(config.action, Action::Read) {
        println!("Reading file: {}", config.file_path);
        read_xmp_meta(&config, &mut file);
    } else {
        println!("Reading file for update: {}", config.file_path);
        write_xmp_meta(&config, &mut file);
        file.save(&config.file_path)?;
    }
    file.close();
    Ok(())
}
