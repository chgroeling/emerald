use super::ParseResult;
use crate::markdown::states::markdown_iterator_state::StateData;
use crate::markdown::utils::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub(crate) fn inline_code_block(state_data: &mut StateData, start_idx: usize) -> ParseResult {
    let open_cnt = gather!(state_data.it, Option::<i32>::None, ' ');

    if open_cnt < 4 {
        return ParseResult::Failed;
    }

    // Opening Inline Code Block was detected
    let mut act_idx = start_idx + open_cnt as usize - 1; // -1 because the first char doesn't count

    loop {
        // end of file detection
        let ConsumeResult::Some((idx, i)) = consume!(state_data.it) else {
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
