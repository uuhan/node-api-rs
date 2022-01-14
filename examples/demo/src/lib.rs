use nodex_api::{api, prelude::*};

nodex_api::init!(init);

fn init(env: api::napi_env, exports: api::napi_value) {
    unsafe {
        let env = Env::from_raw(env);
        let name = std::ffi::CString::new("hello").unwrap();

        let mut obj = JsObject::new(env).unwrap();
        let _value = JsString::new(env, "world").unwrap();

        obj.set(
            JsString::new(env, "a").unwrap(),
            JsString::new(env, "b").unwrap(),
        )
        .unwrap();

        let desc = api::napi_property_descriptor {
            utf8name: name.as_ptr(),
            name: std::ptr::null_mut(),
            method: None,
            getter: None,
            setter: None,
            value: obj.raw(),
            attributes: NapiPropertyAttributes::Default.bits(),
            data: std::ptr::null_mut(),
        };

        let status = api::napi_define_properties(env.raw(), exports, 1, &desc);
        assert_eq!(status, NapiStatus::Ok);
    }
}
