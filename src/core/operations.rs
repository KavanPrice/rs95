//! Operations: from ISA-95.00.04, it is the operations management object model. This covers the
//! full lifecycle of manufacturing work — from defining how something should be made, through
//! scheduling and dispatching it, to recording what was actually done and what the site is
//! capable of.
//!
//! Reference: https://reference.opcfoundation.org/ISA-95/v100/docs/4.2.6
//!
//! The model is organised into four areas:
//!
//! - **Operations Definition** ([`WorkMaster`], [`OperationsDefinition`], [`OperationsSegment`]):
//!   structured descriptions of how work should be performed, referencing [`ProcessSegment`]s and
//!   specifying resource requirements.
//! - **Operations Scheduling** ([`OperationsSchedule`], [`OperationsRequest`],
//!   [`SegmentRequirement`]): planned work to be executed, referencing operations definitions and
//!   carrying timing and priority information.
//! - **Operations Performance** ([`JobOrder`], [`JobResponse`], [`SegmentResponse`],
//!   [`OperationsResponse`], [`OperationsPerformance`]): the transactional record of what was
//!   dispatched and what actually happened.
//! - **Operations Capability** ([`OperationsCapability`], [`OperationsCapabilityElement`]): what
//!   a site or area is currently able to produce.
//!
//! Timing fields use [`Option<String>`] to allow any ISO 8601 duration or datetime format without
//! prescribing a specific time library.
//!
//! [`ProcessSegment`]: super::process_segment::ProcessSegment

use super::process_segment::{
    EquipmentSegmentSpecification, MaterialSegmentSpecification,
    PersonnelSegmentSpecification, PhysicalAssetSegmentSpecification,
    ProcessSegmentDependencyType,
};

// ── Shared enumerations ───────────────────────────────────────────────────────

/// The category of manufacturing operation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OperationType {
    Production,
    Maintenance,
    Quality,
    Inventory,
    Mixed,
}

/// The action requested of a [`JobOrder`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JobOrderCommandType {
    Start,
    Stop,
    Hold,
    Restart,
    Abort,
}

/// The execution status of a [`JobOrder`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JobOrderStatus {
    Waiting,
    Ready,
    Running,
    Completed,
    Aborted,
}

/// The outcome recorded in a [`JobResponse`], [`SegmentResponse`], or [`OperationsResponse`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OperationsResponseResult {
    Completed,
    PartCompleted,
    Aborted,
}

// ── Work master ───────────────────────────────────────────────────────────────

/// A named value attached to a [`WorkMaster`], such as a target cycle time or batch size.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct WorkMasterParameter<ID> {
    pub id: ID,
    pub name: String,
    pub value: String,
    pub unit: String,
}

/// A general, reusable description of how a category of work should be performed. One or more
/// [`OperationsDefinition`]s may reference a WorkMaster to inherit its structure.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct WorkMaster<ID> {
    pub id: ID,
    pub name: String,
    pub version: String,
    pub parameters: Vec<WorkMasterParameter<ID>>,
    /// IDs of other WorkMasters that this one is assembled from.
    pub assembled_from: Vec<ID>,
}

// ── Operations definition ─────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct OperationsDefinitionProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<OperationsDefinitionProperty<ID>>,
}

/// A named value attached to an [`OperationsSegment`], such as a process parameter override.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct OperationsSegmentParameter<ID> {
    pub id: ID,
    pub name: String,
    pub value: String,
    pub unit: String,
}

/// A sequencing constraint between two [`OperationsSegment`]s within the same
/// [`OperationsDefinition`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct OperationsSegmentDependency<ID> {
    pub id: ID,
    pub dependency_type: ProcessSegmentDependencyType,
    pub from_operations_segment_id: ID,
    pub to_operations_segment_id: ID,
}

