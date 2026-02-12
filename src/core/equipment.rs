
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct EquipmentClass<ID> {
    pub id: ID,
    pub name: String,
    pub properties: Vec<EquipmentClassProperty<ID>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct EquipmentClassProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<EquipmentClassProperty<ID>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct Equipment<ID> {
    pub id: ID,
    pub name: String,
    pub equipment_classes: Vec<ID>, // IDs of EquipmentClass
    pub properties: Vec<EquipmentProperty<ID>>,
    pub sub_equipment: Vec<Equipment<ID>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct EquipmentProperty<ID> {
    pub id: ID,
    pub name: String,
    pub maps_to_equipment_class_property_id: Option<ID>,
    pub nested_properties: Vec<EquipmentProperty<ID>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
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
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct EquipmentCapabilityTestResult<ID> {
    pub id: ID,
    pub equipment_capability_test_specification_id: ID,
    pub equipment_property_ids: Vec<ID>,
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use super::*;

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
