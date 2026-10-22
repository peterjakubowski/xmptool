use crate::models::{AppliedToDimensions, Area, Region, RegionType};
use xmpkit::{is_namespace_registered, register_namespace, XmpMeta, XmpValue};

const MWG_NAMESPACE: &str = "http://www.metadataworkinggroup.com/schemas/regions/";
const ADOBE_DIM_NAMESPACE: &str = "http://ns.adobe.com/xap/1.0/sType/Dimensions#";
const ADOBE_AREA_NAMESPACE: &str = "http://ns.adobe.com/xmp/sType/Area#";

fn check_and_register_mwg_namespace() {
    if !is_namespace_registered(MWG_NAMESPACE) {
        // println!("Registering namespace: {}", MWG_NAMESPACE);
        if let Err(e) = register_namespace(MWG_NAMESPACE, "mwg-rs") {
            eprintln!("Error: {}", e);
        }
    }
    if !is_namespace_registered(ADOBE_DIM_NAMESPACE) {
        // println!("Registering namespace {}", ADOBE_DIM_NAMESPACE);
        if let Err(e) = register_namespace(ADOBE_DIM_NAMESPACE, "stDim") {
            eprintln!("Error: {}", e);
        }
    }
    if !is_namespace_registered(ADOBE_AREA_NAMESPACE) {
        // println!("Registering namespace {}", ADOBE_AREA_NAMESPACE);
        if let Err(e) = register_namespace(ADOBE_AREA_NAMESPACE, "stArea") {
            eprintln!("Error: {}", e);
        }
    }
}

fn get_mwg_regions_applied_to_dimensions(meta: &XmpMeta) -> AppliedToDimensions {
    let img_width: i32 = meta
        .get_struct_field(MWG_NAMESPACE, "Regions/AppliedToDimensions", "stDim:w")
        .unwrap_or(XmpValue::Integer(0))
        .to_string()
        .parse()
        .unwrap();
    let img_height: i32 = meta
        .get_struct_field(MWG_NAMESPACE, "Regions/AppliedToDimensions", "stDim:h")
        .unwrap_or(XmpValue::Integer(0))
        .to_string()
        .parse()
        .unwrap();
    let unit: String = meta
        .get_struct_field(MWG_NAMESPACE, "Regions/AppliedToDimensions", "stDim:unit")
        .unwrap_or(XmpValue::String(String::from("default-px")))
        .to_string();
    AppliedToDimensions {
        img_width,
        img_height,
        unit,
    }
}

fn get_mwg_regions_area(meta: &XmpMeta, idx: usize) -> Area {
    let w: f32 = meta
        .get_struct_field(
            MWG_NAMESPACE,
            &format!("Regions/RegionList[{}]/Area", idx),
            "stArea:w",
        )
        .unwrap_or(XmpValue::String(String::from("0.0")))
        .to_string()
        .parse()
        .unwrap();
    let h: f32 = meta
        .get_struct_field(
            MWG_NAMESPACE,
            &format!("Regions/RegionList[{}]/Area", idx),
            "stArea:h",
        )
        .unwrap_or(XmpValue::String(String::from("0.0")))
        .to_string()
        .parse()
        .unwrap();
    let x: f32 = meta
        .get_struct_field(
            MWG_NAMESPACE,
            &format!("Regions/RegionList[{}]/Area", idx),
            "stArea:x",
        )
        .unwrap_or(XmpValue::String(String::from("0.0")))
        .to_string()
        .parse()
        .unwrap();
    let y: f32 = meta
        .get_struct_field(
            MWG_NAMESPACE,
            &format!("Regions/RegionList[{}]/Area", idx),
            "stArea:y",
        )
        .unwrap_or(XmpValue::String(String::from("0.0")))
        .to_string()
        .parse()
        .unwrap();
    let unit: String = meta
        .get_struct_field(
            MWG_NAMESPACE,
            &format!("Regions/RegionList[{}]/Area", idx),
            "stArea:unit",
        )
        .unwrap_or(XmpValue::String(String::from("default-norm")))
        .to_string();
    Area { w, h, x, y, unit }
}

