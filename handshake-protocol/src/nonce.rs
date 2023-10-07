use std::ops;

pub(crate) struct Nonce([u8; 12]);

impl From<u32> for Nonce {
    fn from(value: u32) -> Self {
        let mut nonce = [0_u8; 12];
        nonce[4..8].copy_from_slice(&value.to_le_bytes());

        Nonce(nonce)
    }
}

impl ops::Deref for Nonce {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
