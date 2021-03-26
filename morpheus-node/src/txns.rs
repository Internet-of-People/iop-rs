use super::*;

pub(super) struct DidTransactionItem<'a> {
    pub did: String,
    pub txid: &'a str,
    pub height: BlockHeight,
}

#[derive(Debug, Default, Clone)]
pub(super) struct DidTransactionsState {
    map: HashMap<String, Vec<TransactionIdWithHeight>>,
}

impl DidTransactionsState {
    pub fn get_between(
        &self, did: &str, from_height_inc: BlockHeight, until_height_inc: Option<BlockHeight>,
    ) -> Option<impl Iterator<Item = &'_ TransactionIdWithHeight> + '_> {
        self.map.get(did).map(move |txns| {
            txns.iter().filter(move |item| {
                is_height_in_range_inc_until(item.height, Some(from_height_inc), until_height_inc)
            })
        })
    }

    pub fn apply(&mut self, item: DidTransactionItem) {
        let (did, txid, height) = (item.did, item.txid, item.height);
        let txns = self.map.entry(did).or_default();
        if txns.iter().all(|i| i.transaction_id != txid) {
            txns.insert(0, TransactionIdWithHeight { transaction_id: txid.to_owned(), height });
        }
    }

    pub fn revert(&mut self, item: DidTransactionItem) {
        // NOTE A transaction might include multiple operations related to a single Did.
        //      Reverting the first operation attempt already removed the transactionId
        //      from the array, we are likely processing the next related operation here
        if let Some(txns) = self.map.get_mut(&item.did) {
            txns.retain(|i| i.transaction_id != item.txid);
        }
    }
}
