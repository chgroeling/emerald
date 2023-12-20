use super::ParseResult;
use crate::markdown::utf8_iterator::Utf8Iterator;
use crate::markdown::utils::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub(crate) fn code_block(it: &mut Utf8Iterator, start_idx: usize) -> ParseResult {
    if consume_expected_chars!(it, '`').is_none() {
        return ParseResult::Failed;
    }

    let open_cnt = 1 + gather!(it, Option::<i32>::None, '`');

    loop {
        // end of file detection
        let IterResult::Some((idx, i)) = consume!(it) else {
            break;
        };

        if i == '`' {
            let advance = 1 + gather!(it, Option::<i32>::Some(open_cnt - 1), '`');

            if advance == open_cnt {
                let end_idx = idx + 1 + advance as usize - 1;

                return ParseResult::Yield(start_idx, end_idx);
            }
        }
    }
    ParseResult::Failed
}
