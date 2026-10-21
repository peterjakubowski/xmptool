use std::process;

#[derive(Debug, PartialEq)]
pub enum RegionType {
    Face,
    Pet,
    Focus,
    BarCode,
}

impl RegionType {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "face" => Ok(RegionType::Face),
            "pet" => Ok(RegionType::Pet),
            "focus" => Ok(RegionType::Focus),
            "barcode" => Ok(RegionType::BarCode),
            _ => Err(format!(
                "Invalid region type '{}'. Choose 'face', 'pet', 'focus' or 'barcode'.",
                s
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Face {
    pub region_type: String,
    pub person_shown: String,
    pub dimensions: AppliedToDimensions,
    pub area: Area,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppliedToDimensions {
    pub img_width: i32,
    pub img_height: i32,
    pub unit: String,
}

#[derive(Debug, PartialEq)]
pub struct Area {
    pub w: f32,
    pub h: f32,
    pub x: f32,
    pub y: f32,
    pub unit: String,
}

#[derive(Debug)]
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
#[derive(Debug)]
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

#[derive(Debug)]
pub struct Config {
    pub action: Action,
    pub file_path: String,
    pub meta_field: MetadataField,
    pub value_to_append: Option<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
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

        if matches!(action, Action::Write) {
            if args.len() < 4 {
                return Err("missing value to append argument");
            }

            let value_to_append: String = args[4].clone();

            return Ok(Config {
                action,
                file_path,
                meta_field,
                value_to_append: Some(value_to_append),
            });
        }
        Ok(Config {
            action,
            file_path,
            meta_field,
            value_to_append: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;

    #[test]
    fn test_build_config_returns_ok() {
        let mut args: Vec<String> = Vec::new();
        args.push(String::from("xmptool"));
        args.push(String::from("read"));
        args.push(String::from("file_path.jpg"));
        args.push(String::from("persons"));

        let result = Config::build(&args);
        assert_matches!(result, Ok(Config { .. }))
    }

    #[test]
    fn test_build_config_raises_error_too_few_arguments() {
        let mut args: Vec<String> = Vec::new();
        args.push(String::from("xmptool"));

        let result = Config::build(&args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), String::from("not enough arguments"));
    }

    #[test]
    fn test_metadata_field_from_str_persons_returns_ok() {
        let input_value: String = String::from("persons");
        let result = MetadataField::from_str(&input_value);
        assert_matches!(result, Ok(MetadataField::Persons))
    }

    #[test]
    fn test_metadata_field_from_str_regions_returns_ok() {
        let input_value: String = String::from("regions");
        let result = MetadataField::from_str(&input_value);
        assert_matches!(result, Ok(MetadataField::Regions))
    }

    #[test]
    fn test_metadata_field_from_str_raises_error_invalid_field() {
        let input_value = String::from("people");
        let result = MetadataField::from_str(&input_value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            String::from("Invalid field 'people'. Choose 'persons' or regions'.")
        )
    }

    #[test]
    fn test_action_from_str_read_returns_ok() {
        let input_value: String = String::from("read");
        let result = Action::from_str(&input_value);
        assert_matches!(result, Ok(Action::Read))
    }

    #[test]
    fn test_action_from_str_write_returns_ok() {
        let input_value: String = String::from("write");
        let result = Action::from_str(&input_value);
        assert_matches!(result, Ok(Action::Write))
    }

    #[test]
    fn test_action_from_str_raises_error_invalid_action() {
        let input_value = String::from("open");
        let result = Action::from_str(&input_value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            String::from("Invalid action 'open'. Choose 'read' or 'write'.")
        )
    }
}