/// A step within an [`OperationsDefinition`]. References an underlying [`ProcessSegment`] and
/// carries resource specifications and parameter values specific to this operation.
///
/// [`ProcessSegment`]: super::process_segment::ProcessSegment
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct OperationsSegment<ID> {
    pub id: ID,
    pub name: String,
    /// ID of the [`ProcessSegment`] this operations segment is an instance of, if any.
    ///
    /// [`ProcessSegment`]: super::process_segment::ProcessSegment
    pub process_segment_id: Option<ID>,
    pub equipment_segment_specifications: Vec<EquipmentSegmentSpecification<ID>>,
    pub personnel_segment_specifications: Vec<PersonnelSegmentSpecification<ID>>,
    pub material_segment_specifications: Vec<MaterialSegmentSpecification<ID>>,
    pub physical_asset_segment_specifications: Vec<PhysicalAssetSegmentSpecification<ID>>,
    pub parameters: Vec<OperationsSegmentParameter<ID>>,
    pub sub_segments: Vec<OperationsSegment<ID>>,
    pub dependencies: Vec<OperationsSegmentDependency<ID>>,
}

/// A versioned, operation-type-specific description of how work should be performed. Optionally
/// derived from a [`WorkMaster`] and decomposed into [`OperationsSegment`]s.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct OperationsDefinition<ID> {
    pub id: ID,
    pub name: String,
    pub version: String,
    pub operation_type: OperationType,
    /// ID of the [`WorkMaster`] this definition is based on, if any.
    pub work_master_id: Option<ID>,
    pub properties: Vec<OperationsDefinitionProperty<ID>>,
    pub operations_segments: Vec<OperationsSegment<ID>>,
}

// ── Operations scheduling ─────────────────────────────────────────────────────

/// A named value attached to a [`SegmentRequirement`], used to supply or override parameter
/// values at scheduling time.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct SegmentRequirementParameter<ID> {
    pub id: ID,
    pub name: String,
    pub value: String,
    pub unit: String,
}

/// The scheduled execution of a specific [`OperationsSegment`] within an
/// [`OperationsRequest`], with timing constraints and parameter values.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct SegmentRequirement<ID> {
    pub id: ID,
    /// ID of the [`OperationsSegment`] to be executed.
    pub operations_segment_id: ID,
    pub earliest_start_time: Option<String>,
    pub latest_end_time: Option<String>,
    pub duration: Option<String>,
    pub parameters: Vec<SegmentRequirementParameter<ID>>,
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
pub struct OperationsRequestProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<OperationsRequestProperty<ID>>,
}

/// A request for a specific quantity of work to be performed, referencing an
/// [`OperationsDefinition`] and carrying a set of [`SegmentRequirement`]s.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct OperationsRequest<ID> {
    pub id: ID,
    pub operation_type: OperationType,
    /// ID of the [`OperationsDefinition`] to be executed.
    pub operations_definition_id: Option<ID>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub priority: Option<i32>,
    pub properties: Vec<OperationsRequestProperty<ID>>,
    pub segment_requirements: Vec<SegmentRequirement<ID>>,
}

/// A time-bounded collection of [`OperationsRequest`]s for a given operation type.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct OperationsSchedule<ID> {
    pub id: ID,
    pub operation_type: OperationType,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub operations_requests: Vec<OperationsRequest<ID>>,
}

// ── Job orders ────────────────────────────────────────────────────────────────

/// A named value attached to a [`JobOrder`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct JobOrderParameter<ID> {
    pub id: ID,
    pub name: String,
    pub value: String,
    pub unit: String,
}

/// A dispatched unit of work. References an [`OperationsRequest`] and/or an
/// [`OperationsDefinition`], carries an execution command and a status, and may have
/// timing and parameter information attached.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct JobOrder<ID> {
    pub id: ID,
    pub work_type: OperationType,
    pub command: JobOrderCommandType,
    pub status: JobOrderStatus,
    pub priority: Option<i32>,
    /// ID of the [`OperationsRequest`] this job order fulfils, if any.
    pub operations_request_id: Option<ID>,
    /// ID of the [`OperationsDefinition`] to be executed, if not derived from the request.
    pub operations_definition_id: Option<ID>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub parameters: Vec<JobOrderParameter<ID>>,
}

// ── Operations performance ────────────────────────────────────────────────────

/// A named value recorded as part of a [`JobResponse`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct JobResponseParameter<ID> {
    pub id: ID,
    pub name: String,
    pub value: String,
    pub unit: String,
}

