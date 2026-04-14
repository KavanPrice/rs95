//! Process Segment: from ISA-95.00.02, it is the process segment object model. A process segment
//! defines a named step or phase in a manufacturing process, specifying the resources required to
//! carry it out: equipment, personnel, material, and physical assets.
//!
//! Reference: https://reference.opcfoundation.org/ISA-95/v100/docs/4.2.5
//!
//! A [`ProcessSegment`] may contain nested sub-segments to represent hierarchical process
//! decomposition, and may declare dependencies on other segments to express sequencing
//! constraints. Each segment carries segment specifications — one per resource type — that
//! describe what is required (referencing resource classes or specific instances), in what
//! quantity, and how the resource is used. Parameters carry named values associated with a
//! segment (e.g. duration, temperature setpoint).
//!
//! The resource use enumerations distinguish how material is consumed or produced
//! ([`MaterialUse`]) from the simpler used/any distinction for equipment, personnel, and
//! physical assets ([`ResourceUse`]).

/// Describes how a resource (equipment, personnel, or physical asset) is used in a
/// [`ProcessSegment`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ResourceUse {
    Used,
    Any,
}

/// Describes how material participates in a [`ProcessSegment`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MaterialUse {
    Consumed,
    Produced,
    ByProduct,
    CoProduct,
    Yield,
    Sample,
    Any,
}

/// The ordering relationship between two process segments in a
/// [`ProcessSegmentDependency`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProcessSegmentDependencyType {
    /// The dependent segment starts at the same time as the referenced segment.
    AtStart,
    /// The dependent segment starts after the referenced segment has started.
    AfterStart,
    /// The dependent segment starts after the referenced segment has ended.
    AfterEnd,
    /// The dependent segment must not follow the referenced segment.
    NotFollow,
    /// The two segments may run in parallel.
    PossibleParallel,
    /// The two segments run in parallel.
    Parallel,
}

// ── Equipment segment specification ──────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct EquipmentSegmentSpecification<ID> {
    pub id: ID,
    pub equipment_class_id: Option<ID>,
    pub equipment_id: Option<ID>,
    pub quantity: Option<String>,
    pub unit: Option<String>,
    pub use_type: ResourceUse,
    pub properties: Vec<EquipmentSegmentSpecificationProperty<ID>>,
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
pub struct EquipmentSegmentSpecificationProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<EquipmentSegmentSpecificationProperty<ID>>,
}

// ── Personnel segment specification ───────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct PersonnelSegmentSpecification<ID> {
    pub id: ID,
    pub personnel_class_id: Option<ID>,
    pub person_id: Option<ID>,
    pub quantity: Option<String>,
    pub unit: Option<String>,
    pub use_type: ResourceUse,
    pub properties: Vec<PersonnelSegmentSpecificationProperty<ID>>,
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
pub struct PersonnelSegmentSpecificationProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<PersonnelSegmentSpecificationProperty<ID>>,
}

// ── Material segment specification ────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct MaterialSegmentSpecification<ID> {
    pub id: ID,
    pub material_class_id: Option<ID>,
    pub material_definition_id: Option<ID>,
    pub quantity: Option<String>,
    pub unit: Option<String>,
    pub use_type: MaterialUse,
    pub properties: Vec<MaterialSegmentSpecificationProperty<ID>>,
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
pub struct MaterialSegmentSpecificationProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<MaterialSegmentSpecificationProperty<ID>>,
}

// ── Physical asset segment specification ──────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct PhysicalAssetSegmentSpecification<ID> {
    pub id: ID,
    pub physical_asset_class_id: Option<ID>,
    pub physical_asset_id: Option<ID>,
    pub quantity: Option<String>,
    pub unit: Option<String>,
    pub use_type: ResourceUse,
    pub properties: Vec<PhysicalAssetSegmentSpecificationProperty<ID>>,
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
pub struct PhysicalAssetSegmentSpecificationProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<PhysicalAssetSegmentSpecificationProperty<ID>>,
}

// ── Process segment parameter ─────────────────────────────────────────────────

