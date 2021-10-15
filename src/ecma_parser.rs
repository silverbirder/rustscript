#[macro_use]
use std::path::Path;

use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

pub fn ecma_parser() {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm
        .load_file(Path::new("./src/test.js"))
        .expect("failed to load test.js");
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        // JscTarget defaults to es5
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
        .map_err(|mut e| e.into_diagnostic(&handler).emit())
        .expect("failed to parser module");

    println!("{:?}", _module);
}
