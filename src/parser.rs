pub mod smtlibv2lexer;
pub mod smtlibv2listener;
pub mod smtlibv2parser;
pub mod smtlibv2visitor;

use std::collections::HashMap;
use std::fmt::Display;

use lazy_static::lazy_static;
use regex::Regex;

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

pub type Numeral = String;
pub type Decimal = String;
pub type HexDecimal = String;
pub type Binary = String;
pub type Keyword = String;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Symbol {
    pub s: String,
    quoted: bool,
}

impl Symbol {
    pub fn new_simple(s: String) -> Self {
        Self { s, quoted: false }
    }
    pub fn new_quoted(s: String) -> Self {
        Self { s, quoted: true }
    }

    pub fn new(s: String) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^[a-zA-Z+=/*%?!$-_~&^<>@.](\d|[a-zA-Z+=/*%?!$-_~&^<>@.])*$").unwrap();
        }

        if RE.is_match(s.as_str()) {
            Self::new_simple(s)
        } else {
            Self::new_quoted(s)
        }
    }

    pub fn is_simple(&self) -> bool {
        !self.quoted
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_symbol() {
        assert!(Symbol::new("Int".to_string()).is_simple());
        assert!(Symbol::new("sat".to_string()).is_simple());
        assert!(Symbol::new("v0".to_string()).is_simple());
    }

    #[test]
    fn test_quoted_symbol() {
        assert!(!Symbol::new("foo bar".to_string()).is_simple());
    }
}

impl From<Symbol> for String {
    fn from(s: Symbol) -> Self {
        s.s
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.quoted {
            write!(f, "|{}|", self.s)
        } else {
            write!(f, "{}", self.s)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Bool,
    Real,
    String,
    Fun(String),
    Other(String),
    /*
     * Used for variable definitions without a specified type (e.g. let-bindings)
     */
    Unknown,
}

impl Type {
    fn from(args: &[Sort], result: &Sort) -> Type {
        if args.is_empty() {
            // TODO handle |Int| ...
            match result.to_string().as_str() {
                "Int" => Type::Int,
                "Bool" => Type::Bool,
                "Real" => Type::Real,
                "String" => Type::String,
                s => Type::Other(s.to_string()),
            }
        } else {
            Type::Fun(format!(
                "{} {}",
                args.iter()
                    .fold(String::new(), |a, b| format!("{} {}", a, b)),
                result
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: String,
    pub global: bool,
    pub t: Type,
}

impl Var {
    pub fn new(name: String, t: Type) -> Self {
        Self {
            name,
            global: true,
            t,
        }
    }

    pub fn new_local(name: String, t: Type) -> Self {
        Self {
            name,
            global: false,
            t,
        }
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Symbol::new(self.name.clone()).fmt(f)
    }
}

#[derive(Debug, Clone)]
pub enum Identifier {
    Id(String),
    Var(Var),
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Identifier::*;

        match self {
            Id(id) => id.fmt(f),
            Var(var) => var.fmt(f),
        }
    }
}

impl From<Var> for Identifier {
    fn from(var: Var) -> Self {
        Identifier::Var(var)
    }
}

impl From<Symbol> for Identifier {
    fn from(s: Symbol) -> Self {
        Identifier::Id(s.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Sort {
    name: Identifier,
    sorts: Vec<Sort>,
}

impl Sort {
    pub fn new(name: Identifier, sorts: &[Sort]) -> Self {
        Self {
            name,
            sorts: Vec::from(sorts),
        }
    }
}

impl Display for Sort {
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

impl Display for SpecConstant {
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
    Let(Vec<(Symbol, Term)>, Box<Term>),
}

impl Display for Term {
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
            Let(bindings, subterm) => {
                write!(f, "(let (")?;

                for (sym, term) in bindings {
                    write!(f, "({} {})", sym, term)?;
                }

                write!(f, ") {})", *subterm)
            }
        }
    }
}

impl From<SpecConstant> for Term {
    fn from(c: SpecConstant) -> Self {
        Term::SpecConstant(c)
    }
}

impl From<Identifier> for Term {
    fn from(id: Identifier) -> Self {
        Term::Identifier(id)
    }
}

impl From<Var> for Term {
    fn from(var: Var) -> Self {
        Term::Identifier(var.into())
    }
}

#[derive(Debug, Clone)]
pub enum AttributeValue {
    SpecConstant(SpecConstant),
    Symbol(Symbol),
}

impl Display for AttributeValue {
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

impl Display for Attribute {
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
    DefineFun(Symbol, Vec<(Symbol, Sort)>, Sort, Term),
    CheckSat,
    GetModel,
    Exit,
    SetInfo(Attribute),
    SetLogic(Symbol),
    // Generic command with only a single string (e.g. (reset))
    Generic(String),
}

impl Display for Command {
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
            DefineFun(name, args, return_sort, term) => {
                write!(f, "(define-fun {} (", name)?;

                for (pos, (arg_name, arg_sort)) in args.iter().enumerate() {
                    if pos > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "({} {})", arg_name, arg_sort)?;
                }

                write!(f, ") {} {})", return_sort, term)
            }
            CheckSat => write!(f, "(check-sat)"),
            GetModel => write!(f, "(get-model)"),
            Exit => write!(f, "(exit)"),
            SetInfo(attr) => write!(f, "(set-info {})", attr),
            SetLogic(s) => write!(f, "(set-logic {})", s),
            Generic(s) => write!(f, "({})", s),
        }
    }
}

#[derive(Debug)]
pub struct Script {
    pub commands: Vec<Command>,
    pub global_vars: HashMap<String, Type>,
}

impl Script {
    pub fn new() -> Self {
        Script {
            commands: Vec::new(),
            global_vars: HashMap::new(),
        }
    }
}

impl Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for command in &self.commands {
            command.fmt(f)?;
            writeln!(f)?;
        }

        Ok(())
    }
}

type VisitorError = (String, Interval);
type VisitorResult<T> = Result<T, VisitorError>;

struct Listener {
    global_vars: HashMap<String, Type>,
}

macro_rules! visitor_error {
    ($msg:expr, $ctx:expr) => {
        Err(($msg.to_string(), $ctx.get_source_interval()))
    };
}
impl Listener {
    fn new() -> Self {
        Listener {
            global_vars: HashMap::new(),
        }
    }

