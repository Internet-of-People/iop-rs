trait Verifier {
    async fn validate(
        &self, on_behalf_of: &Did, signer_key: Option<KeyId>, signed_msg: &SignedMessage,
    ) -> TodoColorGreenYellowRed;
}
