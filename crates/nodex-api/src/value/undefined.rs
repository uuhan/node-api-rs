use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsUndefined(pub(crate) JsValue);

impl JsUndefined {
    pub(crate) fn from_value(value: JsValue) -> JsUndefined {
        JsUndefined(value)
    }

    /// This API returns the Undefined object.
    pub fn new(env: NapiEnv) -> NapiResult<JsUndefined> {
        let value = napi_call!(=napi_get_undefined, env.raw());
        Ok(JsUndefined(JsValue::from_raw(env, value)))
    }
}

impl NapiValueT for JsUndefined {
    fn value(&self) -> JsValue {
        self.0
    }
}
