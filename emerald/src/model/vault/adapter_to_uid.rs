use super::uid_map::UidMap;
use super::ResourceId;

pub fn adapter_to_uid<'a>(
    it_src: impl IntoIterator<Item = &'a ResourceId> + 'a,
    uid_map: &'a mut UidMap,
) -> impl Iterator<Item = &'a ResourceId> + 'a {
    it_src.into_iter().map(move |rid| {
        uid_map.get_or_assign_uid(rid);
        rid
    })
}
