use config::{Action, Config, MetadataField};
use std::error::Error;
use models::{AppliedToDimensions, Area, Region, RegionType};
use persons::{append_person_shown, get_all_persons_shown};
use regions::{append_region, get_all_regions};
use xmpkit::{XmpFile, XmpMeta, XmpOptions};

pub mod config;
pub mod models;
pub mod persons;
pub mod regions;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
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

fn read_xmp_meta(config: &Config, file: &mut XmpFile) {
    let _ = file.open(&config.file_path);
    if let Some(meta) = file.get_xmp() {
        if matches!(config.meta_field, MetadataField::Persons) {
            let persons: Vec<String> = get_all_persons_shown(&meta).unwrap_or(Vec::new());
            println!("Field: PersonInImage {:?}", persons);
        } else {
            let regions: Vec<Region> = get_all_regions(&meta).unwrap_or(Vec::new());
            println!("Field: Regions {:?}", regions);
        }
    }
}

fn write_xmp_meta(config: &Config, file: &mut XmpFile) {
    let _ = file.open_with(&config.file_path, XmpOptions::default().for_update());
    let mut meta = file.get_xmp().cloned().unwrap_or_else(XmpMeta::new);
    if matches!(config.meta_field, MetadataField::Persons) {
        // PersonInImage append value logic
        append_person_shown(&mut meta, &config.value_to_append.clone().unwrap());
    } else {
        // Regions append value logic
        let dummy_region: Region = Region {
            region_type: RegionType::Face,
            region_name: String::from("Dummy Name"),
            dimensions: AppliedToDimensions {
                img_width: 100,
                img_height: 100,
                unit: String::from("pixel"),
            },
            area: Area {
                w: 0.1085,
                h: 0.0732,
                x: 0.3358,
                y: 0.2803,
                unit: String::from("normalized"),
            },
        };
        append_region(&mut meta, dummy_region);
        println!("Write to Regions unimplimented");
    }
    file.put_xmp(meta);
}

#[cfg(test)]
mod tests {

    // use super::*;

    #[test]
    fn test_run() {}

    #[test]
    fn test_read_xmp_meta() {}

    #[test]
    fn test_write_xmp_meta() {}
}
