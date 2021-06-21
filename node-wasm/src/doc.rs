use super::*;

#[wasm_bindgen(js_name = DidDocument)]
pub struct JsDidDocument {
    inner: DidDocument,
}

#[wasm_bindgen(js_class = DidDocument)]
impl JsDidDocument {
    // readonly height: number;
    // readonly did: Did;

    // hasRightAt(auth: Crypto.Authentication, right: Sdk.Right, height: number): boolean;
    // isTombstonedAt(height: number): boolean;

    // toData(): IDidDocumentData;
    // fromData(data: IDidDocumentData): void;
}

impl Wraps<DidDocument> for JsDidDocument {
    fn inner(&self) -> &DidDocument {
        &self.inner
    }
}

impl From<DidDocument> for JsDidDocument {
    fn from(inner: DidDocument) -> Self {
        Self { inner }
    }
}
