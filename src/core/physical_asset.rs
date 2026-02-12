#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct PhysicalAssetClass<ID> {
    pub id: ID,
    pub name: String,
    pub properties: Vec<PhysicalAssetClassProperty<ID>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct PhysicalAssetClassProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<PhysicalAssetClassProperty<ID>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct PhysicalAsset<ID> {
    pub id: ID,
    pub name: String,
    pub physical_asset_class_id: ID, // is an instance of 1..1
    pub properties: Vec<PhysicalAssetProperty<ID>>,
    pub sub_assets: Vec<PhysicalAsset<ID>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct PhysicalAssetProperty<ID> {
    pub id: ID,
    pub name: String,
    pub maps_to_physical_asset_class_property_id: Option<ID>,
    pub nested_properties: Vec<PhysicalAssetProperty<ID>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct PhysicalAssetCapabilityTestSpecification<ID> {
    pub id: ID,
    pub name: String,
    pub physical_asset_ids: Vec<ID>,
    pub physical_asset_class_ids: Vec<ID>,
    pub physical_asset_property_ids: Vec<ID>,
    pub physical_asset_class_property_ids: Vec<ID>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct PhysicalAssetCapabilityTestResult<ID> {
    pub id: ID,
    pub physical_asset_capability_test_specification_id: ID,
    pub physical_asset_property_ids: Vec<ID>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_physical_asset_model() {
        let pac_id = Uuid::new_v4();
        let pac = PhysicalAssetClass::<Uuid> {
            id: pac_id,
            name: "CNC Machine".to_string(),
            properties: vec![],
        };
        let asset_id = Uuid::new_v4();
        let asset = PhysicalAsset::<Uuid> {
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
