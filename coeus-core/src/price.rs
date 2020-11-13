use super::*;

// TODO We need Transfer for letting people resell children of their owned domains in a future version
// pub(crate) struct Transfer {
//     pub recipient: String,
//     pub amount: u64,
// }

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub fee: u64,
    // pub transfer: Vec<Transfer>,
}

impl Price {
    pub fn zero() -> Self {
        Self::fee(0)
    }
    pub fn fee(fee: u64) -> Self {
        Self { fee }
    }

    pub fn checked_add(mut self, rhs: Self) -> Option<Self> {
        self.fee = self.fee.checked_add(rhs.fee)?;
        Some(self)
    }
}
