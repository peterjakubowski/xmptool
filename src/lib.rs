use models::{Config, MetadataField};
use xmpkit::core::namespace::ns;
use xmpkit::{XmpFile, XmpMeta, XmpOptions, XmpValue};

pub mod models;

pub fn read_xmp_meta(config: &Config, file: &mut XmpFile) {
    let _ = file.open(&config.file_path);
    if let Some(meta) = file.get_xmp() {
        if matches!(config.meta_field, MetadataField::Persons) {
            let persons = get_all_persons_shown(&meta).unwrap_or(Vec::new());
            println!("Field: PersonInImage {:?}", persons);
        } else {
            let regions = get_all_regions(&meta).unwrap_or(Vec::new());
            println!("Field: Regions {:?}", regions);
        }
    }
}

pub fn write_xmp_meta(config: &Config, file: &mut XmpFile) {
    let _ = file.open_with(&config.file_path, XmpOptions::default().for_update());
    let mut meta = file.get_xmp().cloned().unwrap_or_else(XmpMeta::new);
    if matches!(config.meta_field, MetadataField::Persons) {
        // PersonInImage append value logic
        append_person_shown(&mut meta, &config.value_to_append.clone().unwrap());
        println!("Write to PersonInImage unimplimented");
    } else {
        // Regions append value logic
        println!("Write to Regions unimplimented");
    }
    file.put_xmp(meta);
}

fn get_all_persons_shown(meta: &XmpMeta) -> Option<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    if let Some(n) = meta.get_array_size(ns::IPTC_EXT, "PersonInImage") {
        for i in 0..n + 1 {
            if let Some(person_shown) = meta.get_array_item(ns::IPTC_EXT, "PersonInImage", i) {
                result.push(person_shown.to_string());
            }
        }
        return Some(result);
    }
    None
}

fn get_all_regions(_meta: &XmpMeta) -> Option<Vec<String>> {
    None
}

fn check_person_shown_name_exists(meta: &XmpMeta, name_to_check: &String) -> bool {
    if let Some(all_person_shown_names) = get_all_persons_shown(meta) {
        for name in &all_person_shown_names {
            if name == name_to_check {
                return true;
            }
        }
    }
    false
}

