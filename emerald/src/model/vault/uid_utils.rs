use super::uid::Uid;
use super::uid_map::UidMap;
use super::ExResourceId;

/// Assigns UIDs to given resource IDs using a UidMap.
///
/// # Arguments
///
/// * `it_src` - Iterator over `ExResourceId`.
/// * `uid_map` - A mutable reference to `UidMap` used for assigning UIDs.
///
/// # Returns
///
/// Iterator over `Uid`.
pub fn assign_uids_from_resource_ids<'a>(
    it_src: impl IntoIterator<Item = &'a ExResourceId> + 'a,
    uid_map: &'a mut UidMap,
) -> impl Iterator<Item = Uid> + 'a {
    it_src.into_iter().map(|rid| uid_map.assign_uid(rid))
}
