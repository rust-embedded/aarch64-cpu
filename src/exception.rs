#[derive(Copy, Clone)]
pub enum ExceptionReason {
    DataAbort = 0x24,
    Unimplemented = 0xff,
}

impl From<u64> for ExceptionReason{
    fn from(value: u64) -> Self {
        match value {
            x if x == Self::DataAbort as u64 => Self::DataAbort,
            _ => Self::Unimplemented
        }
    }
}
