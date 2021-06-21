use super::*;

#[derive(Debug, Clone)]
struct KeyEntry {
    auth: Authentication,
    added_at_height: Option<BlockHeight>,
    expires_at_height: Option<BlockHeight>,
    revoked_at: Option<BlockHeight>,
    rights: HashMap<Right, TimeSeries<bool>>,
}

fn min_of_somes<'a, T: Ord>(i: impl Iterator<Item = &'a Option<T>>) -> Option<&'a T> {
    i.filter(|n| n.is_some()).min().and_then(|a| a.as_ref())
}

impl KeyEntry {
    pub fn valid_until(&self, tombstoned_at_height: Option<BlockHeight>) -> Option<BlockHeight> {
        min_of_somes([self.expires_at_height, self.revoked_at, tombstoned_at_height].iter())
            .cloned()
    }

    pub fn is_valid_at(
        &self, tombstoned_at_height: Option<BlockHeight>, height: BlockHeight,
    ) -> bool {
        let valid_until = self.valid_until(tombstoned_at_height);
        is_height_in_range_exc_until(height, self.added_at_height, valid_until)
    }
}

fn system_rights(valid: bool) -> HashMap<Right, TimeSeries<bool>> {
    let state = TimeSeries::new(valid);
    vec![(Right::Update, state.clone()), (Right::Impersonation, state)].drain(..).collect()
}

#[derive(Debug, Clone)]
pub(super) struct DidDocumentState {
    // last entry inserted at the end, because reading performance is unaffected, but writing gets faster
    key_entries: Vec<KeyEntry>,
    tombstoned_at_height: Option<BlockHeight>,
}

impl DidDocumentState {
    pub fn new(did: &Did) -> Self {
        Self {
            key_entries: vec![KeyEntry {
                auth: Authentication::KeyId(did.default_key_id()),
                rights: system_rights(true),
                added_at_height: None,
                expires_at_height: None,
                revoked_at: None,
            }],
            tombstoned_at_height: None,
        }
    }

    pub fn at_height(&self, did: &Did, height: BlockHeight) -> Result<DidDocument> {
        let keys_at_height: Vec<&KeyEntry> = self
            .key_entries
            .iter()
            .filter(|k| k.added_at_height.map(|h| h <= height).unwrap_or(true))
            .collect();

        let keys: Vec<KeyData> =
            keys_at_height.iter().map(|k| self.key_entry_to_data(*k, height)).collect();

        let rights: HashMap<Right, Vec<KeyRightHistory>> = Right::map_all(|r| {
            keys_at_height
                .iter()
                .enumerate()
                .map(|(i, k)| self.key_entry_to_right_history(*k, i, height, r))
                .collect()
        });

        let doc = DidDocument {
            did: did.to_owned(),
            keys,
            rights,
            services: Default::default(),
            queried_at_height: height,
            tombstoned: self.tombstoned_at_height.is_some(),
            tombstoned_at_height: self.tombstoned_at_height,
        };

        Ok(doc)
    }

    fn key_entry_to_right_history(
        &self, key_entry: &KeyEntry, idx: usize, height: BlockHeight, right: &Right,
    ) -> KeyRightHistory {
        let (items, valid) = if let Some(history) = key_entry.rights.get(right) {
            (
                history
                    .iter()
                    .map(|(height, valid)| KeyRightHistoryItem { height, valid: *valid })
                    .collect(),
                *history.get(height),
            )
        } else {
            (vec![], false)
        };

        let state = KeyRightState { history: items };
        let key_link = format!("#{}", idx);
        let derived = KeyRightDerived { key_link, valid };
        KeyRightHistory { state, derived }
    }

    fn key_entry_to_data(&self, key_entry: &KeyEntry, height: BlockHeight) -> KeyData {
        let state = KeyState {
            authentication: key_entry.auth.to_owned(),
            valid_from_block: key_entry.added_at_height,
            valid_until_block: key_entry.valid_until(self.tombstoned_at_height),
        };
        let derived =
            KeyDataDerived { valid: key_entry.is_valid_at(self.tombstoned_at_height, height) };
        KeyData { state, derived }
    }

    fn last_by_auth(&mut self, auth: &Authentication) -> Option<&mut KeyEntry> {
        self.key_entries.iter_mut().rev().find(|i| &i.auth == auth)
    }

    fn right_history(
        &mut self, did: &Did, height: BlockHeight, auth: &Authentication, right: &str,
    ) -> Result<&mut TimeSeries<bool>> {
        let right: Right = right.parse()?;
        let tombstoned_at_height = self.tombstoned_at_height;
        if let Some(entry) = self.last_by_auth(auth) {
            ensure!(
                entry.is_valid_at(tombstoned_at_height, height),
                "Key matching {} of DID {} is invalid at height {}",
                auth,
                did,
                height
            );

            if let Some(history) = entry.rights.get_mut(&right) {
                Ok(history)
            } else {
                bail!(
                    "Key matching {} of DID {} has no right history of right {}",
                    auth,
                    did,
                    right
                );
            }
        } else {
            bail!("DID {} has no key matching {}", did, auth)
        }
    }

    fn ensure_min_height(&self, height: BlockHeight) -> Result<()> {
        ensure!(height > 1, "Keys cannot be added before height 2");
        Ok(())
    }

    fn ensure_not_tombstoned(&self) -> Result<()> {
        ensure!(
            self.tombstoned_at_height.is_none(),
            "did is tombstoned at height {}, cannot be updated anymore"
        );
        Ok(())
    }

    fn ensure_different_auth(&self, signer: &Authentication, auth: &Authentication) -> Result<()> {
        ensure!(signer != auth, "{} cannot modify its own authorization (as {})", signer, auth);
        Ok(())
    }

