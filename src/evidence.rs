#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Evidence {
    LabelsContentConforms {
        labels: String,
        record: String,
        type_content: String,
    },
}
