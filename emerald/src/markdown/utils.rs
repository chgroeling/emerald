pub(crate) enum ConsumeResult {
    Some((usize, char)),
    None,
    Eof,
}

/// Represents the result of a character consumption operation in a character iterator.
///
/// # Variants
///
/// - `Some((usize, char))`: Indicates that a character was successfully consumed from the iterator.
///   It holds a tuple containing the index of the character in the original string and the character itself.
///   This variant is typically returned when the character meets the criteria specified in the c
///   onsumption operation.
///
/// - `None`: Represents the scenario where the current character in the iterator does not meet the
///   consumption criteria, and therefore, is not consumed. This variant allows the iterator to remain at
///   the current character, which can then be re-evaluated or consumed by a subsequent operation.
///
/// - `Eof`: Signifies that the end of the input has been reached. This variant is returned when there are
///   no more characters to consume, indicating that the parsing process has reached the end of the input string.
impl ConsumeResult {
    /// Checks if the result contains a index and a character.
    #[allow(dead_code)]
    pub(crate) fn is_some(&self) -> bool {
        matches!(self, ConsumeResult::Some(_))
    }

    /// Checks if the result is `None`.
    #[allow(dead_code)]
    pub(crate) fn is_none(&self) -> bool {
        matches!(self, ConsumeResult::None)
    }

    /// Checks if the result indicates the end of the input (`Eof`).
    #[allow(dead_code)]
    pub(crate) fn is_eof(&self) -> bool {
        matches!(self, ConsumeResult::Eof)
    }

    /// Checks if the result is either `None` or `Eof`.
    #[allow(dead_code)]
    pub(crate) fn is_none_or_eof(&self) -> bool {
        matches!(self, ConsumeResult::None | ConsumeResult::Eof)
    }
}

/// Consumes the next character from the iterator.
///
/// This macro advances the given character iterator and consumes the next character.
/// If there is a next character, it returns a `ConsumeResult::Some`, containing the index
/// and character. If the iterator is at the end of the input, it returns `ConsumeResult::Eof`.
///
/// # Parameters
/// - `$iter`: A mutable reference to a `Peekable<CharIndices>` iterator over a string slice.
///   The iterator must be able to yield pairs of index and character (`(usize, char)`).
///
/// # Returns
/// - `ConsumeResult::Some((usize, char))` if there is a next character in the iterator.
/// - `ConsumeResult::Eof` if the iterator has reached the end of the input string.
macro_rules! consume {
    ($iter:expr) => {
        if let Some((idx, ch)) = $iter.next() {
            ConsumeResult::Some((idx, ch))
        } else {
            ConsumeResult::Eof
        }
    };
}

/// Consumes the next character in the iterator if it matches any of the provided patterns.
///
/// This macro peeks at the next character in the given character iterator. If the character
/// matches any of the specified patterns, the macro consumes this character (advances the iterator)
/// and returns a `ConsumeResult::Some` containing the index and character. If the character does not
/// match any of the patterns, it returns `ConsumeResult::None` without advancing the iterator.
/// If the iterator is at the end, it returns `ConsumeResult::Eof`.
///
/// # Parameters
/// - `$iter`: A mutable reference to a `Peekable<CharIndices>` iterator over a string slice.
/// - `$($a:pat)+`: One or more patterns to match against the next character. These can be
///   simple characters, ranges, or any pattern that can be used in a `match` arm.
///
/// # Returns
/// - `ConsumeResult::Some((usize, char))` if the next character matches any of the provided patterns.
/// - `ConsumeResult::None` if the next character does not match any of the patterns.
/// - `ConsumeResult::Eof` if the iterator has reached the end of the input string.
macro_rules! consume_expected_chars{
    ($iter:expr, $($a:pat)+) => {
        if let Some((index,ch)) = $iter.peek().cloned() {
            match ch {
                $($a)|+ => {
                    $iter.next(); // consume
                    ConsumeResult::Some((index, ch))
                }
                _ => {
                    ConsumeResult::None
                }
            }
        } else {
            ConsumeResult::Eof
        }
    };
}

/// Collects and counts characters from an iterator that match a given pattern.
///
/// This macro iterates over characters from a given character iterator,
/// counting the number of characters that match the specified pattern(s).
/// The iteration stops either when a non-matching character is found or
/// when the specified limit is reached, if provided.
///
/// # Arguments
/// * `$iter` - An expression yielding a `Peekable<CharIndices>` iterator.
/// * `$limit` - An optional limit for the number of characters to count.
///             If `None`, the macro counts until a non-matching character is encountered.
/// * `$($a:pat)+` - One or more patterns to match against each character.
///
/// # Returns
/// The count of matching characters found up to the limit, or until a non-matching character.
/// ```
macro_rules! gather {
    ($iter:expr, $limit: expr, $($a:pat)+) => {{
        // special case .. limit is zero or negative .. do nothing
        if $limit.is_some_and(|limit| limit <= 0) {
            0
        } else {
            let mut cnt: i32 = 0;
            loop {
                if consume_expected_chars!($iter, $($a)|+).is_some() {
                    cnt += 1;

                    if $limit.is_some_and(|l| cnt >= l) {
                        break;
                    }
                } else {
                    break;
                }
            }
            cnt
        }
    }};
}

pub(crate) use consume;
pub(crate) use consume_expected_chars;
pub(crate) use gather;
