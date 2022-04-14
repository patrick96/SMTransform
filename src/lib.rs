#![feature(try_blocks)]

use std::ops::Deref;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::tree::{ParseTree, ParseTreeListener, Tree};

mod parser;

use parser::smtlibv2lexer;
use parser::smtlibv2listener;
use parser::smtlibv2parser;

use smtlibv2lexer::SMTLIBv2Lexer;
use smtlibv2listener::SMTLIBv2Listener;
use smtlibv2parser::*;

type Numeral = u64;
type HexDecimal = Vec<bool>;
type Binary = Vec<bool>;

enum SpecConstant {
    Numeral(Numeral),
    HexDecimal(HexDecimal),
    Binary(Binary),
    String(String),
}

enum Term {
    SpecConstant(SpecConstant),
    // TODO remove
    Unknown(String),
}

enum Command {
    Assert(Term),
    Unknown(String),
}

struct Script {
    commands: Vec<Command>,
}

impl Script {
    fn new() -> Self {
        Script {
            commands: Vec::new(),
        }
    }
}

struct Listener {
    script: Script,
}

impl Listener {
    fn new() -> Self {
        Listener {
            script: Script::new(),
        }
    }

    fn command(&self, ctx: &CommandContextAll) -> Command {
        // if ctx.cmd_declareFun().is_some() {
        //     println!("Found declare-fun");
        // }
        if ctx.cmd_assert().is_some() {
            Command::Assert(self.term(&*ctx.term(0).unwrap()))
        } else {
            Command::Unknown(ctx.get_text())
        }
    }

    fn term(&self, ctx: &TermContextAll) -> Term {
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
            Term::SpecConstant(self.spec_constant(&*spec_constant))
        } else {
            Term::Unknown(ctx.get_text())
        }
    }

    fn spec_constant(&self, ctx: &Spec_constantContextAll) -> SpecConstant {
        if let Some(numeral) = ctx.numeral() {
            SpecConstant::Numeral(self.numeral(&*numeral))
        } else if let Some(_decimal) = ctx.decimal() {
            panic!("Decimal numbers are not supported yet");
        } else if let Some(hex) = ctx.hexadecimal() {
            SpecConstant::HexDecimal(self.hexadecimal(&*hex))
        } else if let Some(bin) = ctx.binary() {
            SpecConstant::Binary(self.binary(&*bin))
        } else {
            SpecConstant::String(self.string(&*ctx.string().unwrap()))
        }
    }

    fn numeral(&self, ctx: &NumeralContextAll) -> Numeral {
        let text = ctx.get_text();
        text.parse().expect(&text[..])
    }

    fn hexadecimal(&self, ctx: &HexadecimalContextAll) -> HexDecimal {
        // TODO
        Vec::new()
    }

    fn binary(&self, ctx: &BinaryContextAll) -> Binary {
        // TODO
        Vec::new()
    }

    fn string(&self, ctx: &StringContextAll) -> String {
        ctx.get_text()
    }
}

impl ParseTreeListener<'_, SMTLIBv2ParserContextType> for Listener {}
impl SMTLIBv2Listener<'_> for Listener {
    fn exit_script(&mut self, _ctx: &ScriptContext) {
        println!("exit_script");
        for cmd in _ctx.command_all() {
            self.script.commands.push(self.command(&*cmd));
        }
    }
}

pub fn parse(script: &str) -> i32 {
    let lexer = SMTLIBv2Lexer::new(InputStream::new(script));
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = SMTLIBv2Parser::new(token_source);
    let listener_id = parser.add_parse_listener(Box::new(Listener::new()));
    let result = parser.start();
    assert!(result.is_ok());
    let listener = parser.remove_parse_listener(listener_id);

    println!("Parsing finished.");

    let script = listener.script;
    println!("Parsed {} commands", script.commands.len());

    return 12;
}
