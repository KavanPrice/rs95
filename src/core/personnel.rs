use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PersonnelClass {
    pub id: Uuid,
    pub name: String,
    pub properties: Vec<PersonnelClassProperty>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PersonnelClassProperty {
    pub id: Uuid,
    pub name: String,
    pub nested_properties: Vec<PersonnelClassProperty>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub personnel_classes: Vec<Uuid>, // IDs of PersonnelClass
    pub properties: Vec<PersonProperty>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PersonProperty {
    pub id: Uuid,
    pub name: String,
    pub maps_to_personnel_class_property_id: Option<Uuid>,
    pub nested_properties: Vec<PersonProperty>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QualificationTestSpecification {
    pub id: Uuid,
    pub name: String,
    pub person_ids: Vec<Uuid>,
    pub personnel_class_ids: Vec<Uuid>,
    pub person_property_ids: Vec<Uuid>,
    pub personnel_class_property_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QualificationTestResult {
    pub id: Uuid,
    pub qualification_test_specification_id: Uuid,
    pub person_property_ids: Vec<Uuid>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_personnel_model() {
        let pc_prop_id = Uuid::new_v4();
        let pc_prop = PersonnelClassProperty {
            id: pc_prop_id,
            name: "Skill".to_string(),
            nested_properties: vec![],
        };
        let pc_id = Uuid::new_v4();
        let pc = PersonnelClass {
            id: pc_id,
            name: "Operator".to_string(),
            properties: vec![pc_prop],
        };
        let person_id = Uuid::new_v4();
        let person = Person {
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
