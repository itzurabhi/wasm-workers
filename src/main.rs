
fn main() {
    // Initialize V8.
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    {
        // Create a new Isolate and make it the current one.
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

        // Create a stack-allocated handle scope.
        let handle_scope = &mut v8::HandleScope::new(isolate);

        // Create a new context.
        let context = v8::Context::new(handle_scope);

        // Enter the context for compiling and running the hello world script.
        let scope = &mut v8::ContextScope::new(handle_scope, context);


        // WASM Binary
        let wasm_bytes = [
            0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01, 0x60, 0x02, 0x7f,
            0x7f, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x07, 0x01, 0x03, 0x61, 0x64, 0x64,
            0x00, 0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b,
        ];

        let compiled_wasm =
            v8::WasmModuleObject::compile(scope, &wasm_bytes).expect("could not compiler the WASM");
        let wasm_global = v8::String::new(scope, "WebAssembly").unwrap();
        let instance_global = v8::String::new(scope, "Instance").unwrap();
        let exports_global = v8::String::new(scope, "exports").unwrap();
        let global_webassembly = context
            .global(scope)
            .get(scope, wasm_global.into())
            .expect("could not find global WebAssembly")
            .to_object(scope)
            .expect("could not convert to WebAssembly Object");
        let global_instance = global_webassembly
            .get(scope, instance_global.into())
            .expect("could not find global Instance")
            .to_object(scope)
            .unwrap();

        let instance_cons = v8::Local::<v8::Function>::try_from(global_instance)
            .expect("Instance to constructor function expected");
        let try_catch = &mut v8::TryCatch::new(scope);

        let wasm_instance = match instance_cons.new_instance(try_catch, &[compiled_wasm.into()]) {
            Some(ab) => ab,
            None => {
                let exception_string = try_catch
                    .stack_trace()
                    .or_else(|| try_catch.exception())
                    .map(|value| value.to_rust_string_lossy(try_catch))
                    .unwrap_or_else(|| "no stack trace".into());

                panic!("{}", exception_string);
            }
        };

        let wasm_exports_ob = wasm_instance.get(try_catch,exports_global.into()).expect("could not find global exports").to_object(try_catch).expect("could not convert to Object");
        let add_method_name = v8::String::new(try_catch, "add").unwrap();
        let add_method_obj = wasm_exports_ob.get(try_catch,add_method_name.into()).expect("could not find add method");
        let add_method_ref = v8::Local::<v8::Function>::try_from(add_method_obj)
            .expect("could not get function from method");
        let arg1 = v8::Integer::new(try_catch, 1 as i32);
        let arg2 = v8::Integer::new(try_catch, 2 as i32);

        let call_res = match add_method_ref.call(try_catch,wasm_instance.into(),&[arg1.into(),arg2.into()]){
            Some(ab) => ab,
            None => {
                let exception_string = try_catch
                    .stack_trace()
                    .or_else(|| try_catch.exception())
                    .map(|value| value.to_rust_string_lossy(try_catch))
                    .unwrap_or_else(|| "no stack trace".into());

                panic!("{}", exception_string);
            }
        };

        println!("{}", call_res.to_rust_string_lossy(try_catch));

 
    }

    unsafe {
        v8::V8::dispose();
    }
    v8::V8::dispose_platform();
}