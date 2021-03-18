use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DoStartBlock {
    pub(super) height: BlockHeight,
}

impl Command for DoStartBlock {
    fn execute(self, state: &mut State) -> Result<UndoOperation> {
        ensure!(
            state.last_seen_height() < self.height,
            "Already seen height {}, cannot set it to {}",
            state.last_seen_height(),
            self.height
        );
        let undo_operation = UndoStartBlock { height: state.last_seen_height() };
        state.set_last_seen_height(self.height);
        Ok(UndoOperation::StartBlock(undo_operation))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UndoStartBlock {
    height: BlockHeight,
}

impl UndoCommand for UndoStartBlock {
    fn execute(self, state: &mut State) -> Result<()> {
        state.set_last_seen_height(self.height);
        Ok(())
    }
}
