pub enum NoteUpdateCommand {
    ChangeEntry { entry: String, value: String },
    DoNothing,
}
