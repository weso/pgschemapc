use std::collections::HashSet;

use crate::{Name, record_type::RecordType};

pub struct FormalBaseType {
    labels: HashSet<Name>,
    content: HashSet<RecordType>, // Define the structure of FormalBaseType as needed
}

impl FormalBaseType {
    pub fn new() -> Self {
        FormalBaseType {
            labels: HashSet::new(),
            content: HashSet::new(),
        }
    }

    pub fn from_label(label: Name) -> Self {
        let mut base_type = FormalBaseType::new();
        base_type.labels.insert(label);
        base_type
    }

    pub fn add_label(&mut self, label: Name) {
        self.labels.insert(label);
    }

    pub fn add_content(&mut self, record_type: RecordType) {
        self.content.insert(record_type);
    }

    pub fn union(&self, other: &FormalBaseType) -> Self {
        let mut result = FormalBaseType::new();
        result.labels.extend(self.labels.iter().cloned());
        result.labels.extend(other.labels.iter().cloned());
        result.content.extend(self.content.iter().cloned());
        result.content.extend(other.content.iter().cloned());
        result
    }

    pub fn combine(&self, other: &FormalBaseType) -> Self {
        let mut result = FormalBaseType::new();
        let combined_labels: HashSet<_> = self.labels.union(&other.labels).cloned().collect();
        FormalBaseType {
            labels: combined_labels,
            content: combine_set_records(&self.content, &other.content),
        }
    }

    pub fn type_0() -> FormalBaseType {
        let mut content = HashSet::new();
        content.insert(RecordType::empty());
        FormalBaseType {
            labels: HashSet::new(),
            content,
        }
    }
}

fn combine_set_records(
    set1: &HashSet<RecordType>,
    set2: &HashSet<RecordType>,
) -> HashSet<RecordType> {
    let mut combined: HashSet<RecordType> = HashSet::new();
    for record1 in set1 {
        for record2 in set2 {
            let combined_record = record1.combine(&record2);
            combined.insert(combined_record);
        }
    }
    combined
}
