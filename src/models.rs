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
pub struct Region {
    pub region_type: RegionType,
    pub region_name: String,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;

    #[test]
    fn test_region_type_from_str_face_returns_ok() {
        let input_value: String = String::from("Face");
        let result = RegionType::from_str(&input_value);
        assert_matches!(result, Ok(RegionType::Face));
    }

    #[test]
    fn test_region_type_from_str_raises_error_invalid_type() {
        let input_value = String::from("Cat");
        let result = RegionType::from_str(&input_value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            String::from("Invalid region type 'Cat'. Choose 'face', 'pet', 'focus' or 'barcode'.")
        )
    }
}