pub fn get_all_regions(meta: &XmpMeta) -> Option<Vec<Region>> {
    check_and_register_mwg_namespace();
    let mut result: Vec<Region> = Vec::new();
    let dimensions: AppliedToDimensions = get_mwg_regions_applied_to_dimensions(meta);
    if let Some(n) = meta.get_array_size(MWG_NAMESPACE, "Regions/RegionList") {
        for i in 1..n + 1 {
            let region_type: String = meta
                .get_struct_field(MWG_NAMESPACE, &format!("Regions/RegionList[{}]", i), "Type")
                .unwrap_or(XmpValue::String(String::from("Face")))
                .to_string();
            let region_type: RegionType =
                RegionType::from_str(&region_type).unwrap_or(RegionType::Face);
            let region_name: String = meta
                .get_struct_field(MWG_NAMESPACE, &format!("Regions/RegionList[{}]", i), "Name")
                .unwrap_or(XmpValue::String(String::from("Missing Name")))
                .to_string();
            let area: Area = get_mwg_regions_area(meta, i);
            result.push(Region {
                region_type,
                region_name,
                dimensions: dimensions.clone(),
                area,
            });
        }
        return Some(result);
    }
    None
}

fn check_region_name_exists(meta: &XmpMeta, name_to_check: &String) -> bool {
    if let Some(all_region_names) = get_all_regions(meta) {
        for name in &all_region_names {
            if &name.region_name == name_to_check {
                return true;
            }
        }
    }
    false
}

fn check_applied_to_dimensions_exists(meta: &XmpMeta) -> bool {
    meta.has_property(MWG_NAMESPACE, "Regions/AppliedToDimensions")
}

fn set_applied_to_dimensions(meta: &mut XmpMeta, dimensions: AppliedToDimensions) {
    if let Err(e) = meta.set_struct_field(
        MWG_NAMESPACE,
        "Regions/AppliedToDimensions",
        "stDim:h",
        XmpValue::Integer(dimensions.img_height.into()),
    ) {
        eprintln!("Error {}", e);
    }
}

