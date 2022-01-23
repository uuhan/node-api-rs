// common macros
#[macro_export]
macro_rules! napi_module {
    ($init:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn napi_register_module_v1(
            env: $crate::api::napi_env,
            exports: $crate::api::napi_value,
        ) -> $crate::api::napi_value {
            let exports = $crate::value::JsObject::napi_module_exports(env, exports);
            let env = $crate::env::NapiEnv::from_raw(env);

            // TODO: deal with exception
            match std::panic::catch_unwind(|| $init(env, exports)) {
                Ok(r) => {}
                Err(e) => {}
            }

            exports.raw()
        }
    };
}

#[macro_export]
macro_rules! napi_call {
    // [out] result: napi function which has output
    (=$napi:ident, $($args:expr),+ $(,)?) => {
        unsafe {
            let mut result = std::mem::MaybeUninit::uninit();
            let status = $crate::api::$napi($($args),+, result.as_mut_ptr());
            if status.err() {
                return Err(status);
            }
            result.assume_init()
        }
    };

    (?$napi:ident, $($args:expr),+ $(,)?) => {
        unsafe {
            let mut result = std::mem::MaybeUninit::uninit();
            let status = $crate::api::$napi($($args),+, result.as_mut_ptr());
            (status, result.assume_init())
        }
    };

    ($napi:ident, $($args:expr),+ $(,)?) => {
        unsafe {
            let status = $crate::api::$napi($($args),+);
            if status.err() {
                return Err(status);
            } else {
                NapiResult::Ok(())
            }
        }
    }
}

#[macro_export]
macro_rules! napi_guard {
    ($version:expr) => {
        assert!(
            $version >= $crate::napi_version_guard(),
            "Oops, your node(napi {}) is too old to support napi >= {}",
            $version,
            $crate::napi_version_guard(),
        );
    };
}

#[macro_export]
macro_rules! napi_from_raw {
    ($T:ident) => {
        fn from_raw(env: $crate::env::NapiEnv, raw: $crate::api::napi_value) -> $T {
            $T($crate::value::JsValue(env, raw))
        }
    };
}

#[macro_export]
macro_rules! napi_get_value {
    ($T:ident) => {
        fn value(&self) -> $crate::value::JsValue {
            self.0
        }
    };
}

#[macro_export]
macro_rules! napi_value_t {
    ($T:ident) => {
        impl $crate::value::NapiValueT for $T {
            napi_from_raw!($T);
            napi_get_value!($T);
        }
    };
}

#[macro_export]
macro_rules! napi_s {
    ($s:expr) => {
        std::ffi::CString::new($s).map_err(|_| $crate::NapiStatus::StringExpected)
    };
}
