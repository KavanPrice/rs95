//! Operational Location: from ISA-95.00.02, it is the operational location object model. An
//! operational location describes a physical place within a manufacturing operation — where
//! resources are, rather than what role they fulfil.
//!
//! Reference: https://reference.opcfoundation.org/ISA-95/v100/docs/4.2.3
//!
//! Operational location is intentionally separate from the equipment hierarchy. The equipment
//! hierarchy captures the functional organisation of manufacturing (what work centres and work
//! units exist and what they do); operational location captures physical space (where things
//! are). A mobile resource such as a forklift or a quality testing kit has a fixed place in
//! the equipment hierarchy but a changing operational location.
//!
//! [`OperationalLocation`]s can be nested to represent rooms within buildings, bays within
//! warehouses, and so on. Each location can be associated with one or more
//! [`OperationalLocationClass`]es and holds references to the resources currently present:
//! equipment, personnel, physical assets, and material lots.

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct OperationalLocationClass<ID> {
    pub id: ID,
    pub name: String,
    pub properties: Vec<OperationalLocationClassProperty<ID>>,
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
pub struct OperationalLocationClassProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<OperationalLocationClassProperty<ID>>,
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
pub struct OperationalLocation<ID> {
    pub id: ID,
    pub name: String,
    pub operational_location_classes: Vec<ID>, // IDs of OperationalLocationClass
    pub properties: Vec<OperationalLocationProperty<ID>>,
    pub sub_locations: Vec<OperationalLocation<ID>>,
    /// IDs of Equipment currently at this location.
    pub equipment_ids: Vec<ID>,
    /// IDs of Person currently at this location.
    pub personnel_ids: Vec<ID>,
    /// IDs of PhysicalAsset currently at this location.
    pub physical_asset_ids: Vec<ID>,
    /// IDs of MaterialLot currently at this location.
    pub material_lot_ids: Vec<ID>,
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
pub struct OperationalLocationProperty<ID> {
    pub id: ID,
    pub name: String,
    pub maps_to_operational_location_class_property_id: Option<ID>,
    pub nested_properties: Vec<OperationalLocationProperty<ID>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_operational_location() {
        let class_id = Uuid::new_v4();
        let class = OperationalLocationClass::<Uuid> {
            id: class_id,
            name: "Cold Storage".to_string(),
            properties: vec![],
        };

        let inner = OperationalLocation::<Uuid> {
            id: Uuid::new_v4(),
            name: "Bay 3".to_string(),
            operational_location_classes: vec![class_id],
            properties: vec![],
            sub_locations: vec![],
            equipment_ids: vec![],
            personnel_ids: vec![],
            physical_asset_ids: vec![Uuid::new_v4()],
            material_lot_ids: vec![Uuid::new_v4(), Uuid::new_v4()],
        };

        let outer = OperationalLocation::<Uuid> {
            id: Uuid::new_v4(),
            name: "Warehouse A".to_string(),
            operational_location_classes: vec![],
            properties: vec![],
            sub_locations: vec![inner],
            equipment_ids: vec![],
            personnel_ids: vec![],
            physical_asset_ids: vec![],
            material_lot_ids: vec![],
        };

        assert_eq!(class.name, "Cold Storage");
        assert_eq!(outer.name, "Warehouse A");
        assert_eq!(outer.sub_locations[0].name, "Bay 3");
        assert_eq!(outer.sub_locations[0].material_lot_ids.len(), 2);
    }
}
