
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct PersonnelClass<ID> {
    pub id: ID,
    pub name: String,
    pub properties: Vec<PersonnelClassProperty<ID>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct PersonnelClassProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<PersonnelClassProperty<ID>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct Person<ID> {
    pub id: ID,
    pub name: String,
    pub personnel_classes: Vec<ID>, // IDs of PersonnelClass
    pub properties: Vec<PersonProperty<ID>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct PersonProperty<ID> {
    pub id: ID,
    pub name: String,
    pub maps_to_personnel_class_property_id: Option<ID>,
    pub nested_properties: Vec<PersonProperty<ID>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct QualificationTestSpecification<ID> {
    pub id: ID,
    pub name: String,
    pub person_ids: Vec<ID>,
    pub personnel_class_ids: Vec<ID>,
    pub person_property_ids: Vec<ID>,
    pub personnel_class_property_ids: Vec<ID>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "ID: serde::Serialize", deserialize = "ID: serde::Deserialize<'de>")))]
pub struct QualificationTestResult<ID> {
    pub id: ID,
    pub qualification_test_specification_id: ID,
    pub person_property_ids: Vec<ID>,
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use super::*;

    #[test]
    fn test_personnel_model() {
        let pc_prop_id = Uuid::new_v4();
        let pc_prop = PersonnelClassProperty::<Uuid> {
            id: pc_prop_id,
            name: "Skill".to_string(),
            nested_properties: vec![],
        };
        let pc_id = Uuid::new_v4();
        let pc = PersonnelClass::<Uuid> {
            id: pc_id,
            name: "Operator".to_string(),
            properties: vec![pc_prop],
        };
        let person_id = Uuid::new_v4();
        let person = Person::<Uuid> {
            id: person_id,
            name: "John Doe".to_string(),
            personnel_classes: vec![pc_id],
            properties: vec![PersonProperty {
                id: Uuid::new_v4(),
                name: "Welding Skill".to_string(),
                maps_to_personnel_class_property_id: Some(pc_prop_id),
                nested_properties: vec![],
            }],
        };
        assert_eq!(person.name, "John Doe");
        assert_eq!(pc.name, "Operator");
    }
}
