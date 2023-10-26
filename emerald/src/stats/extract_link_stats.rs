use super::vault_link_stats::VaultLinkStats;
use crate::types;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn extract_link_stats<'a>(
    it_src: impl IntoIterator<Item = &'a types::LinkSrc2Tgt>,
) -> VaultLinkStats {
    let mut valid_backlink_cnt: usize = 0;
    let mut invalid_backlink_cnt: usize = 0;
    let mut note_valid_backlink_cnt: usize = 0;
    let mut note_invalid_backlink_cnt: usize = 0;

    let mut iter_mut = it_src.into_iter();
    let mut opt_last_src: Option<&types::ResourceId> = None;
    loop {
        let Some(s2t) = iter_mut.next() else {
            if let Some(last_src) = opt_last_src {
                if note_valid_backlink_cnt == 0 {
                    trace!("No valid links found in {:?}", &last_src);
                }
            }
            valid_backlink_cnt += note_valid_backlink_cnt;
            invalid_backlink_cnt += note_invalid_backlink_cnt;

            break;
        };

        // Check if this element has a different source than the one before
        if let Some(last_src) = opt_last_src {
            if last_src != &s2t.src {
                if note_valid_backlink_cnt == 0 {
                    trace!("No valid links found in {:?}", &last_src);
                }
                valid_backlink_cnt += note_valid_backlink_cnt;
                invalid_backlink_cnt += note_invalid_backlink_cnt;
                note_valid_backlink_cnt = 0;
                note_invalid_backlink_cnt = 0;
            }
        }

        match &s2t {
            types::LinkSrc2Tgt {
                src,
                link,
                tgt: None,
            } => {
                note_invalid_backlink_cnt += 1;
                warn!("Invalid link '{:?}' found in '{:?}'", &link, &src);
            }
            _ => note_valid_backlink_cnt += 1,
        }

        opt_last_src = Some(&s2t.src);
    }

    VaultLinkStats {
        valid_backlink_cnt,
        invalid_backlink_cnt,
    }
}
/*
#[cfg(test)]
mod link_mapper_tests {
    use super::Src2TargetIndex;
    use crate::types;
    use types::LinkSrc2Tgt;
    use types::ResourceId;

    fn create_test_data() -> Vec<LinkSrc2Tgt> {
        vec![LinkSrc2Tgt::new(
            "resource_id_0".into(),
            "link_0".into(),
            None,
        )]
    }

    #[test]
    fn test_link() {
        // arrange
        let test_data = create_test_data();
        let dut = Src2TargetIndex::new(test_data);

        // act
    }
}
*/
