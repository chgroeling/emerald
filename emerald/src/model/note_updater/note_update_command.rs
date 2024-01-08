pub enum NoteUpdateCommand {
    UpdateOrInsert { key: String, value: String },
    DoNothing,
}
