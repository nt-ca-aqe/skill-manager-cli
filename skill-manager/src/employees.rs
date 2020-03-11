use crate::{projects::Project, skills::SkillLabel};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use time::Date;
use uuid::Uuid;

pub mod in_memory;
pub mod usecase;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct Employee {
    pub id: EmployeeId,
    pub first_name: FirstName,
    pub last_name: LastName,
    pub skills: BTreeMap<SkillLabel, Knowledge>,
    pub projects: Vec<ProjectAssignment>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct ProjectAssignment {
    pub id: ProjectAssignmentId,
    pub project: Project,
    pub contribution: ProjectContribution,
    pub start_date: Date,
    pub end_date: Option<Date>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct Knowledge {
    pub level: SkillLevel,
}

gen_wrapper!(
    EmployeeId: Uuid,
    FirstName: String,
    LastName: String,
    ProjectAssignmentId: Uuid,
    ProjectContribution: String,
    SkillLevel: usize
);