/// The actual outcome of a [`JobOrder`]: when it ran, what the result was, and any recorded
/// parameter values.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct JobResponse<ID> {
    pub id: ID,
    /// ID of the [`JobOrder`] this response is for.
    pub job_order_id: ID,
    pub actual_start_time: Option<String>,
    pub actual_end_time: Option<String>,
    pub result: OperationsResponseResult,
    pub parameters: Vec<JobResponseParameter<ID>>,
}

/// The actual execution of a single [`OperationsSegment`], recording timing, result, and the
/// specific resources that were consumed or used.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct SegmentResponse<ID> {
    pub id: ID,
    /// ID of the [`OperationsSegment`] that was executed.
    pub operations_segment_id: ID,
    pub actual_start_time: Option<String>,
    pub actual_end_time: Option<String>,
    pub result: OperationsResponseResult,
    pub actual_equipment_ids: Vec<ID>,   // IDs of Equipment
    pub actual_person_ids: Vec<ID>,      // IDs of Person
    pub actual_material_lot_ids: Vec<ID>, // IDs of MaterialLot
    pub actual_physical_asset_ids: Vec<ID>, // IDs of PhysicalAsset
}

/// The actual outcome of an [`OperationsRequest`], aggregating [`SegmentResponse`]s and
/// [`JobResponse`]s.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct OperationsResponse<ID> {
    pub id: ID,
    /// ID of the [`OperationsRequest`] this response fulfils.
    pub operations_request_id: ID,
    pub actual_start_time: Option<String>,
    pub actual_end_time: Option<String>,
    pub result: OperationsResponseResult,
    pub segment_responses: Vec<SegmentResponse<ID>>,
    pub job_responses: Vec<JobResponse<ID>>,
}

/// A time-bounded record of what was actually performed, grouping [`OperationsResponse`]s for a
/// given operation type.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct OperationsPerformance<ID> {
    pub id: ID,
    pub operation_type: OperationType,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub operations_responses: Vec<OperationsResponse<ID>>,
}

// ── Operations capability ─────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct OperationsCapabilityProperty<ID> {
    pub id: ID,
    pub name: String,
    pub nested_properties: Vec<OperationsCapabilityProperty<ID>>,
}

/// A single capability entry, indicating that a given [`OperationsDefinition`] can be executed
/// within a stated time window.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct OperationsCapabilityElement<ID> {
    pub id: ID,
    pub operation_type: OperationType,
    /// ID of the [`OperationsDefinition`] this element describes capability for.
    pub operations_definition_id: Option<ID>,
    pub available_start_time: Option<String>,
    pub available_end_time: Option<String>,
    pub properties: Vec<OperationsCapabilityProperty<ID>>,
}

