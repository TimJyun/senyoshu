use std::collections::HashMap;

use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

use knowledge::Knowledge;
use learn_knowledge_history::{LearnKnowledgeHistory, OperateRecord};

use crate::types::learn::plan::Plan;

pub mod knowledge;
pub mod learn_knowledge_history;
pub mod plan;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LearnRecord {
    pub knowledge: Knowledge,
    pub history: Vec<OperateRecord>,
    pub plan: Plan,
}

#[derive(Serialize, Deserialize, Clone, Debug, Deref, DerefMut, Default)]
pub struct LearnHistoryMap(HashMap<Knowledge, LearnKnowledgeHistory>);

impl LearnHistoryMap {
    pub fn new(map: HashMap<Knowledge, LearnKnowledgeHistory>) -> Self {
        Self(map)
    }
    pub fn into_iter(self) -> impl Iterator<Item=(Knowledge, LearnKnowledgeHistory)> {
        self.0.into_iter()
    }
}
