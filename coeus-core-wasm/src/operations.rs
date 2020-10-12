use super::*;

#[wasm_bindgen(js_name = Operation)]
pub struct JsOperation {
    inner: Operation,
}

#[wasm_bindgen(js_class = Operation)]
impl JsOperation {
    pub fn start_block(height: BlockHeight) -> JsOperation {
        Operation::from(SystemOperation::start_block(height)).into()
    }

    pub fn register(
        name: &JsDomainName, owner: &JsPrincipal, subtree_policies: &JsSubtreePolicies,
        data: &JsValue, expires_at_height: BlockHeight,
    ) -> Result<JsOperation, JsValue> {
        let reg_op = UserOperation::register(
            name.inner().to_owned(),
            owner.inner().to_owned(),
            subtree_policies.inner().to_owned(),
            RegistrationPolicy::default(),
            data.into_serde().map_err_to_js()?,
            expires_at_height,
        );
        Ok(Operation::from(reg_op).into())
    }

    pub fn update(name: &JsDomainName, data: &JsValue) -> Result<JsOperation, JsValue> {
        let name = name.inner().to_owned();
        let upd_op = UserOperation::update(name, data.into_serde().map_err_to_js()?);
        Ok(Operation::from(upd_op).into())
    }

    pub fn renew(name: &JsDomainName, expires_at_height: BlockHeight) -> JsOperation {
        let name = name.inner().to_owned();
        let ren_op = UserOperation::renew(name, expires_at_height);
        Operation::from(ren_op).into()
    }

    pub fn transfer(name: &JsDomainName, to_owner: &JsPrincipal) -> JsOperation {
        let name = name.inner().to_owned();
        let tr_op = UserOperation::transfer(name, to_owner.inner().to_owned());
        Operation::from(tr_op).into()
    }

    pub fn delete(name: &JsDomainName) -> JsOperation {
        let del_op = UserOperation::delete(name.inner().to_owned());
        Operation::from(del_op).into()
    }
}

impl From<Operation> for JsOperation {
    fn from(inner: Operation) -> Self {
        Self { inner }
    }
}

impl Wraps<Operation> for JsOperation {
    fn inner(&self) -> &Operation {
        &self.inner
    }
}
