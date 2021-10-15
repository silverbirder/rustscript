extern crate inkwell;

use inkwell::context::Context;
use inkwell::IntPredicate::EQ;
use inkwell::OptimizationLevel;
use std::error::Error;

pub fn fizz_buzz() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let i64_type = context.i64_type();
    let void_type = context.void_type();
    let i8_type = context.i8_type();

    let i8_ptr_type = i8_type.ptr_type(inkwell::AddressSpace::Generic);
    let fn_type = i64_type.fn_type(&[i64_type.into()], false);
    let null = i8_ptr_type.const_null();

    // Module
    let module = context.create_module("fizz_buzz");

    // Function
    let printf_fn_type = void_type.fn_type(&[i8_ptr_type.into()], true);
    let printf_function = module.add_function("printf", printf_fn_type, None);
    let fizz_buzz_function = module.add_function("fizz_buzz", fn_type, None);

    // Block
    let block = context.append_basic_block(fizz_buzz_function, "entry");

    // Instruction
    let builder = context.create_builder();
    builder.position_at_end(block);

    let fizz_buzz_string_ptr = builder.build_global_string_ptr("FizzBuzz\n", "fizz_buzz");
    let fizz_string_ptr = builder.build_global_string_ptr("Fizz\n", "fizz");
    let buzz_string_ptr = builder.build_global_string_ptr("Buzz\n", "buzz");

    let param_0 = fizz_buzz_function
        .get_nth_param(0)
        .unwrap()
        .into_int_value();

    let rem_divied_by_3 =
        builder.build_int_signed_rem(param_0, i64_type.const_int(3, false), "rem_3");
    let rem_divied_by5 =
        builder.build_int_signed_rem(param_0, i64_type.const_int(5, false), "rem_5");
    let rem_divied_by15 =
        builder.build_int_signed_rem(param_0, i64_type.const_int(15, false), "rem_15");

    let comp_that_is_divisible_by_3 = builder.build_int_compare(
        EQ,
        rem_divied_by_3,
        i64_type.const_int(0, false),
        "if_can_divide_by_3",
    );
    let comp_that_is_divisible_by_5 = builder.build_int_compare(
        EQ,
        rem_divied_by5,
        i64_type.const_int(0, false),
        "if_can_divide_by_5",
    );
    let comp_that_is_divisible_by_15 = builder.build_int_compare(
        EQ,
        rem_divied_by15,
        i64_type.const_int(0, false),
        "if_can_divide_by_15",
    );

    // Prepare Block
    let fizz_buzz_block = context.append_basic_block(fizz_buzz_function, "fizz_buzz");
    let fizz_block = context.append_basic_block(fizz_buzz_function, "fizz");
    let buzz_block = context.append_basic_block(fizz_buzz_function, "buzz");
    let num_block = context.append_basic_block(fizz_buzz_function, "num");
    let else_1_block = context.append_basic_block(fizz_buzz_function, "else_1");
    let else_2_block = context.append_basic_block(fizz_buzz_function, "else_2");
    let end_block = context.append_basic_block(fizz_buzz_function, "end_block");

    // Instruction
    builder.build_conditional_branch(comp_that_is_divisible_by_15, fizz_buzz_block, else_1_block);
    builder.position_at_end(fizz_buzz_block);
    builder.build_call(
        printf_function,
        &[fizz_buzz_string_ptr.as_pointer_value().into()],
        "print_fizz_buzz",
    );
    builder.build_unconditional_branch(end_block);

    // Instruction
    builder.position_at_end(else_1_block);
    builder.build_conditional_branch(comp_that_is_divisible_by_3, fizz_block, else_2_block);
    builder.position_at_end(fizz_block);
    builder.build_call(
        printf_function,
        &[fizz_string_ptr.as_pointer_value().into()],
        "print_fizz",
    );
    builder.build_unconditional_branch(end_block);

    // Instruction
    builder.position_at_end(else_2_block);
    builder.build_conditional_branch(comp_that_is_divisible_by_5, buzz_block, num_block);
    builder.position_at_end(buzz_block);
    builder.build_call(
        printf_function,
        &[buzz_string_ptr.as_pointer_value().into()],
        "print_buzz",
    );
    builder.build_unconditional_branch(end_block);

    // Instruction
    builder.position_at_end(num_block);
    builder.build_call(
        printf_function,
        &[buzz_string_ptr.as_pointer_value().into()], // TODO: Print input num.
        "print_num",
    );
    builder.build_unconditional_branch(end_block);

    // Instruction
    builder.position_at_end(end_block);
    builder.build_return(Some(&null));

    let e = module.create_jit_execution_engine(OptimizationLevel::None)?;
    unsafe {
        let x = 15u64;
        e.get_function::<unsafe extern "C" fn(u64) -> ()>("fizz_buzz")?
            .call(x);
    }
    Ok(())
}