fn append_person_shown(meta: &mut XmpMeta, name_to_append: &String) {
    if !check_person_shown_name_exists(meta, name_to_append) {
        let value_to_append = XmpValue::String(name_to_append.to_string());
        let _ = meta.append_array_item(ns::IPTC_EXT, "PersonInImage", value_to_append);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_xmp_packet(xmp_packet: &str) -> XmpMeta {
        let xmp_meta: XmpMeta = XmpMeta::parse(xmp_packet).unwrap_or(XmpMeta::new());
        xmp_meta
    }

    #[test]
    fn test_read_xmp_meta() {}

    #[test]
    fn test_write_xmp_meta() {}

    #[test]
    fn test_get_all_persons_shown_with_no_xmp() {
        let xmp_packet: &str = r#"<??>"#;
        let xmp_meta: XmpMeta = parse_xmp_packet(xmp_packet);
        let result = get_all_persons_shown(&xmp_meta);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_all_persons_shown_with_xmp() {
        let xmp_packet: &str = r#"<?xpacket begin="" id=""?>
            <x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core 7.0">
             <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
              <rdf:Description rdf:about=""
                xmlns:Iptc4xmpExt="http://iptc.org/std/Iptc4xmpExt/2008-02-29/">
               <Iptc4xmpExt:PersonInImage>
                <rdf:Bag>
                 <rdf:li>Test Name</rdf:li>
                </rdf:Bag>
               </Iptc4xmpExt:PersonInImage>
              </rdf:Description>
             </rdf:RDF>
            </x:xmpmeta>
            <?xpacket end="w"?>"#;
        let xmp_meta: XmpMeta = parse_xmp_packet(xmp_packet);
        let result = get_all_persons_shown(&xmp_meta).unwrap_or(Vec::new());
        assert_eq!(result.len(), 1);
        assert_eq!(result, ["Test Name"]);
    }

    #[test]
    fn test_check_person_shown_name_exists_with_no_xmp() {
        let xmp_packet: &str = r#"<??>"#;

        let xmp_meta: XmpMeta = parse_xmp_packet(xmp_packet);

        assert!(!check_person_shown_name_exists(
            &xmp_meta,
            &String::from("Test Name")
        ));
    }

    #[test]
    fn test_check_person_shown_name_exists_with_xmp() {
        let xmp_packet: &str = r#"<?xpacket begin="" id=""?>
            <x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core 7.0">
             <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
              <rdf:Description rdf:about=""
                xmlns:Iptc4xmpExt="http://iptc.org/std/Iptc4xmpExt/2008-02-29/">
               <Iptc4xmpExt:PersonInImage>
                <rdf:Bag>
                 <rdf:li>Test Name</rdf:li>
                </rdf:Bag>
               </Iptc4xmpExt:PersonInImage>
              </rdf:Description>
             </rdf:RDF>
            </x:xmpmeta>
            <?xpacket end="w"?>"#;
        let xmp_meta: XmpMeta = parse_xmp_packet(xmp_packet);
        assert!(check_person_shown_name_exists(
            &xmp_meta,
            &String::from("Test Name")
        ));
    }

    #[test]
    fn test_check_person_shown_name_exists_with_xmp_no_match() {
        let xmp_packet: &str = r#"<?xpacket begin="" id=""?>
            <x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core 7.0">
             <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
              <rdf:Description rdf:about=""
                xmlns:Iptc4xmpExt="http://iptc.org/std/Iptc4xmpExt/2008-02-29/">
               <Iptc4xmpExt:PersonInImage>
                <rdf:Bag>
                 <rdf:li>My Name</rdf:li>
                 <rdf:li>Your Name</rdf:li>
                 <rdf:li>Their Name</rdf:li>
                </rdf:Bag>
               </Iptc4xmpExt:PersonInImage>
              </rdf:Description>
             </rdf:RDF>
            </x:xmpmeta>
            <?xpacket end="w"?>"#;
        let xmp_meta: XmpMeta = parse_xmp_packet(xmp_packet);
        assert!(!check_person_shown_name_exists(
            &xmp_meta,
            &String::from("Test Name")
        ));
    }

    #[test]
    fn test_append_person_shown_with_new_xmp() {
        let mut xmp_meta: XmpMeta = XmpMeta::new();
        let name_to_append: String = String::from("Test Name");
        let result = append_person_shown(&mut xmp_meta, &name_to_append);
        assert_eq!(result, ());
        let all_persons_shown = get_all_persons_shown(&xmp_meta).unwrap_or(Vec::new());
        assert_eq!(all_persons_shown.len(), 1);
    }

    #[test]
    fn test_append_person_shown_with_no_xmp() {
        let xmp_packet: &str = r#"<??>"#;
        let mut xmp_meta: XmpMeta = parse_xmp_packet(xmp_packet);
        let name_to_append: String = String::from("Test Name");
        let result = append_person_shown(&mut xmp_meta, &name_to_append);
        assert_eq!(result, ());
        let all_persons_shown = get_all_persons_shown(&xmp_meta).unwrap_or(Vec::new());
        assert_eq!(all_persons_shown.len(), 1);
    }

    #[test]
    fn test_append_person_shown_with_xmp_name_exists() {
        let xmp_packet: &str = r#"<?xpacket begin="" id=""?>
            <x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core 7.0">
             <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
              <rdf:Description rdf:about=""
                xmlns:Iptc4xmpExt="http://iptc.org/std/Iptc4xmpExt/2008-02-29/">
               <Iptc4xmpExt:PersonInImage>
                <rdf:Bag>
                 <rdf:li>Test Name</rdf:li>
                </rdf:Bag>
               </Iptc4xmpExt:PersonInImage>
              </rdf:Description>
             </rdf:RDF>
            </x:xmpmeta>
            <?xpacket end="w"?>"#;
        let mut xmp_meta: XmpMeta = parse_xmp_packet(xmp_packet);
        let name_to_append: String = String::from("Test Name");
        let result = append_person_shown(&mut xmp_meta, &name_to_append);
        assert_eq!(result, ());
        let all_persons_shown = get_all_persons_shown(&xmp_meta).unwrap_or(Vec::new());
        assert_eq!(all_persons_shown.len(), 1);
    }

    #[test]
    fn test_append_person_shown_with_xmp_name_does_not_exist() {
        let xmp_packet: &str = r#"<?xpacket begin="" id=""?>
            <x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core 7.0">
             <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
              <rdf:Description rdf:about=""
                xmlns:Iptc4xmpExt="http://iptc.org/std/Iptc4xmpExt/2008-02-29/">
               <Iptc4xmpExt:PersonInImage>
                <rdf:Bag>
                 <rdf:li>My Name</rdf:li>
                 <rdf:li>Your Name</rdf:li>
                 <rdf:li>Their Name</rdf:li>
                </rdf:Bag>
               </Iptc4xmpExt:PersonInImage>
              </rdf:Description>
             </rdf:RDF>
            </x:xmpmeta>
            <?xpacket end="w"?>"#;
        let mut xmp_meta: XmpMeta = parse_xmp_packet(xmp_packet);
        let name_to_append: String = String::from("Test Name");
        let result = append_person_shown(&mut xmp_meta, &name_to_append);
        assert_eq!(result, ());
        let all_persons_shown = get_all_persons_shown(&xmp_meta).unwrap_or(Vec::new());
        assert_eq!(all_persons_shown.len(), 4);
    }
}
