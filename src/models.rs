use std::process;

pub enum Action {
    Read,
    Write,
}

impl Action {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "read" => Ok(Action::Read),
            "write" | "append" => Ok(Action::Write),
            _ => Err(format!("Invalid action '{}'. Choose 'read' or 'write'.", s)),
        }
    }
}
pub enum MetadataField {
    Persons,
    Regions,
}

impl MetadataField {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "persons" | "personinimage" => Ok(MetadataField::Persons),
            "regions" => Ok(MetadataField::Regions),
            _ => Err(format!(
                "Invalid field '{}'. Choose 'persons' or regions'.",
                s
            )),
        }
    }
}

pub struct Config {
    pub action: Action,
    pub file_path: String,
    pub meta_field: MetadataField,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let action: String = args[1].clone();
        let file_path: String = args[2].clone();
        let meta_field: String = args[3].clone();

        let action: Action = match Action::from_str(&action) {
            Ok(action) => action,
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        };

        let meta_field: MetadataField = match MetadataField::from_str(&meta_field) {
            Ok(meta_field) => meta_field,
            Err(e) => {
                eprint!("Error: {}", e);
                process::exit(1);
            }
        };
        Ok(Config {
            action,
            file_path,
            meta_field,
        })
    }
}