    fn add_global(&mut self, name: &Symbol, t: &Type) -> Result<(), String> {
        if self.global_vars.contains_key(&name.s) {
            return Err(format!("Global variable '{}' already exists", name));
        }

        self.global_vars.insert(name.clone().into(), t.clone());

        Ok(())
    }

    fn script(&mut self, ctx: &ScriptContextAll) -> VisitorResult<Script> {
        let commands = ctx
            .command_all()
            .iter()
            .map(|cmd| self.command(&*cmd))
            .collect::<VisitorResult<Vec<Command>>>()?;

        Ok(Script {
            commands,
            global_vars: self.global_vars.clone(),
        })
    }

    fn command(&mut self, ctx: &CommandContextAll) -> VisitorResult<Command> {
        if ctx.cmd_assert().is_some() {
            Ok(Command::Assert(
                self.term(&*ctx.term(0).unwrap(), HashMap::new())?,
            ))
        } else if ctx.cmd_declareConst().is_some() {
            let name = self.symbol(&*ctx.symbol(0).unwrap())?;
            let return_sort = self.sort(&*ctx.sort(0).unwrap())?;
            let fun_type = Type::from(&[], &return_sort);

            if let Err(msg) = self.add_global(&name, &fun_type) {
                return visitor_error!(msg.as_str(), ctx);
            }

            Ok(Command::DeclareFun(name, Vec::new(), return_sort))
        } else if ctx.cmd_declareFun().is_some() {
            let name = self.symbol(&*ctx.symbol(0).unwrap())?;
            let sorts = &ctx.sort_all();

            let arg_sorts: Vec<Sort> = sorts[..sorts.len() - 1]
                .iter()
                .map(|sort| self.sort(&*sort))
                .collect::<VisitorResult<Vec<Sort>>>()?;

            let return_sort = self.sort(&*sorts.last().unwrap())?;
            let fun_type = Type::from(arg_sorts.as_slice(), &return_sort);

            if let Err(msg) = self.add_global(&name, &fun_type) {
                return visitor_error!(msg.as_str(), ctx);
            }

            Ok(Command::DeclareFun(name, arg_sorts, return_sort))
        } else if ctx.cmd_defineFun().is_some() {
            let def_ctx = &*ctx.function_def().unwrap();

            let name = self.symbol(&*def_ctx.symbol().unwrap())?;

            let args = def_ctx
                .sorted_var_all()
                .iter()
                .map(|sorted_var| self.sorted_var(&*sorted_var))
                .collect::<VisitorResult<Vec<(Symbol, Sort)>>>()?;

            let local_vars = HashMap::from_iter(
                args.clone()
                    .into_iter()
                    .map(|(sym, sort)| (sym, Type::from(&[], &sort))),
            );

            let return_sort = self.sort(&*def_ctx.sort().unwrap())?;
            let term = self.term(&*def_ctx.term().unwrap(), local_vars)?;

            Ok(Command::DefineFun(name, args, return_sort, term))
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
            visitor_error!("Unsupported command", ctx)
        }
    }

