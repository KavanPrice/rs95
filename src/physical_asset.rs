use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalAssetClass {
    pub id: Uuid,
    pub name: String,
    pub properties: Vec<PhysicalAssetClassProperty>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalAssetClassProperty {
    pub id: Uuid,
    pub name: String,
    pub nested_properties: Vec<PhysicalAssetClassProperty>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalAsset {
    pub id: Uuid,
    pub name: String,
    pub physical_asset_class_id: Uuid, // is an instance of 1..1
    pub properties: Vec<PhysicalAssetProperty>,
    pub sub_assets: Vec<PhysicalAsset>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalAssetProperty {
    pub id: Uuid,
    pub name: String,
    pub maps_to_physical_asset_class_property_id: Option<Uuid>,
    pub nested_properties: Vec<PhysicalAssetProperty>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalAssetCapabilityTestSpecification {
    pub id: Uuid,
    pub name: String,
    pub physical_asset_ids: Vec<Uuid>,
    pub physical_asset_class_ids: Vec<Uuid>,
    pub physical_asset_property_ids: Vec<Uuid>,
    pub physical_asset_class_property_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalAssetCapabilityTestResult {
    pub id: Uuid,
    pub physical_asset_capability_test_specification_id: Uuid,
    pub physical_asset_property_ids: Vec<Uuid>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physical_asset_model() {
        let pac_id = Uuid::new_v4();
        let pac = PhysicalAssetClass {
            id: pac_id,
            name: "CNC Machine".to_string(),
            properties: vec![],
        };
        let asset_id = Uuid::new_v4();
        let asset = PhysicalAsset {
            id: asset_id,
            name: "CNC-001".to_string(),
            physical_asset_class_id: pac_id,
            properties: vec![],
            sub_assets: vec![],
        };
        assert_eq!(asset.name, "CNC-001");
        assert_eq!(pac.name, "CNC Machine");
    }
}
