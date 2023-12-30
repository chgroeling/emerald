use super::uid::Uid;
use super::uid_map::UidMap;
use super::VaultResourceId;

pub fn adapter_to_uid<'a>(
    it_src: impl IntoIterator<Item = &'a VaultResourceId> + 'a,
    uid_map: &'a mut UidMap,
) -> impl Iterator<Item = Uid> + 'a {
    it_src.into_iter().map(|rid| uid_map.assign_uid(rid))
}
