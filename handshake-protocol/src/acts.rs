use std::{fmt, ops};

use secp256k1::PublicKey;

use crate::tag::Tag;

pub const ACT_1_SIZE: usize = 50;
pub const ACT_2_SIZE: usize = 50;
pub const ACT_3_SIZE: usize = 66;

pub struct Act1([u8; ACT_1_SIZE]);

pub type Act2 = Act1;

// just reuse
pub struct Act3([u8; ACT_3_SIZE]);

impl Act1 {
    pub(crate) fn new(version: u8, public_key: PublicKey, c: Tag) -> Self {
        let mut data = [0_u8; ACT_1_SIZE];
        data[0] = version;
        data[1..34].copy_from_slice(&public_key.serialize());
        data[34..].copy_from_slice(&c);

        Act1(data)
    }

    pub fn version(&self) -> u8 {
        self.0[0]
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey::from_slice(&self.0[1..34]).unwrap()
    }

    pub fn tag(&self) -> Tag {
        Tag::from(&self.0[34..])
    }
}

impl Default for Act1 {
    fn default() -> Self {
        Act1([0_u8; ACT_1_SIZE])
    }
}

impl ops::Deref for Act1 {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Act1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Debug for Act1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Act1: {}", hex::encode(self.0))
    }
}

impl From<[u8; ACT_2_SIZE]> for Act2 {
    fn from(array: [u8; ACT_2_SIZE]) -> Self {
        Act1(array)
    }
}

impl Act3 {
    pub(crate) fn new(version: u8, key: Vec<u8>, tag1: Tag, tag2: Tag) -> Self {
        let mut data = [0_u8; ACT_3_SIZE];
        data[0] = version;
        data[1..34].copy_from_slice(key.as_slice());
        data[34..50].copy_from_slice(&tag1);
        data[50..].copy_from_slice(&tag2);

        Act3(data)
    }

    pub fn version(&self) -> u8 {
        self.0[0]
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey::from_slice(&self.0[1..34]).unwrap()
    }

    pub fn tag1(&self) -> Tag {
        Tag::from(&self.0[34..50])
    }

    pub fn tag2(&self) -> Tag {
        Tag::from(&self.0[50..])
    }
}

impl ops::Deref for Act3 {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
