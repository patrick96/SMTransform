pub mod smtlibv2lexer;
pub mod smtlibv2listener;
pub mod smtlibv2parser;
pub mod smtlibv2visitor;

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

use smtlibv2lexer::SMTLIBv2Lexer;
use smtlibv2listener::SMTLIBv2Listener;
use smtlibv2parser::*;

pub type Symbol = String;
pub type Numeral = String;
pub type Decimal = String;
pub type HexDecimal = String;
pub type Binary = String;
pub type Keyword = String;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Sort {
    name: Identifier,
    sorts: Vec<Sort>,
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.sorts.is_empty() {
            write!(f, "(")?;
        }

        write!(f, "{}", self.name)?;

        for sort in &self.sorts {
            write!(f, " {}", sort)?;
        }

        if !self.sorts.is_empty() {
            write!(f, ")")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum SpecConstant {
    Numeral(Numeral),
    Decimal(Decimal),
    HexDecimal(HexDecimal),
    Binary(Binary),
    String(String),
}

impl std::fmt::Display for SpecConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SpecConstant::*;

        match self {
            Numeral(n) => n.fmt(f),
            Decimal(d) => d.fmt(f),
            HexDecimal(h) => h.fmt(f),
            Binary(b) => b.fmt(f),
            String(s) => s.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum AttributeValue {
    SpecConstant(SpecConstant),
    Symbol(Symbol),
}

impl std::fmt::Display for AttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AttributeValue::*;

        match self {
            SpecConstant(constant) => write!(f, "{}", constant),
            Symbol(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub keyword: Keyword,
    pub value: Option<AttributeValue>,
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.keyword)?;

        if let Some(value) = &self.value {
            write!(f, " {}", value)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Assert(Term),
    DeclareFun(Symbol, Vec<Sort>, Sort),
    CheckSat,
    GetModel,
    Exit,
    SetInfo(Attribute),
    SetLogic(Symbol),
    // Generic command with only a single string (e.g. (reset))
    Generic(String),
    Unknown(String),
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Command::*;

        match self {
            Assert(assert) => write!(f, "(assert {})", assert),
            DeclareFun(name, arg_sorts, return_sort) => {
                write!(f, "(declare-fun {} (", name)?;

                for (pos, arg_sort) in arg_sorts.iter().enumerate() {
                    if pos > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", arg_sort)?;
                }

                write!(f, ") {})", return_sort)
            }
            CheckSat => write!(f, "(check-sat)"),
            GetModel => write!(f, "(get-model)"),
            Exit => write!(f, "(exit)"),
            SetInfo(attr) => write!(f, "(set-info {})", attr),
            SetLogic(s) => write!(f, "(set-logic {})", s),
            Generic(s) => write!(f, "({})", s),
            Unknown(s) => write!(f, "(unknown: {})", s),
        }
    }
}

pub struct Script {
    pub commands: Vec<Command>,
}

impl Script {
    pub fn new() -> Self {
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
        } else if ctx.cmd_declareFun().is_some() {
            let name = self.symbol(&*ctx.symbol(0).unwrap())?;
            let sorts = &ctx.sort_all();

            let arg_sorts: VisitorResult<Vec<Sort>> = sorts[..sorts.len() - 1]
                .iter()
                .map(|sort| -> VisitorResult<Sort> { self.sort(&*sort) })
                .collect();

            let return_sort = self.sort(&*sorts.last().unwrap())?;

            Ok(Command::DeclareFun(name, arg_sorts?, return_sort))
        } else if ctx.cmd_checkSat().is_some() {
            Ok(Command::CheckSat)
        } else if ctx.cmd_getModel().is_some() {
            Ok(Command::GetModel)
        } else if ctx.cmd_exit().is_some() {
            Ok(Command::Exit)
        } else if ctx.cmd_setInfo().is_some() {
            Ok(Command::SetInfo(
                self.attribute(&*ctx.attribute().unwrap())?,
            ))
        } else if ctx.cmd_setLogic().is_some() {
            Ok(Command::SetLogic(self.symbol(&*ctx.symbol(0).unwrap())?))
        } else if ctx.get_child_count() == 3 {
            Ok(Command::Generic(ctx.get_child(1).unwrap().get_text()))
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
        } else if let Some(decimal) = ctx.decimal() {
            Ok(SpecConstant::Decimal(self.decimal(&*decimal)?))
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

    fn decimal(&self, ctx: &DecimalContextAll) -> VisitorResult<Decimal> {
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

            let mut id = format!("(_ {}", symbol);

            for index in &ctx.index_all() {
                id += " ";
                id += &index.get_text();
            }

            id += ")";

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

    fn sort(&self, ctx: &SortContextAll) -> VisitorResult<Sort> {
        Ok(Sort {
            name: self.identifier(&*ctx.identifier().unwrap())?,
            sorts: ctx
                .sort_all()
                .iter()
                .map(|sort| self.sort(&*sort))
                .collect::<VisitorResult<Vec<Sort>>>()?,
        })
    }

    fn attribute(&self, ctx: &AttributeContextAll) -> VisitorResult<Attribute> {
        let keyword = self.keyword(&*ctx.keyword().unwrap())?;
        if let Some(attribute_value) = ctx.attribute_value() {
            Ok(Attribute {
                keyword: keyword,
                value: Some(self.attribute_value(&*attribute_value)?),
            })
        } else {
            Ok(Attribute {
                keyword: keyword,
                value: None,
            })
        }
    }

    fn attribute_value(&self, ctx: &Attribute_valueContextAll) -> VisitorResult<AttributeValue> {
        if let Some(spec_constant) = ctx.spec_constant() {
            Ok(AttributeValue::SpecConstant(
                self.spec_constant(&*spec_constant)?,
            ))
        } else if let Some(symbol) = ctx.symbol() {
            Ok(AttributeValue::Symbol(self.symbol(&*symbol)?))
        } else {
            visitor_error!("Unsupported attribute value", ctx)
        }
    }

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

    let script_result = listener.script(&*result.unwrap().script().unwrap());

    script_result.map_err(|(e, interval)| -> String {
        let input = &parser.input;
        let token1 = input.get(interval.a);
        let token2 = input.get(interval.b);

        let mut token_string = String::new();

        for i in interval.a..=interval.b {
            if i != interval.a {
                token_string.push(' ');
            }
            token_string += input.get(i).get_text();
        }

        format!(
            "line {}:{}-{}:{}: {}, offending token: '{}'",
            token1.get_line(),
            token1.get_column(),
            token2.get_line(),
            token2.get_column(),
            e,
            token_string
        )
    })
}
