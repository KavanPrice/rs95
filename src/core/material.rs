
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct MaterialClass<ID> {
    pub id: ID,
    pub name: String,
    pub properties: Vec<MaterialClassProperty<ID>>,
    pub assembled_from: Vec<ID>, // IDs of MaterialClass
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct MaterialClassProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<MaterialClassProperty<ID>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct MaterialDefinition<ID> {
    pub id: ID,
    pub name: String,
    pub material_classes: Vec<ID>, // IDs of MaterialClass
    pub properties: Vec<MaterialDefinitionProperty<ID>>,
    pub assembled_from: Vec<ID>, // IDs of MaterialDefinition
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct MaterialDefinitionProperty<ID> {
    pub id: ID,
    pub name: String,
    pub maps_to_material_class_property_id: Option<ID>,
    pub nested_properties: Vec<MaterialDefinitionProperty<ID>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct MaterialLot<ID> {
    pub id: ID,
    pub name: String,
    pub material_definition_id: ID, // Defined by 1..1
    pub properties: Vec<MaterialLotProperty<ID>>,
    pub assembled_from: Vec<ID>, // IDs of MaterialLot
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
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
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct MaterialLotProperty<ID> {
    pub id: ID,
    pub name: String,
    pub maps_to_material_definition_property_id: Option<ID>,
    pub nested_properties: Vec<MaterialLotProperty<ID>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
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
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct QATestResult<ID> {
    pub id: ID,
    pub name: String,
    pub material_test_specification_id: ID,
    pub material_lot_property_ids: Vec<ID>,
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use super::*;

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
