//! Equipment: from ISA-95.00.02, it is the equipment object model. This is the definition of the roles that equipment and equipment classes perform in production, inventory management, and quality test labs. This information may be used to associate production with specific equipment as part of a production record, or with equipment classes to schedule production and allocate costs.
//!
//! Reference: https://reference.opcfoundation.org/ISA-95/v100/docs/4.2.2
//!
//! Each instance of the Equipment Class defines a classification of equipment, such as a tank. Each Equipment Class may have specific properties, such as a volume. These properties can be nested, in that a property can have its own properties. Each piece of Equipment can be associated to one or more Equipment Classes. If the Equipment is a tank, then the Equipment Properties define the values for the volume of the tank. Equipment can also be nested, in that they can contain other Equipment. The tank may also include several sensors such as temperature. The modeller can choose between extra properties or next properties and nest equipment depending on their own criteria. The Equipment Capability Test Specification identifies a test that may be associated with determining the value for a property. The information obtained from running the test can be modelled in the Equipment Capability Test Result.
//!
//! The Modelling approach for ISA-95 results in multiple levels of Equipment and Equipment Classes. By definition, if Equipment is defined by an Equipment Class, then it will have Equipment Properties that correspond to the Equipment Class Properties in the defining Equipment Class.

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct EquipmentClass<ID> {
    pub id: ID,
    pub name: String,
    pub properties: Vec<EquipmentClassProperty<ID>>,
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
pub struct EquipmentClassProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<EquipmentClassProperty<ID>>,
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
pub struct Equipment<ID> {
    pub id: ID,
    pub name: String,
    pub equipment_classes: Vec<ID>, // IDs of EquipmentClass
    pub properties: Vec<EquipmentProperty<ID>>,
    pub sub_equipment: Vec<Equipment<ID>>,
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
pub struct EquipmentProperty<ID> {
    pub id: ID,
    pub name: String,
    pub maps_to_equipment_class_property_id: Option<ID>,
    pub nested_properties: Vec<EquipmentProperty<ID>>,
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
pub struct EquipmentCapabilityTestSpecification<ID> {
    pub id: ID,
    pub name: String,
    pub equipment_ids: Vec<ID>,
    pub equipment_class_ids: Vec<ID>,
    pub equipment_property_ids: Vec<ID>,
    pub equipment_class_property_ids: Vec<ID>,
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
pub struct EquipmentCapabilityTestResult<ID> {
    pub id: ID,
    pub equipment_capability_test_specification_id: ID,
    pub equipment_property_ids: Vec<ID>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_equipment_model() {
        let ec_id = Uuid::new_v4();
        let ec = EquipmentClass::<Uuid> {
            id: ec_id,
            name: "Lathe".to_string(),
            properties: vec![],
        };
        let equipment_id = Uuid::new_v4();
        let equipment = Equipment::<Uuid> {
            id: equipment_id,
            name: "Lathe-01".to_string(),
            equipment_classes: vec![ec_id],
            properties: vec![],
            sub_equipment: vec![],
        };
        assert_eq!(equipment.name, "Lathe-01");
        assert_eq!(ec.name, "Lathe");
    }
}
