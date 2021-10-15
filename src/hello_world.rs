use inkwell::context::Context;
use inkwell::OptimizationLevel;

pub fn hello_world() {
    let context = Context::create();
    let i32_type = context.i32_type();
    let i8_type = context.i8_type();
    let i8_ptr_type = i8_type.ptr_type(inkwell::AddressSpace::Generic);

    // Module
    let module = context.create_module("main");

    // Function
    let printf_fn_type = i32_type.fn_type(&[i8_ptr_type.into()], true);
    let printf_function = module.add_function("printf", printf_fn_type, None);
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function("main", main_fn_type, None);

    // Block
    let entry_basic_block = context.append_basic_block(main_function, "entry");

    // Instruction(Builder)
    let builder = context.create_builder();
    builder.position_at_end(entry_basic_block);
    let hw_string_ptr = builder.build_global_string_ptr("Hello, world!\n", "hw");
    builder.build_call(printf_function, &[hw_string_ptr.as_pointer_value().into()], "call");
    builder.build_return(Some(&i32_type.const_int(0, false)));

    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::Aggressive).unwrap();
    unsafe {
        execution_engine.get_function::<unsafe extern "C" fn()>("main").unwrap().call();
    }
}