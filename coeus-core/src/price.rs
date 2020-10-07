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
}

impl ops::AddAssign for Price {
    fn add_assign(&mut self, rhs: Self) {
        self.fee += rhs.fee;
    }
}
