#![feature(try_blocks)]
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::tree::{ParseTree, ParseTreeListener};

mod parser;

use parser::smtlibv2lexer;
use parser::smtlibv2listener;
use parser::smtlibv2parser;

use smtlibv2lexer::SMTLIBv2Lexer;
use smtlibv2listener::SMTLIBv2Listener;
use smtlibv2parser::*;

pub fn parse() -> i32 {
    let lexer = SMTLIBv2Lexer::new(InputStream::new("(declare-fun x () Int)"));
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = SMTLIBv2Parser::new(token_source);
    return 12;
}
