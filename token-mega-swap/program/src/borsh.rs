use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

/// Forces rust auto cast, it does not work with write in default serialize
/// In Rust 1.51 can make it const N generic
pub trait BorshSerializeConst {
    fn serialize_const(&self, writer: &mut [u8]) -> std::io::Result<()>;
}

impl<T> BorshSerializeConst for T
where
    T: BorshSerialize,
{
    fn serialize_const(&self, writer: &mut [u8]) -> std::io::Result<()> {
        let mut writer = &mut writer[..];
        self.serialize(&mut writer)
    }
}

pub trait BorshDeserialiseConst<T: BorshDeserialize> {
    fn deserialize_const(reader: &[u8]) -> std::io::Result<T>;
}

impl<T> BorshDeserialiseConst<T> for T
where
    T: BorshDeserialize,
{
    fn deserialize_const(writer: &[u8]) -> std::io::Result<T> {
        let mut writer = &writer[..];
        Self::deserialize(&mut writer)
    }
}

pub trait AccountWithBorsh {
    fn read_data_with_borsh<T: BorshDeserialize>(&self) -> Result<T, ProgramError>;
}

impl<'a> AccountWithBorsh for AccountInfo<'a> {
    fn read_data_with_borsh<T: BorshDeserialize>(&self) -> Result<T, ProgramError> {
        let data = self.try_borrow_data()?;
        Ok(T::deserialize_const(&data)?)
    }
}
