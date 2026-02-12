use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EquipmentClass {
    pub id: Uuid,
    pub name: String,
    pub properties: Vec<EquipmentClassProperty>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EquipmentClassProperty {
    pub id: Uuid,
    pub name: String,
    pub nested_properties: Vec<EquipmentClassProperty>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Equipment {
    pub id: Uuid,
    pub name: String,
    pub equipment_classes: Vec<Uuid>, // IDs of EquipmentClass
    pub properties: Vec<EquipmentProperty>,
    pub sub_equipment: Vec<Equipment>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EquipmentProperty {
    pub id: Uuid,
    pub name: String,
    pub maps_to_equipment_class_property_id: Option<Uuid>,
    pub nested_properties: Vec<EquipmentProperty>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EquipmentCapabilityTestSpecification {
    pub id: Uuid,
    pub name: String,
    pub equipment_ids: Vec<Uuid>,
    pub equipment_class_ids: Vec<Uuid>,
    pub equipment_property_ids: Vec<Uuid>,
    pub equipment_class_property_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EquipmentCapabilityTestResult {
    pub id: Uuid,
    pub equipment_capability_test_specification_id: Uuid,
    pub equipment_property_ids: Vec<Uuid>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equipment_model() {
        let ec_id = Uuid::new_v4();
        let ec = EquipmentClass {
            id: ec_id,
            name: "Lathe".to_string(),
            properties: vec![],
        };
        let equipment_id = Uuid::new_v4();
        let equipment = Equipment {
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
