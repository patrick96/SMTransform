#![feature(try_blocks)]

use std::ops::Deref;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::errors::ANTLRError;
use antlr_rust::input_stream::InputStream;
use antlr_rust::interval_set::Interval;
use antlr_rust::recognizer::Recognizer;
use antlr_rust::token::Token;
use antlr_rust::token_factory::TokenFactory;
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::{ParseTree, ParseTreeListener, Tree};
use antlr_rust::Parser;

mod parser;

use parser::smtlibv2lexer;
use parser::smtlibv2listener;
use parser::smtlibv2parser;

use smtlibv2lexer::SMTLIBv2Lexer;
use smtlibv2listener::SMTLIBv2Listener;
use smtlibv2parser::*;

pub type Symbol = String;
pub type Numeral = String;
pub type HexDecimal = String;
pub type Binary = String;
pub type Keyword = String;

pub enum SpecConstant {
    Numeral(Numeral),
    HexDecimal(HexDecimal),
    Binary(Binary),
    String(String),
}

pub enum Term {
    SpecConstant(SpecConstant),
    // TODO remove
    Unknown(String),
}

pub enum Command {
    Assert(Term),
    Unknown(String),
}

pub struct Script {
    pub commands: Vec<Command>,
}

impl Script {
    fn new() -> Self {
        Script {
            commands: Vec::new(),
        }
    }
}

type VisitorError = (&'static str, Interval);
type VisitorResult<T> = Result<T, VisitorError>;

struct Listener {}

impl Listener {
    fn new() -> Self {
        Listener {}
    }

    fn script(&self, ctx: &ScriptContextAll) -> VisitorResult<Script> {
        let mut script = Script::new();
        for cmd in ctx.command_all() {
            script.commands.push(self.command(&*cmd)?)
        }

        Ok(script)
    }

    fn command(&self, ctx: &CommandContextAll) -> VisitorResult<Command> {
        if ctx.cmd_assert().is_some() {
            Ok(Command::Assert(self.term(&*ctx.term(0).unwrap())?))
        } else {
            Ok(Command::Unknown(ctx.get_text()))
        }
    }

    fn term(&self, ctx: &TermContextAll) -> VisitorResult<Term> {
        /*
         * term
         *     : spec_constant
         *     | qual_identifier
         *     | ParOpen qual_identifier term+ ParClose
         *     | ParOpen GRW_Let ParOpen var_binding+ ParClose term ParClose
         *     | ParOpen GRW_Forall ParOpen sorted_var+ ParClose term ParClose
         *     | ParOpen GRW_Exists ParOpen sorted_var+ ParClose term ParClose
         *     | ParOpen GRW_Match term ParOpen match_case+ ParClose ParClose
         *     | ParOpen GRW_Exclamation term attribute+ ParClose
         *     ;
         */
        let num_par_open = ctx.ParOpen_all().len();
        let num_par_close = ctx.ParClose_all().len();

        if let Some(spec_constant) = ctx.spec_constant() {
            Ok(Term::SpecConstant(self.spec_constant(&*spec_constant)?))
        } else {
            Ok(Term::Unknown(ctx.get_text()))
        }
    }

    fn spec_constant(&self, ctx: &Spec_constantContextAll) -> VisitorResult<SpecConstant> {
        if let Some(numeral) = ctx.numeral() {
            Ok(SpecConstant::Numeral(self.numeral(&*numeral)?))
        } else if let Some(_decimal) = ctx.decimal() {
            Err((
                "Decimal numbers are not supported yet",
                ctx.get_source_interval(),
            ))
        } else if let Some(hex) = ctx.hexadecimal() {
            Ok(SpecConstant::HexDecimal(self.hexadecimal(&*hex)?))
        } else if let Some(bin) = ctx.binary() {
            Ok(SpecConstant::Binary(self.binary(&*bin)?))
        } else {
            Ok(SpecConstant::String(self.string(&*ctx.string().unwrap())?))
        }
    }

    fn numeral(&self, ctx: &NumeralContextAll) -> VisitorResult<Numeral> {
        Ok(ctx.get_text())
    }

    fn hexadecimal(&self, ctx: &HexadecimalContextAll) -> VisitorResult<HexDecimal> {
        Ok(ctx.get_text())
    }

    fn binary(&self, ctx: &BinaryContextAll) -> VisitorResult<Binary> {
        Ok(ctx.get_text())
    }

    fn string(&self, ctx: &StringContextAll) -> VisitorResult<String> {
        Ok(ctx.get_text())
    }
}

impl ParseTreeListener<'_, SMTLIBv2ParserContextType> for Listener {}
impl SMTLIBv2Listener<'_> for Listener {}

pub struct PanicErrorListener {}

impl<'a, T: Recognizer<'a>> ErrorListener<'a, T> for PanicErrorListener {
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<T::TF as TokenFactory<'a>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _e: Option<&ANTLRError>,
    ) {
        panic!("line {}:{} {}", line, column, msg);
    }
}

pub fn parse(script: &str) -> Result<Script, String> {
    let mut lexer = SMTLIBv2Lexer::new(InputStream::new(script));
    lexer.add_error_listener(Box::new(PanicErrorListener {}));
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = SMTLIBv2Parser::new(token_source);
    let listener_id = parser.add_parse_listener(Box::new(Listener::new()));
    parser.add_error_listener(Box::new(PanicErrorListener {}));
    let result = parser.start();
    assert!(result.is_ok());
    let listener = parser.remove_parse_listener(listener_id);

    println!("Parsing finished.");

    let script_result = listener.script(&*result.unwrap().script().unwrap());

    script_result.map_err(|(e, interval)| -> String {
        let input = &parser.input;
        let token = input.get(interval.a);
        format!(
            "line {}:{}: {}, offending token: '{}'",
            token.get_line(),
            token.get_column(),
            e,
            input.get_text_from_interval(interval.a, interval.b)
        )
    })
}
