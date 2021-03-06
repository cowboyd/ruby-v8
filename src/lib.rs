#[macro_use]
extern crate rutie;

use rutie::{Module, Object, AnyObject, RString, VM, NilClass, Boolean, Integer, Float};
use rusty_v8 as v8;

module!(V8);

methods!(
    V8,
    _rtself,

    fn pub_eval(input: RString) -> AnyObject {
        let input = input.map_err(|e| VM::raise_ex(e)).unwrap();
        let source = input.to_str();

        let isolate = &mut v8::Isolate::new(Default::default());

        let scope = &mut v8::HandleScope::new(isolate);
        let context = v8::Context::new(scope);
        let scope = &mut v8::ContextScope::new(scope, context);

        let code = v8::String::new(scope, source).unwrap();

        let script = v8::Script::compile(scope, code, None).unwrap();
        let result = script.run(scope).unwrap();

        if result.is_null_or_undefined() {
            NilClass::new().into()
        } else if result.is_boolean() {
            let state = result.to_boolean(scope).boolean_value(scope);
            Boolean::new(state).into()
        } else if result.is_int32() {
            let num = result.to_int32(scope).unwrap().int32_value(scope).unwrap();
            Integer::from(num).into()
        } else if result.is_uint32() {
            let uint = result.to_uint32(scope).unwrap().uint32_value(scope).unwrap();
            Integer::from(uint).into()
        } else if result.is_number() {
            let num = result.to_number(scope).unwrap().number_value(scope).unwrap();
            Float::new(num).into()
        } else {
            let result = result.to_string(scope).unwrap();
            let result_string = result.to_rust_string_lossy(scope);

            RString::new_utf8(&result_string).into()
        }
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_rubyracer() {
    Module::new("V8").define(|klass| {
        klass.def_self("eval", pub_eval);
    });

    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}
