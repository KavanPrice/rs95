//! Material: from ISA-95.00.02, it is the ISA-95 material object model. This is the definition of the lots, sublots, material definitions, and material classes involved in production. This information allows Level 3 and Level 4 systems to unambiguously identify material specified in production schedules and consumed or produced in actual production. A prime business driver for using the material model is to provide the ability to trace all materials used in the manufacture of a product to aide in quality analysis and product recalls.
//!
//! Reference: https://reference.opcfoundation.org/ISA-95/v100/docs/4.2.4
//!
//! Each Material Definition instance defines a type of material, such as Acetic Acid, grade 4. Each Material Definition may have specific properties, such as pH. These properties can be nested, in that a property can have its own properties. Material Definitions may be assembled from other Material Definitions, as in the case of a sub-assembly in car manufacturing, such as a transaxle which is identified as a Material Definition but is pre-assembled from multiple other Material Definitions. The sub-assembly record keeping is important to maintain traceability.
//!
//! Material Definitions may belong to a Material Class. Material Classes are used as logical groups to manage Material definitions. Material Classes also have properties that can be nested and may also use the same assembly construct as used by Material Definitions.
//!
//! Each Material Lot is an instance of a Material Definition that is uniquely identified. Material Lots also have properties can be nested and may also use an assembly construct similar to that used by Material Definitions, the difference is that a Material Lot may be assembled from other Material Lots and/or Material Sublots. Material Lot instance properties are typically used to track specific shipments or orders of material.
//!
//! Each Material Sublot is an instance of a Material Definition that is uniquely identified. Material Sublots do not have properties since each sublot instance must have the same properties as the Material Lot it is part of. Material Sublots may also use the same assembly construct as used by Material Lots. Material Sublot instances are typically used to provide tracking resolution within Material Lots, for example a Material Lot may have been received from a vendor in ten barrels, each barrel may be identified as a separate Material Sublot in order to track its movements and environmental conditions during storage and production.
//!
//! The Material Test Specification identifies a test that may be associated with determining the value for a property of a Material Class, Material Definition or Material Lot instance. The information obtained from running the test can be modelled in the QA Test Result.
//!
//! ![ISA-95 material model UML diagram](https://reference.opcfoundation.org/api/image/get/115/image006.png)

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct MaterialClass<ID> {
    pub id: ID,
    pub name: String,
    pub properties: Vec<MaterialClassProperty<ID>>,
    pub assembled_from: Vec<ID>, // IDs of MaterialClass
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
pub struct MaterialClassProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<MaterialClassProperty<ID>>,
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
pub struct MaterialDefinition<ID> {
    pub id: ID,
    pub name: String,
    pub material_classes: Vec<ID>, // IDs of MaterialClass
    pub properties: Vec<MaterialDefinitionProperty<ID>>,
    pub assembled_from: Vec<ID>, // IDs of MaterialDefinition
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
pub struct MaterialDefinitionProperty<ID> {
    pub id: ID,
    pub name: String,
    pub maps_to_material_class_property_id: Option<ID>,
    pub nested_properties: Vec<MaterialDefinitionProperty<ID>>,
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
pub struct MaterialLot<ID> {
    pub id: ID,
    pub name: String,
    pub material_definition_id: ID, // Defined by 1..1
    pub properties: Vec<MaterialLotProperty<ID>>,
    pub assembled_from: Vec<ID>, // IDs of MaterialLot
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
pub struct MaterialSublot<ID> {
    pub id: ID,
    pub name: String,
    pub material_lot_id: ID, // Made up of (part of MaterialLot)
    pub properties: Vec<MaterialLotProperty<ID>>, // "Has values for" points to MaterialLotProperty
    pub sublots: Vec<MaterialSublot<ID>>,
    pub assembled_from: Vec<ID>, // IDs of MaterialSublot
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
pub struct MaterialLotProperty<ID> {
    pub id: ID,
    pub name: String,
    pub maps_to_material_definition_property_id: Option<ID>,
    pub nested_properties: Vec<MaterialLotProperty<ID>>,
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
pub struct MaterialTestSpecification<ID> {
    pub id: ID,
    pub name: String,
    pub material_class_ids: Vec<ID>,
    pub material_definition_ids: Vec<ID>,
    pub material_lot_ids: Vec<ID>,
    pub material_class_property_ids: Vec<ID>,
    pub material_definition_property_ids: Vec<ID>,
    pub material_lot_property_ids: Vec<ID>,
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
pub struct QATestResult<ID> {
    pub id: ID,
    pub name: String,
    pub material_test_specification_id: ID,
    pub material_lot_property_ids: Vec<ID>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_material_model() {
        let mc_id = Uuid::new_v4();
        let mc = MaterialClass::<Uuid> {
            id: mc_id,
            name: "Steel".to_string(),
            properties: vec![],
            assembled_from: vec![],
        };
        let md_id = Uuid::new_v4();
        let md = MaterialDefinition::<Uuid> {
            id: md_id,
            name: "Steel Sheet".to_string(),
            material_classes: vec![mc_id],
            properties: vec![],
            assembled_from: vec![],
        };
        let ml_id = Uuid::new_v4();
        let ml = MaterialLot::<Uuid> {
            id: ml_id,
            name: "Lot-001".to_string(),
            material_definition_id: md_id,
            properties: vec![],
            assembled_from: vec![],
        };
        assert_eq!(ml.name, "Lot-001");
        assert_eq!(md.name, "Steel Sheet");
        assert_eq!(mc.name, "Steel");
    }
}
