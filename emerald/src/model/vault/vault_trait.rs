use super::{uid_trait::UidTrait, Note};

pub trait Vault<U>
where
    U: UidTrait,
{
    fn get_note(&self, uid: &U) -> Note<U>;
}
