pub(crate) enum ParseResult {
    Failed,
    Ok,
    Yield(usize, usize),
}
