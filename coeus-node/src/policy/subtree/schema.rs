use super::*;

impl SubtreePolicy for SchemaPolicy {
    fn validate(
        &self, _state: &State, policy_domain: &Domain, domain_after_op: &Domain,
    ) -> Result<()> {
        let mut scope = json_schema::Scope::new();
        let schema = scope
            .compile_and_return(self.schema.clone(), true)
            .with_context(|| format!("Domain {} has invalid schema", policy_domain.name()))?;
        let validation_state = schema.validate(domain_after_op.data());
        ensure!(
            validation_state.is_strictly_valid(),
            "Domain {} data does not match schema of {}",
            domain_after_op.name(),
            policy_domain.name(),
        );
        Ok(())
    }
}
