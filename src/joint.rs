use crate::bone::Bone;

pub struct Joint<'a> {
    parent: &'a Bone,
    child: &'a Bone,
}
