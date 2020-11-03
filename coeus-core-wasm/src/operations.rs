use super::*;

#[wasm_bindgen(js_name = UserOperation)]
pub struct JsUserOperation {
    inner: UserOperation,
}

#[wasm_bindgen(js_class = UserOperation)]
impl JsUserOperation {
    pub fn register(
        name: &JsDomainName, owner: &JsPrincipal, subtree_policies: &JsSubtreePolicies,
        data: &JsValue, expires_at_height: BlockHeight,
    ) -> Result<JsUserOperation, JsValue> {
        let reg_op = UserOperation::register(
            name.inner().to_owned(),
            owner.inner().to_owned(),
            subtree_policies.inner().to_owned(),
            RegistrationPolicy::default(),
            data.into_serde().map_err_to_js()?,
            expires_at_height,
        );
        Ok(reg_op.into())
    }

    pub fn update(name: &JsDomainName, data: &JsValue) -> Result<JsUserOperation, JsValue> {
        let name = name.inner().to_owned();
        let upd_op = UserOperation::update(name, data.into_serde().map_err_to_js()?);
        Ok(upd_op.into())
    }

    pub fn renew(name: &JsDomainName, expires_at_height: BlockHeight) -> JsUserOperation {
        let name = name.inner().to_owned();
        let ren_op = UserOperation::renew(name, expires_at_height);
        ren_op.into()
    }

    pub fn transfer(name: &JsDomainName, to_owner: &JsPrincipal) -> JsUserOperation {
        let name = name.inner().to_owned();
        let tr_op = UserOperation::transfer(name, to_owner.inner().to_owned());
        tr_op.into()
    }

    pub fn delete(name: &JsDomainName) -> JsUserOperation {
        let del_op = UserOperation::delete(name.inner().to_owned());
        del_op.into()
    }
}

impl From<UserOperation> for JsUserOperation {
    fn from(inner: UserOperation) -> Self {
        Self { inner }
    }
}

impl Wraps<UserOperation> for JsUserOperation {
    fn inner(&self) -> &UserOperation {
        &self.inner
    }
}
