use crate::markdown::markdown_iterator_state::{ActionResult, State, StateData, Yield};
use crate::markdown::utils::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub(crate) fn code_block(state_data: &mut StateData, start_idx: usize) -> ActionResult {
    if consume_expected_chars!(state_data.it, '`').is_none() {
        return ActionResult::NextState(State::Text);
    }

    let open_cnt = 1 + gather!(state_data.it, Option::<i32>::None, '`');

    loop {
        // end of file detection
        let ConsumeResult::Some((idx, i)) = consume!(state_data.it) else {
            break;
        };

        if i == '`' {
            let advance = 1 + gather!(state_data.it, Option::<i32>::Some(open_cnt - 1), '`');

            if advance == open_cnt {
                let end_idx = idx + 1 + advance as usize - 1;

                return ActionResult::YieldState(State::Text, Yield::CodeBlock(start_idx, end_idx));
            }
        }
    }
    ActionResult::NextState(State::Text)
}
