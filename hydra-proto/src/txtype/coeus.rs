use super::*;

#[derive(Clone, Debug)]
pub struct Transaction<'a> {
    common_fields: CommonTransactionFields<'a>,
    asset: CoeusAsset,
}

impl<'a> Transaction<'a> {
    pub fn new(
        common_fields: CommonTransactionFields<'a>, signed_operations: Vec<SignedBundle>,
    ) -> Self {
        Self { common_fields, asset: CoeusAsset { bundles: signed_operations } }
    }

    pub fn fee(&self) -> u64 {
        self.asset.fee()
    }
}

impl<'a> Aip29Transaction for Transaction<'a> {
    fn fee(&self) -> u64 {
        self.asset.fee()
    }

    fn to_data(&self) -> TransactionData {
        let mut tx_data: TransactionData = self.common_fields.to_data();
        tx_data.typed_asset = self.asset.to_owned().into();
        tx_data.fee = self.common_fields.calculate_fee(self).to_string();
        tx_data
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use iop_keyvault::multicipher::MPrivateKey;
    use iop_keyvault::PrivateKey;

    #[test]
    fn binary_roundtrip() {
        let domain: DomainName = ".schema.test".parse().unwrap();

        let ark_passphrase =
            "scout try doll stuff cake welcome random taste load town clerk ostrich";
        let secp_privkey = SecpPrivateKey::from_ark_passphrase(ark_passphrase).unwrap();
        let privkey = MPrivateKey::from(secp_privkey);

        let op_renew = UserOperation::renew(domain.clone(), 12345);
        let op_transfer =
            UserOperation::transfer(domain, Principal::PublicKey(privkey.public_key()));
        let nonced_ops = NoncedBundle::new(vec![op_renew, op_transfer], 42);
        let signed_ops = nonced_ops.sign(&privkey).unwrap();

        let original_asset = CoeusAsset { bundles: vec![signed_ops] };
        let bytes = original_asset.to_bytes().unwrap();
        let loaded_asset = CoeusAsset::from_bytes(&bytes).unwrap();
        assert_eq!(original_asset, loaded_asset);
    }
}
