use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MaterialClass {
    pub id: Uuid,
    pub name: String,
    pub properties: Vec<MaterialClassProperty>,
    pub assembled_from: Vec<Uuid>, // IDs of MaterialClass
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MaterialClassProperty {
    pub id: Uuid,
    pub name: String,
    pub nested_properties: Vec<MaterialClassProperty>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MaterialDefinition {
    pub id: Uuid,
    pub name: String,
    pub material_classes: Vec<Uuid>, // IDs of MaterialClass
    pub properties: Vec<MaterialDefinitionProperty>,
    pub assembled_from: Vec<Uuid>, // IDs of MaterialDefinition
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MaterialDefinitionProperty {
    pub id: Uuid,
    pub name: String,
    pub maps_to_material_class_property_id: Option<Uuid>,
    pub nested_properties: Vec<MaterialDefinitionProperty>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MaterialLot {
    pub id: Uuid,
    pub name: String,
    pub material_definition_id: Uuid, // Defined by 1..1
    pub properties: Vec<MaterialLotProperty>,
    pub assembled_from: Vec<Uuid>, // IDs of MaterialLot
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MaterialSublot {
    pub id: Uuid,
    pub name: String,
    pub material_lot_id: Uuid, // Made up of (part of MaterialLot)
    pub properties: Vec<MaterialLotProperty>, // "Has values for" points to MaterialLotProperty
    pub sublots: Vec<MaterialSublot>,
    pub assembled_from: Vec<Uuid>, // IDs of MaterialSublot
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MaterialLotProperty {
    pub id: Uuid,
    pub name: String,
    pub maps_to_material_definition_property_id: Option<Uuid>,
    pub nested_properties: Vec<MaterialLotProperty>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MaterialTestSpecification {
    pub id: Uuid,
    pub name: String,
    pub material_class_ids: Vec<Uuid>,
    pub material_definition_ids: Vec<Uuid>,
    pub material_lot_ids: Vec<Uuid>,
    pub material_class_property_ids: Vec<Uuid>,
    pub material_definition_property_ids: Vec<Uuid>,
    pub material_lot_property_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QATestResult {
    pub id: Uuid,
    pub name: String,
    pub material_test_specification_id: Uuid,
    pub material_lot_property_ids: Vec<Uuid>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_model() {
        let mc_id = Uuid::new_v4();
        let mc = MaterialClass {
            id: mc_id,
            name: "Steel".to_string(),
            properties: vec![],
            assembled_from: vec![],
        };
        let md_id = Uuid::new_v4();
        let md = MaterialDefinition {
            id: md_id,
            name: "Steel Sheet".to_string(),
            material_classes: vec![mc_id],
            properties: vec![],
            assembled_from: vec![],
        };
        let ml_id = Uuid::new_v4();
        let ml = MaterialLot {
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
