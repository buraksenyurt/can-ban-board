pub enum ExecuteResult {
    NoRowsAffected,
    UpdatedOneRow,
    WorkItemCreated(u32),
    MovedToArchive,
    Deleted,
}
