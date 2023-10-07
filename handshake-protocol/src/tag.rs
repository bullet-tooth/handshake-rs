use std::{fmt, ops};

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Tag([u8; 16]);

impl From<[u8; 16]> for Tag {
    fn from(tag: [u8; 16]) -> Self {
        Tag(tag)
    }
}

impl From<&[u8]> for Tag {
    fn from(slice: &[u8]) -> Self {
        let mut data = [0; 16];
        data.copy_from_slice(slice);

        Tag(data)
    }
}

impl AsRef<[u8]> for Tag {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl ops::Deref for Tag {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tag: {}", hex::encode(self.0))
    }
}
