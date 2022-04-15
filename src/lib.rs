#![feature(try_blocks)]

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::errors::ANTLRError;
use antlr_rust::input_stream::InputStream;
use antlr_rust::interval_set::Interval;
use antlr_rust::recognizer::Recognizer;
use antlr_rust::token::Token;
use antlr_rust::token_factory::TokenFactory;
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::{ParseTree, ParseTreeListener};
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

pub enum Identifier {
    Id(String),
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Identifier::*;

        match self {
            Id(id) => id.fmt(f),
        }
    }
}

pub enum SpecConstant {
    Numeral(Numeral),
    HexDecimal(HexDecimal),
    Binary(Binary),
    String(String),
}

impl std::fmt::Display for SpecConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SpecConstant::*;

        match self {
            Numeral(n) => n.fmt(f),
            HexDecimal(h) => h.fmt(f),
            Binary(b) => b.fmt(f),
            String(s) => s.fmt(f),
        }
    }
}

pub enum Term {
    SpecConstant(SpecConstant),
    Identifier(Identifier),
    Op(Identifier, Vec<Term>),
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Term::*;

        match self {
            SpecConstant(c) => c.fmt(f),
            Identifier(s) => s.fmt(f),
            Op(id, terms) => {
                write!(f, "(")?;
                id.fmt(f)?;
                for term in terms {
                    write!(f, " ")?;
                    term.fmt(f)?;
                }
                write!(f, ")")
            }
        }
    }
}

pub enum Command {
    Assert(Term),
    CheckSat,
    Unknown(String),
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Command::*;

        match self {
            Assert(assert) => {
                write!(f, "(assert ")?;
                assert.fmt(f)?;
                write!(f, ")")
            }
            CheckSat => write!(f, "(check-sat)"),
            Unknown(s) => write!(f, "(unknown: {})", s),
        }
    }
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

impl std::fmt::Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for command in &self.commands {
            command.fmt(f)?;
            writeln!(f)?;
        }

        Ok(())
    }
}

type VisitorError = (&'static str, Interval);
type VisitorResult<T> = Result<T, VisitorError>;

struct Listener {}

macro_rules! visitor_error {
    ($msg:literal, $ctx:expr) => {
        Err(($msg, $ctx.get_source_interval()))
    };
}
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
        } else if ctx.cmd_checkSat().is_some() {
            Ok(Command::CheckSat)
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
        } else if let Some(qual_identifier) = ctx.qual_identifier() {
            if num_par_open >= 1 && num_par_close >= 1 {
                let op = self.qual_identifier(&*qual_identifier)?;
                let mut terms = Vec::new();

                for term in &ctx.term_all() {
                    terms.push(self.term(&*term)?);
                }

                Ok(Term::Op(op, terms))
            } else {
                Ok(Term::Identifier(self.qual_identifier(&*qual_identifier)?))
            }
        } else {
            visitor_error!("Unsupported term", ctx)
        }
    }

    fn spec_constant(&self, ctx: &Spec_constantContextAll) -> VisitorResult<SpecConstant> {
        if let Some(numeral) = ctx.numeral() {
            Ok(SpecConstant::Numeral(self.numeral(&*numeral)?))
        } else if let Some(_decimal) = ctx.decimal() {
            visitor_error!("Decimal numbers are not supported yet", ctx)
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

    fn qual_identifier(&self, ctx: &Qual_identifierContextAll) -> VisitorResult<Identifier> {
        if ctx.GRW_As().is_some() {
            visitor_error!("'as' identifiers no supported", ctx)
        } else if let Some(id) = ctx.identifier() {
            self.identifier(&*id)
        } else {
            visitor_error!("Unsupported qual_identifier", ctx)
        }
    }

    fn identifier(&self, ctx: &IdentifierContextAll) -> VisitorResult<Identifier> {
        if ctx.GRW_Underscore().is_some() {
            let symbol = self.symbol(&*ctx.symbol().unwrap())?;

            let mut id = format!("_ {}", symbol);

            for index in &ctx.index_all() {
                id += " ";
                id += &index.get_text();
            }

            Ok(Identifier::Id(id))
        } else if let Some(symbol) = ctx.symbol() {
            Ok(Identifier::Id(self.symbol(&*symbol)?))
        } else {
            visitor_error!("Unsupported identifier", ctx)
        }
    }

    fn symbol(&self, ctx: &SymbolContextAll) -> VisitorResult<Symbol> {
        Ok(ctx.get_text())
    }

    #[allow(dead_code)]
    fn keyword(&self, ctx: &KeywordContextAll) -> VisitorResult<Keyword> {
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
        let token1 = input.get(interval.a);
        let token2 = input.get(interval.b);
        format!(
            "line {}:{}-{}:{}: {}, offending token: '{}'",
            token1.get_line(),
            token1.get_column(),
            token2.get_line(),
            token2.get_column(),
            e,
            input.get_text_from_interval(interval.a, interval.b)
        )
    })
}