pub fn append_region(meta: &mut XmpMeta, region_to_append: Region) {
    if !check_region_name_exists(meta, &region_to_append.region_name) {
        if !check_applied_to_dimensions_exists(&meta) {
            set_applied_to_dimensions(meta, region_to_append.dimensions);
            println!("Append new region unimplimented.");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

    fn parse_xmp_packet(xmp_packet: &str) -> XmpMeta {
        let xmp_meta: XmpMeta = XmpMeta::parse(xmp_packet).unwrap_or(XmpMeta::new());
        xmp_meta
    }
    #[test]
    fn test_get_all_regions_with_no_xmp() {
        let xmp_packet: &str = r#"<??>"#;
        let xmp_meta: XmpMeta = parse_xmp_packet(xmp_packet);
        let result: Option<Vec<Region>> = get_all_regions(&xmp_meta);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_all_regions_with_xmp() {
        let xmp_packet: &str = r#"<?xpacket begin="" id=""?>
                <x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core 7.0">
                    <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
                        <rdf:Description rdf:about="" 
                            xmlns:mwg-rs="http://www.metadataworkinggroup.com/schemas/regions/"
                            xmlns:stDim="http://ns.adobe.com/xap/1.0/sType/Dimensions#"
                            xmlns:stArea="http://ns.adobe.com/xmp/sType/Area#">
                            <mwg-rs:Regions rdf:parseType="Resource">
                                <mwg-rs:AppliedToDimensions rdf:parseType="Resource">
                                    <stDim:h>1600</stDim:h>
                                    <stDim:unit>pixel</stDim:unit>
                                    <stDim:w>2400</stDim:w>
                                </mwg-rs:AppliedToDimensions>
                                <mwg-rs:RegionList>
                                    <rdf:Bag>
                                        <rdf:li rdf:parseType="Resource">
                                            <mwg-rs:Area rdf:parseType="Resource">
                                                <stArea:h>0.0732</stArea:h>
                                                <stArea:unit>normalized</stArea:unit>
                                                <stArea:w>0.1085</stArea:w>
                                                <stArea:x>0.3358</stArea:x>
                                                <stArea:y>0.2803</stArea:y>
                                            </mwg-rs:Area>
                                            <mwg-rs:Name>Felix</mwg-rs:Name>
                                            <mwg-rs:Type>Pet</mwg-rs:Type>
                                        </rdf:li>
                                    </rdf:Bag>
                                </mwg-rs:RegionList>
                            </mwg-rs:Regions>
                        </rdf:Description>
                    </rdf:RDF>
                </x:xmpmeta>
            <?xpacket end="w"?>"#;
        let xmp_meta: XmpMeta = parse_xmp_packet(xmp_packet);
        let result: Vec<Region> = get_all_regions(&xmp_meta).unwrap_or(Vec::new());
        assert_eq!(result.len(), 1);
        let region: &Region = &result[0];
        assert_eq!(region.region_name, "Felix");
        assert_matches!(region.region_type, RegionType::Pet);
        assert_eq!(region.area.w, 0.1085);
        assert_eq!(region.area.h, 0.0732);
        assert_eq!(region.area.x, 0.3358);
        assert_eq!(region.area.y, 0.2803);
        assert_eq!(region.area.unit, "normalized");
        assert_eq!(region.dimensions.img_width, 2400);
        assert_eq!(region.dimensions.img_height, 1600);
        assert_eq!(region.dimensions.unit, "pixel");
    }

    #[test]
    fn test_check_region_name_exists_with_no_xmp() {
        let xmp_packet: &str = r#"<??>"#;
        let xmp_meta: XmpMeta = parse_xmp_packet(xmp_packet);
        assert!(!check_region_name_exists(
            &xmp_meta,
            &String::from("Test Name")
        ));
    }

    #[test]
    fn test_check_region_name_exists_with_xmp() {
        let xmp_packet: &str = r#"<?xpacket begin="" id=""?>
                <x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core 7.0">
                    <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
                        <rdf:Description rdf:about="" 
                            xmlns:mwg-rs="http://www.metadataworkinggroup.com/schemas/regions/"
                            xmlns:stDim="http://ns.adobe.com/xap/1.0/sType/Dimensions#"
                            xmlns:stArea="http://ns.adobe.com/xmp/sType/Area#">
                            <mwg-rs:Regions rdf:parseType="Resource">
                                <mwg-rs:AppliedToDimensions rdf:parseType="Resource">
                                    <stDim:h>1600</stDim:h>
                                    <stDim:unit>pixel</stDim:unit>
                                    <stDim:w>2400</stDim:w>
                                </mwg-rs:AppliedToDimensions>
                                <mwg-rs:RegionList>
                                    <rdf:Bag>
                                        <rdf:li rdf:parseType="Resource">
                                            <mwg-rs:Area rdf:parseType="Resource">
                                                <stArea:h>0.0732</stArea:h>
                                                <stArea:unit>normalized</stArea:unit>
                                                <stArea:w>0.1085</stArea:w>
                                                <stArea:x>0.3358</stArea:x>
                                                <stArea:y>0.2803</stArea:y>
                                            </mwg-rs:Area>
                                            <mwg-rs:Name>Test Name</mwg-rs:Name>
                                            <mwg-rs:Type>Face</mwg-rs:Type>
                                        </rdf:li>
                                    </rdf:Bag>
                                </mwg-rs:RegionList>
                            </mwg-rs:Regions>
                        </rdf:Description>
                    </rdf:RDF>
                </x:xmpmeta>
            <?xpacket end="w"?>"#;
        let xmp_meta: XmpMeta = parse_xmp_packet(xmp_packet);
        assert!(check_region_name_exists(
            &xmp_meta,
            &String::from("Test Name")
        ));
    }

    #[test]
    fn test_check_region_name_exists_with_xmp_no_match() {
        let xmp_packet: &str = r#"<?xpacket begin="" id=""?>
                <x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core 7.0">
                    <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
                        <rdf:Description rdf:about="" 
                            xmlns:mwg-rs="http://www.metadataworkinggroup.com/schemas/regions/"
                            xmlns:stDim="http://ns.adobe.com/xap/1.0/sType/Dimensions#"
                            xmlns:stArea="http://ns.adobe.com/xmp/sType/Area#">
                            <mwg-rs:Regions rdf:parseType="Resource">
                                <mwg-rs:AppliedToDimensions rdf:parseType="Resource">
                                    <stDim:h>1600</stDim:h>
                                    <stDim:unit>pixel</stDim:unit>
                                    <stDim:w>2400</stDim:w>
                                </mwg-rs:AppliedToDimensions>
                                <mwg-rs:RegionList>
                                    <rdf:Bag>
                                        <rdf:li rdf:parseType="Resource">
                                            <mwg-rs:Area rdf:parseType="Resource">
                                                <stArea:h>0.0732</stArea:h>
                                                <stArea:unit>normalized</stArea:unit>
                                                <stArea:w>0.1085</stArea:w>
                                                <stArea:x>0.3358</stArea:x>
                                                <stArea:y>0.2803</stArea:y>
                                            </mwg-rs:Area>
                                            <mwg-rs:Name>Felix</mwg-rs:Name>
                                            <mwg-rs:Type>Face</mwg-rs:Type>
                                        </rdf:li>
                                    </rdf:Bag>
                                </mwg-rs:RegionList>
                            </mwg-rs:Regions>
                        </rdf:Description>
                    </rdf:RDF>
                </x:xmpmeta>
            <?xpacket end="w"?>"#;
        let xmp_meta: XmpMeta = parse_xmp_packet(xmp_packet);
        assert!(!check_region_name_exists(
            &xmp_meta,
            &String::from("Test Name")
        ));
    }

    #[test]
    fn test_check_applied_dimensions_exists_no_xmp() {
        let xmp_packet: &str = r#"<??>"#;
        let xmp_meta: XmpMeta = parse_xmp_packet(xmp_packet);
        assert!(!check_applied_to_dimensions_exists(&xmp_meta));
    }

    #[test]
    fn test_check_applied_dimensions_exists_with_xmp() {
        let xmp_packet: &str = r#"<?xpacket begin="" id=""?>
                <x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core 7.0">
                    <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
                        <rdf:Description rdf:about="" 
                            xmlns:mwg-rs="http://www.metadataworkinggroup.com/schemas/regions/"
                            xmlns:stDim="http://ns.adobe.com/xap/1.0/sType/Dimensions#">
                            <mwg-rs:Regions rdf:parseType="Resource">
                                <mwg-rs:AppliedToDimensions rdf:parseType="Resource">
                                    <stDim:h>1600</stDim:h>
                                    <stDim:unit>pixel</stDim:unit>
                                    <stDim:w>2400</stDim:w>
                                </mwg-rs:AppliedToDimensions>
                            </mwg-rs:Regions>
                        </rdf:Description>
                    </rdf:RDF>
                </x:xmpmeta>
            <?xpacket end="w"?>"#;
        let xmp_meta: XmpMeta = parse_xmp_packet(xmp_packet);
        assert!(check_applied_to_dimensions_exists(&xmp_meta));
    }

    #[test]
    fn test_set_applied_to_dimension_with_no_xmp() {}

    #[test]
    fn test_set_applied_to_dimension_with_xmp() {}

    #[test]
    fn test_append_region_with_no_xmp() {}

    #[test]
    fn test_append_region_with_xmp_name_exists() {}

    #[test]
    fn test_append_region_with_xmp_names_does_not_exist() {}
}
