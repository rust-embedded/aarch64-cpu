pub mod v3;
pub mod v2;
pub mod registers;

pub enum IRQState {
    Inactive,
    Pending,
    Active,
    PendingActive,
}

impl From<u64> for IRQState {
    fn from(value: u64) -> Self {
        match value {
            0 => IRQState::Inactive,
            1 => IRQState::Pending,
            2 => IRQState::Active,
            3 => IRQState::PendingActive,
            _ => panic!("illegal irq state input")
        }
    }
}
