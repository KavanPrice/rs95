//! Equipment Hierarchy: from ISA-95.00.02, it is the equipment hierarchy object model. The
//! hierarchy provides a structured decomposition of manufacturing operations from the enterprise
//! level down to individual pieces of equipment.
//!
//! Reference: https://reference.opcfoundation.org/ISA-95/v100/docs/4.2.2
//!
//! The five levels of the hierarchy are:
//!
//! - **Enterprise**: the top-level organisation (e.g. a company).
//! - **Site**: a physical location within an enterprise (e.g. a factory or warehouse).
//! - **Area**: a functional grouping within a site (e.g. an assembly floor or storage zone).
//! - **Work Center**: a grouping of work units, typed as one of: `ProcessCell`, `ProductionLine`,
//!   `ProductionUnit`, or `StorageZone`.
//! - **Work Unit**: the lowest level of the hierarchy, directly containing equipment, typed as one
//!   of: `Unit`, `WorkCell`, `StorageUnit`, or `ProductionEquipment`.
//!
//! Work Centers and Work Units can be associated with Equipment Classes and may have capability
//! test specifications and results recorded against them, consistent with the equipment model.
//! Enterprise, Site, and Area are organisational containers and carry only descriptive properties.

use super::equipment::{Equipment, EquipmentCapabilityTestResult, EquipmentCapabilityTestSpecification};

/// The type of a [`WorkCenter`], as defined by ISA-95.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WorkCenterType {
    ProcessCell,
    ProductionLine,
    ProductionUnit,
    StorageZone,
}

/// The type of a [`WorkUnit`], as defined by ISA-95.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WorkUnitType {
    Unit,
    WorkCell,
    StorageUnit,
    ProductionEquipment,
}

// ── Enterprise ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct Enterprise<ID> {
    pub id: ID,
    pub name: String,
    pub properties: Vec<EnterpriseProperty<ID>>,
    pub sites: Vec<Site<ID>>,
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
pub struct EnterpriseProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<EnterpriseProperty<ID>>,
}

// ── Site ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct Site<ID> {
    pub id: ID,
    pub name: String,
    pub properties: Vec<SiteProperty<ID>>,
    pub areas: Vec<Area<ID>>,
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
pub struct SiteProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<SiteProperty<ID>>,
}

// ── Area ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct Area<ID> {
    pub id: ID,
    pub name: String,
    pub properties: Vec<AreaProperty<ID>>,
    pub work_centers: Vec<WorkCenter<ID>>,
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
pub struct AreaProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<AreaProperty<ID>>,
}

// ── Work Center ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct WorkCenter<ID> {
    pub id: ID,
    pub name: String,
    pub work_center_type: WorkCenterType,
    pub equipment_classes: Vec<ID>, // IDs of EquipmentClass
    pub properties: Vec<WorkCenterProperty<ID>>,
    pub work_units: Vec<WorkUnit<ID>>,
    pub equipment_capability_test_specifications: Vec<EquipmentCapabilityTestSpecification<ID>>,
    pub equipment_capability_test_results: Vec<EquipmentCapabilityTestResult<ID>>,
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
pub struct WorkCenterProperty<ID> {
    pub id: ID,
    pub name: String,
    pub maps_to_equipment_class_property_id: Option<ID>,
    pub nested_properties: Vec<WorkCenterProperty<ID>>,
}

// ── Work Unit ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct WorkUnit<ID> {
    pub id: ID,
    pub name: String,
    pub work_unit_type: WorkUnitType,
    pub equipment_classes: Vec<ID>, // IDs of EquipmentClass
    pub properties: Vec<WorkUnitProperty<ID>>,
    pub equipment: Vec<Equipment<ID>>,
    pub equipment_capability_test_specifications: Vec<EquipmentCapabilityTestSpecification<ID>>,
    pub equipment_capability_test_results: Vec<EquipmentCapabilityTestResult<ID>>,
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
pub struct WorkUnitProperty<ID> {
    pub id: ID,
    pub name: String,
    pub maps_to_equipment_class_property_id: Option<ID>,
    pub nested_properties: Vec<WorkUnitProperty<ID>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::equipment::Equipment;
    use uuid::Uuid;

    #[test]
    fn test_equipment_hierarchy() {
        let equipment = Equipment::<Uuid> {
            id: Uuid::new_v4(),
            name: "Lathe-01".to_string(),
            equipment_classes: vec![],
            properties: vec![],
            sub_equipment: vec![],
        };

        let work_unit = WorkUnit::<Uuid> {
            id: Uuid::new_v4(),
            name: "Work Cell A".to_string(),
            work_unit_type: WorkUnitType::WorkCell,
            equipment_classes: vec![],
            properties: vec![],
            equipment: vec![equipment],
            equipment_capability_test_specifications: vec![],
            equipment_capability_test_results: vec![],
        };

        let work_center = WorkCenter::<Uuid> {
            id: Uuid::new_v4(),
            name: "Production Line 1".to_string(),
            work_center_type: WorkCenterType::ProductionLine,
            equipment_classes: vec![],
            properties: vec![],
            work_units: vec![work_unit],
            equipment_capability_test_specifications: vec![],
            equipment_capability_test_results: vec![],
        };

        let area = Area::<Uuid> {
            id: Uuid::new_v4(),
            name: "Assembly Area".to_string(),
            properties: vec![],
            work_centers: vec![work_center],
        };

        let site = Site::<Uuid> {
            id: Uuid::new_v4(),
            name: "Main Plant".to_string(),
            properties: vec![],
            areas: vec![area],
        };

        let enterprise = Enterprise::<Uuid> {
            id: Uuid::new_v4(),
            name: "Acme Corp".to_string(),
            properties: vec![],
            sites: vec![site],
        };

        assert_eq!(enterprise.name, "Acme Corp");
        assert_eq!(enterprise.sites[0].name, "Main Plant");
        assert_eq!(enterprise.sites[0].areas[0].name, "Assembly Area");
        assert_eq!(
            enterprise.sites[0].areas[0].work_centers[0].name,
            "Production Line 1"
        );
        assert_eq!(
            enterprise.sites[0].areas[0].work_centers[0].work_units[0].name,
            "Work Cell A"
        );
        assert_eq!(
            enterprise.sites[0].areas[0].work_centers[0].work_units[0].equipment[0].name,
            "Lathe-01"
        );
    }
}
