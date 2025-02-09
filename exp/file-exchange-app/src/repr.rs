use std::marker::PhantomData;
use std::ops::Deref;
use std::{fmt, io};

use bs58::decode::DecodeTarget;
use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::serde::{de, ser, Deserialize, Serialize};

#[derive(Eq, Copy, Clone, PartialEq, Debug)]
pub enum Bs58 {}

#[derive(Eq, Copy, Clone, PartialEq, Debug)]
pub enum Raw {}

mod private {
    pub trait Sealed {}
}

pub trait ReprFormat: private::Sealed {}

impl private::Sealed for Bs58 {}
impl ReprFormat for Bs58 {}

impl private::Sealed for Raw {}
impl ReprFormat for Raw {}

pub trait ReprBytes {
    type Bytes: AsRef<[u8]>;
    fn to_bytes(&self) -> Self::Bytes;
    fn from_bytes<F, E>(f: F) -> Option<Result<Self, E>>
    where
        F: FnOnce(&mut Self::Bytes) -> Option<E>,
        Self: Sized;
}

#[derive(Eq, Copy, Clone, PartialEq, Debug)]
pub struct Repr<T, F = Bs58> {
    data: T,
    _phantom: PhantomData<F>,
}

impl<T> Repr<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }
}

impl<T: ReprBytes> BorshSerialize for Repr<T, Raw> {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        let bytes = self.data.to_bytes();
        writer.write_all(bytes.as_ref())?;
        Ok(())
    }
}

impl<T: ReprBytes> BorshDeserialize for Repr<T, Raw> {
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut bytes = vec![0u8; std::mem::size_of::<<T as ReprBytes>::Bytes>()];
        reader.read_exact(&mut bytes)?;
        let data = T::from_bytes(|data| {
            data.copy_from_slice(&bytes);
            None::<()>
        }).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid data"))??;
        Ok(Repr::from(data))
    }
}

impl<T> From<T> for Repr<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

impl<T> From<T> for Repr<T, Raw> {
    fn from(data: T) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }
}

impl<T> Deref for Repr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}