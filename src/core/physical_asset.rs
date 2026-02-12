//! Physical Asset: from ISA-95.00.02, it is the physical asset object model. This is an identification of the specific physical asset (by serial number or asset ID) used in manufacturing operations. It also includes the make and model information that identifies the type of physical asset and its properties.
//!
//! Reference: https://reference.opcfoundation.org/ISA-95/v100/docs/4.2.3
//!
//! Each instance of the Physical Asset Class defines a classification of Physical Asset, such as a valve. Each class may have specific properties, such as percent open. These properties can be nested, in that a property can have its own properties. Each Physical Asset is associated with one Physical Asset Classes. This relationship is more like a type of, for example, a car would be a Porsche 911 Carrera S, and it could not also be any other model. If the Physical Asset is a car, then the Physical Asset Properties define the values for the colour of the car and maybe the option package associated with the car. The option package would have its own properties such as leather interior, sunroof, etc. Physical Assets can also be nested, in that they can contain other Physical Assets. For example, a Car would contain four tyres that have their own Physical Asset Class. The Physical Asset Capability Test Specification identifies a test that may be associated with determining the value for a property. The information obtained from running the test can be modelled in the Physical Asset Capability Test Result.
//!
//! The Modelling approach for ISA-95 results in multiple levels of Physical Assets. By definition, if a Physical Asset is defined by a Physical Asset Class, then it will have Physical Asset Properties that correspond to the Physical Asset Class Properties in the defining Physical Asset Class.

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
