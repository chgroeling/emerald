use super::vault_link_stats::VaultLinkStats;
use crate::{model::link, types};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn extract_link_stats(it_src: &impl link::LinksIterSrc) -> VaultLinkStats {
    let mut valid_backlink_cnt: usize = 0;
    let mut invalid_backlink_cnt: usize = 0;
    let mut note_valid_backlink_cnt: usize = 0;
    let mut note_invalid_backlink_cnt: usize = 0;

    let mut iter_mut = it_src.create_iter();
    let mut opt_last_src: Option<types::ResourceId> = None;
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
            if last_src != s2t.src {
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

        opt_last_src = Some(s2t.src);
    }

    VaultLinkStats {
        valid_backlinks: valid_backlink_cnt,
        invalid_backlinks: invalid_backlink_cnt,
    }
}

#[cfg(test)]
mod link_mapper_tests {
    use super::extract_link_stats;
    use crate::model::link;
    use crate::types;
    use types::LinkSrc2Tgt;

    #[rustfmt::skip]
    fn create_test_data() -> link::MockLinksIterSrc {

        let mut ret = link::MockLinksIterSrc::new();
        ret.expect_create_iter().returning(||
            vec![
                LinkSrc2Tgt::new("resource_id_0".into(), "link_0".into(), None),
                LinkSrc2Tgt::new("resource_id_0".into(), "link_1".into(), None),
                LinkSrc2Tgt::new("resource_id_0".into(), "link_2".into(), Some("resource_id_a".into())),
                LinkSrc2Tgt::new("resource_id_1".into(), "link_3".into(), Some("resource_id_a".into())),
                LinkSrc2Tgt::new("resource_id_1".into(), "link_4".into(), Some("resource_id_b".into())),
                LinkSrc2Tgt::new("resource_id_2".into(), "link_5".into(), None),
                LinkSrc2Tgt::new("resource_id_2".into(), "link_6".into(), Some("resource_id_b".into())),
            ].into_iter()
        );
        ret
    }

    #[test]
    fn test_invalid_backlink_count_with_test_data() {
        // arrange
        let test_data = create_test_data();

        // act
        let link_stats = extract_link_stats(&test_data);

        // assert
        assert_eq!(link_stats.invalid_backlinks, 3);
    }

    #[test]
    fn test_valid_backlink_count_with_test_data() {
        // arrange
        let test_data = create_test_data();

        // act
        let link_stats = extract_link_stats(&test_data);

        // assert
        assert_eq!(link_stats.valid_backlinks, 4);
    }
}