    pub fn apply(
        &mut self, did: &Did, height: BlockHeight, signer: &Authentication,
        op: &SignableOperationDetails,
    ) -> Result<()> {
        use SignableOperationDetails::*;
        match op {
            AddKey { auth, expires_at_height } => {
                self.ensure_min_height(height)?;
                self.ensure_not_tombstoned()?;
                if let Some(existing_entry) = self.last_by_auth(auth) {
                    ensure!(
                        !existing_entry.is_valid_at(None, height),
                        "DID {} already has a still valid key matching {}",
                        did,
                        auth
                    )
                }
                self.key_entries.push(KeyEntry {
                    auth: auth.clone(),
                    rights: system_rights(false),
                    added_at_height: Some(height),
                    expires_at_height: expires_at_height.clone(),
                    revoked_at: None,
                })
            }
            RevokeKey { auth } => {
                self.ensure_different_auth(signer, auth)?;
                self.ensure_min_height(height)?;
                self.ensure_not_tombstoned()?;
                if let Some(existing_entry) = self.last_by_auth(auth) {
                    ensure!(
                        existing_entry.is_valid_at(None, height),
                        "DID {} has a key matching {}, but it's already invalidated",
                        did,
                        auth
                    );
                    ensure!(
                        existing_entry.revoked_at.is_none(),
                        "key matching {} in DID {} was already revoked",
                        auth,
                        did
                    );
                    existing_entry.revoked_at = Some(height);
                } else {
                    bail!("DID {} does not have a key matching {}", did, auth)
                }
            }
            AddRight { auth, right } => {
                self.ensure_different_auth(signer, auth)?;
                self.ensure_not_tombstoned()?;
                let history = self.right_history(did, height, auth, right)?;
                history
                    .apply(height, true, || format!("Validity of key {} in DID {}", auth, did))?;
            }
            RevokeRight { auth, right } => {
                self.ensure_different_auth(signer, auth)?;
                self.ensure_not_tombstoned()?;
                let history = self.right_history(did, height, auth, right)?;
                history
                    .apply(height, false, || format!("Validity of key {} in DID {}", auth, did))?;
            }
            TombstoneDid {} => {
                self.ensure_not_tombstoned()?;
                self.tombstoned_at_height = Some(height);
            }
        }
        Ok(())
    }

    pub fn revert(
        &mut self, did: &Did, height: BlockHeight, signer: &Authentication,
        op: &SignableOperationDetails,
    ) -> Result<()> {
        use SignableOperationDetails::*;
        match op {
            AddKey { auth, expires_at_height } => {
                self.ensure_min_height(height)?;
                self.ensure_not_tombstoned()?;
                if let Some(last_entry) = self.key_entries.pop() {
                    ensure!(
                        &last_entry.auth == auth,
                        "Cannot revert addKey in DID {}, because the key does not match the last added one.",
                        did
                    );
                    ensure!(
                        &last_entry.added_at_height == &Some(height),
                        "Cannot revert addKey in DID {}, because it was not added at the specified height.",
                        did
                    );
                    ensure!(
                        &last_entry.expires_at_height == expires_at_height,
                        "Cannot revert addKey in DID {}, because it was not added with the same expiration.",
                        did
                    )
                } else {
                    bail!("Cannot revert addKey in DID {}, because there are no keys", did);
                }
            }
            RevokeKey { auth } => {
                self.ensure_different_auth(signer, auth)?;
                self.ensure_min_height(height)?;
                self.ensure_not_tombstoned()?;
                if let Some(existing_entry) = self.last_by_auth(auth) {
                    ensure!(
                        existing_entry.revoked_at.is_some(),
                        "Cannot revert revokeKey in DID {} because key matching {} was not revoked",
                        did,
                        auth
                    );
                    existing_entry.revoked_at = None;
                    ensure!(
                        existing_entry.is_valid_at(None, height),
                        "Failed to revert revokeKey in DID {} for key matching {}. It's still invalid after reverted revoking.",
                        did,
                        auth
                    );
                } else {
                    bail!("Cannot revert revokeKey in DID {} because it does not have a key matching {}", did, auth)
                }
            }
            AddRight { auth, right } => {
                self.ensure_different_auth(signer, auth)?;
                self.ensure_not_tombstoned()?;
                let history = self.right_history(did, height, auth, right)?;
                history
                    .revert(height, true, || format!("Validity of key {} in DID {}", auth, did))?;
            }
            RevokeRight { auth, right } => {
                self.ensure_different_auth(signer, auth)?;
                self.ensure_not_tombstoned()?;
                let history = self.right_history(did, height, auth, right)?;
                history
                    .revert(height, false, || format!("Validity of key {} in DID {}", auth, did))?;
            }
            TombstoneDid {} => {
                ensure!(
                    self.tombstoned_at_height.is_some(),
                    "Failed to revert tombstoning DID {}. It was not tombstoned yet.",
                    did
                );
                self.tombstoned_at_height = None;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    #[test]
    fn std_option_ordering() {
        assert_eq!(None.cmp(&Some(0u32)), Ordering::Less);
        assert_eq!(None.partial_cmp(&Some(0u32)), Some(Ordering::Less));
        assert_eq!(Some(0u32).cmp(&None), Ordering::Greater);
        assert_eq!(Some(0u32).partial_cmp(&None), Some(Ordering::Greater));
        assert_eq!(Option::<u32>::None.cmp(&None), Ordering::Equal);
        assert_eq!(Option::<u32>::None.partial_cmp(&None), Some(Ordering::Equal));
    }
}