    fn sorted_var(&self, ctx: &Sorted_varContextAll) -> VisitorResult<(Symbol, Sort)> {
        Ok((
            self.symbol(&*ctx.symbol().unwrap())?,
            self.sort(&*ctx.sort().unwrap())?,
        ))
    }

    fn term(
        &self,
        ctx: &TermContextAll,
        mut local_vars: HashMap<Symbol, Type>,
    ) -> VisitorResult<Term> {
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
                let op = self.qual_identifier(&*qual_identifier, &local_vars)?;

                let terms = ctx
                    .term_all()
                    .iter()
                    .map(|term| self.term(&*term, local_vars.clone()))
                    .collect::<VisitorResult<Vec<Term>>>()?;

                Ok(Term::Op(op, terms))
            } else {
                Ok(Term::Identifier(
                    self.qual_identifier(&*qual_identifier, &local_vars)?,
                ))
            }
        } else if ctx.GRW_Let().is_some() {
            let bindings = ctx
                .var_binding_all()
                .iter()
                .map(|var_binding| self.var_binding(var_binding, local_vars.clone()))
                .collect::<VisitorResult<Vec<(Symbol, Term)>>>()?;

            for (name, _) in &bindings {
                local_vars.insert(name.clone(), Type::Unknown);
            }

            Ok(Term::Let(
                bindings,
                Box::new(self.term(&*ctx.term(0).unwrap(), local_vars)?),
            ))
        } else {
            visitor_error!("Unsupported term", ctx)
        }
    }

    fn var_binding(
        &self,
        ctx: &Var_bindingContextAll,
        local_vars: HashMap<Symbol, Type>,
    ) -> VisitorResult<(Symbol, Term)> {
        Ok((
            self.symbol(&*ctx.symbol().unwrap())?,
            self.term(&*ctx.term().unwrap(), local_vars)?,
        ))
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

    fn qual_identifier(
        &self,
        ctx: &Qual_identifierContextAll,
        local_vars: &HashMap<Symbol, Type>,
    ) -> VisitorResult<Identifier> {
        if ctx.GRW_As().is_some() {
            visitor_error!("'as' identifiers no supported", ctx)
        } else if let Some(id) = ctx.identifier() {
            self.identifier(&*id, local_vars)
        } else {
            visitor_error!("Unsupported qual_identifier", ctx)
        }
    }

    fn identifier(
        &self,
        ctx: &IdentifierContextAll,
        local_vars: &HashMap<Symbol, Type>,
    ) -> VisitorResult<Identifier> {
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
            let sym = self.symbol(&*symbol)?;

            if let Some(t) = local_vars.get(&sym) {
                Ok(Var::new_local(sym.into(), t.clone()).into())
            } else if let Some(t) = self.global_vars.get(&sym.s) {
                Ok(Var::new(sym.into(), t.clone()).into())
            } else {
                Ok(Identifier::Id(sym.to_string()))
            }
        } else {
            visitor_error!("Unsupported identifier", ctx)
        }
    }

    fn symbol(&self, ctx: &SymbolContextAll) -> VisitorResult<Symbol> {
        if let Some(quoted) = ctx.quotedSymbol() {
            let text = quoted.get_text();
            Ok(Symbol::new_quoted(text[1..(text.len() - 1)].to_string()))
        } else if let Some(simple) = ctx.simpleSymbol() {
            Ok(Symbol::new_simple(simple.get_text()))
        } else {
            visitor_error!("Unsupported symbol", ctx)
        }
    }

    fn sort(&self, ctx: &SortContextAll) -> VisitorResult<Sort> {
        Ok(Sort {
            name: self.identifier(&*ctx.identifier().unwrap(), &HashMap::new())?,
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
                keyword,
                value: Some(self.attribute_value(&*attribute_value)?),
            })
        } else {
            Ok(Attribute {
                keyword,
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
    let mut listener = parser.remove_parse_listener(listener_id);

    let script_result = listener.script(&*result.unwrap().script().unwrap());

    script_result.map_err(|(e, interval)| -> String {
        let input = &parser.input;
        let token1 = input.get(interval.a);
        let token2 = input.get(interval.b);

        let mut token_string = String::new();

        for i in interval.a..=interval.b {
            let token = input.get(i);
            let token_type = token.get_token_type();

            if i != interval.a
                && token_type != smtlibv2lexer::ParClose
                && input.get(i - 1).get_token_type() != smtlibv2lexer::ParOpen
            {
                token_string.push(' ');
            }
            token_string.push_str(token.get_text());
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