/// A time-bounded declaration of what a site or area can produce or perform.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct OperationsCapability<ID> {
    pub id: ID,
    pub operation_type: OperationType,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub elements: Vec<OperationsCapabilityElement<ID>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_operations_model() {
        // Build a WorkMaster and OperationsDefinition
        let work_master = WorkMaster::<Uuid> {
            id: Uuid::new_v4(),
            name: "Bottling".to_string(),
            version: "1.0".to_string(),
            parameters: vec![WorkMasterParameter {
                id: Uuid::new_v4(),
                name: "BatchSize".to_string(),
                value: "500".to_string(),
                unit: "units".to_string(),
            }],
            assembled_from: vec![],
        };

        let ops_segment_id = Uuid::new_v4();
        let ops_def = OperationsDefinition::<Uuid> {
            id: Uuid::new_v4(),
            name: "Bottling Run".to_string(),
            version: "1.0".to_string(),
            operation_type: OperationType::Production,
            work_master_id: Some(work_master.id),
            properties: vec![],
            operations_segments: vec![OperationsSegment {
                id: ops_segment_id,
                name: "Fill".to_string(),
                process_segment_id: None,
                equipment_segment_specifications: vec![],
                personnel_segment_specifications: vec![],
                material_segment_specifications: vec![],
                physical_asset_segment_specifications: vec![],
                parameters: vec![OperationsSegmentParameter {
                    id: Uuid::new_v4(),
                    name: "FillVolume".to_string(),
                    value: "330".to_string(),
                    unit: "ml".to_string(),
                }],
                sub_segments: vec![],
                dependencies: vec![],
            }],
        };

        // Schedule it
        let request_id = Uuid::new_v4();
        let schedule = OperationsSchedule::<Uuid> {
            id: Uuid::new_v4(),
            operation_type: OperationType::Production,
            start_time: Some("2026-04-14T06:00:00Z".to_string()),
            end_time: Some("2026-04-14T14:00:00Z".to_string()),
            operations_requests: vec![OperationsRequest {
                id: request_id,
                operation_type: OperationType::Production,
                operations_definition_id: Some(ops_def.id),
                start_time: Some("2026-04-14T08:00:00Z".to_string()),
                end_time: Some("2026-04-14T10:00:00Z".to_string()),
                priority: Some(1),
                properties: vec![],
                segment_requirements: vec![SegmentRequirement {
                    id: Uuid::new_v4(),
                    operations_segment_id: ops_segment_id,
                    earliest_start_time: Some("2026-04-14T08:00:00Z".to_string()),
                    latest_end_time: Some("2026-04-14T10:00:00Z".to_string()),
                    duration: Some("PT2H".to_string()),
                    parameters: vec![],
                }],
            }],
        };

        // Dispatch a job order
        let job_order = JobOrder::<Uuid> {
            id: Uuid::new_v4(),
            work_type: OperationType::Production,
            command: JobOrderCommandType::Start,
            status: JobOrderStatus::Waiting,
            priority: Some(1),
            operations_request_id: Some(request_id),
            operations_definition_id: Some(ops_def.id),
            start_time: Some("2026-04-14T08:00:00Z".to_string()),
            end_time: None,
            parameters: vec![],
        };

        // Record performance
        let performance = OperationsPerformance::<Uuid> {
            id: Uuid::new_v4(),
            operation_type: OperationType::Production,
            start_time: Some("2026-04-14T08:00:00Z".to_string()),
            end_time: Some("2026-04-14T09:45:00Z".to_string()),
            operations_responses: vec![OperationsResponse {
                id: Uuid::new_v4(),
                operations_request_id: request_id,
                actual_start_time: Some("2026-04-14T08:00:00Z".to_string()),
                actual_end_time: Some("2026-04-14T09:45:00Z".to_string()),
                result: OperationsResponseResult::Completed,
                segment_responses: vec![SegmentResponse {
                    id: Uuid::new_v4(),
                    operations_segment_id: ops_segment_id,
                    actual_start_time: Some("2026-04-14T08:00:00Z".to_string()),
                    actual_end_time: Some("2026-04-14T09:45:00Z".to_string()),
                    result: OperationsResponseResult::Completed,
                    actual_equipment_ids: vec![],
                    actual_person_ids: vec![],
                    actual_material_lot_ids: vec![],
                    actual_physical_asset_ids: vec![],
                }],
                job_responses: vec![JobResponse {
                    id: Uuid::new_v4(),
                    job_order_id: job_order.id,
                    actual_start_time: Some("2026-04-14T08:00:00Z".to_string()),
                    actual_end_time: Some("2026-04-14T09:45:00Z".to_string()),
                    result: OperationsResponseResult::Completed,
                    parameters: vec![],
                }],
            }],
        };

        // Declare capability
        let capability = OperationsCapability::<Uuid> {
            id: Uuid::new_v4(),
            operation_type: OperationType::Production,
            start_time: Some("2026-04-14T00:00:00Z".to_string()),
            end_time: Some("2026-04-15T00:00:00Z".to_string()),
            elements: vec![OperationsCapabilityElement {
                id: Uuid::new_v4(),
                operation_type: OperationType::Production,
                operations_definition_id: Some(ops_def.id),
                available_start_time: Some("2026-04-14T06:00:00Z".to_string()),
                available_end_time: Some("2026-04-14T22:00:00Z".to_string()),
                properties: vec![],
            }],
        };

        assert_eq!(work_master.name, "Bottling");
        assert_eq!(ops_def.operations_segments[0].name, "Fill");
        assert_eq!(schedule.operations_requests[0].priority, Some(1));
        assert_eq!(job_order.command, JobOrderCommandType::Start);
        assert_eq!(
            performance.operations_responses[0].result,
            OperationsResponseResult::Completed
        );
        assert_eq!(capability.elements[0].operation_type, OperationType::Production);
    }
}
