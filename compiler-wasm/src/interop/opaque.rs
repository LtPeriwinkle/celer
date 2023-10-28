//! Workaround for wasm-bindgen-futures currently not allowing lifetime in function signatures

use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::prelude::*;

use celerc::types::ExecDoc;

pub struct OpaqueExecDoc(JsValue);
impl OpaqueExecDoc {
    pub fn wrap(exec_doc: Option<ExecDoc<'_>>) -> Result<Self, JsValue> {
        match exec_doc {
            Some(exec_doc) => Ok(Self(exec_doc.try_to_js_value()?)),
            None => Ok(Self(JsValue::undefined())),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ExecDoc | undefined")]
    type JsType;
}

impl WasmDescribe for OpaqueExecDoc {
    fn describe() {
        JsType::describe();
    }
}

impl From<OpaqueExecDoc> for JsValue {
    fn from(x: OpaqueExecDoc) -> Self {
        x.0
    }
}