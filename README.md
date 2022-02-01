## Nodex - Nodejs eXtension 🥳

Yet another crate to create native nodejs addons :)

This crate aims to make creating native nodejs addons very easy and comfortable.

[click here: uuhan/nodex@dev](https://github.com/uuhan/nodex) to see the most recent developments.

## Features

- [x] ergonomical api design.
- [ ] export the codebase from crates world, make it easy to call rust function from js world.
    - [ ] sweet syntax, like: #[nodex::function] fn foo()
- [ ] import the huge codebase from npm world, make it easy to call js function from rust side.
    - [ ] sweet syntax, like: let lodash = nodex::import!(lodash);
- [ ] nodejs async runtime to drive rust async code
    - [ ] async runtime for async rust
    - [ ] macros like: #[nodex::rt] async fn main(), so you can use nodejs to run any rust async-code.
        - [ ] node --require=main.node
        - [ ] rust code introspection with nodejs repl
- [ ] cargo-nodex cargo subcommand to make ease of create nodejs addons, e.g. auto generate ts typings.
    - [ ] cargo nodex build
    - [ ] cargo nodex typings
    - [ ] cargo nodex package

## Usage

```toml
[lib]
crate-type = ["cdylib"]

[dependencies.nodex-api]
version = "0.1.4"
features = ["v8"]
```

The default napi version is set to v1, you can use other version with your need.

We have v1,v2,v3,...v8 versions.

**Currently, nodex just reexports nodex-api:**

```toml
[lib]
crate-type = ["cdylib"]

[dependencies.nodex]
version = "0.1.4"
features = ["v8"]
```

If you do not need the default link options, e.g. you are cross-compiling to x86_64-unknown-linux-gnu in macos platform.

```toml
features = ["no-linkage"]
```

This feature will disable the following link options:

```rust
#[cfg(not(feature = "no-linkage"))]
{
    // NB: macos link options
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-cdylib-link-arg=-Wl");
        println!("cargo:rustc-cdylib-link-arg=-undefined");
        println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
    }
}
```

## Napi Level

### v1

* NapiValueT::wrap::\<T, Finalizer>() - Wraps a native instance, call finalizer when value is garbage-collected.
* NapiValueT::remove_wrap::\<T>() - Remove the wrapped native instance. The finalizer will not be called if the wrapped instance is removed.
* NapiValueT::unwrap::\<T>() - Access the wrapped instance.
* NapiValueT::gc::\<Finalizer>() - Hook fired when value is gabage-collected.

### v3

* NapiEnv::add_cleanup_hook() - Do the cleanup when nodejs environment exits.

### v4

* NapiThreadsafeFunction::\<Data, const N: usize> - Thread safe function.

### v5

* NapiValueT::finalizer() - Adds a napi_finalize callback which will be called when the JavaScript object is ready for gc.

### v6

* NapiEnv::set_instance_data::\<Data, Finalizer> - Set data to current agent.
* NapiENv::get_instance_data::\<Data> - Get Option\<&mut Data> from current agent.

### v8

* NapiEnv::add_async_cleanup_hook() - Do the cleanup when nodejs environment exits, asynchronous.

## Examples

### Init Module

simply define your module by:

```rust
use nodex::prelude::*;
nodex::napi_module!(init);
fn init(env: NapiEnv, exports: JsObject) -> NapiResult<()> {
    Ok(())
}
```

### Version Guard

make sure the node api version is large or equal than your compiled addon's.

```rust
nodex::napi_guard!(env.napi_version()?);
```

### Nodejs Version & Napi Version

get the runtime version:

```rust
let node_version = env.node_version()?;
let napi_version = env.napi_version()?;
```

### Define Js Variable

```rust
// String & Symbol
let label: JsSymbol = env.symbol()?;
let name: JsString = env.string("")?;

// Object
let mut obj: JsObject = env.object()?;
obj.set_property(name, env.null()?)?;

// Function
let func: JsFunction = env.func(move |this, [a1, a2, a3]: [JsValue; 3]| {
    let env = this.env();
    a1.as_function()?.call::<JsValue, 0>(this, [])?;
    a1.as_function()?.call(this, [env.string("I am from rust world.")?])
})?;

let func: JsFunction = env.func(move |this, [a1]: [JsFunction; 1]| {
    let env = this.env();
    a1.call(this, [env.string("I am from rust world.")?])
})?;

// Error
let error: JsError = JsError::error("error", None)?;

```

### Napi handle scope

```rust
// napi handle scope
let _scope: NapiHandleScope = env.handle_scope()?;
let _escapable_scope: NapiEscapableHandleScope = env.escapable_handle_scope()?;
```

### Napi cleanup hook

#### sync

```rust
env.add_cleanup_hook(|| {
    println!("clean hook fired");
    Ok(())
})?;

let hook_to_remove = env.add_cleanup_hook(|| {
    println!("clean hook fired");
    Ok(())
})?;

hook_to_remove.remove()?;
```

#### aync

```rust
match env.add_async_cleanup_hook(|hook| {
    // DO SOME CLEANUP
    // NB: should call remove after done
    hook.remove()
})? {
    Some(hook) => {
        // NB: also the hook can be removed before it is fired.
        hook.remove()?;
    }
    None => {}
}
```

### Set Property Descriptor

```rust
let mut obj: JsObject = env.object()?;
obj.define_properties(&[DescriptorValueBuilder::new()
    .with_utf8name("myvalue")
    .with_value(env.string("myvalue")?)
    .build()?])?;

obj.define_properties(&[DescriptorMethodBuilder::new()
    .with_utf8name("mymethod")
    .with_method(move |this, []: [JsValue; 0]| this.env().double(200.))
    .build()?])?;

obj.define_properties(&[DescriptorAccessorBuilder::new()
    .with_utf8name("myaccessor")
    .with_getter(|this| this.env().double(100.))
    .with_setter(|_this: JsObject, [n]: [JsNumber; 1]| {
        println!("setter: {}", n.get_value_int32()?);
        Ok(())
    })
    .build()?])?;
```

### Create An Async Work

```rust
// without shared state
env.async_work(
    "my-test-async-task",
    (),
    move |_| {
        // you can do the hard work in the thread-pool context.
        // NB: js work is not allowed here.
        println!("execute async task");
    },
    move |_, status, _| {
        // you can do some js work in this context
        println!("[{}] complete async task", status);
        Ok(())
    },
)?
.queue()?;
```

### gabage-collected hook

for napi less than 5, implement by napi_wrap, otherwise by napi_add_finalizer.

```rust
let mut obj = env.object()?;
obj.gc(move |_| {
    println!("obj garbage-collected");
    Ok(())
});
```

### Wrap native instance

```rust
let mut obj = env.object()?;
obj.wrap([1usize; 2], move |_, wrapped| {
    Ok(())
})?;
obj.unwrap::<[usize; 2]>()?; // access the wrapped instance
obj.remove_wrap::<[usize; 2]>()?; // the finalizer will not be called
```

### Thread safe function

require: napi >= 4

```rust
let tsfn = NapiThreadsafeFunction::<_, 0>::new(
    env,
    "tsfn-task",
    env.func(|this, [a1]: [JsString; 1]| {
        println!("callback result: {}", a1.get()?);
        this.env().undefined()
    })?,
    // finalizer
    move |_| Ok(()),
    // js-callback
    move |f, data: String| {
        f.call::<JsString, 1>(env.object()?, [env.string(&data)?])?;
        Ok(())
    },
)?;

std::thread::spawn(move || {
    tsfn.non_blocking("hello, world - 1".into()).unwrap();
    tsfn.non_blocking("hello, world - 2".into()).unwrap();
    tsfn.release().unwrap();
});
```

### Promise for some heavy work

```rust
let promise: JsPromise<JsString, JsError> = env.promise(
    move |result| {
        for i in 1..=3 {
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("[{}] Doing...", i);
        }

        *result = resolve;
    },
    move |promise, _, result| {
        let env = promise.env();
        if result {
            promise.resolve(env.string("the promise is resolved.")?)?;
        } else {
            promise.reject(env.error("the promise is rejected.")?)?;
        }
        Ok(())
    },
)?;
// the `promise.value()` can return to js world as a Promise
```

### Run script

```rust
let func: Function<JsUndefined> = env.run_script(
    r#"
        function hello() {
            console.log(this);
        }

        hello
    "#,
)?;

func.call::<JsValue, 0>(env.global()?.cast(), [])?;
```

### More

[examples/demo](./examples/demo)

Run:

```bash
bash demo.sh

# output
# [1] called
# { func: [Function (anonymous)], [Symbol()]: 100 }
# [2] called
# { func: [Function (anonymous)], [Symbol()]: 100 }
# 100
```

## How to participate in

## Code of conduct

```bash
cat >> .git/hooks/pre-push << EOF
#!/bin/sh

cargo fmt || exit
cargo clippy -- -D warnings || exit
EOF
```

## License

Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