/// A named value associated with a [`ProcessSegment`], such as a duration, a temperature
/// setpoint, or a pressure limit.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct ProcessSegmentParameter<ID> {
    pub id: ID,
    pub name: String,
    pub value: String,
    pub unit: String,
}

// ── Process segment dependency ────────────────────────────────────────────────

/// A sequencing constraint between two [`ProcessSegment`]s.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct ProcessSegmentDependency<ID> {
    pub id: ID,
    pub dependency_type: ProcessSegmentDependencyType,
    pub from_process_segment_id: ID,
    pub to_process_segment_id: ID,
}

// ── Process segment ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct ProcessSegment<ID> {
    pub id: ID,
    pub name: String,
    /// Expected duration of the segment, represented as a string to allow any ISO 8601 duration
    /// or site-specific format (e.g. `"PT30M"`, `"2h"`).
    pub duration: Option<String>,
    pub equipment_segment_specifications: Vec<EquipmentSegmentSpecification<ID>>,
    pub personnel_segment_specifications: Vec<PersonnelSegmentSpecification<ID>>,
    pub material_segment_specifications: Vec<MaterialSegmentSpecification<ID>>,
    pub physical_asset_segment_specifications: Vec<PhysicalAssetSegmentSpecification<ID>>,
    pub parameters: Vec<ProcessSegmentParameter<ID>>,
    pub sub_segments: Vec<ProcessSegment<ID>>,
    pub dependencies: Vec<ProcessSegmentDependency<ID>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_process_segment() {
        let mixing_id = Uuid::new_v4();
        let filling_id = Uuid::new_v4();

        let mixing = ProcessSegment::<Uuid> {
            id: mixing_id,
            name: "Mixing".to_string(),
            duration: Some("PT30M".to_string()),
            equipment_segment_specifications: vec![EquipmentSegmentSpecification {
                id: Uuid::new_v4(),
                equipment_class_id: Some(Uuid::new_v4()),
                equipment_id: None,
                quantity: Some("1".to_string()),
                unit: None,
                use_type: ResourceUse::Used,
                properties: vec![],
            }],
            personnel_segment_specifications: vec![],
            material_segment_specifications: vec![
                MaterialSegmentSpecification {
                    id: Uuid::new_v4(),
                    material_class_id: None,
                    material_definition_id: Some(Uuid::new_v4()),
                    quantity: Some("100".to_string()),
                    unit: Some("kg".to_string()),
                    use_type: MaterialUse::Consumed,
                    properties: vec![],
                },
                MaterialSegmentSpecification {
                    id: Uuid::new_v4(),
                    material_class_id: None,
                    material_definition_id: Some(Uuid::new_v4()),
                    quantity: Some("95".to_string()),
                    unit: Some("kg".to_string()),
                    use_type: MaterialUse::Produced,
                    properties: vec![],
                },
            ],
            physical_asset_segment_specifications: vec![],
            parameters: vec![ProcessSegmentParameter {
                id: Uuid::new_v4(),
                name: "MixingSpeed".to_string(),
                value: "120".to_string(),
                unit: "rpm".to_string(),
            }],
            sub_segments: vec![],
            dependencies: vec![],
        };

        let filling = ProcessSegment::<Uuid> {
            id: filling_id,
            name: "Filling".to_string(),
            duration: Some("PT15M".to_string()),
            equipment_segment_specifications: vec![],
            personnel_segment_specifications: vec![],
            material_segment_specifications: vec![],
            physical_asset_segment_specifications: vec![],
            parameters: vec![],
            sub_segments: vec![],
            dependencies: vec![ProcessSegmentDependency {
                id: Uuid::new_v4(),
                dependency_type: ProcessSegmentDependencyType::AfterEnd,
                from_process_segment_id: mixing_id,
                to_process_segment_id: filling_id,
            }],
        };

        assert_eq!(mixing.name, "Mixing");
        assert_eq!(mixing.equipment_segment_specifications.len(), 1);
        assert_eq!(mixing.material_segment_specifications.len(), 2);
        assert_eq!(mixing.parameters[0].name, "MixingSpeed");
        assert_eq!(filling.dependencies[0].dependency_type, ProcessSegmentDependencyType::AfterEnd);
    }
}
