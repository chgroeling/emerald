pub enum NoteUpdateCommand {
    UpdateEntry { entry: String, value: String },
    DoNothing,
}
