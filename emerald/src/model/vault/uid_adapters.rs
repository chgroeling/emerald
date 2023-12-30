use super::uid::Uid;
use super::uid_map::UidMap;
use super::ExResourceId;

pub fn adapter_to_uid<'a>(
    it_src: impl IntoIterator<Item = &'a ExResourceId> + 'a,
    uid_map: &'a mut UidMap,
) -> impl Iterator<Item = Uid> + 'a {
    it_src.into_iter().map(|rid| uid_map.assign_uid(rid))
}
