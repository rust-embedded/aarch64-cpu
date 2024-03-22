#[derive(Copy, Clone)]
pub enum ExceptionReason {
    DataAbortLowerEl = 0b100100,
    DataAbortCurrentEl = 0b100101,
    Unimplemented = 0xff,
}

impl From<u64> for ExceptionReason{
    fn from(value: u64) -> Self {
        match value {
            x if x == Self::DataAbortLowerEl as u64 => Self::DataAbortLowerEl,
            x if x == Self::DataAbortCurrentEl as u64 => Self::DataAbortCurrentEl,
            _ => Self::Unimplemented
        }
    }
}
