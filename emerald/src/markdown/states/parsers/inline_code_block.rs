use super::ParseResult;
use crate::markdown::utf8_iterator::Utf8Iterator;
use crate::markdown::utils::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub(crate) fn inline_code_block(it: &mut Utf8Iterator, start_idx: usize) -> ParseResult {
    let open_cnt = gather!(it, Option::<i32>::None, ' ');

    if open_cnt < 4 {
        return ParseResult::Failed;
    }

    // Opening Inline Code Block was detected
    let mut act_idx = start_idx + open_cnt as usize - 1; // -1 because the first char doesn't count

    loop {
        // end of file detection
        let IterResult::Some((idx, i)) = consume!(it) else {
            break;
        };

        if i == '\n' {
            return ParseResult::Yield(start_idx, idx);
        }

        act_idx = idx;
    }

    // end of file handling
    ParseResult::Yield(start_idx, act_idx + 1)
}
