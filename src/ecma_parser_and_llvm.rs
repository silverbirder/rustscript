extern crate inkwell;
extern crate swc_common;
extern crate swc_ecma_ast;
extern crate swc_ecma_parser;

use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::error::Error;
use std::path::Path;
use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_ast::Lit::Num;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

pub fn ecma_parser_and_llvm() -> Result<(), Box<dyn Error>> {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm
        .load_file(Path::new("./src/test.js"))
        .expect("failed to load test.js");
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }
    let _module = parser
        .parse_module()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("failed to parser module");

    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    for b in _module.body {
        if b.is_stmt() {
            let stmt = b.stmt().unwrap();
            if stmt.is_expr() {
                let expr_stmt = stmt.expr().unwrap();
                let expr = expr_stmt.expr;
                if expr.is_bin() {
                    let bin_expr = expr.bin().unwrap();
                    let left_expr = bin_expr.left;
                    let right_expr = bin_expr.right;
                    let binary_op = bin_expr.op;
                    if left_expr.is_lit() && right_expr.is_lit() {
                        let left_lit = left_expr.lit().unwrap();
                        let right_lit = right_expr.lit().unwrap();
                        let left_value = match left_lit {
                            Num(n) => n.value,
                            _ => 0f64,
                        };
                        let right_value = match right_lit {
                            Num(n) => n.value,
                            _ => 0f64,
                        };
                        let i64_type = context.i64_type();
                        let void_type = context.void_type();
                        let fn_type = void_type.fn_type(&[], false);
                        let function = module.add_function("main", fn_type, None);
                        let basic_block = context.append_basic_block(function, "entry");
                        builder.position_at_end(basic_block);
                        let x = i64_type.const_int(left_value as u64, true);
                        let y = i64_type.const_int(right_value as u64, true);
                        let result = match binary_op {
                            swc_ecma_ast::BinaryOp::Add => builder.build_int_add(x, y, "main"),
                            swc_ecma_ast::BinaryOp::Sub => builder.build_int_sub(x, y, "main"),
                            swc_ecma_ast::BinaryOp::Div => {
                                builder.build_int_signed_div(x, y, "main")
                            }
                            swc_ecma_ast::BinaryOp::Mul => builder.build_int_mul(x, y, "main"),
                            _ => i64_type.const_int(0u64, true),
                        };
                        builder.build_return(Some(&result));
                        let e = module.create_jit_execution_engine(OptimizationLevel::None)?;
                        unsafe {
                            let r = e
                                .get_function::<unsafe extern "C" fn() -> u64>("main")?
                                .call();
                            println!("{:?}", r);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
