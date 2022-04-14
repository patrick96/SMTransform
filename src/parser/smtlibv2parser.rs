// Generated from src/parser/SMTLIBv2.g4 by ANTLR 4.8
#![allow(warnings)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use super::smtlibv2listener::*;
use super::smtlibv2visitor::*;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::parser::{BaseParser, Parser, ParserNodeType, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{cast, cast_mut, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, Token, TOKEN_EOF};
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::*;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const Comment: isize = 1;
pub const ParOpen: isize = 2;
pub const ParClose: isize = 3;
pub const Semicolon: isize = 4;
pub const String: isize = 5;
pub const QuotedSymbol: isize = 6;
pub const PS_Not: isize = 7;
pub const PS_Bool: isize = 8;
pub const PS_ContinuedExecution: isize = 9;
pub const PS_Error: isize = 10;
pub const PS_False: isize = 11;
pub const PS_ImmediateExit: isize = 12;
pub const PS_Incomplete: isize = 13;
pub const PS_Logic: isize = 14;
pub const PS_Memout: isize = 15;
pub const PS_Sat: isize = 16;
pub const PS_Success: isize = 17;
pub const PS_Theory: isize = 18;
pub const PS_True: isize = 19;
pub const PS_Unknown: isize = 20;
pub const PS_Unsupported: isize = 21;
pub const PS_Unsat: isize = 22;
pub const CMD_Assert: isize = 23;
pub const CMD_CheckSat: isize = 24;
pub const CMD_CheckSatAssuming: isize = 25;
pub const CMD_DeclareConst: isize = 26;
pub const CMD_DeclareDatatype: isize = 27;
pub const CMD_DeclareDatatypes: isize = 28;
pub const CMD_DeclareFun: isize = 29;
pub const CMD_DeclareSort: isize = 30;
pub const CMD_DefineFun: isize = 31;
pub const CMD_DefineFunRec: isize = 32;
pub const CMD_DefineFunsRec: isize = 33;
pub const CMD_DefineSort: isize = 34;
pub const CMD_Echo: isize = 35;
pub const CMD_Exit: isize = 36;
pub const CMD_GetAssertions: isize = 37;
pub const CMD_GetAssignment: isize = 38;
pub const CMD_GetInfo: isize = 39;
pub const CMD_GetModel: isize = 40;
pub const CMD_GetOption: isize = 41;
pub const CMD_GetProof: isize = 42;
pub const CMD_GetUnsatAssumptions: isize = 43;
pub const CMD_GetUnsatCore: isize = 44;
pub const CMD_GetValue: isize = 45;
pub const CMD_Pop: isize = 46;
pub const CMD_Push: isize = 47;
pub const CMD_Reset: isize = 48;
pub const CMD_ResetAssertions: isize = 49;
pub const CMD_SetInfo: isize = 50;
pub const CMD_SetLogic: isize = 51;
pub const CMD_SetOption: isize = 52;
pub const GRW_Exclamation: isize = 53;
pub const GRW_Underscore: isize = 54;
pub const GRW_As: isize = 55;
pub const GRW_Binary: isize = 56;
pub const GRW_Decimal: isize = 57;
pub const GRW_Exists: isize = 58;
pub const GRW_Hexadecimal: isize = 59;
pub const GRW_Forall: isize = 60;
pub const GRW_Let: isize = 61;
pub const GRW_Match: isize = 62;
pub const GRW_Numeral: isize = 63;
pub const GRW_Par: isize = 64;
pub const GRW_String: isize = 65;
pub const Numeral: isize = 66;
pub const Binary: isize = 67;
pub const HexDecimal: isize = 68;
pub const Decimal: isize = 69;
pub const Colon: isize = 70;
pub const PK_AllStatistics: isize = 71;
pub const PK_AssertionStackLevels: isize = 72;
pub const PK_Authors: isize = 73;
pub const PK_Category: isize = 74;
pub const PK_Chainable: isize = 75;
pub const PK_Definition: isize = 76;
pub const PK_DiagnosticOutputChannel: isize = 77;
pub const PK_ErrorBehaviour: isize = 78;
pub const PK_Extension: isize = 79;
pub const PK_Funs: isize = 80;
pub const PK_FunsDescription: isize = 81;
pub const PK_GlobalDeclarations: isize = 82;
pub const PK_InteractiveMode: isize = 83;
pub const PK_Language: isize = 84;
pub const PK_LeftAssoc: isize = 85;
pub const PK_License: isize = 86;
pub const PK_Named: isize = 87;
pub const PK_Name: isize = 88;
pub const PK_Notes: isize = 89;
pub const PK_Pattern: isize = 90;
pub const PK_PrintSuccess: isize = 91;
pub const PK_ProduceAssertions: isize = 92;
pub const PK_ProduceAssignments: isize = 93;
pub const PK_ProduceModels: isize = 94;
pub const PK_ProduceProofs: isize = 95;
pub const PK_ProduceUnsatAssumptions: isize = 96;
pub const PK_ProduceUnsatCores: isize = 97;
pub const PK_RandomSeed: isize = 98;
pub const PK_ReasonUnknown: isize = 99;
pub const PK_RegularOutputChannel: isize = 100;
pub const PK_ReproducibleResourceLimit: isize = 101;
pub const PK_RightAssoc: isize = 102;
pub const PK_SmtLibVersion: isize = 103;
pub const PK_Sorts: isize = 104;
pub const PK_SortsDescription: isize = 105;
pub const PK_Source: isize = 106;
pub const PK_Status: isize = 107;
pub const PK_Theories: isize = 108;
pub const PK_Values: isize = 109;
pub const PK_Verbosity: isize = 110;
pub const PK_Version: isize = 111;
pub const UndefinedSymbol: isize = 112;
pub const WS: isize = 113;
pub const RULE_start: usize = 0;
pub const RULE_response: usize = 1;
pub const RULE_generalReservedWord: usize = 2;
pub const RULE_simpleSymbol: usize = 3;
pub const RULE_quotedSymbol: usize = 4;
pub const RULE_predefSymbol: usize = 5;
pub const RULE_predefKeyword: usize = 6;
pub const RULE_symbol: usize = 7;
pub const RULE_numeral: usize = 8;
pub const RULE_decimal: usize = 9;
pub const RULE_hexadecimal: usize = 10;
pub const RULE_binary: usize = 11;
pub const RULE_string: usize = 12;
pub const RULE_keyword: usize = 13;
pub const RULE_spec_constant: usize = 14;
pub const RULE_s_expr: usize = 15;
pub const RULE_index: usize = 16;
pub const RULE_identifier: usize = 17;
pub const RULE_attribute_value: usize = 18;
pub const RULE_attribute: usize = 19;
pub const RULE_sort: usize = 20;
pub const RULE_qual_identifier: usize = 21;
pub const RULE_var_binding: usize = 22;
pub const RULE_sorted_var: usize = 23;
pub const RULE_pattern: usize = 24;
pub const RULE_match_case: usize = 25;
pub const RULE_term: usize = 26;
pub const RULE_sort_symbol_decl: usize = 27;
pub const RULE_meta_spec_constant: usize = 28;
pub const RULE_fun_symbol_decl: usize = 29;
pub const RULE_par_fun_symbol_decl: usize = 30;
pub const RULE_theory_attribute: usize = 31;
pub const RULE_theory_decl: usize = 32;
pub const RULE_logic_attribue: usize = 33;
pub const RULE_logic: usize = 34;
pub const RULE_sort_dec: usize = 35;
pub const RULE_selector_dec: usize = 36;
pub const RULE_constructor_dec: usize = 37;
pub const RULE_datatype_dec: usize = 38;
pub const RULE_function_dec: usize = 39;
pub const RULE_function_def: usize = 40;
pub const RULE_prop_literal: usize = 41;
pub const RULE_script: usize = 42;
pub const RULE_cmd_assert: usize = 43;
pub const RULE_cmd_checkSat: usize = 44;
pub const RULE_cmd_checkSatAssuming: usize = 45;
pub const RULE_cmd_declareConst: usize = 46;
pub const RULE_cmd_declareDatatype: usize = 47;
pub const RULE_cmd_declareDatatypes: usize = 48;
pub const RULE_cmd_declareFun: usize = 49;
pub const RULE_cmd_declareSort: usize = 50;
pub const RULE_cmd_defineFun: usize = 51;
pub const RULE_cmd_defineFunRec: usize = 52;
pub const RULE_cmd_defineFunsRec: usize = 53;
pub const RULE_cmd_defineSort: usize = 54;
pub const RULE_cmd_echo: usize = 55;
pub const RULE_cmd_exit: usize = 56;
pub const RULE_cmd_getAssertions: usize = 57;
pub const RULE_cmd_getAssignment: usize = 58;
pub const RULE_cmd_getInfo: usize = 59;
pub const RULE_cmd_getModel: usize = 60;
pub const RULE_cmd_getOption: usize = 61;
pub const RULE_cmd_getProof: usize = 62;
pub const RULE_cmd_getUnsatAssumptions: usize = 63;
pub const RULE_cmd_getUnsatCore: usize = 64;
pub const RULE_cmd_getValue: usize = 65;
pub const RULE_cmd_pop: usize = 66;
pub const RULE_cmd_push: usize = 67;
pub const RULE_cmd_reset: usize = 68;
pub const RULE_cmd_resetAssertions: usize = 69;
pub const RULE_cmd_setInfo: usize = 70;
pub const RULE_cmd_setLogic: usize = 71;
pub const RULE_cmd_setOption: usize = 72;
pub const RULE_command: usize = 73;
pub const RULE_b_value: usize = 74;
pub const RULE_option: usize = 75;
pub const RULE_info_flag: usize = 76;
pub const RULE_error_behaviour: usize = 77;
pub const RULE_reason_unknown: usize = 78;
pub const RULE_model_response: usize = 79;
pub const RULE_info_response: usize = 80;
pub const RULE_valuation_pair: usize = 81;
pub const RULE_t_valuation_pair: usize = 82;
pub const RULE_check_sat_response: usize = 83;
pub const RULE_echo_response: usize = 84;
pub const RULE_get_assertions_response: usize = 85;
pub const RULE_get_assignment_response: usize = 86;
pub const RULE_get_info_response: usize = 87;
pub const RULE_get_model_response: usize = 88;
pub const RULE_get_option_response: usize = 89;
pub const RULE_get_proof_response: usize = 90;
pub const RULE_get_unsat_assump_response: usize = 91;
pub const RULE_get_unsat_core_response: usize = 92;
pub const RULE_get_value_response: usize = 93;
pub const RULE_specific_success_response: usize = 94;
pub const RULE_general_response: usize = 95;
pub const ruleNames: [&'static str; 96] = [
    "start",
    "response",
    "generalReservedWord",
    "simpleSymbol",
    "quotedSymbol",
    "predefSymbol",
    "predefKeyword",
    "symbol",
    "numeral",
    "decimal",
    "hexadecimal",
    "binary",
    "string",
    "keyword",
    "spec_constant",
    "s_expr",
    "index",
    "identifier",
    "attribute_value",
    "attribute",
    "sort",
    "qual_identifier",
    "var_binding",
    "sorted_var",
    "pattern",
    "match_case",
    "term",
    "sort_symbol_decl",
    "meta_spec_constant",
    "fun_symbol_decl",
    "par_fun_symbol_decl",
    "theory_attribute",
    "theory_decl",
    "logic_attribue",
    "logic",
    "sort_dec",
    "selector_dec",
    "constructor_dec",
    "datatype_dec",
    "function_dec",
    "function_def",
    "prop_literal",
    "script",
    "cmd_assert",
    "cmd_checkSat",
    "cmd_checkSatAssuming",
    "cmd_declareConst",
    "cmd_declareDatatype",
    "cmd_declareDatatypes",
    "cmd_declareFun",
    "cmd_declareSort",
    "cmd_defineFun",
    "cmd_defineFunRec",
    "cmd_defineFunsRec",
    "cmd_defineSort",
    "cmd_echo",
    "cmd_exit",
    "cmd_getAssertions",
    "cmd_getAssignment",
    "cmd_getInfo",
    "cmd_getModel",
    "cmd_getOption",
    "cmd_getProof",
    "cmd_getUnsatAssumptions",
    "cmd_getUnsatCore",
    "cmd_getValue",
    "cmd_pop",
    "cmd_push",
    "cmd_reset",
    "cmd_resetAssertions",
    "cmd_setInfo",
    "cmd_setLogic",
    "cmd_setOption",
    "command",
    "b_value",
    "option",
    "info_flag",
    "error_behaviour",
    "reason_unknown",
    "model_response",
    "info_response",
    "valuation_pair",
    "t_valuation_pair",
    "check_sat_response",
    "echo_response",
    "get_assertions_response",
    "get_assignment_response",
    "get_info_response",
    "get_model_response",
    "get_option_response",
    "get_proof_response",
    "get_unsat_assump_response",
    "get_unsat_core_response",
    "get_value_response",
    "specific_success_response",
    "general_response",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 112] = [
    None,
    None,
    Some("'('"),
    Some("')'"),
    Some("';'"),
    None,
    None,
    Some("'not'"),
    Some("'Bool'"),
    Some("'continued-execution'"),
    Some("'error'"),
    Some("'false'"),
    Some("'immediate-exit'"),
    Some("'incomplete'"),
    Some("'logic'"),
    Some("'memout'"),
    Some("'sat'"),
    Some("'success'"),
    Some("'theory'"),
    Some("'true'"),
    Some("'unknown'"),
    Some("'unsupported'"),
    Some("'unsat'"),
    Some("'assert'"),
    Some("'check-sat'"),
    Some("'check-sat-assuming'"),
    Some("'declare-const'"),
    Some("'declare-datatype'"),
    Some("'declare-datatypes'"),
    Some("'declare-fun'"),
    Some("'declare-sort'"),
    Some("'define-fun'"),
    Some("'define-fun-rec'"),
    Some("'define-funs-rec'"),
    Some("'define-sort'"),
    Some("'echo'"),
    Some("'exit'"),
    Some("'get-assertions'"),
    Some("'get-assignment'"),
    Some("'get-info'"),
    Some("'get-model'"),
    Some("'get-option'"),
    Some("'get-proof'"),
    Some("'get-unsat-assumptions'"),
    Some("'get-unsat-core'"),
    Some("'get-value'"),
    Some("'pop'"),
    Some("'push'"),
    Some("'reset'"),
    Some("'reset-assertions'"),
    Some("'set-info'"),
    Some("'set-logic'"),
    Some("'set-option'"),
    Some("'!'"),
    Some("'_'"),
    Some("'as'"),
    Some("'BINARY'"),
    Some("'DECIMAL'"),
    Some("'exists'"),
    Some("'HEXADECIMAL'"),
    Some("'forall'"),
    Some("'let'"),
    Some("'match'"),
    Some("'NUMERAL'"),
    Some("'par'"),
    Some("'string'"),
    None,
    None,
    None,
    None,
    Some("':'"),
    Some("':all-statistics'"),
    Some("':assertion-stack-levels'"),
    Some("':authors'"),
    Some("':category'"),
    Some("':chainable'"),
    Some("':definition'"),
    Some("':diagnostic-output-channel'"),
    Some("':error-behavior'"),
    Some("':extensions'"),
    Some("':funs'"),
    Some("':funs-description'"),
    Some("':global-declarations'"),
    Some("':interactive-mode'"),
    Some("':language'"),
    Some("':left-assoc'"),
    Some("':license'"),
    Some("':named'"),
    Some("':name'"),
    Some("':notes'"),
    Some("':pattern'"),
    Some("':print-success'"),
    Some("':produce-assertions'"),
    Some("':produce-assignments'"),
    Some("':produce-models'"),
    Some("':produce-proofs'"),
    Some("':produce-unsat-assumptions'"),
    Some("':produce-unsat-cores'"),
    Some("':random-seed'"),
    Some("':reason-unknown'"),
    Some("':regular-output-channel'"),
    Some("':reproducible-resource-limit'"),
    Some("':right-assoc'"),
    Some("':smt-lib-version'"),
    Some("':sorts'"),
    Some("':sorts-description'"),
    Some("':source'"),
    Some("':status'"),
    Some("':theories'"),
    Some("':values'"),
    Some("':verbosity'"),
    Some("':version'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 114] = [
    None,
    Some("Comment"),
    Some("ParOpen"),
    Some("ParClose"),
    Some("Semicolon"),
    Some("String"),
    Some("QuotedSymbol"),
    Some("PS_Not"),
    Some("PS_Bool"),
    Some("PS_ContinuedExecution"),
    Some("PS_Error"),
    Some("PS_False"),
    Some("PS_ImmediateExit"),
    Some("PS_Incomplete"),
    Some("PS_Logic"),
    Some("PS_Memout"),
    Some("PS_Sat"),
    Some("PS_Success"),
    Some("PS_Theory"),
    Some("PS_True"),
    Some("PS_Unknown"),
    Some("PS_Unsupported"),
    Some("PS_Unsat"),
    Some("CMD_Assert"),
    Some("CMD_CheckSat"),
    Some("CMD_CheckSatAssuming"),
    Some("CMD_DeclareConst"),
    Some("CMD_DeclareDatatype"),
    Some("CMD_DeclareDatatypes"),
    Some("CMD_DeclareFun"),
    Some("CMD_DeclareSort"),
    Some("CMD_DefineFun"),
    Some("CMD_DefineFunRec"),
    Some("CMD_DefineFunsRec"),
    Some("CMD_DefineSort"),
    Some("CMD_Echo"),
    Some("CMD_Exit"),
    Some("CMD_GetAssertions"),
    Some("CMD_GetAssignment"),
    Some("CMD_GetInfo"),
    Some("CMD_GetModel"),
    Some("CMD_GetOption"),
    Some("CMD_GetProof"),
    Some("CMD_GetUnsatAssumptions"),
    Some("CMD_GetUnsatCore"),
    Some("CMD_GetValue"),
    Some("CMD_Pop"),
    Some("CMD_Push"),
    Some("CMD_Reset"),
    Some("CMD_ResetAssertions"),
    Some("CMD_SetInfo"),
    Some("CMD_SetLogic"),
    Some("CMD_SetOption"),
    Some("GRW_Exclamation"),
    Some("GRW_Underscore"),
    Some("GRW_As"),
    Some("GRW_Binary"),
    Some("GRW_Decimal"),
    Some("GRW_Exists"),
    Some("GRW_Hexadecimal"),
    Some("GRW_Forall"),
    Some("GRW_Let"),
    Some("GRW_Match"),
    Some("GRW_Numeral"),
    Some("GRW_Par"),
    Some("GRW_String"),
    Some("Numeral"),
    Some("Binary"),
    Some("HexDecimal"),
    Some("Decimal"),
    Some("Colon"),
    Some("PK_AllStatistics"),
    Some("PK_AssertionStackLevels"),
    Some("PK_Authors"),
    Some("PK_Category"),
    Some("PK_Chainable"),
    Some("PK_Definition"),
    Some("PK_DiagnosticOutputChannel"),
    Some("PK_ErrorBehaviour"),
    Some("PK_Extension"),
    Some("PK_Funs"),
    Some("PK_FunsDescription"),
    Some("PK_GlobalDeclarations"),
    Some("PK_InteractiveMode"),
    Some("PK_Language"),
    Some("PK_LeftAssoc"),
    Some("PK_License"),
    Some("PK_Named"),
    Some("PK_Name"),
    Some("PK_Notes"),
    Some("PK_Pattern"),
    Some("PK_PrintSuccess"),
    Some("PK_ProduceAssertions"),
    Some("PK_ProduceAssignments"),
    Some("PK_ProduceModels"),
    Some("PK_ProduceProofs"),
    Some("PK_ProduceUnsatAssumptions"),
    Some("PK_ProduceUnsatCores"),
    Some("PK_RandomSeed"),
    Some("PK_ReasonUnknown"),
    Some("PK_RegularOutputChannel"),
    Some("PK_ReproducibleResourceLimit"),
    Some("PK_RightAssoc"),
    Some("PK_SmtLibVersion"),
    Some("PK_Sorts"),
    Some("PK_SortsDescription"),
    Some("PK_Source"),
    Some("PK_Status"),
    Some("PK_Theories"),
    Some("PK_Values"),
    Some("PK_Verbosity"),
    Some("PK_Version"),
    Some("UndefinedSymbol"),
    Some("WS"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

type BaseParserType<'input, I> = BaseParser<
    'input,
    SMTLIBv2ParserExt,
    I,
    SMTLIBv2ParserContextType,
    dyn SMTLIBv2Listener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type SMTLIBv2TreeWalker<'input, 'a> =
    ParseTreeWalker<'input, 'a, SMTLIBv2ParserContextType, dyn SMTLIBv2Listener<'input> + 'a>;

/// Parser for SMTLIBv2 grammar
pub struct SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_serialized_atn() -> &'static str {
        _serializedATN
    }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
        antlr_rust::recognizer::check_version("0", "2");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(
                input,
                Arc::clone(&interpreter),
                SMTLIBv2ParserExt {},
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> SMTLIBv2Parser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> SMTLIBv2Parser<'input, I, DefaultErrorStrategy<'input, SMTLIBv2ParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for SMTLIBv2Parser
pub trait SMTLIBv2ParserContext<'input>:
    for<'x> Listenable<dyn SMTLIBv2Listener<'input> + 'x>
    + for<'x> Visitable<dyn SMTLIBv2Visitor<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = SMTLIBv2ParserContextType>
{
}

impl<'input, 'x, T> VisitableDyn<T> for dyn SMTLIBv2ParserContext<'input> + 'input
where
    T: SMTLIBv2Visitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn SMTLIBv2Visitor<'input> + 'x))
    }
}

impl<'input> SMTLIBv2ParserContext<'input> for TerminalNode<'input, SMTLIBv2ParserContextType> {}
impl<'input> SMTLIBv2ParserContext<'input> for ErrorNode<'input, SMTLIBv2ParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn SMTLIBv2ParserContext<'input> + 'input {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn SMTLIBv2Listener<'input> + 'input {}

pub struct SMTLIBv2ParserContextType;
antlr_rust::type_id! {SMTLIBv2ParserContextType}

impl<'input> ParserNodeType<'input> for SMTLIBv2ParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn SMTLIBv2ParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct SMTLIBv2ParserExt {}

impl SMTLIBv2ParserExt {}

impl<'input> TokenAware<'input> for SMTLIBv2ParserExt {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for SMTLIBv2ParserExt
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for SMTLIBv2ParserExt
{
    fn get_grammar_file_name(&self) -> &str {
        "SMTLIBv2.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
}
//------------------- start ----------------
pub type StartContextAll<'input> = StartContext<'input>;

pub type StartContext<'input> = BaseParserRuleContext<'input, StartContextExt<'input>>;

#[derive(Clone)]
pub struct StartContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for StartContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for StartContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_start(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_start(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for StartContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_start(self);
    }
}

impl<'input> CustomRuleContext<'input> for StartContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_start
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_start }
}
antlr_rust::type_id! {StartContextExt<'a>}

impl<'input> StartContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<StartContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            StartContextExt { ph: PhantomData },
        ))
    }
}

pub trait StartContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<StartContextExt<'input>>
{
    fn script(&self) -> Option<Rc<ScriptContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EOF, 0)
    }
}

impl<'input> StartContextAttrs<'input> for StartContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn start(&mut self) -> Result<Rc<StartContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = StartContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_start);
        let mut _localctx: Rc<StartContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule script*/
                recog.base.set_state(192);
                recog.script()?;

                recog.base.set_state(193);
                recog.base.match_token(EOF, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- response ----------------
pub type ResponseContextAll<'input> = ResponseContext<'input>;

pub type ResponseContext<'input> = BaseParserRuleContext<'input, ResponseContextExt<'input>>;

#[derive(Clone)]
pub struct ResponseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for ResponseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for ResponseContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_response(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_response(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for ResponseContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_response(self);
    }
}

impl<'input> CustomRuleContext<'input> for ResponseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_response
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_response }
}
antlr_rust::type_id! {ResponseContextExt<'a>}

impl<'input> ResponseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ResponseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ResponseContextExt { ph: PhantomData },
        ))
    }
}

pub trait ResponseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<ResponseContextExt<'input>>
{
    fn general_response(&self) -> Option<Rc<General_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EOF, 0)
    }
}

impl<'input> ResponseContextAttrs<'input> for ResponseContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn response(&mut self) -> Result<Rc<ResponseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ResponseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_response);
        let mut _localctx: Rc<ResponseContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule general_response*/
                recog.base.set_state(195);
                recog.general_response()?;

                recog.base.set_state(196);
                recog.base.match_token(EOF, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- generalReservedWord ----------------
pub type GeneralReservedWordContextAll<'input> = GeneralReservedWordContext<'input>;

pub type GeneralReservedWordContext<'input> =
    BaseParserRuleContext<'input, GeneralReservedWordContextExt<'input>>;

#[derive(Clone)]
pub struct GeneralReservedWordContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for GeneralReservedWordContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for GeneralReservedWordContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_generalReservedWord(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_generalReservedWord(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a>
    for GeneralReservedWordContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_generalReservedWord(self);
    }
}

impl<'input> CustomRuleContext<'input> for GeneralReservedWordContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_generalReservedWord
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_generalReservedWord }
}
antlr_rust::type_id! {GeneralReservedWordContextExt<'a>}

impl<'input> GeneralReservedWordContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<GeneralReservedWordContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            GeneralReservedWordContextExt { ph: PhantomData },
        ))
    }
}

pub trait GeneralReservedWordContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<GeneralReservedWordContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token GRW_Exclamation
    /// Returns `None` if there is no child corresponding to token GRW_Exclamation
    fn GRW_Exclamation(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Exclamation, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Underscore
    /// Returns `None` if there is no child corresponding to token GRW_Underscore
    fn GRW_Underscore(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Underscore, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_As
    /// Returns `None` if there is no child corresponding to token GRW_As
    fn GRW_As(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_As, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Binary
    /// Returns `None` if there is no child corresponding to token GRW_Binary
    fn GRW_Binary(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Binary, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Decimal
    /// Returns `None` if there is no child corresponding to token GRW_Decimal
    fn GRW_Decimal(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Decimal, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Exists
    /// Returns `None` if there is no child corresponding to token GRW_Exists
    fn GRW_Exists(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Exists, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Hexadecimal
    /// Returns `None` if there is no child corresponding to token GRW_Hexadecimal
    fn GRW_Hexadecimal(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Hexadecimal, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Forall
    /// Returns `None` if there is no child corresponding to token GRW_Forall
    fn GRW_Forall(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Forall, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Let
    /// Returns `None` if there is no child corresponding to token GRW_Let
    fn GRW_Let(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Let, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Match
    /// Returns `None` if there is no child corresponding to token GRW_Match
    fn GRW_Match(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Match, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Numeral
    /// Returns `None` if there is no child corresponding to token GRW_Numeral
    fn GRW_Numeral(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Numeral, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Par
    /// Returns `None` if there is no child corresponding to token GRW_Par
    fn GRW_Par(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Par, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_String
    /// Returns `None` if there is no child corresponding to token GRW_String
    fn GRW_String(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_String, 0)
    }
}

impl<'input> GeneralReservedWordContextAttrs<'input> for GeneralReservedWordContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn generalReservedWord(
        &mut self,
    ) -> Result<Rc<GeneralReservedWordContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            GeneralReservedWordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 4, RULE_generalReservedWord);
        let mut _localctx: Rc<GeneralReservedWordContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(198);
                _la = recog.base.input.la(1);
                if {
                    !(((_la - 53) & !0x3f) == 0
                        && ((1usize << (_la - 53))
                            & ((1usize << (GRW_Exclamation - 53))
                                | (1usize << (GRW_Underscore - 53))
                                | (1usize << (GRW_As - 53))
                                | (1usize << (GRW_Binary - 53))
                                | (1usize << (GRW_Decimal - 53))
                                | (1usize << (GRW_Exists - 53))
                                | (1usize << (GRW_Hexadecimal - 53))
                                | (1usize << (GRW_Forall - 53))
                                | (1usize << (GRW_Let - 53))
                                | (1usize << (GRW_Match - 53))
                                | (1usize << (GRW_Numeral - 53))
                                | (1usize << (GRW_Par - 53))
                                | (1usize << (GRW_String - 53))))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- simpleSymbol ----------------
pub type SimpleSymbolContextAll<'input> = SimpleSymbolContext<'input>;

pub type SimpleSymbolContext<'input> =
    BaseParserRuleContext<'input, SimpleSymbolContextExt<'input>>;

#[derive(Clone)]
pub struct SimpleSymbolContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for SimpleSymbolContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for SimpleSymbolContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_simpleSymbol(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_simpleSymbol(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for SimpleSymbolContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_simpleSymbol(self);
    }
}

impl<'input> CustomRuleContext<'input> for SimpleSymbolContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_simpleSymbol
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_simpleSymbol }
}
antlr_rust::type_id! {SimpleSymbolContextExt<'a>}

impl<'input> SimpleSymbolContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SimpleSymbolContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SimpleSymbolContextExt { ph: PhantomData },
        ))
    }
}

pub trait SimpleSymbolContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<SimpleSymbolContextExt<'input>>
{
    fn predefSymbol(&self) -> Option<Rc<PredefSymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token UndefinedSymbol
    /// Returns `None` if there is no child corresponding to token UndefinedSymbol
    fn UndefinedSymbol(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UndefinedSymbol, 0)
    }
}

impl<'input> SimpleSymbolContextAttrs<'input> for SimpleSymbolContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn simpleSymbol(&mut self) -> Result<Rc<SimpleSymbolContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = SimpleSymbolContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 6, RULE_simpleSymbol);
        let mut _localctx: Rc<SimpleSymbolContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(202);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                PS_Not
                | PS_Bool
                | PS_ContinuedExecution
                | PS_Error
                | PS_False
                | PS_ImmediateExit
                | PS_Incomplete
                | PS_Logic
                | PS_Memout
                | PS_Sat
                | PS_Success
                | PS_Theory
                | PS_True
                | PS_Unknown
                | PS_Unsupported
                | PS_Unsat => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule predefSymbol*/
                        recog.base.set_state(200);
                        recog.predefSymbol()?;
                    }
                }

                UndefinedSymbol => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(201);
                        recog
                            .base
                            .match_token(UndefinedSymbol, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- quotedSymbol ----------------
pub type QuotedSymbolContextAll<'input> = QuotedSymbolContext<'input>;

pub type QuotedSymbolContext<'input> =
    BaseParserRuleContext<'input, QuotedSymbolContextExt<'input>>;

#[derive(Clone)]
pub struct QuotedSymbolContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for QuotedSymbolContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for QuotedSymbolContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_quotedSymbol(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_quotedSymbol(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for QuotedSymbolContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_quotedSymbol(self);
    }
}

impl<'input> CustomRuleContext<'input> for QuotedSymbolContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_quotedSymbol
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_quotedSymbol }
}
antlr_rust::type_id! {QuotedSymbolContextExt<'a>}

impl<'input> QuotedSymbolContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<QuotedSymbolContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            QuotedSymbolContextExt { ph: PhantomData },
        ))
    }
}

pub trait QuotedSymbolContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<QuotedSymbolContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token QuotedSymbol
    /// Returns `None` if there is no child corresponding to token QuotedSymbol
    fn QuotedSymbol(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(QuotedSymbol, 0)
    }
}

impl<'input> QuotedSymbolContextAttrs<'input> for QuotedSymbolContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn quotedSymbol(&mut self) -> Result<Rc<QuotedSymbolContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = QuotedSymbolContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 8, RULE_quotedSymbol);
        let mut _localctx: Rc<QuotedSymbolContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(204);
                recog
                    .base
                    .match_token(QuotedSymbol, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- predefSymbol ----------------
pub type PredefSymbolContextAll<'input> = PredefSymbolContext<'input>;

pub type PredefSymbolContext<'input> =
    BaseParserRuleContext<'input, PredefSymbolContextExt<'input>>;

#[derive(Clone)]
pub struct PredefSymbolContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for PredefSymbolContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for PredefSymbolContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_predefSymbol(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_predefSymbol(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for PredefSymbolContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_predefSymbol(self);
    }
}

impl<'input> CustomRuleContext<'input> for PredefSymbolContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_predefSymbol
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_predefSymbol }
}
antlr_rust::type_id! {PredefSymbolContextExt<'a>}

impl<'input> PredefSymbolContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<PredefSymbolContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            PredefSymbolContextExt { ph: PhantomData },
        ))
    }
}

pub trait PredefSymbolContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<PredefSymbolContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PS_Not
    /// Returns `None` if there is no child corresponding to token PS_Not
    fn PS_Not(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Not, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Bool
    /// Returns `None` if there is no child corresponding to token PS_Bool
    fn PS_Bool(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Bool, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_ContinuedExecution
    /// Returns `None` if there is no child corresponding to token PS_ContinuedExecution
    fn PS_ContinuedExecution(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_ContinuedExecution, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Error
    /// Returns `None` if there is no child corresponding to token PS_Error
    fn PS_Error(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Error, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_False
    /// Returns `None` if there is no child corresponding to token PS_False
    fn PS_False(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_False, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_ImmediateExit
    /// Returns `None` if there is no child corresponding to token PS_ImmediateExit
    fn PS_ImmediateExit(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_ImmediateExit, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Incomplete
    /// Returns `None` if there is no child corresponding to token PS_Incomplete
    fn PS_Incomplete(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Incomplete, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Logic
    /// Returns `None` if there is no child corresponding to token PS_Logic
    fn PS_Logic(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Logic, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Memout
    /// Returns `None` if there is no child corresponding to token PS_Memout
    fn PS_Memout(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Memout, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Sat
    /// Returns `None` if there is no child corresponding to token PS_Sat
    fn PS_Sat(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Sat, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Success
    /// Returns `None` if there is no child corresponding to token PS_Success
    fn PS_Success(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Success, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Theory
    /// Returns `None` if there is no child corresponding to token PS_Theory
    fn PS_Theory(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Theory, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_True
    /// Returns `None` if there is no child corresponding to token PS_True
    fn PS_True(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_True, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Unknown
    /// Returns `None` if there is no child corresponding to token PS_Unknown
    fn PS_Unknown(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Unknown, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Unsupported
    /// Returns `None` if there is no child corresponding to token PS_Unsupported
    fn PS_Unsupported(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Unsupported, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Unsat
    /// Returns `None` if there is no child corresponding to token PS_Unsat
    fn PS_Unsat(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Unsat, 0)
    }
}

impl<'input> PredefSymbolContextAttrs<'input> for PredefSymbolContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn predefSymbol(&mut self) -> Result<Rc<PredefSymbolContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = PredefSymbolContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 10, RULE_predefSymbol);
        let mut _localctx: Rc<PredefSymbolContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(206);
                _la = recog.base.input.la(1);
                if {
                    !(((_la) & !0x3f) == 0
                        && ((1usize << _la)
                            & ((1usize << PS_Not)
                                | (1usize << PS_Bool)
                                | (1usize << PS_ContinuedExecution)
                                | (1usize << PS_Error)
                                | (1usize << PS_False)
                                | (1usize << PS_ImmediateExit)
                                | (1usize << PS_Incomplete)
                                | (1usize << PS_Logic)
                                | (1usize << PS_Memout)
                                | (1usize << PS_Sat)
                                | (1usize << PS_Success)
                                | (1usize << PS_Theory)
                                | (1usize << PS_True)
                                | (1usize << PS_Unknown)
                                | (1usize << PS_Unsupported)
                                | (1usize << PS_Unsat)))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- predefKeyword ----------------
pub type PredefKeywordContextAll<'input> = PredefKeywordContext<'input>;

pub type PredefKeywordContext<'input> =
    BaseParserRuleContext<'input, PredefKeywordContextExt<'input>>;

#[derive(Clone)]
pub struct PredefKeywordContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for PredefKeywordContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for PredefKeywordContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_predefKeyword(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_predefKeyword(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for PredefKeywordContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_predefKeyword(self);
    }
}

impl<'input> CustomRuleContext<'input> for PredefKeywordContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_predefKeyword
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_predefKeyword }
}
antlr_rust::type_id! {PredefKeywordContextExt<'a>}

impl<'input> PredefKeywordContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<PredefKeywordContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            PredefKeywordContextExt { ph: PhantomData },
        ))
    }
}

pub trait PredefKeywordContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<PredefKeywordContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PK_AllStatistics
    /// Returns `None` if there is no child corresponding to token PK_AllStatistics
    fn PK_AllStatistics(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_AllStatistics, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_AssertionStackLevels
    /// Returns `None` if there is no child corresponding to token PK_AssertionStackLevels
    fn PK_AssertionStackLevels(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_AssertionStackLevels, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Authors
    /// Returns `None` if there is no child corresponding to token PK_Authors
    fn PK_Authors(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Authors, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Category
    /// Returns `None` if there is no child corresponding to token PK_Category
    fn PK_Category(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Category, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Chainable
    /// Returns `None` if there is no child corresponding to token PK_Chainable
    fn PK_Chainable(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Chainable, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Definition
    /// Returns `None` if there is no child corresponding to token PK_Definition
    fn PK_Definition(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Definition, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_DiagnosticOutputChannel
    /// Returns `None` if there is no child corresponding to token PK_DiagnosticOutputChannel
    fn PK_DiagnosticOutputChannel(
        &self,
    ) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_DiagnosticOutputChannel, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ErrorBehaviour
    /// Returns `None` if there is no child corresponding to token PK_ErrorBehaviour
    fn PK_ErrorBehaviour(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ErrorBehaviour, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Extension
    /// Returns `None` if there is no child corresponding to token PK_Extension
    fn PK_Extension(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Extension, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Funs
    /// Returns `None` if there is no child corresponding to token PK_Funs
    fn PK_Funs(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Funs, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_FunsDescription
    /// Returns `None` if there is no child corresponding to token PK_FunsDescription
    fn PK_FunsDescription(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_FunsDescription, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_GlobalDeclarations
    /// Returns `None` if there is no child corresponding to token PK_GlobalDeclarations
    fn PK_GlobalDeclarations(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_GlobalDeclarations, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_InteractiveMode
    /// Returns `None` if there is no child corresponding to token PK_InteractiveMode
    fn PK_InteractiveMode(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_InteractiveMode, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Language
    /// Returns `None` if there is no child corresponding to token PK_Language
    fn PK_Language(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Language, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_LeftAssoc
    /// Returns `None` if there is no child corresponding to token PK_LeftAssoc
    fn PK_LeftAssoc(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_LeftAssoc, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_License
    /// Returns `None` if there is no child corresponding to token PK_License
    fn PK_License(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_License, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Named
    /// Returns `None` if there is no child corresponding to token PK_Named
    fn PK_Named(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Named, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Name
    /// Returns `None` if there is no child corresponding to token PK_Name
    fn PK_Name(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Name, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Notes
    /// Returns `None` if there is no child corresponding to token PK_Notes
    fn PK_Notes(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Notes, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Pattern
    /// Returns `None` if there is no child corresponding to token PK_Pattern
    fn PK_Pattern(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Pattern, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_PrintSuccess
    /// Returns `None` if there is no child corresponding to token PK_PrintSuccess
    fn PK_PrintSuccess(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_PrintSuccess, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ProduceAssertions
    /// Returns `None` if there is no child corresponding to token PK_ProduceAssertions
    fn PK_ProduceAssertions(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ProduceAssertions, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ProduceAssignments
    /// Returns `None` if there is no child corresponding to token PK_ProduceAssignments
    fn PK_ProduceAssignments(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ProduceAssignments, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ProduceModels
    /// Returns `None` if there is no child corresponding to token PK_ProduceModels
    fn PK_ProduceModels(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ProduceModels, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ProduceProofs
    /// Returns `None` if there is no child corresponding to token PK_ProduceProofs
    fn PK_ProduceProofs(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ProduceProofs, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ProduceUnsatAssumptions
    /// Returns `None` if there is no child corresponding to token PK_ProduceUnsatAssumptions
    fn PK_ProduceUnsatAssumptions(
        &self,
    ) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ProduceUnsatAssumptions, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ProduceUnsatCores
    /// Returns `None` if there is no child corresponding to token PK_ProduceUnsatCores
    fn PK_ProduceUnsatCores(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ProduceUnsatCores, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_RandomSeed
    /// Returns `None` if there is no child corresponding to token PK_RandomSeed
    fn PK_RandomSeed(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_RandomSeed, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ReasonUnknown
    /// Returns `None` if there is no child corresponding to token PK_ReasonUnknown
    fn PK_ReasonUnknown(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ReasonUnknown, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_RegularOutputChannel
    /// Returns `None` if there is no child corresponding to token PK_RegularOutputChannel
    fn PK_RegularOutputChannel(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_RegularOutputChannel, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ReproducibleResourceLimit
    /// Returns `None` if there is no child corresponding to token PK_ReproducibleResourceLimit
    fn PK_ReproducibleResourceLimit(
        &self,
    ) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ReproducibleResourceLimit, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_RightAssoc
    /// Returns `None` if there is no child corresponding to token PK_RightAssoc
    fn PK_RightAssoc(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_RightAssoc, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_SmtLibVersion
    /// Returns `None` if there is no child corresponding to token PK_SmtLibVersion
    fn PK_SmtLibVersion(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_SmtLibVersion, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Sorts
    /// Returns `None` if there is no child corresponding to token PK_Sorts
    fn PK_Sorts(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Sorts, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_SortsDescription
    /// Returns `None` if there is no child corresponding to token PK_SortsDescription
    fn PK_SortsDescription(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_SortsDescription, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Source
    /// Returns `None` if there is no child corresponding to token PK_Source
    fn PK_Source(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Source, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Status
    /// Returns `None` if there is no child corresponding to token PK_Status
    fn PK_Status(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Status, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Theories
    /// Returns `None` if there is no child corresponding to token PK_Theories
    fn PK_Theories(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Theories, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Values
    /// Returns `None` if there is no child corresponding to token PK_Values
    fn PK_Values(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Values, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Verbosity
    /// Returns `None` if there is no child corresponding to token PK_Verbosity
    fn PK_Verbosity(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Verbosity, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Version
    /// Returns `None` if there is no child corresponding to token PK_Version
    fn PK_Version(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Version, 0)
    }
}

impl<'input> PredefKeywordContextAttrs<'input> for PredefKeywordContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn predefKeyword(&mut self) -> Result<Rc<PredefKeywordContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            PredefKeywordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 12, RULE_predefKeyword);
        let mut _localctx: Rc<PredefKeywordContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(208);
                _la = recog.base.input.la(1);
                if {
                    !(((_la - 71) & !0x3f) == 0
                        && ((1usize << (_la - 71))
                            & ((1usize << (PK_AllStatistics - 71))
                                | (1usize << (PK_AssertionStackLevels - 71))
                                | (1usize << (PK_Authors - 71))
                                | (1usize << (PK_Category - 71))
                                | (1usize << (PK_Chainable - 71))
                                | (1usize << (PK_Definition - 71))
                                | (1usize << (PK_DiagnosticOutputChannel - 71))
                                | (1usize << (PK_ErrorBehaviour - 71))
                                | (1usize << (PK_Extension - 71))
                                | (1usize << (PK_Funs - 71))
                                | (1usize << (PK_FunsDescription - 71))
                                | (1usize << (PK_GlobalDeclarations - 71))
                                | (1usize << (PK_InteractiveMode - 71))
                                | (1usize << (PK_Language - 71))
                                | (1usize << (PK_LeftAssoc - 71))
                                | (1usize << (PK_License - 71))
                                | (1usize << (PK_Named - 71))
                                | (1usize << (PK_Name - 71))
                                | (1usize << (PK_Notes - 71))
                                | (1usize << (PK_Pattern - 71))
                                | (1usize << (PK_PrintSuccess - 71))
                                | (1usize << (PK_ProduceAssertions - 71))
                                | (1usize << (PK_ProduceAssignments - 71))
                                | (1usize << (PK_ProduceModels - 71))
                                | (1usize << (PK_ProduceProofs - 71))
                                | (1usize << (PK_ProduceUnsatAssumptions - 71))
                                | (1usize << (PK_ProduceUnsatCores - 71))
                                | (1usize << (PK_RandomSeed - 71))
                                | (1usize << (PK_ReasonUnknown - 71))
                                | (1usize << (PK_RegularOutputChannel - 71))
                                | (1usize << (PK_ReproducibleResourceLimit - 71))
                                | (1usize << (PK_RightAssoc - 71))
                                | (1usize << (PK_SmtLibVersion - 71))
                                | (1usize << (PK_Sorts - 71))
                                | (1usize << (PK_SortsDescription - 71))
                                | (1usize << (PK_Source - 71))
                                | (1usize << (PK_Status - 71))
                                | (1usize << (PK_Theories - 71))
                                | (1usize << (PK_Values - 71))
                                | (1usize << (PK_Verbosity - 71))
                                | (1usize << (PK_Version - 71))))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- symbol ----------------
pub type SymbolContextAll<'input> = SymbolContext<'input>;

pub type SymbolContext<'input> = BaseParserRuleContext<'input, SymbolContextExt<'input>>;

#[derive(Clone)]
pub struct SymbolContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for SymbolContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for SymbolContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_symbol(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_symbol(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for SymbolContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_symbol(self);
    }
}

impl<'input> CustomRuleContext<'input> for SymbolContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_symbol
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_symbol }
}
antlr_rust::type_id! {SymbolContextExt<'a>}

impl<'input> SymbolContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SymbolContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SymbolContextExt { ph: PhantomData },
        ))
    }
}

pub trait SymbolContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<SymbolContextExt<'input>>
{
    fn simpleSymbol(&self) -> Option<Rc<SimpleSymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn quotedSymbol(&self) -> Option<Rc<QuotedSymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> SymbolContextAttrs<'input> for SymbolContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn symbol(&mut self) -> Result<Rc<SymbolContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = SymbolContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_symbol);
        let mut _localctx: Rc<SymbolContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(212);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                PS_Not
                | PS_Bool
                | PS_ContinuedExecution
                | PS_Error
                | PS_False
                | PS_ImmediateExit
                | PS_Incomplete
                | PS_Logic
                | PS_Memout
                | PS_Sat
                | PS_Success
                | PS_Theory
                | PS_True
                | PS_Unknown
                | PS_Unsupported
                | PS_Unsat
                | UndefinedSymbol => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule simpleSymbol*/
                        recog.base.set_state(210);
                        recog.simpleSymbol()?;
                    }
                }

                QuotedSymbol => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule quotedSymbol*/
                        recog.base.set_state(211);
                        recog.quotedSymbol()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- numeral ----------------
pub type NumeralContextAll<'input> = NumeralContext<'input>;

pub type NumeralContext<'input> = BaseParserRuleContext<'input, NumeralContextExt<'input>>;

#[derive(Clone)]
pub struct NumeralContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for NumeralContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for NumeralContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_numeral(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_numeral(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for NumeralContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_numeral(self);
    }
}

impl<'input> CustomRuleContext<'input> for NumeralContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_numeral
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_numeral }
}
antlr_rust::type_id! {NumeralContextExt<'a>}

impl<'input> NumeralContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<NumeralContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            NumeralContextExt { ph: PhantomData },
        ))
    }
}

pub trait NumeralContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<NumeralContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Numeral
    /// Returns `None` if there is no child corresponding to token Numeral
    fn Numeral(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Numeral, 0)
    }
}

impl<'input> NumeralContextAttrs<'input> for NumeralContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn numeral(&mut self) -> Result<Rc<NumeralContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = NumeralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_numeral);
        let mut _localctx: Rc<NumeralContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(214);
                recog.base.match_token(Numeral, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- decimal ----------------
pub type DecimalContextAll<'input> = DecimalContext<'input>;

pub type DecimalContext<'input> = BaseParserRuleContext<'input, DecimalContextExt<'input>>;

#[derive(Clone)]
pub struct DecimalContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for DecimalContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for DecimalContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_decimal(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_decimal(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for DecimalContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_decimal(self);
    }
}

impl<'input> CustomRuleContext<'input> for DecimalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_decimal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_decimal }
}
antlr_rust::type_id! {DecimalContextExt<'a>}

impl<'input> DecimalContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<DecimalContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            DecimalContextExt { ph: PhantomData },
        ))
    }
}

pub trait DecimalContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<DecimalContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Decimal
    /// Returns `None` if there is no child corresponding to token Decimal
    fn Decimal(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Decimal, 0)
    }
}

impl<'input> DecimalContextAttrs<'input> for DecimalContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn decimal(&mut self) -> Result<Rc<DecimalContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = DecimalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_decimal);
        let mut _localctx: Rc<DecimalContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(216);
                recog.base.match_token(Decimal, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- hexadecimal ----------------
pub type HexadecimalContextAll<'input> = HexadecimalContext<'input>;

pub type HexadecimalContext<'input> = BaseParserRuleContext<'input, HexadecimalContextExt<'input>>;

#[derive(Clone)]
pub struct HexadecimalContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for HexadecimalContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for HexadecimalContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_hexadecimal(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_hexadecimal(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for HexadecimalContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_hexadecimal(self);
    }
}

impl<'input> CustomRuleContext<'input> for HexadecimalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_hexadecimal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_hexadecimal }
}
antlr_rust::type_id! {HexadecimalContextExt<'a>}

impl<'input> HexadecimalContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<HexadecimalContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            HexadecimalContextExt { ph: PhantomData },
        ))
    }
}

pub trait HexadecimalContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<HexadecimalContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token HexDecimal
    /// Returns `None` if there is no child corresponding to token HexDecimal
    fn HexDecimal(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(HexDecimal, 0)
    }
}

impl<'input> HexadecimalContextAttrs<'input> for HexadecimalContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn hexadecimal(&mut self) -> Result<Rc<HexadecimalContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = HexadecimalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 20, RULE_hexadecimal);
        let mut _localctx: Rc<HexadecimalContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(218);
                recog.base.match_token(HexDecimal, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- binary ----------------
pub type BinaryContextAll<'input> = BinaryContext<'input>;

pub type BinaryContext<'input> = BaseParserRuleContext<'input, BinaryContextExt<'input>>;

#[derive(Clone)]
pub struct BinaryContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for BinaryContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for BinaryContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_binary(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_binary(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for BinaryContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_binary(self);
    }
}

impl<'input> CustomRuleContext<'input> for BinaryContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_binary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_binary }
}
antlr_rust::type_id! {BinaryContextExt<'a>}

impl<'input> BinaryContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<BinaryContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            BinaryContextExt { ph: PhantomData },
        ))
    }
}

pub trait BinaryContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<BinaryContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Binary
    /// Returns `None` if there is no child corresponding to token Binary
    fn Binary(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Binary, 0)
    }
}

impl<'input> BinaryContextAttrs<'input> for BinaryContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn binary(&mut self) -> Result<Rc<BinaryContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = BinaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_binary);
        let mut _localctx: Rc<BinaryContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(220);
                recog.base.match_token(Binary, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- string ----------------
pub type StringContextAll<'input> = StringContext<'input>;

pub type StringContext<'input> = BaseParserRuleContext<'input, StringContextExt<'input>>;

#[derive(Clone)]
pub struct StringContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for StringContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for StringContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_string(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_string(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for StringContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_string(self);
    }
}

impl<'input> CustomRuleContext<'input> for StringContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_string
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_string }
}
antlr_rust::type_id! {StringContextExt<'a>}

impl<'input> StringContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<StringContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            StringContextExt { ph: PhantomData },
        ))
    }
}

pub trait StringContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<StringContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token String
    /// Returns `None` if there is no child corresponding to token String
    fn String(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(String, 0)
    }
}

impl<'input> StringContextAttrs<'input> for StringContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn string(&mut self) -> Result<Rc<StringContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = StringContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_string);
        let mut _localctx: Rc<StringContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(222);
                recog.base.match_token(String, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- keyword ----------------
pub type KeywordContextAll<'input> = KeywordContext<'input>;

pub type KeywordContext<'input> = BaseParserRuleContext<'input, KeywordContextExt<'input>>;

#[derive(Clone)]
pub struct KeywordContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for KeywordContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for KeywordContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_keyword(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_keyword(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for KeywordContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_keyword(self);
    }
}

impl<'input> CustomRuleContext<'input> for KeywordContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_keyword
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_keyword }
}
antlr_rust::type_id! {KeywordContextExt<'a>}

impl<'input> KeywordContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<KeywordContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            KeywordContextExt { ph: PhantomData },
        ))
    }
}

pub trait KeywordContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<KeywordContextExt<'input>>
{
    fn predefKeyword(&self) -> Option<Rc<PredefKeywordContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token Colon
    /// Returns `None` if there is no child corresponding to token Colon
    fn Colon(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Colon, 0)
    }
    fn simpleSymbol(&self) -> Option<Rc<SimpleSymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> KeywordContextAttrs<'input> for KeywordContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn keyword(&mut self) -> Result<Rc<KeywordContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = KeywordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_keyword);
        let mut _localctx: Rc<KeywordContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(227);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                PK_AllStatistics
                | PK_AssertionStackLevels
                | PK_Authors
                | PK_Category
                | PK_Chainable
                | PK_Definition
                | PK_DiagnosticOutputChannel
                | PK_ErrorBehaviour
                | PK_Extension
                | PK_Funs
                | PK_FunsDescription
                | PK_GlobalDeclarations
                | PK_InteractiveMode
                | PK_Language
                | PK_LeftAssoc
                | PK_License
                | PK_Named
                | PK_Name
                | PK_Notes
                | PK_Pattern
                | PK_PrintSuccess
                | PK_ProduceAssertions
                | PK_ProduceAssignments
                | PK_ProduceModels
                | PK_ProduceProofs
                | PK_ProduceUnsatAssumptions
                | PK_ProduceUnsatCores
                | PK_RandomSeed
                | PK_ReasonUnknown
                | PK_RegularOutputChannel
                | PK_ReproducibleResourceLimit
                | PK_RightAssoc
                | PK_SmtLibVersion
                | PK_Sorts
                | PK_SortsDescription
                | PK_Source
                | PK_Status
                | PK_Theories
                | PK_Values
                | PK_Verbosity
                | PK_Version => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule predefKeyword*/
                        recog.base.set_state(224);
                        recog.predefKeyword()?;
                    }
                }

                Colon => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(225);
                        recog.base.match_token(Colon, &mut recog.err_handler)?;

                        /*InvokeRule simpleSymbol*/
                        recog.base.set_state(226);
                        recog.simpleSymbol()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- spec_constant ----------------
pub type Spec_constantContextAll<'input> = Spec_constantContext<'input>;

pub type Spec_constantContext<'input> =
    BaseParserRuleContext<'input, Spec_constantContextExt<'input>>;

#[derive(Clone)]
pub struct Spec_constantContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Spec_constantContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Spec_constantContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_spec_constant(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_spec_constant(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Spec_constantContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_spec_constant(self);
    }
}

impl<'input> CustomRuleContext<'input> for Spec_constantContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_spec_constant
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_spec_constant }
}
antlr_rust::type_id! {Spec_constantContextExt<'a>}

impl<'input> Spec_constantContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Spec_constantContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Spec_constantContextExt { ph: PhantomData },
        ))
    }
}

pub trait Spec_constantContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Spec_constantContextExt<'input>>
{
    fn numeral(&self) -> Option<Rc<NumeralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn decimal(&self) -> Option<Rc<DecimalContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn hexadecimal(&self) -> Option<Rc<HexadecimalContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn binary(&self) -> Option<Rc<BinaryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn string(&self) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Spec_constantContextAttrs<'input> for Spec_constantContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn spec_constant(&mut self) -> Result<Rc<Spec_constantContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Spec_constantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 28, RULE_spec_constant);
        let mut _localctx: Rc<Spec_constantContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(234);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                Numeral => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule numeral*/
                        recog.base.set_state(229);
                        recog.numeral()?;
                    }
                }

                Decimal => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule decimal*/
                        recog.base.set_state(230);
                        recog.decimal()?;
                    }
                }

                HexDecimal => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule hexadecimal*/
                        recog.base.set_state(231);
                        recog.hexadecimal()?;
                    }
                }

                Binary => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        /*InvokeRule binary*/
                        recog.base.set_state(232);
                        recog.binary()?;
                    }
                }

                String => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        /*InvokeRule string*/
                        recog.base.set_state(233);
                        recog.string()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- s_expr ----------------
pub type S_exprContextAll<'input> = S_exprContext<'input>;

pub type S_exprContext<'input> = BaseParserRuleContext<'input, S_exprContextExt<'input>>;

#[derive(Clone)]
pub struct S_exprContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for S_exprContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for S_exprContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_s_expr(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_s_expr(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for S_exprContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_s_expr(self);
    }
}

impl<'input> CustomRuleContext<'input> for S_exprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_s_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_s_expr }
}
antlr_rust::type_id! {S_exprContextExt<'a>}

impl<'input> S_exprContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<S_exprContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            S_exprContextExt { ph: PhantomData },
        ))
    }
}

pub trait S_exprContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<S_exprContextExt<'input>>
{
    fn spec_constant(&self) -> Option<Rc<Spec_constantContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn symbol(&self) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn keyword(&self) -> Option<Rc<KeywordContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn s_expr_all(&self) -> Vec<Rc<S_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn s_expr(&self, i: usize) -> Option<Rc<S_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> S_exprContextAttrs<'input> for S_exprContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn s_expr(&mut self) -> Result<Rc<S_exprContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = S_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_s_expr);
        let mut _localctx: Rc<S_exprContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(247);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                String | Numeral | Binary | HexDecimal | Decimal => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule spec_constant*/
                        recog.base.set_state(236);
                        recog.spec_constant()?;
                    }
                }

                QuotedSymbol
                | PS_Not
                | PS_Bool
                | PS_ContinuedExecution
                | PS_Error
                | PS_False
                | PS_ImmediateExit
                | PS_Incomplete
                | PS_Logic
                | PS_Memout
                | PS_Sat
                | PS_Success
                | PS_Theory
                | PS_True
                | PS_Unknown
                | PS_Unsupported
                | PS_Unsat
                | UndefinedSymbol => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule symbol*/
                        recog.base.set_state(237);
                        recog.symbol()?;
                    }
                }

                Colon
                | PK_AllStatistics
                | PK_AssertionStackLevels
                | PK_Authors
                | PK_Category
                | PK_Chainable
                | PK_Definition
                | PK_DiagnosticOutputChannel
                | PK_ErrorBehaviour
                | PK_Extension
                | PK_Funs
                | PK_FunsDescription
                | PK_GlobalDeclarations
                | PK_InteractiveMode
                | PK_Language
                | PK_LeftAssoc
                | PK_License
                | PK_Named
                | PK_Name
                | PK_Notes
                | PK_Pattern
                | PK_PrintSuccess
                | PK_ProduceAssertions
                | PK_ProduceAssignments
                | PK_ProduceModels
                | PK_ProduceProofs
                | PK_ProduceUnsatAssumptions
                | PK_ProduceUnsatCores
                | PK_RandomSeed
                | PK_ReasonUnknown
                | PK_RegularOutputChannel
                | PK_ReproducibleResourceLimit
                | PK_RightAssoc
                | PK_SmtLibVersion
                | PK_Sorts
                | PK_SortsDescription
                | PK_Source
                | PK_Status
                | PK_Theories
                | PK_Values
                | PK_Verbosity
                | PK_Version => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule keyword*/
                        recog.base.set_state(238);
                        recog.keyword()?;
                    }
                }

                ParOpen => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(239);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(243);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while (((_la) & !0x3f) == 0
                            && ((1usize << _la)
                                & ((1usize << ParOpen)
                                    | (1usize << String)
                                    | (1usize << QuotedSymbol)
                                    | (1usize << PS_Not)
                                    | (1usize << PS_Bool)
                                    | (1usize << PS_ContinuedExecution)
                                    | (1usize << PS_Error)
                                    | (1usize << PS_False)
                                    | (1usize << PS_ImmediateExit)
                                    | (1usize << PS_Incomplete)
                                    | (1usize << PS_Logic)
                                    | (1usize << PS_Memout)
                                    | (1usize << PS_Sat)
                                    | (1usize << PS_Success)
                                    | (1usize << PS_Theory)
                                    | (1usize << PS_True)
                                    | (1usize << PS_Unknown)
                                    | (1usize << PS_Unsupported)
                                    | (1usize << PS_Unsat)))
                                != 0)
                            || (((_la - 66) & !0x3f) == 0
                                && ((1usize << (_la - 66))
                                    & ((1usize << (Numeral - 66))
                                        | (1usize << (Binary - 66))
                                        | (1usize << (HexDecimal - 66))
                                        | (1usize << (Decimal - 66))
                                        | (1usize << (Colon - 66))
                                        | (1usize << (PK_AllStatistics - 66))
                                        | (1usize << (PK_AssertionStackLevels - 66))
                                        | (1usize << (PK_Authors - 66))
                                        | (1usize << (PK_Category - 66))
                                        | (1usize << (PK_Chainable - 66))
                                        | (1usize << (PK_Definition - 66))
                                        | (1usize << (PK_DiagnosticOutputChannel - 66))
                                        | (1usize << (PK_ErrorBehaviour - 66))
                                        | (1usize << (PK_Extension - 66))
                                        | (1usize << (PK_Funs - 66))
                                        | (1usize << (PK_FunsDescription - 66))
                                        | (1usize << (PK_GlobalDeclarations - 66))
                                        | (1usize << (PK_InteractiveMode - 66))
                                        | (1usize << (PK_Language - 66))
                                        | (1usize << (PK_LeftAssoc - 66))
                                        | (1usize << (PK_License - 66))
                                        | (1usize << (PK_Named - 66))
                                        | (1usize << (PK_Name - 66))
                                        | (1usize << (PK_Notes - 66))
                                        | (1usize << (PK_Pattern - 66))
                                        | (1usize << (PK_PrintSuccess - 66))
                                        | (1usize << (PK_ProduceAssertions - 66))
                                        | (1usize << (PK_ProduceAssignments - 66))
                                        | (1usize << (PK_ProduceModels - 66))
                                        | (1usize << (PK_ProduceProofs - 66))
                                        | (1usize << (PK_ProduceUnsatAssumptions - 66))
                                        | (1usize << (PK_ProduceUnsatCores - 66))
                                        | (1usize << (PK_RandomSeed - 66))
                                        | (1usize << (PK_ReasonUnknown - 66))
                                        | (1usize << (PK_RegularOutputChannel - 66))
                                        | (1usize << (PK_ReproducibleResourceLimit - 66))
                                        | (1usize << (PK_RightAssoc - 66))
                                        | (1usize << (PK_SmtLibVersion - 66))
                                        | (1usize << (PK_Sorts - 66))
                                        | (1usize << (PK_SortsDescription - 66))
                                        | (1usize << (PK_Source - 66))
                                        | (1usize << (PK_Status - 66))
                                        | (1usize << (PK_Theories - 66))
                                        | (1usize << (PK_Values - 66))
                                        | (1usize << (PK_Verbosity - 66))
                                        | (1usize << (PK_Version - 66))
                                        | (1usize << (UndefinedSymbol - 66))))
                                    != 0)
                        {
                            {
                                {
                                    /*InvokeRule s_expr*/
                                    recog.base.set_state(240);
                                    recog.s_expr()?;
                                }
                            }
                            recog.base.set_state(245);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(246);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- index ----------------
pub type IndexContextAll<'input> = IndexContext<'input>;

pub type IndexContext<'input> = BaseParserRuleContext<'input, IndexContextExt<'input>>;

#[derive(Clone)]
pub struct IndexContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for IndexContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for IndexContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_index(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_index(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for IndexContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_index(self);
    }
}

impl<'input> CustomRuleContext<'input> for IndexContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_index
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_index }
}
antlr_rust::type_id! {IndexContextExt<'a>}

impl<'input> IndexContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<IndexContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            IndexContextExt { ph: PhantomData },
        ))
    }
}

pub trait IndexContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<IndexContextExt<'input>>
{
    fn numeral(&self) -> Option<Rc<NumeralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn symbol(&self) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> IndexContextAttrs<'input> for IndexContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn index(&mut self) -> Result<Rc<IndexContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = IndexContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_index);
        let mut _localctx: Rc<IndexContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(251);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                Numeral => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule numeral*/
                        recog.base.set_state(249);
                        recog.numeral()?;
                    }
                }

                QuotedSymbol
                | PS_Not
                | PS_Bool
                | PS_ContinuedExecution
                | PS_Error
                | PS_False
                | PS_ImmediateExit
                | PS_Incomplete
                | PS_Logic
                | PS_Memout
                | PS_Sat
                | PS_Success
                | PS_Theory
                | PS_True
                | PS_Unknown
                | PS_Unsupported
                | PS_Unsat
                | UndefinedSymbol => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule symbol*/
                        recog.base.set_state(250);
                        recog.symbol()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- identifier ----------------
pub type IdentifierContextAll<'input> = IdentifierContext<'input>;

pub type IdentifierContext<'input> = BaseParserRuleContext<'input, IdentifierContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifierContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for IdentifierContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for IdentifierContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_identifier(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_identifier(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for IdentifierContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_identifier(self);
    }
}

impl<'input> CustomRuleContext<'input> for IdentifierContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_identifier
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_identifier }
}
antlr_rust::type_id! {IdentifierContextExt<'a>}

impl<'input> IdentifierContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<IdentifierContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            IdentifierContextExt { ph: PhantomData },
        ))
    }
}

pub trait IdentifierContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<IdentifierContextExt<'input>>
{
    fn symbol(&self) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Underscore
    /// Returns `None` if there is no child corresponding to token GRW_Underscore
    fn GRW_Underscore(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Underscore, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn index_all(&self) -> Vec<Rc<IndexContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn index(&self, i: usize) -> Option<Rc<IndexContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> IdentifierContextAttrs<'input> for IdentifierContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn identifier(&mut self) -> Result<Rc<IdentifierContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = IdentifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 34, RULE_identifier);
        let mut _localctx: Rc<IdentifierContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(264);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                QuotedSymbol
                | PS_Not
                | PS_Bool
                | PS_ContinuedExecution
                | PS_Error
                | PS_False
                | PS_ImmediateExit
                | PS_Incomplete
                | PS_Logic
                | PS_Memout
                | PS_Sat
                | PS_Success
                | PS_Theory
                | PS_True
                | PS_Unknown
                | PS_Unsupported
                | PS_Unsat
                | UndefinedSymbol => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule symbol*/
                        recog.base.set_state(253);
                        recog.symbol()?;
                    }
                }

                ParOpen => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(254);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(255);
                        recog
                            .base
                            .match_token(GRW_Underscore, &mut recog.err_handler)?;

                        /*InvokeRule symbol*/
                        recog.base.set_state(256);
                        recog.symbol()?;

                        recog.base.set_state(258);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule index*/
                                    recog.base.set_state(257);
                                    recog.index()?;
                                }
                            }
                            recog.base.set_state(260);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !((((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << QuotedSymbol)
                                        | (1usize << PS_Not)
                                        | (1usize << PS_Bool)
                                        | (1usize << PS_ContinuedExecution)
                                        | (1usize << PS_Error)
                                        | (1usize << PS_False)
                                        | (1usize << PS_ImmediateExit)
                                        | (1usize << PS_Incomplete)
                                        | (1usize << PS_Logic)
                                        | (1usize << PS_Memout)
                                        | (1usize << PS_Sat)
                                        | (1usize << PS_Success)
                                        | (1usize << PS_Theory)
                                        | (1usize << PS_True)
                                        | (1usize << PS_Unknown)
                                        | (1usize << PS_Unsupported)
                                        | (1usize << PS_Unsat)))
                                    != 0)
                                || _la == Numeral
                                || _la == UndefinedSymbol)
                            {
                                break;
                            }
                        }
                        recog.base.set_state(262);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- attribute_value ----------------
pub type Attribute_valueContextAll<'input> = Attribute_valueContext<'input>;

pub type Attribute_valueContext<'input> =
    BaseParserRuleContext<'input, Attribute_valueContextExt<'input>>;

#[derive(Clone)]
pub struct Attribute_valueContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Attribute_valueContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Attribute_valueContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_attribute_value(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_attribute_value(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Attribute_valueContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_attribute_value(self);
    }
}

impl<'input> CustomRuleContext<'input> for Attribute_valueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_attribute_value
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_attribute_value }
}
antlr_rust::type_id! {Attribute_valueContextExt<'a>}

impl<'input> Attribute_valueContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Attribute_valueContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Attribute_valueContextExt { ph: PhantomData },
        ))
    }
}

pub trait Attribute_valueContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Attribute_valueContextExt<'input>>
{
    fn spec_constant(&self) -> Option<Rc<Spec_constantContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn symbol(&self) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn s_expr_all(&self) -> Vec<Rc<S_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn s_expr(&self, i: usize) -> Option<Rc<S_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Attribute_valueContextAttrs<'input> for Attribute_valueContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn attribute_value(&mut self) -> Result<Rc<Attribute_valueContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Attribute_valueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 36, RULE_attribute_value);
        let mut _localctx: Rc<Attribute_valueContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(276);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                String | Numeral | Binary | HexDecimal | Decimal => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule spec_constant*/
                        recog.base.set_state(266);
                        recog.spec_constant()?;
                    }
                }

                QuotedSymbol
                | PS_Not
                | PS_Bool
                | PS_ContinuedExecution
                | PS_Error
                | PS_False
                | PS_ImmediateExit
                | PS_Incomplete
                | PS_Logic
                | PS_Memout
                | PS_Sat
                | PS_Success
                | PS_Theory
                | PS_True
                | PS_Unknown
                | PS_Unsupported
                | PS_Unsat
                | UndefinedSymbol => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule symbol*/
                        recog.base.set_state(267);
                        recog.symbol()?;
                    }
                }

                ParOpen => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(268);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(272);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while (((_la) & !0x3f) == 0
                            && ((1usize << _la)
                                & ((1usize << ParOpen)
                                    | (1usize << String)
                                    | (1usize << QuotedSymbol)
                                    | (1usize << PS_Not)
                                    | (1usize << PS_Bool)
                                    | (1usize << PS_ContinuedExecution)
                                    | (1usize << PS_Error)
                                    | (1usize << PS_False)
                                    | (1usize << PS_ImmediateExit)
                                    | (1usize << PS_Incomplete)
                                    | (1usize << PS_Logic)
                                    | (1usize << PS_Memout)
                                    | (1usize << PS_Sat)
                                    | (1usize << PS_Success)
                                    | (1usize << PS_Theory)
                                    | (1usize << PS_True)
                                    | (1usize << PS_Unknown)
                                    | (1usize << PS_Unsupported)
                                    | (1usize << PS_Unsat)))
                                != 0)
                            || (((_la - 66) & !0x3f) == 0
                                && ((1usize << (_la - 66))
                                    & ((1usize << (Numeral - 66))
                                        | (1usize << (Binary - 66))
                                        | (1usize << (HexDecimal - 66))
                                        | (1usize << (Decimal - 66))
                                        | (1usize << (Colon - 66))
                                        | (1usize << (PK_AllStatistics - 66))
                                        | (1usize << (PK_AssertionStackLevels - 66))
                                        | (1usize << (PK_Authors - 66))
                                        | (1usize << (PK_Category - 66))
                                        | (1usize << (PK_Chainable - 66))
                                        | (1usize << (PK_Definition - 66))
                                        | (1usize << (PK_DiagnosticOutputChannel - 66))
                                        | (1usize << (PK_ErrorBehaviour - 66))
                                        | (1usize << (PK_Extension - 66))
                                        | (1usize << (PK_Funs - 66))
                                        | (1usize << (PK_FunsDescription - 66))
                                        | (1usize << (PK_GlobalDeclarations - 66))
                                        | (1usize << (PK_InteractiveMode - 66))
                                        | (1usize << (PK_Language - 66))
                                        | (1usize << (PK_LeftAssoc - 66))
                                        | (1usize << (PK_License - 66))
                                        | (1usize << (PK_Named - 66))
                                        | (1usize << (PK_Name - 66))
                                        | (1usize << (PK_Notes - 66))
                                        | (1usize << (PK_Pattern - 66))
                                        | (1usize << (PK_PrintSuccess - 66))
                                        | (1usize << (PK_ProduceAssertions - 66))
                                        | (1usize << (PK_ProduceAssignments - 66))
                                        | (1usize << (PK_ProduceModels - 66))
                                        | (1usize << (PK_ProduceProofs - 66))
                                        | (1usize << (PK_ProduceUnsatAssumptions - 66))
                                        | (1usize << (PK_ProduceUnsatCores - 66))
                                        | (1usize << (PK_RandomSeed - 66))
                                        | (1usize << (PK_ReasonUnknown - 66))
                                        | (1usize << (PK_RegularOutputChannel - 66))
                                        | (1usize << (PK_ReproducibleResourceLimit - 66))
                                        | (1usize << (PK_RightAssoc - 66))
                                        | (1usize << (PK_SmtLibVersion - 66))
                                        | (1usize << (PK_Sorts - 66))
                                        | (1usize << (PK_SortsDescription - 66))
                                        | (1usize << (PK_Source - 66))
                                        | (1usize << (PK_Status - 66))
                                        | (1usize << (PK_Theories - 66))
                                        | (1usize << (PK_Values - 66))
                                        | (1usize << (PK_Verbosity - 66))
                                        | (1usize << (PK_Version - 66))
                                        | (1usize << (UndefinedSymbol - 66))))
                                    != 0)
                        {
                            {
                                {
                                    /*InvokeRule s_expr*/
                                    recog.base.set_state(269);
                                    recog.s_expr()?;
                                }
                            }
                            recog.base.set_state(274);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(275);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- attribute ----------------
pub type AttributeContextAll<'input> = AttributeContext<'input>;

pub type AttributeContext<'input> = BaseParserRuleContext<'input, AttributeContextExt<'input>>;

#[derive(Clone)]
pub struct AttributeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for AttributeContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for AttributeContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_attribute(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_attribute(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for AttributeContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_attribute(self);
    }
}

impl<'input> CustomRuleContext<'input> for AttributeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_attribute
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_attribute }
}
antlr_rust::type_id! {AttributeContextExt<'a>}

impl<'input> AttributeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AttributeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            AttributeContextExt { ph: PhantomData },
        ))
    }
}

pub trait AttributeContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<AttributeContextExt<'input>>
{
    fn keyword(&self) -> Option<Rc<KeywordContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn attribute_value(&self) -> Option<Rc<Attribute_valueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> AttributeContextAttrs<'input> for AttributeContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn attribute(&mut self) -> Result<Rc<AttributeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AttributeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_attribute);
        let mut _localctx: Rc<AttributeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(282);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(11, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule keyword*/
                        recog.base.set_state(278);
                        recog.keyword()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule keyword*/
                        recog.base.set_state(279);
                        recog.keyword()?;

                        /*InvokeRule attribute_value*/
                        recog.base.set_state(280);
                        recog.attribute_value()?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- sort ----------------
pub type SortContextAll<'input> = SortContext<'input>;

pub type SortContext<'input> = BaseParserRuleContext<'input, SortContextExt<'input>>;

#[derive(Clone)]
pub struct SortContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for SortContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for SortContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_sort(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_sort(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for SortContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_sort(self);
    }
}

impl<'input> CustomRuleContext<'input> for SortContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_sort
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_sort }
}
antlr_rust::type_id! {SortContextExt<'a>}

impl<'input> SortContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SortContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SortContextExt { ph: PhantomData },
        ))
    }
}

pub trait SortContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<SortContextExt<'input>>
{
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn sort_all(&self) -> Vec<Rc<SortContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn sort(&self, i: usize) -> Option<Rc<SortContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> SortContextAttrs<'input> for SortContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn sort(&mut self) -> Result<Rc<SortContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = SortContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_sort);
        let mut _localctx: Rc<SortContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(294);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(13, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule identifier*/
                        recog.base.set_state(284);
                        recog.identifier()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(285);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule identifier*/
                        recog.base.set_state(286);
                        recog.identifier()?;

                        recog.base.set_state(288);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule sort*/
                                    recog.base.set_state(287);
                                    recog.sort()?;
                                }
                            }
                            recog.base.set_state(290);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !((((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << ParOpen)
                                        | (1usize << QuotedSymbol)
                                        | (1usize << PS_Not)
                                        | (1usize << PS_Bool)
                                        | (1usize << PS_ContinuedExecution)
                                        | (1usize << PS_Error)
                                        | (1usize << PS_False)
                                        | (1usize << PS_ImmediateExit)
                                        | (1usize << PS_Incomplete)
                                        | (1usize << PS_Logic)
                                        | (1usize << PS_Memout)
                                        | (1usize << PS_Sat)
                                        | (1usize << PS_Success)
                                        | (1usize << PS_Theory)
                                        | (1usize << PS_True)
                                        | (1usize << PS_Unknown)
                                        | (1usize << PS_Unsupported)
                                        | (1usize << PS_Unsat)))
                                    != 0)
                                || _la == UndefinedSymbol)
                            {
                                break;
                            }
                        }
                        recog.base.set_state(292);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- qual_identifier ----------------
pub type Qual_identifierContextAll<'input> = Qual_identifierContext<'input>;

pub type Qual_identifierContext<'input> =
    BaseParserRuleContext<'input, Qual_identifierContextExt<'input>>;

#[derive(Clone)]
pub struct Qual_identifierContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Qual_identifierContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Qual_identifierContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_qual_identifier(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_qual_identifier(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Qual_identifierContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_qual_identifier(self);
    }
}

impl<'input> CustomRuleContext<'input> for Qual_identifierContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_qual_identifier
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_qual_identifier }
}
antlr_rust::type_id! {Qual_identifierContextExt<'a>}

impl<'input> Qual_identifierContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Qual_identifierContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Qual_identifierContextExt { ph: PhantomData },
        ))
    }
}

pub trait Qual_identifierContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Qual_identifierContextExt<'input>>
{
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_As
    /// Returns `None` if there is no child corresponding to token GRW_As
    fn GRW_As(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_As, 0)
    }
    fn sort(&self) -> Option<Rc<SortContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
}

impl<'input> Qual_identifierContextAttrs<'input> for Qual_identifierContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn qual_identifier(&mut self) -> Result<Rc<Qual_identifierContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Qual_identifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 42, RULE_qual_identifier);
        let mut _localctx: Rc<Qual_identifierContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(303);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(14, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule identifier*/
                        recog.base.set_state(296);
                        recog.identifier()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(297);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(298);
                        recog.base.match_token(GRW_As, &mut recog.err_handler)?;

                        /*InvokeRule identifier*/
                        recog.base.set_state(299);
                        recog.identifier()?;

                        /*InvokeRule sort*/
                        recog.base.set_state(300);
                        recog.sort()?;

                        recog.base.set_state(301);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- var_binding ----------------
pub type Var_bindingContextAll<'input> = Var_bindingContext<'input>;

pub type Var_bindingContext<'input> = BaseParserRuleContext<'input, Var_bindingContextExt<'input>>;

#[derive(Clone)]
pub struct Var_bindingContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Var_bindingContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Var_bindingContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_var_binding(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_var_binding(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Var_bindingContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_var_binding(self);
    }
}

impl<'input> CustomRuleContext<'input> for Var_bindingContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_var_binding
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_var_binding }
}
antlr_rust::type_id! {Var_bindingContextExt<'a>}

impl<'input> Var_bindingContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Var_bindingContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Var_bindingContextExt { ph: PhantomData },
        ))
    }
}

pub trait Var_bindingContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Var_bindingContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    fn symbol(&self) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn term(&self) -> Option<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
}

impl<'input> Var_bindingContextAttrs<'input> for Var_bindingContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn var_binding(&mut self) -> Result<Rc<Var_bindingContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Var_bindingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 44, RULE_var_binding);
        let mut _localctx: Rc<Var_bindingContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(305);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                /*InvokeRule symbol*/
                recog.base.set_state(306);
                recog.symbol()?;

                /*InvokeRule term*/
                recog.base.set_state(307);
                recog.term()?;

                recog.base.set_state(308);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- sorted_var ----------------
pub type Sorted_varContextAll<'input> = Sorted_varContext<'input>;

pub type Sorted_varContext<'input> = BaseParserRuleContext<'input, Sorted_varContextExt<'input>>;

#[derive(Clone)]
pub struct Sorted_varContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Sorted_varContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Sorted_varContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_sorted_var(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_sorted_var(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Sorted_varContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_sorted_var(self);
    }
}

impl<'input> CustomRuleContext<'input> for Sorted_varContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_sorted_var
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_sorted_var }
}
antlr_rust::type_id! {Sorted_varContextExt<'a>}

impl<'input> Sorted_varContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Sorted_varContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Sorted_varContextExt { ph: PhantomData },
        ))
    }
}

pub trait Sorted_varContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Sorted_varContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    fn symbol(&self) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn sort(&self) -> Option<Rc<SortContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
}

impl<'input> Sorted_varContextAttrs<'input> for Sorted_varContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn sorted_var(&mut self) -> Result<Rc<Sorted_varContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Sorted_varContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 46, RULE_sorted_var);
        let mut _localctx: Rc<Sorted_varContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(310);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                /*InvokeRule symbol*/
                recog.base.set_state(311);
                recog.symbol()?;

                /*InvokeRule sort*/
                recog.base.set_state(312);
                recog.sort()?;

                recog.base.set_state(313);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- pattern ----------------
pub type PatternContextAll<'input> = PatternContext<'input>;

pub type PatternContext<'input> = BaseParserRuleContext<'input, PatternContextExt<'input>>;

#[derive(Clone)]
pub struct PatternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for PatternContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for PatternContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for PatternContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for PatternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_pattern }
}
antlr_rust::type_id! {PatternContextExt<'a>}

impl<'input> PatternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<PatternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            PatternContextExt { ph: PhantomData },
        ))
    }
}

pub trait PatternContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<PatternContextExt<'input>>
{
    fn symbol_all(&self) -> Vec<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn symbol(&self, i: usize) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
}

impl<'input> PatternContextAttrs<'input> for PatternContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn pattern(&mut self) -> Result<Rc<PatternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = PatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_pattern);
        let mut _localctx: Rc<PatternContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(325);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                QuotedSymbol
                | PS_Not
                | PS_Bool
                | PS_ContinuedExecution
                | PS_Error
                | PS_False
                | PS_ImmediateExit
                | PS_Incomplete
                | PS_Logic
                | PS_Memout
                | PS_Sat
                | PS_Success
                | PS_Theory
                | PS_True
                | PS_Unknown
                | PS_Unsupported
                | PS_Unsat
                | UndefinedSymbol => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule symbol*/
                        recog.base.set_state(315);
                        recog.symbol()?;
                    }
                }

                ParOpen => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(316);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule symbol*/
                        recog.base.set_state(317);
                        recog.symbol()?;

                        recog.base.set_state(319);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule symbol*/
                                    recog.base.set_state(318);
                                    recog.symbol()?;
                                }
                            }
                            recog.base.set_state(321);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !((((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << QuotedSymbol)
                                        | (1usize << PS_Not)
                                        | (1usize << PS_Bool)
                                        | (1usize << PS_ContinuedExecution)
                                        | (1usize << PS_Error)
                                        | (1usize << PS_False)
                                        | (1usize << PS_ImmediateExit)
                                        | (1usize << PS_Incomplete)
                                        | (1usize << PS_Logic)
                                        | (1usize << PS_Memout)
                                        | (1usize << PS_Sat)
                                        | (1usize << PS_Success)
                                        | (1usize << PS_Theory)
                                        | (1usize << PS_True)
                                        | (1usize << PS_Unknown)
                                        | (1usize << PS_Unsupported)
                                        | (1usize << PS_Unsat)))
                                    != 0)
                                || _la == UndefinedSymbol)
                            {
                                break;
                            }
                        }
                        recog.base.set_state(323);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- match_case ----------------
pub type Match_caseContextAll<'input> = Match_caseContext<'input>;

pub type Match_caseContext<'input> = BaseParserRuleContext<'input, Match_caseContextExt<'input>>;

#[derive(Clone)]
pub struct Match_caseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Match_caseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Match_caseContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_match_case(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_match_case(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Match_caseContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_match_case(self);
    }
}

impl<'input> CustomRuleContext<'input> for Match_caseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_match_case
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_match_case }
}
antlr_rust::type_id! {Match_caseContextExt<'a>}

impl<'input> Match_caseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Match_caseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Match_caseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Match_caseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Match_caseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn term(&self) -> Option<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
}

impl<'input> Match_caseContextAttrs<'input> for Match_caseContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn match_case(&mut self) -> Result<Rc<Match_caseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Match_caseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 50, RULE_match_case);
        let mut _localctx: Rc<Match_caseContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(327);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                /*InvokeRule pattern*/
                recog.base.set_state(328);
                recog.pattern()?;

                /*InvokeRule term*/
                recog.base.set_state(329);
                recog.term()?;

                recog.base.set_state(330);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- term ----------------
pub type TermContextAll<'input> = TermContext<'input>;

pub type TermContext<'input> = BaseParserRuleContext<'input, TermContextExt<'input>>;

#[derive(Clone)]
pub struct TermContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for TermContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for TermContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_term(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_term(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for TermContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_term(self);
    }
}

impl<'input> CustomRuleContext<'input> for TermContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_term
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_term }
}
antlr_rust::type_id! {TermContextExt<'a>}

impl<'input> TermContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TermContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TermContextExt { ph: PhantomData },
        ))
    }
}

pub trait TermContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<TermContextExt<'input>>
{
    fn spec_constant(&self) -> Option<Rc<Spec_constantContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn qual_identifier(&self) -> Option<Rc<Qual_identifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ParOpen in current rule
    fn ParOpen_all(&self) -> Vec<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ParOpen, starting from 0.
    /// Returns `None` if number of children corresponding to token ParOpen is less or equal than `i`.
    fn ParOpen(&self, i: usize) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ParClose in current rule
    fn ParClose_all(&self) -> Vec<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ParClose, starting from 0.
    /// Returns `None` if number of children corresponding to token ParClose is less or equal than `i`.
    fn ParClose(&self, i: usize) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, i)
    }
    fn term_all(&self) -> Vec<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Let
    /// Returns `None` if there is no child corresponding to token GRW_Let
    fn GRW_Let(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Let, 0)
    }
    fn var_binding_all(&self) -> Vec<Rc<Var_bindingContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn var_binding(&self, i: usize) -> Option<Rc<Var_bindingContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Forall
    /// Returns `None` if there is no child corresponding to token GRW_Forall
    fn GRW_Forall(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Forall, 0)
    }
    fn sorted_var_all(&self) -> Vec<Rc<Sorted_varContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn sorted_var(&self, i: usize) -> Option<Rc<Sorted_varContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Exists
    /// Returns `None` if there is no child corresponding to token GRW_Exists
    fn GRW_Exists(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Exists, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Match
    /// Returns `None` if there is no child corresponding to token GRW_Match
    fn GRW_Match(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Match, 0)
    }
    fn match_case_all(&self) -> Vec<Rc<Match_caseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn match_case(&self, i: usize) -> Option<Rc<Match_caseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Exclamation
    /// Returns `None` if there is no child corresponding to token GRW_Exclamation
    fn GRW_Exclamation(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Exclamation, 0)
    }
    fn attribute_all(&self) -> Vec<Rc<AttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn attribute(&self, i: usize) -> Option<Rc<AttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> TermContextAttrs<'input> for TermContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn term(&mut self) -> Result<Rc<TermContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = TermContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_term);
        let mut _localctx: Rc<TermContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(401);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(23, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule spec_constant*/
                        recog.base.set_state(332);
                        recog.spec_constant()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule qual_identifier*/
                        recog.base.set_state(333);
                        recog.qual_identifier()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(334);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule qual_identifier*/
                        recog.base.set_state(335);
                        recog.qual_identifier()?;

                        recog.base.set_state(337);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule term*/
                                    recog.base.set_state(336);
                                    recog.term()?;
                                }
                            }
                            recog.base.set_state(339);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !((((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << ParOpen)
                                        | (1usize << String)
                                        | (1usize << QuotedSymbol)
                                        | (1usize << PS_Not)
                                        | (1usize << PS_Bool)
                                        | (1usize << PS_ContinuedExecution)
                                        | (1usize << PS_Error)
                                        | (1usize << PS_False)
                                        | (1usize << PS_ImmediateExit)
                                        | (1usize << PS_Incomplete)
                                        | (1usize << PS_Logic)
                                        | (1usize << PS_Memout)
                                        | (1usize << PS_Sat)
                                        | (1usize << PS_Success)
                                        | (1usize << PS_Theory)
                                        | (1usize << PS_True)
                                        | (1usize << PS_Unknown)
                                        | (1usize << PS_Unsupported)
                                        | (1usize << PS_Unsat)))
                                    != 0)
                                || (((_la - 66) & !0x3f) == 0
                                    && ((1usize << (_la - 66))
                                        & ((1usize << (Numeral - 66))
                                            | (1usize << (Binary - 66))
                                            | (1usize << (HexDecimal - 66))
                                            | (1usize << (Decimal - 66))
                                            | (1usize << (UndefinedSymbol - 66))))
                                        != 0))
                            {
                                break;
                            }
                        }
                        recog.base.set_state(341);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(343);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(344);
                        recog.base.match_token(GRW_Let, &mut recog.err_handler)?;

                        recog.base.set_state(345);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(347);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule var_binding*/
                                    recog.base.set_state(346);
                                    recog.var_binding()?;
                                }
                            }
                            recog.base.set_state(349);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !(_la == ParOpen) {
                                break;
                            }
                        }
                        recog.base.set_state(351);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        /*InvokeRule term*/
                        recog.base.set_state(352);
                        recog.term()?;

                        recog.base.set_state(353);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        recog.base.set_state(355);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(356);
                        recog.base.match_token(GRW_Forall, &mut recog.err_handler)?;

                        recog.base.set_state(357);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(359);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule sorted_var*/
                                    recog.base.set_state(358);
                                    recog.sorted_var()?;
                                }
                            }
                            recog.base.set_state(361);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !(_la == ParOpen) {
                                break;
                            }
                        }
                        recog.base.set_state(363);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        /*InvokeRule term*/
                        recog.base.set_state(364);
                        recog.term()?;

                        recog.base.set_state(365);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        recog.base.set_state(367);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(368);
                        recog.base.match_token(GRW_Exists, &mut recog.err_handler)?;

                        recog.base.set_state(369);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(371);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule sorted_var*/
                                    recog.base.set_state(370);
                                    recog.sorted_var()?;
                                }
                            }
                            recog.base.set_state(373);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !(_la == ParOpen) {
                                break;
                            }
                        }
                        recog.base.set_state(375);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        /*InvokeRule term*/
                        recog.base.set_state(376);
                        recog.term()?;

                        recog.base.set_state(377);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        recog.base.set_state(379);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(380);
                        recog.base.match_token(GRW_Match, &mut recog.err_handler)?;

                        /*InvokeRule term*/
                        recog.base.set_state(381);
                        recog.term()?;

                        recog.base.set_state(382);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(384);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule match_case*/
                                    recog.base.set_state(383);
                                    recog.match_case()?;
                                }
                            }
                            recog.base.set_state(386);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !(_la == ParOpen) {
                                break;
                            }
                        }
                        recog.base.set_state(388);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        recog.base.set_state(389);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                8 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8);
                    recog.base.enter_outer_alt(None, 8);
                    {
                        recog.base.set_state(391);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(392);
                        recog
                            .base
                            .match_token(GRW_Exclamation, &mut recog.err_handler)?;

                        /*InvokeRule term*/
                        recog.base.set_state(393);
                        recog.term()?;

                        recog.base.set_state(395);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule attribute*/
                                    recog.base.set_state(394);
                                    recog.attribute()?;
                                }
                            }
                            recog.base.set_state(397);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !(((_la - 70) & !0x3f) == 0
                                && ((1usize << (_la - 70))
                                    & ((1usize << (Colon - 70))
                                        | (1usize << (PK_AllStatistics - 70))
                                        | (1usize << (PK_AssertionStackLevels - 70))
                                        | (1usize << (PK_Authors - 70))
                                        | (1usize << (PK_Category - 70))
                                        | (1usize << (PK_Chainable - 70))
                                        | (1usize << (PK_Definition - 70))
                                        | (1usize << (PK_DiagnosticOutputChannel - 70))
                                        | (1usize << (PK_ErrorBehaviour - 70))
                                        | (1usize << (PK_Extension - 70))
                                        | (1usize << (PK_Funs - 70))
                                        | (1usize << (PK_FunsDescription - 70))
                                        | (1usize << (PK_GlobalDeclarations - 70))
                                        | (1usize << (PK_InteractiveMode - 70))
                                        | (1usize << (PK_Language - 70))
                                        | (1usize << (PK_LeftAssoc - 70))
                                        | (1usize << (PK_License - 70))
                                        | (1usize << (PK_Named - 70))
                                        | (1usize << (PK_Name - 70))
                                        | (1usize << (PK_Notes - 70))
                                        | (1usize << (PK_Pattern - 70))
                                        | (1usize << (PK_PrintSuccess - 70))
                                        | (1usize << (PK_ProduceAssertions - 70))
                                        | (1usize << (PK_ProduceAssignments - 70))
                                        | (1usize << (PK_ProduceModels - 70))
                                        | (1usize << (PK_ProduceProofs - 70))
                                        | (1usize << (PK_ProduceUnsatAssumptions - 70))
                                        | (1usize << (PK_ProduceUnsatCores - 70))
                                        | (1usize << (PK_RandomSeed - 70))
                                        | (1usize << (PK_ReasonUnknown - 70))
                                        | (1usize << (PK_RegularOutputChannel - 70))
                                        | (1usize << (PK_ReproducibleResourceLimit - 70))
                                        | (1usize << (PK_RightAssoc - 70))
                                        | (1usize << (PK_SmtLibVersion - 70))
                                        | (1usize << (PK_Sorts - 70))
                                        | (1usize << (PK_SortsDescription - 70))
                                        | (1usize << (PK_Source - 70))
                                        | (1usize << (PK_Status - 70))
                                        | (1usize << (PK_Theories - 70))
                                        | (1usize << (PK_Values - 70))
                                        | (1usize << (PK_Verbosity - 70))
                                        | (1usize << (PK_Version - 70))))
                                    != 0)
                            {
                                break;
                            }
                        }
                        recog.base.set_state(399);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- sort_symbol_decl ----------------
pub type Sort_symbol_declContextAll<'input> = Sort_symbol_declContext<'input>;

pub type Sort_symbol_declContext<'input> =
    BaseParserRuleContext<'input, Sort_symbol_declContextExt<'input>>;

#[derive(Clone)]
pub struct Sort_symbol_declContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Sort_symbol_declContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Sort_symbol_declContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_sort_symbol_decl(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_sort_symbol_decl(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Sort_symbol_declContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_sort_symbol_decl(self);
    }
}

impl<'input> CustomRuleContext<'input> for Sort_symbol_declContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_sort_symbol_decl
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_sort_symbol_decl }
}
antlr_rust::type_id! {Sort_symbol_declContextExt<'a>}

impl<'input> Sort_symbol_declContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Sort_symbol_declContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Sort_symbol_declContextExt { ph: PhantomData },
        ))
    }
}

pub trait Sort_symbol_declContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Sort_symbol_declContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn numeral(&self) -> Option<Rc<NumeralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn attribute_all(&self) -> Vec<Rc<AttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn attribute(&self, i: usize) -> Option<Rc<AttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Sort_symbol_declContextAttrs<'input> for Sort_symbol_declContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn sort_symbol_decl(
        &mut self,
    ) -> Result<Rc<Sort_symbol_declContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Sort_symbol_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 54, RULE_sort_symbol_decl);
        let mut _localctx: Rc<Sort_symbol_declContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(403);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                /*InvokeRule identifier*/
                recog.base.set_state(404);
                recog.identifier()?;

                /*InvokeRule numeral*/
                recog.base.set_state(405);
                recog.numeral()?;

                recog.base.set_state(409);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la - 70) & !0x3f) == 0
                    && ((1usize << (_la - 70))
                        & ((1usize << (Colon - 70))
                            | (1usize << (PK_AllStatistics - 70))
                            | (1usize << (PK_AssertionStackLevels - 70))
                            | (1usize << (PK_Authors - 70))
                            | (1usize << (PK_Category - 70))
                            | (1usize << (PK_Chainable - 70))
                            | (1usize << (PK_Definition - 70))
                            | (1usize << (PK_DiagnosticOutputChannel - 70))
                            | (1usize << (PK_ErrorBehaviour - 70))
                            | (1usize << (PK_Extension - 70))
                            | (1usize << (PK_Funs - 70))
                            | (1usize << (PK_FunsDescription - 70))
                            | (1usize << (PK_GlobalDeclarations - 70))
                            | (1usize << (PK_InteractiveMode - 70))
                            | (1usize << (PK_Language - 70))
                            | (1usize << (PK_LeftAssoc - 70))
                            | (1usize << (PK_License - 70))
                            | (1usize << (PK_Named - 70))
                            | (1usize << (PK_Name - 70))
                            | (1usize << (PK_Notes - 70))
                            | (1usize << (PK_Pattern - 70))
                            | (1usize << (PK_PrintSuccess - 70))
                            | (1usize << (PK_ProduceAssertions - 70))
                            | (1usize << (PK_ProduceAssignments - 70))
                            | (1usize << (PK_ProduceModels - 70))
                            | (1usize << (PK_ProduceProofs - 70))
                            | (1usize << (PK_ProduceUnsatAssumptions - 70))
                            | (1usize << (PK_ProduceUnsatCores - 70))
                            | (1usize << (PK_RandomSeed - 70))
                            | (1usize << (PK_ReasonUnknown - 70))
                            | (1usize << (PK_RegularOutputChannel - 70))
                            | (1usize << (PK_ReproducibleResourceLimit - 70))
                            | (1usize << (PK_RightAssoc - 70))
                            | (1usize << (PK_SmtLibVersion - 70))
                            | (1usize << (PK_Sorts - 70))
                            | (1usize << (PK_SortsDescription - 70))
                            | (1usize << (PK_Source - 70))
                            | (1usize << (PK_Status - 70))
                            | (1usize << (PK_Theories - 70))
                            | (1usize << (PK_Values - 70))
                            | (1usize << (PK_Verbosity - 70))
                            | (1usize << (PK_Version - 70))))
                        != 0)
                {
                    {
                        {
                            /*InvokeRule attribute*/
                            recog.base.set_state(406);
                            recog.attribute()?;
                        }
                    }
                    recog.base.set_state(411);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(412);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- meta_spec_constant ----------------
pub type Meta_spec_constantContextAll<'input> = Meta_spec_constantContext<'input>;

pub type Meta_spec_constantContext<'input> =
    BaseParserRuleContext<'input, Meta_spec_constantContextExt<'input>>;

#[derive(Clone)]
pub struct Meta_spec_constantContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Meta_spec_constantContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Meta_spec_constantContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_meta_spec_constant(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_meta_spec_constant(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Meta_spec_constantContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_meta_spec_constant(self);
    }
}

impl<'input> CustomRuleContext<'input> for Meta_spec_constantContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_meta_spec_constant
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_meta_spec_constant }
}
antlr_rust::type_id! {Meta_spec_constantContextExt<'a>}

impl<'input> Meta_spec_constantContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Meta_spec_constantContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Meta_spec_constantContextExt { ph: PhantomData },
        ))
    }
}

pub trait Meta_spec_constantContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Meta_spec_constantContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token GRW_Numeral
    /// Returns `None` if there is no child corresponding to token GRW_Numeral
    fn GRW_Numeral(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Numeral, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Decimal
    /// Returns `None` if there is no child corresponding to token GRW_Decimal
    fn GRW_Decimal(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Decimal, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_String
    /// Returns `None` if there is no child corresponding to token GRW_String
    fn GRW_String(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_String, 0)
    }
}

impl<'input> Meta_spec_constantContextAttrs<'input> for Meta_spec_constantContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn meta_spec_constant(
        &mut self,
    ) -> Result<Rc<Meta_spec_constantContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Meta_spec_constantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 56, RULE_meta_spec_constant);
        let mut _localctx: Rc<Meta_spec_constantContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(414);
                _la = recog.base.input.la(1);
                if {
                    !(((_la - 57) & !0x3f) == 0
                        && ((1usize << (_la - 57))
                            & ((1usize << (GRW_Decimal - 57))
                                | (1usize << (GRW_Numeral - 57))
                                | (1usize << (GRW_String - 57))))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- fun_symbol_decl ----------------
pub type Fun_symbol_declContextAll<'input> = Fun_symbol_declContext<'input>;

pub type Fun_symbol_declContext<'input> =
    BaseParserRuleContext<'input, Fun_symbol_declContextExt<'input>>;

#[derive(Clone)]
pub struct Fun_symbol_declContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Fun_symbol_declContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Fun_symbol_declContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_fun_symbol_decl(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_fun_symbol_decl(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Fun_symbol_declContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_fun_symbol_decl(self);
    }
}

impl<'input> CustomRuleContext<'input> for Fun_symbol_declContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_fun_symbol_decl
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_fun_symbol_decl }
}
antlr_rust::type_id! {Fun_symbol_declContextExt<'a>}

impl<'input> Fun_symbol_declContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Fun_symbol_declContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Fun_symbol_declContextExt { ph: PhantomData },
        ))
    }
}

pub trait Fun_symbol_declContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Fun_symbol_declContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    fn spec_constant(&self) -> Option<Rc<Spec_constantContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn sort_all(&self) -> Vec<Rc<SortContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn sort(&self, i: usize) -> Option<Rc<SortContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn attribute_all(&self) -> Vec<Rc<AttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn attribute(&self, i: usize) -> Option<Rc<AttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn meta_spec_constant(&self) -> Option<Rc<Meta_spec_constantContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Fun_symbol_declContextAttrs<'input> for Fun_symbol_declContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn fun_symbol_decl(&mut self) -> Result<Rc<Fun_symbol_declContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Fun_symbol_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 58, RULE_fun_symbol_decl);
        let mut _localctx: Rc<Fun_symbol_declContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(453);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(29, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(416);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule spec_constant*/
                        recog.base.set_state(417);
                        recog.spec_constant()?;

                        /*InvokeRule sort*/
                        recog.base.set_state(418);
                        recog.sort()?;

                        recog.base.set_state(422);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while (((_la - 70) & !0x3f) == 0
                            && ((1usize << (_la - 70))
                                & ((1usize << (Colon - 70))
                                    | (1usize << (PK_AllStatistics - 70))
                                    | (1usize << (PK_AssertionStackLevels - 70))
                                    | (1usize << (PK_Authors - 70))
                                    | (1usize << (PK_Category - 70))
                                    | (1usize << (PK_Chainable - 70))
                                    | (1usize << (PK_Definition - 70))
                                    | (1usize << (PK_DiagnosticOutputChannel - 70))
                                    | (1usize << (PK_ErrorBehaviour - 70))
                                    | (1usize << (PK_Extension - 70))
                                    | (1usize << (PK_Funs - 70))
                                    | (1usize << (PK_FunsDescription - 70))
                                    | (1usize << (PK_GlobalDeclarations - 70))
                                    | (1usize << (PK_InteractiveMode - 70))
                                    | (1usize << (PK_Language - 70))
                                    | (1usize << (PK_LeftAssoc - 70))
                                    | (1usize << (PK_License - 70))
                                    | (1usize << (PK_Named - 70))
                                    | (1usize << (PK_Name - 70))
                                    | (1usize << (PK_Notes - 70))
                                    | (1usize << (PK_Pattern - 70))
                                    | (1usize << (PK_PrintSuccess - 70))
                                    | (1usize << (PK_ProduceAssertions - 70))
                                    | (1usize << (PK_ProduceAssignments - 70))
                                    | (1usize << (PK_ProduceModels - 70))
                                    | (1usize << (PK_ProduceProofs - 70))
                                    | (1usize << (PK_ProduceUnsatAssumptions - 70))
                                    | (1usize << (PK_ProduceUnsatCores - 70))
                                    | (1usize << (PK_RandomSeed - 70))
                                    | (1usize << (PK_ReasonUnknown - 70))
                                    | (1usize << (PK_RegularOutputChannel - 70))
                                    | (1usize << (PK_ReproducibleResourceLimit - 70))
                                    | (1usize << (PK_RightAssoc - 70))
                                    | (1usize << (PK_SmtLibVersion - 70))
                                    | (1usize << (PK_Sorts - 70))
                                    | (1usize << (PK_SortsDescription - 70))
                                    | (1usize << (PK_Source - 70))
                                    | (1usize << (PK_Status - 70))
                                    | (1usize << (PK_Theories - 70))
                                    | (1usize << (PK_Values - 70))
                                    | (1usize << (PK_Verbosity - 70))
                                    | (1usize << (PK_Version - 70))))
                                != 0)
                        {
                            {
                                {
                                    /*InvokeRule attribute*/
                                    recog.base.set_state(419);
                                    recog.attribute()?;
                                }
                            }
                            recog.base.set_state(424);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(425);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(427);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule meta_spec_constant*/
                        recog.base.set_state(428);
                        recog.meta_spec_constant()?;

                        /*InvokeRule sort*/
                        recog.base.set_state(429);
                        recog.sort()?;

                        recog.base.set_state(433);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while (((_la - 70) & !0x3f) == 0
                            && ((1usize << (_la - 70))
                                & ((1usize << (Colon - 70))
                                    | (1usize << (PK_AllStatistics - 70))
                                    | (1usize << (PK_AssertionStackLevels - 70))
                                    | (1usize << (PK_Authors - 70))
                                    | (1usize << (PK_Category - 70))
                                    | (1usize << (PK_Chainable - 70))
                                    | (1usize << (PK_Definition - 70))
                                    | (1usize << (PK_DiagnosticOutputChannel - 70))
                                    | (1usize << (PK_ErrorBehaviour - 70))
                                    | (1usize << (PK_Extension - 70))
                                    | (1usize << (PK_Funs - 70))
                                    | (1usize << (PK_FunsDescription - 70))
                                    | (1usize << (PK_GlobalDeclarations - 70))
                                    | (1usize << (PK_InteractiveMode - 70))
                                    | (1usize << (PK_Language - 70))
                                    | (1usize << (PK_LeftAssoc - 70))
                                    | (1usize << (PK_License - 70))
                                    | (1usize << (PK_Named - 70))
                                    | (1usize << (PK_Name - 70))
                                    | (1usize << (PK_Notes - 70))
                                    | (1usize << (PK_Pattern - 70))
                                    | (1usize << (PK_PrintSuccess - 70))
                                    | (1usize << (PK_ProduceAssertions - 70))
                                    | (1usize << (PK_ProduceAssignments - 70))
                                    | (1usize << (PK_ProduceModels - 70))
                                    | (1usize << (PK_ProduceProofs - 70))
                                    | (1usize << (PK_ProduceUnsatAssumptions - 70))
                                    | (1usize << (PK_ProduceUnsatCores - 70))
                                    | (1usize << (PK_RandomSeed - 70))
                                    | (1usize << (PK_ReasonUnknown - 70))
                                    | (1usize << (PK_RegularOutputChannel - 70))
                                    | (1usize << (PK_ReproducibleResourceLimit - 70))
                                    | (1usize << (PK_RightAssoc - 70))
                                    | (1usize << (PK_SmtLibVersion - 70))
                                    | (1usize << (PK_Sorts - 70))
                                    | (1usize << (PK_SortsDescription - 70))
                                    | (1usize << (PK_Source - 70))
                                    | (1usize << (PK_Status - 70))
                                    | (1usize << (PK_Theories - 70))
                                    | (1usize << (PK_Values - 70))
                                    | (1usize << (PK_Verbosity - 70))
                                    | (1usize << (PK_Version - 70))))
                                != 0)
                        {
                            {
                                {
                                    /*InvokeRule attribute*/
                                    recog.base.set_state(430);
                                    recog.attribute()?;
                                }
                            }
                            recog.base.set_state(435);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(436);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(438);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule identifier*/
                        recog.base.set_state(439);
                        recog.identifier()?;

                        recog.base.set_state(441);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule sort*/
                                    recog.base.set_state(440);
                                    recog.sort()?;
                                }
                            }
                            recog.base.set_state(443);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !((((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << ParOpen)
                                        | (1usize << QuotedSymbol)
                                        | (1usize << PS_Not)
                                        | (1usize << PS_Bool)
                                        | (1usize << PS_ContinuedExecution)
                                        | (1usize << PS_Error)
                                        | (1usize << PS_False)
                                        | (1usize << PS_ImmediateExit)
                                        | (1usize << PS_Incomplete)
                                        | (1usize << PS_Logic)
                                        | (1usize << PS_Memout)
                                        | (1usize << PS_Sat)
                                        | (1usize << PS_Success)
                                        | (1usize << PS_Theory)
                                        | (1usize << PS_True)
                                        | (1usize << PS_Unknown)
                                        | (1usize << PS_Unsupported)
                                        | (1usize << PS_Unsat)))
                                    != 0)
                                || _la == UndefinedSymbol)
                            {
                                break;
                            }
                        }
                        recog.base.set_state(448);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while (((_la - 70) & !0x3f) == 0
                            && ((1usize << (_la - 70))
                                & ((1usize << (Colon - 70))
                                    | (1usize << (PK_AllStatistics - 70))
                                    | (1usize << (PK_AssertionStackLevels - 70))
                                    | (1usize << (PK_Authors - 70))
                                    | (1usize << (PK_Category - 70))
                                    | (1usize << (PK_Chainable - 70))
                                    | (1usize << (PK_Definition - 70))
                                    | (1usize << (PK_DiagnosticOutputChannel - 70))
                                    | (1usize << (PK_ErrorBehaviour - 70))
                                    | (1usize << (PK_Extension - 70))
                                    | (1usize << (PK_Funs - 70))
                                    | (1usize << (PK_FunsDescription - 70))
                                    | (1usize << (PK_GlobalDeclarations - 70))
                                    | (1usize << (PK_InteractiveMode - 70))
                                    | (1usize << (PK_Language - 70))
                                    | (1usize << (PK_LeftAssoc - 70))
                                    | (1usize << (PK_License - 70))
                                    | (1usize << (PK_Named - 70))
                                    | (1usize << (PK_Name - 70))
                                    | (1usize << (PK_Notes - 70))
                                    | (1usize << (PK_Pattern - 70))
                                    | (1usize << (PK_PrintSuccess - 70))
                                    | (1usize << (PK_ProduceAssertions - 70))
                                    | (1usize << (PK_ProduceAssignments - 70))
                                    | (1usize << (PK_ProduceModels - 70))
                                    | (1usize << (PK_ProduceProofs - 70))
                                    | (1usize << (PK_ProduceUnsatAssumptions - 70))
                                    | (1usize << (PK_ProduceUnsatCores - 70))
                                    | (1usize << (PK_RandomSeed - 70))
                                    | (1usize << (PK_ReasonUnknown - 70))
                                    | (1usize << (PK_RegularOutputChannel - 70))
                                    | (1usize << (PK_ReproducibleResourceLimit - 70))
                                    | (1usize << (PK_RightAssoc - 70))
                                    | (1usize << (PK_SmtLibVersion - 70))
                                    | (1usize << (PK_Sorts - 70))
                                    | (1usize << (PK_SortsDescription - 70))
                                    | (1usize << (PK_Source - 70))
                                    | (1usize << (PK_Status - 70))
                                    | (1usize << (PK_Theories - 70))
                                    | (1usize << (PK_Values - 70))
                                    | (1usize << (PK_Verbosity - 70))
                                    | (1usize << (PK_Version - 70))))
                                != 0)
                        {
                            {
                                {
                                    /*InvokeRule attribute*/
                                    recog.base.set_state(445);
                                    recog.attribute()?;
                                }
                            }
                            recog.base.set_state(450);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(451);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- par_fun_symbol_decl ----------------
pub type Par_fun_symbol_declContextAll<'input> = Par_fun_symbol_declContext<'input>;

pub type Par_fun_symbol_declContext<'input> =
    BaseParserRuleContext<'input, Par_fun_symbol_declContextExt<'input>>;

#[derive(Clone)]
pub struct Par_fun_symbol_declContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Par_fun_symbol_declContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Par_fun_symbol_declContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_par_fun_symbol_decl(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_par_fun_symbol_decl(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a>
    for Par_fun_symbol_declContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_par_fun_symbol_decl(self);
    }
}

impl<'input> CustomRuleContext<'input> for Par_fun_symbol_declContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_par_fun_symbol_decl
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_par_fun_symbol_decl }
}
antlr_rust::type_id! {Par_fun_symbol_declContextExt<'a>}

impl<'input> Par_fun_symbol_declContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Par_fun_symbol_declContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Par_fun_symbol_declContextExt { ph: PhantomData },
        ))
    }
}

pub trait Par_fun_symbol_declContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Par_fun_symbol_declContextExt<'input>>
{
    fn fun_symbol_decl(&self) -> Option<Rc<Fun_symbol_declContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ParOpen in current rule
    fn ParOpen_all(&self) -> Vec<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ParOpen, starting from 0.
    /// Returns `None` if number of children corresponding to token ParOpen is less or equal than `i`.
    fn ParOpen(&self, i: usize) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, i)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Par
    /// Returns `None` if there is no child corresponding to token GRW_Par
    fn GRW_Par(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Par, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ParClose in current rule
    fn ParClose_all(&self) -> Vec<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ParClose, starting from 0.
    /// Returns `None` if number of children corresponding to token ParClose is less or equal than `i`.
    fn ParClose(&self, i: usize) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, i)
    }
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn symbol_all(&self) -> Vec<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn symbol(&self, i: usize) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn sort_all(&self) -> Vec<Rc<SortContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn sort(&self, i: usize) -> Option<Rc<SortContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn attribute_all(&self) -> Vec<Rc<AttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn attribute(&self, i: usize) -> Option<Rc<AttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Par_fun_symbol_declContextAttrs<'input> for Par_fun_symbol_declContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn par_fun_symbol_decl(
        &mut self,
    ) -> Result<Rc<Par_fun_symbol_declContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Par_fun_symbol_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 60, RULE_par_fun_symbol_decl);
        let mut _localctx: Rc<Par_fun_symbol_declContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(481);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(33, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule fun_symbol_decl*/
                        recog.base.set_state(455);
                        recog.fun_symbol_decl()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(456);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(457);
                        recog.base.match_token(GRW_Par, &mut recog.err_handler)?;

                        recog.base.set_state(458);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(460);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule symbol*/
                                    recog.base.set_state(459);
                                    recog.symbol()?;
                                }
                            }
                            recog.base.set_state(462);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !((((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << QuotedSymbol)
                                        | (1usize << PS_Not)
                                        | (1usize << PS_Bool)
                                        | (1usize << PS_ContinuedExecution)
                                        | (1usize << PS_Error)
                                        | (1usize << PS_False)
                                        | (1usize << PS_ImmediateExit)
                                        | (1usize << PS_Incomplete)
                                        | (1usize << PS_Logic)
                                        | (1usize << PS_Memout)
                                        | (1usize << PS_Sat)
                                        | (1usize << PS_Success)
                                        | (1usize << PS_Theory)
                                        | (1usize << PS_True)
                                        | (1usize << PS_Unknown)
                                        | (1usize << PS_Unsupported)
                                        | (1usize << PS_Unsat)))
                                    != 0)
                                || _la == UndefinedSymbol)
                            {
                                break;
                            }
                        }
                        recog.base.set_state(464);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        recog.base.set_state(465);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule identifier*/
                        recog.base.set_state(466);
                        recog.identifier()?;

                        recog.base.set_state(468);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule sort*/
                                    recog.base.set_state(467);
                                    recog.sort()?;
                                }
                            }
                            recog.base.set_state(470);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !((((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << ParOpen)
                                        | (1usize << QuotedSymbol)
                                        | (1usize << PS_Not)
                                        | (1usize << PS_Bool)
                                        | (1usize << PS_ContinuedExecution)
                                        | (1usize << PS_Error)
                                        | (1usize << PS_False)
                                        | (1usize << PS_ImmediateExit)
                                        | (1usize << PS_Incomplete)
                                        | (1usize << PS_Logic)
                                        | (1usize << PS_Memout)
                                        | (1usize << PS_Sat)
                                        | (1usize << PS_Success)
                                        | (1usize << PS_Theory)
                                        | (1usize << PS_True)
                                        | (1usize << PS_Unknown)
                                        | (1usize << PS_Unsupported)
                                        | (1usize << PS_Unsat)))
                                    != 0)
                                || _la == UndefinedSymbol)
                            {
                                break;
                            }
                        }
                        recog.base.set_state(475);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while (((_la - 70) & !0x3f) == 0
                            && ((1usize << (_la - 70))
                                & ((1usize << (Colon - 70))
                                    | (1usize << (PK_AllStatistics - 70))
                                    | (1usize << (PK_AssertionStackLevels - 70))
                                    | (1usize << (PK_Authors - 70))
                                    | (1usize << (PK_Category - 70))
                                    | (1usize << (PK_Chainable - 70))
                                    | (1usize << (PK_Definition - 70))
                                    | (1usize << (PK_DiagnosticOutputChannel - 70))
                                    | (1usize << (PK_ErrorBehaviour - 70))
                                    | (1usize << (PK_Extension - 70))
                                    | (1usize << (PK_Funs - 70))
                                    | (1usize << (PK_FunsDescription - 70))
                                    | (1usize << (PK_GlobalDeclarations - 70))
                                    | (1usize << (PK_InteractiveMode - 70))
                                    | (1usize << (PK_Language - 70))
                                    | (1usize << (PK_LeftAssoc - 70))
                                    | (1usize << (PK_License - 70))
                                    | (1usize << (PK_Named - 70))
                                    | (1usize << (PK_Name - 70))
                                    | (1usize << (PK_Notes - 70))
                                    | (1usize << (PK_Pattern - 70))
                                    | (1usize << (PK_PrintSuccess - 70))
                                    | (1usize << (PK_ProduceAssertions - 70))
                                    | (1usize << (PK_ProduceAssignments - 70))
                                    | (1usize << (PK_ProduceModels - 70))
                                    | (1usize << (PK_ProduceProofs - 70))
                                    | (1usize << (PK_ProduceUnsatAssumptions - 70))
                                    | (1usize << (PK_ProduceUnsatCores - 70))
                                    | (1usize << (PK_RandomSeed - 70))
                                    | (1usize << (PK_ReasonUnknown - 70))
                                    | (1usize << (PK_RegularOutputChannel - 70))
                                    | (1usize << (PK_ReproducibleResourceLimit - 70))
                                    | (1usize << (PK_RightAssoc - 70))
                                    | (1usize << (PK_SmtLibVersion - 70))
                                    | (1usize << (PK_Sorts - 70))
                                    | (1usize << (PK_SortsDescription - 70))
                                    | (1usize << (PK_Source - 70))
                                    | (1usize << (PK_Status - 70))
                                    | (1usize << (PK_Theories - 70))
                                    | (1usize << (PK_Values - 70))
                                    | (1usize << (PK_Verbosity - 70))
                                    | (1usize << (PK_Version - 70))))
                                != 0)
                        {
                            {
                                {
                                    /*InvokeRule attribute*/
                                    recog.base.set_state(472);
                                    recog.attribute()?;
                                }
                            }
                            recog.base.set_state(477);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(478);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        recog.base.set_state(479);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- theory_attribute ----------------
pub type Theory_attributeContextAll<'input> = Theory_attributeContext<'input>;

pub type Theory_attributeContext<'input> =
    BaseParserRuleContext<'input, Theory_attributeContextExt<'input>>;

#[derive(Clone)]
pub struct Theory_attributeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Theory_attributeContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Theory_attributeContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_theory_attribute(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_theory_attribute(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Theory_attributeContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_theory_attribute(self);
    }
}

impl<'input> CustomRuleContext<'input> for Theory_attributeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_theory_attribute
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_theory_attribute }
}
antlr_rust::type_id! {Theory_attributeContextExt<'a>}

impl<'input> Theory_attributeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Theory_attributeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Theory_attributeContextExt { ph: PhantomData },
        ))
    }
}

pub trait Theory_attributeContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Theory_attributeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PK_Sorts
    /// Returns `None` if there is no child corresponding to token PK_Sorts
    fn PK_Sorts(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Sorts, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn sort_symbol_decl_all(&self) -> Vec<Rc<Sort_symbol_declContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn sort_symbol_decl(&self, i: usize) -> Option<Rc<Sort_symbol_declContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Funs
    /// Returns `None` if there is no child corresponding to token PK_Funs
    fn PK_Funs(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Funs, 0)
    }
    fn par_fun_symbol_decl_all(&self) -> Vec<Rc<Par_fun_symbol_declContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn par_fun_symbol_decl(&self, i: usize) -> Option<Rc<Par_fun_symbol_declContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token PK_SortsDescription
    /// Returns `None` if there is no child corresponding to token PK_SortsDescription
    fn PK_SortsDescription(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_SortsDescription, 0)
    }
    fn string(&self) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_FunsDescription
    /// Returns `None` if there is no child corresponding to token PK_FunsDescription
    fn PK_FunsDescription(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_FunsDescription, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Definition
    /// Returns `None` if there is no child corresponding to token PK_Definition
    fn PK_Definition(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Definition, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Values
    /// Returns `None` if there is no child corresponding to token PK_Values
    fn PK_Values(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Values, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Notes
    /// Returns `None` if there is no child corresponding to token PK_Notes
    fn PK_Notes(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Notes, 0)
    }
    fn attribute(&self) -> Option<Rc<AttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Theory_attributeContextAttrs<'input> for Theory_attributeContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn theory_attribute(
        &mut self,
    ) -> Result<Rc<Theory_attributeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Theory_attributeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 62, RULE_theory_attribute);
        let mut _localctx: Rc<Theory_attributeContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(512);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(36, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(483);
                        recog.base.match_token(PK_Sorts, &mut recog.err_handler)?;

                        recog.base.set_state(484);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(486);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule sort_symbol_decl*/
                                    recog.base.set_state(485);
                                    recog.sort_symbol_decl()?;
                                }
                            }
                            recog.base.set_state(488);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !(_la == ParOpen) {
                                break;
                            }
                        }
                        recog.base.set_state(490);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(492);
                        recog.base.match_token(PK_Funs, &mut recog.err_handler)?;

                        recog.base.set_state(493);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(495);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule par_fun_symbol_decl*/
                                    recog.base.set_state(494);
                                    recog.par_fun_symbol_decl()?;
                                }
                            }
                            recog.base.set_state(497);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !(_la == ParOpen) {
                                break;
                            }
                        }
                        recog.base.set_state(499);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(501);
                        recog
                            .base
                            .match_token(PK_SortsDescription, &mut recog.err_handler)?;

                        /*InvokeRule string*/
                        recog.base.set_state(502);
                        recog.string()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(503);
                        recog
                            .base
                            .match_token(PK_FunsDescription, &mut recog.err_handler)?;

                        /*InvokeRule string*/
                        recog.base.set_state(504);
                        recog.string()?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        recog.base.set_state(505);
                        recog
                            .base
                            .match_token(PK_Definition, &mut recog.err_handler)?;

                        /*InvokeRule string*/
                        recog.base.set_state(506);
                        recog.string()?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        recog.base.set_state(507);
                        recog.base.match_token(PK_Values, &mut recog.err_handler)?;

                        /*InvokeRule string*/
                        recog.base.set_state(508);
                        recog.string()?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        recog.base.set_state(509);
                        recog.base.match_token(PK_Notes, &mut recog.err_handler)?;

                        /*InvokeRule string*/
                        recog.base.set_state(510);
                        recog.string()?;
                    }
                }
                8 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8);
                    recog.base.enter_outer_alt(None, 8);
                    {
                        /*InvokeRule attribute*/
                        recog.base.set_state(511);
                        recog.attribute()?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- theory_decl ----------------
pub type Theory_declContextAll<'input> = Theory_declContext<'input>;

pub type Theory_declContext<'input> = BaseParserRuleContext<'input, Theory_declContextExt<'input>>;

#[derive(Clone)]
pub struct Theory_declContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Theory_declContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Theory_declContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_theory_decl(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_theory_decl(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Theory_declContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_theory_decl(self);
    }
}

impl<'input> CustomRuleContext<'input> for Theory_declContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_theory_decl
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_theory_decl }
}
antlr_rust::type_id! {Theory_declContextExt<'a>}

impl<'input> Theory_declContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Theory_declContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Theory_declContextExt { ph: PhantomData },
        ))
    }
}

pub trait Theory_declContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Theory_declContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Theory
    /// Returns `None` if there is no child corresponding to token PS_Theory
    fn PS_Theory(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Theory, 0)
    }
    fn symbol(&self) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn theory_attribute_all(&self) -> Vec<Rc<Theory_attributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn theory_attribute(&self, i: usize) -> Option<Rc<Theory_attributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Theory_declContextAttrs<'input> for Theory_declContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn theory_decl(&mut self) -> Result<Rc<Theory_declContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Theory_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 64, RULE_theory_decl);
        let mut _localctx: Rc<Theory_declContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(514);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                recog.base.set_state(515);
                recog.base.match_token(PS_Theory, &mut recog.err_handler)?;

                /*InvokeRule symbol*/
                recog.base.set_state(516);
                recog.symbol()?;

                recog.base.set_state(518);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule theory_attribute*/
                            recog.base.set_state(517);
                            recog.theory_attribute()?;
                        }
                    }
                    recog.base.set_state(520);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(((_la - 70) & !0x3f) == 0
                        && ((1usize << (_la - 70))
                            & ((1usize << (Colon - 70))
                                | (1usize << (PK_AllStatistics - 70))
                                | (1usize << (PK_AssertionStackLevels - 70))
                                | (1usize << (PK_Authors - 70))
                                | (1usize << (PK_Category - 70))
                                | (1usize << (PK_Chainable - 70))
                                | (1usize << (PK_Definition - 70))
                                | (1usize << (PK_DiagnosticOutputChannel - 70))
                                | (1usize << (PK_ErrorBehaviour - 70))
                                | (1usize << (PK_Extension - 70))
                                | (1usize << (PK_Funs - 70))
                                | (1usize << (PK_FunsDescription - 70))
                                | (1usize << (PK_GlobalDeclarations - 70))
                                | (1usize << (PK_InteractiveMode - 70))
                                | (1usize << (PK_Language - 70))
                                | (1usize << (PK_LeftAssoc - 70))
                                | (1usize << (PK_License - 70))
                                | (1usize << (PK_Named - 70))
                                | (1usize << (PK_Name - 70))
                                | (1usize << (PK_Notes - 70))
                                | (1usize << (PK_Pattern - 70))
                                | (1usize << (PK_PrintSuccess - 70))
                                | (1usize << (PK_ProduceAssertions - 70))
                                | (1usize << (PK_ProduceAssignments - 70))
                                | (1usize << (PK_ProduceModels - 70))
                                | (1usize << (PK_ProduceProofs - 70))
                                | (1usize << (PK_ProduceUnsatAssumptions - 70))
                                | (1usize << (PK_ProduceUnsatCores - 70))
                                | (1usize << (PK_RandomSeed - 70))
                                | (1usize << (PK_ReasonUnknown - 70))
                                | (1usize << (PK_RegularOutputChannel - 70))
                                | (1usize << (PK_ReproducibleResourceLimit - 70))
                                | (1usize << (PK_RightAssoc - 70))
                                | (1usize << (PK_SmtLibVersion - 70))
                                | (1usize << (PK_Sorts - 70))
                                | (1usize << (PK_SortsDescription - 70))
                                | (1usize << (PK_Source - 70))
                                | (1usize << (PK_Status - 70))
                                | (1usize << (PK_Theories - 70))
                                | (1usize << (PK_Values - 70))
                                | (1usize << (PK_Verbosity - 70))
                                | (1usize << (PK_Version - 70))))
                            != 0)
                    {
                        break;
                    }
                }
                recog.base.set_state(522);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- logic_attribue ----------------
pub type Logic_attribueContextAll<'input> = Logic_attribueContext<'input>;

pub type Logic_attribueContext<'input> =
    BaseParserRuleContext<'input, Logic_attribueContextExt<'input>>;

#[derive(Clone)]
pub struct Logic_attribueContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Logic_attribueContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Logic_attribueContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_logic_attribue(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_logic_attribue(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Logic_attribueContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_logic_attribue(self);
    }
}

impl<'input> CustomRuleContext<'input> for Logic_attribueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_logic_attribue
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_logic_attribue }
}
antlr_rust::type_id! {Logic_attribueContextExt<'a>}

impl<'input> Logic_attribueContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Logic_attribueContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Logic_attribueContextExt { ph: PhantomData },
        ))
    }
}

pub trait Logic_attribueContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Logic_attribueContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PK_Theories
    /// Returns `None` if there is no child corresponding to token PK_Theories
    fn PK_Theories(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Theories, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn symbol_all(&self) -> Vec<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn symbol(&self, i: usize) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Language
    /// Returns `None` if there is no child corresponding to token PK_Language
    fn PK_Language(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Language, 0)
    }
    fn string(&self) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Extension
    /// Returns `None` if there is no child corresponding to token PK_Extension
    fn PK_Extension(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Extension, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Values
    /// Returns `None` if there is no child corresponding to token PK_Values
    fn PK_Values(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Values, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Notes
    /// Returns `None` if there is no child corresponding to token PK_Notes
    fn PK_Notes(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Notes, 0)
    }
    fn attribute(&self) -> Option<Rc<AttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Logic_attribueContextAttrs<'input> for Logic_attribueContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn logic_attribue(&mut self) -> Result<Rc<Logic_attribueContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Logic_attribueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 66, RULE_logic_attribue);
        let mut _localctx: Rc<Logic_attribueContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(542);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(39, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(524);
                        recog
                            .base
                            .match_token(PK_Theories, &mut recog.err_handler)?;

                        recog.base.set_state(525);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(527);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule symbol*/
                                    recog.base.set_state(526);
                                    recog.symbol()?;
                                }
                            }
                            recog.base.set_state(529);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !((((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << QuotedSymbol)
                                        | (1usize << PS_Not)
                                        | (1usize << PS_Bool)
                                        | (1usize << PS_ContinuedExecution)
                                        | (1usize << PS_Error)
                                        | (1usize << PS_False)
                                        | (1usize << PS_ImmediateExit)
                                        | (1usize << PS_Incomplete)
                                        | (1usize << PS_Logic)
                                        | (1usize << PS_Memout)
                                        | (1usize << PS_Sat)
                                        | (1usize << PS_Success)
                                        | (1usize << PS_Theory)
                                        | (1usize << PS_True)
                                        | (1usize << PS_Unknown)
                                        | (1usize << PS_Unsupported)
                                        | (1usize << PS_Unsat)))
                                    != 0)
                                || _la == UndefinedSymbol)
                            {
                                break;
                            }
                        }
                        recog.base.set_state(531);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(533);
                        recog
                            .base
                            .match_token(PK_Language, &mut recog.err_handler)?;

                        /*InvokeRule string*/
                        recog.base.set_state(534);
                        recog.string()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(535);
                        recog
                            .base
                            .match_token(PK_Extension, &mut recog.err_handler)?;

                        /*InvokeRule string*/
                        recog.base.set_state(536);
                        recog.string()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(537);
                        recog.base.match_token(PK_Values, &mut recog.err_handler)?;

                        /*InvokeRule string*/
                        recog.base.set_state(538);
                        recog.string()?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        recog.base.set_state(539);
                        recog.base.match_token(PK_Notes, &mut recog.err_handler)?;

                        /*InvokeRule string*/
                        recog.base.set_state(540);
                        recog.string()?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        /*InvokeRule attribute*/
                        recog.base.set_state(541);
                        recog.attribute()?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- logic ----------------
pub type LogicContextAll<'input> = LogicContext<'input>;

pub type LogicContext<'input> = BaseParserRuleContext<'input, LogicContextExt<'input>>;

#[derive(Clone)]
pub struct LogicContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for LogicContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for LogicContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_logic(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_logic(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for LogicContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_logic(self);
    }
}

impl<'input> CustomRuleContext<'input> for LogicContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_logic
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_logic }
}
antlr_rust::type_id! {LogicContextExt<'a>}

impl<'input> LogicContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<LogicContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            LogicContextExt { ph: PhantomData },
        ))
    }
}

pub trait LogicContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<LogicContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Logic
    /// Returns `None` if there is no child corresponding to token PS_Logic
    fn PS_Logic(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Logic, 0)
    }
    fn symbol(&self) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn logic_attribue_all(&self) -> Vec<Rc<Logic_attribueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn logic_attribue(&self, i: usize) -> Option<Rc<Logic_attribueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> LogicContextAttrs<'input> for LogicContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn logic(&mut self) -> Result<Rc<LogicContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = LogicContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_logic);
        let mut _localctx: Rc<LogicContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(544);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                recog.base.set_state(545);
                recog.base.match_token(PS_Logic, &mut recog.err_handler)?;

                /*InvokeRule symbol*/
                recog.base.set_state(546);
                recog.symbol()?;

                recog.base.set_state(548);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule logic_attribue*/
                            recog.base.set_state(547);
                            recog.logic_attribue()?;
                        }
                    }
                    recog.base.set_state(550);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(((_la - 70) & !0x3f) == 0
                        && ((1usize << (_la - 70))
                            & ((1usize << (Colon - 70))
                                | (1usize << (PK_AllStatistics - 70))
                                | (1usize << (PK_AssertionStackLevels - 70))
                                | (1usize << (PK_Authors - 70))
                                | (1usize << (PK_Category - 70))
                                | (1usize << (PK_Chainable - 70))
                                | (1usize << (PK_Definition - 70))
                                | (1usize << (PK_DiagnosticOutputChannel - 70))
                                | (1usize << (PK_ErrorBehaviour - 70))
                                | (1usize << (PK_Extension - 70))
                                | (1usize << (PK_Funs - 70))
                                | (1usize << (PK_FunsDescription - 70))
                                | (1usize << (PK_GlobalDeclarations - 70))
                                | (1usize << (PK_InteractiveMode - 70))
                                | (1usize << (PK_Language - 70))
                                | (1usize << (PK_LeftAssoc - 70))
                                | (1usize << (PK_License - 70))
                                | (1usize << (PK_Named - 70))
                                | (1usize << (PK_Name - 70))
                                | (1usize << (PK_Notes - 70))
                                | (1usize << (PK_Pattern - 70))
                                | (1usize << (PK_PrintSuccess - 70))
                                | (1usize << (PK_ProduceAssertions - 70))
                                | (1usize << (PK_ProduceAssignments - 70))
                                | (1usize << (PK_ProduceModels - 70))
                                | (1usize << (PK_ProduceProofs - 70))
                                | (1usize << (PK_ProduceUnsatAssumptions - 70))
                                | (1usize << (PK_ProduceUnsatCores - 70))
                                | (1usize << (PK_RandomSeed - 70))
                                | (1usize << (PK_ReasonUnknown - 70))
                                | (1usize << (PK_RegularOutputChannel - 70))
                                | (1usize << (PK_ReproducibleResourceLimit - 70))
                                | (1usize << (PK_RightAssoc - 70))
                                | (1usize << (PK_SmtLibVersion - 70))
                                | (1usize << (PK_Sorts - 70))
                                | (1usize << (PK_SortsDescription - 70))
                                | (1usize << (PK_Source - 70))
                                | (1usize << (PK_Status - 70))
                                | (1usize << (PK_Theories - 70))
                                | (1usize << (PK_Values - 70))
                                | (1usize << (PK_Verbosity - 70))
                                | (1usize << (PK_Version - 70))))
                            != 0)
                    {
                        break;
                    }
                }
                recog.base.set_state(552);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- sort_dec ----------------
pub type Sort_decContextAll<'input> = Sort_decContext<'input>;

pub type Sort_decContext<'input> = BaseParserRuleContext<'input, Sort_decContextExt<'input>>;

#[derive(Clone)]
pub struct Sort_decContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Sort_decContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Sort_decContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_sort_dec(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_sort_dec(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Sort_decContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_sort_dec(self);
    }
}

impl<'input> CustomRuleContext<'input> for Sort_decContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_sort_dec
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_sort_dec }
}
antlr_rust::type_id! {Sort_decContextExt<'a>}

impl<'input> Sort_decContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Sort_decContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Sort_decContextExt { ph: PhantomData },
        ))
    }
}

pub trait Sort_decContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Sort_decContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    fn symbol(&self) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn numeral(&self) -> Option<Rc<NumeralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
}

impl<'input> Sort_decContextAttrs<'input> for Sort_decContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn sort_dec(&mut self) -> Result<Rc<Sort_decContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Sort_decContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_sort_dec);
        let mut _localctx: Rc<Sort_decContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(554);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                /*InvokeRule symbol*/
                recog.base.set_state(555);
                recog.symbol()?;

                /*InvokeRule numeral*/
                recog.base.set_state(556);
                recog.numeral()?;

                recog.base.set_state(557);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- selector_dec ----------------
pub type Selector_decContextAll<'input> = Selector_decContext<'input>;

pub type Selector_decContext<'input> =
    BaseParserRuleContext<'input, Selector_decContextExt<'input>>;

#[derive(Clone)]
pub struct Selector_decContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Selector_decContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Selector_decContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_selector_dec(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_selector_dec(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Selector_decContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_selector_dec(self);
    }
}

impl<'input> CustomRuleContext<'input> for Selector_decContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_selector_dec
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_selector_dec }
}
antlr_rust::type_id! {Selector_decContextExt<'a>}

impl<'input> Selector_decContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Selector_decContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Selector_decContextExt { ph: PhantomData },
        ))
    }
}

pub trait Selector_decContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Selector_decContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    fn symbol(&self) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn sort(&self) -> Option<Rc<SortContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
}

impl<'input> Selector_decContextAttrs<'input> for Selector_decContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn selector_dec(&mut self) -> Result<Rc<Selector_decContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Selector_decContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 72, RULE_selector_dec);
        let mut _localctx: Rc<Selector_decContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(559);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                /*InvokeRule symbol*/
                recog.base.set_state(560);
                recog.symbol()?;

                /*InvokeRule sort*/
                recog.base.set_state(561);
                recog.sort()?;

                recog.base.set_state(562);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- constructor_dec ----------------
pub type Constructor_decContextAll<'input> = Constructor_decContext<'input>;

pub type Constructor_decContext<'input> =
    BaseParserRuleContext<'input, Constructor_decContextExt<'input>>;

#[derive(Clone)]
pub struct Constructor_decContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Constructor_decContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Constructor_decContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_constructor_dec(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_constructor_dec(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Constructor_decContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_constructor_dec(self);
    }
}

impl<'input> CustomRuleContext<'input> for Constructor_decContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_constructor_dec
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_constructor_dec }
}
antlr_rust::type_id! {Constructor_decContextExt<'a>}

impl<'input> Constructor_decContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Constructor_decContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Constructor_decContextExt { ph: PhantomData },
        ))
    }
}

pub trait Constructor_decContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Constructor_decContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    fn symbol(&self) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn selector_dec_all(&self) -> Vec<Rc<Selector_decContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn selector_dec(&self, i: usize) -> Option<Rc<Selector_decContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Constructor_decContextAttrs<'input> for Constructor_decContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn constructor_dec(&mut self) -> Result<Rc<Constructor_decContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Constructor_decContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 74, RULE_constructor_dec);
        let mut _localctx: Rc<Constructor_decContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(564);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                /*InvokeRule symbol*/
                recog.base.set_state(565);
                recog.symbol()?;

                recog.base.set_state(569);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == ParOpen {
                    {
                        {
                            /*InvokeRule selector_dec*/
                            recog.base.set_state(566);
                            recog.selector_dec()?;
                        }
                    }
                    recog.base.set_state(571);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(572);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- datatype_dec ----------------
pub type Datatype_decContextAll<'input> = Datatype_decContext<'input>;

pub type Datatype_decContext<'input> =
    BaseParserRuleContext<'input, Datatype_decContextExt<'input>>;

#[derive(Clone)]
pub struct Datatype_decContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Datatype_decContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Datatype_decContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_datatype_dec(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_datatype_dec(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Datatype_decContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_datatype_dec(self);
    }
}

impl<'input> CustomRuleContext<'input> for Datatype_decContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_datatype_dec
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_datatype_dec }
}
antlr_rust::type_id! {Datatype_decContextExt<'a>}

impl<'input> Datatype_decContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Datatype_decContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Datatype_decContextExt { ph: PhantomData },
        ))
    }
}

pub trait Datatype_decContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Datatype_decContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token ParOpen in current rule
    fn ParOpen_all(&self) -> Vec<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ParOpen, starting from 0.
    /// Returns `None` if number of children corresponding to token ParOpen is less or equal than `i`.
    fn ParOpen(&self, i: usize) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ParClose in current rule
    fn ParClose_all(&self) -> Vec<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ParClose, starting from 0.
    /// Returns `None` if number of children corresponding to token ParClose is less or equal than `i`.
    fn ParClose(&self, i: usize) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, i)
    }
    fn constructor_dec_all(&self) -> Vec<Rc<Constructor_decContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn constructor_dec(&self, i: usize) -> Option<Rc<Constructor_decContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token GRW_Par
    /// Returns `None` if there is no child corresponding to token GRW_Par
    fn GRW_Par(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GRW_Par, 0)
    }
    fn symbol_all(&self) -> Vec<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn symbol(&self, i: usize) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Datatype_decContextAttrs<'input> for Datatype_decContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn datatype_dec(&mut self) -> Result<Rc<Datatype_decContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Datatype_decContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 76, RULE_datatype_dec);
        let mut _localctx: Rc<Datatype_decContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(600);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(45, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(574);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(576);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule constructor_dec*/
                                    recog.base.set_state(575);
                                    recog.constructor_dec()?;
                                }
                            }
                            recog.base.set_state(578);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !(_la == ParOpen) {
                                break;
                            }
                        }
                        recog.base.set_state(580);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(582);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(583);
                        recog.base.match_token(GRW_Par, &mut recog.err_handler)?;

                        recog.base.set_state(584);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(586);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule symbol*/
                                    recog.base.set_state(585);
                                    recog.symbol()?;
                                }
                            }
                            recog.base.set_state(588);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !((((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << QuotedSymbol)
                                        | (1usize << PS_Not)
                                        | (1usize << PS_Bool)
                                        | (1usize << PS_ContinuedExecution)
                                        | (1usize << PS_Error)
                                        | (1usize << PS_False)
                                        | (1usize << PS_ImmediateExit)
                                        | (1usize << PS_Incomplete)
                                        | (1usize << PS_Logic)
                                        | (1usize << PS_Memout)
                                        | (1usize << PS_Sat)
                                        | (1usize << PS_Success)
                                        | (1usize << PS_Theory)
                                        | (1usize << PS_True)
                                        | (1usize << PS_Unknown)
                                        | (1usize << PS_Unsupported)
                                        | (1usize << PS_Unsat)))
                                    != 0)
                                || _la == UndefinedSymbol)
                            {
                                break;
                            }
                        }
                        recog.base.set_state(590);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        recog.base.set_state(591);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(593);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule constructor_dec*/
                                    recog.base.set_state(592);
                                    recog.constructor_dec()?;
                                }
                            }
                            recog.base.set_state(595);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !(_la == ParOpen) {
                                break;
                            }
                        }
                        recog.base.set_state(597);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        recog.base.set_state(598);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- function_dec ----------------
pub type Function_decContextAll<'input> = Function_decContext<'input>;

pub type Function_decContext<'input> =
    BaseParserRuleContext<'input, Function_decContextExt<'input>>;

#[derive(Clone)]
pub struct Function_decContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Function_decContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Function_decContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_function_dec(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_function_dec(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Function_decContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_function_dec(self);
    }
}

impl<'input> CustomRuleContext<'input> for Function_decContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_function_dec
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_function_dec }
}
antlr_rust::type_id! {Function_decContextExt<'a>}

impl<'input> Function_decContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Function_decContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Function_decContextExt { ph: PhantomData },
        ))
    }
}

pub trait Function_decContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Function_decContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token ParOpen in current rule
    fn ParOpen_all(&self) -> Vec<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ParOpen, starting from 0.
    /// Returns `None` if number of children corresponding to token ParOpen is less or equal than `i`.
    fn ParOpen(&self, i: usize) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, i)
    }
    fn symbol(&self) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ParClose in current rule
    fn ParClose_all(&self) -> Vec<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ParClose, starting from 0.
    /// Returns `None` if number of children corresponding to token ParClose is less or equal than `i`.
    fn ParClose(&self, i: usize) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, i)
    }
    fn sort(&self) -> Option<Rc<SortContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn sorted_var_all(&self) -> Vec<Rc<Sorted_varContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn sorted_var(&self, i: usize) -> Option<Rc<Sorted_varContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Function_decContextAttrs<'input> for Function_decContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn function_dec(&mut self) -> Result<Rc<Function_decContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Function_decContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 78, RULE_function_dec);
        let mut _localctx: Rc<Function_decContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(602);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                /*InvokeRule symbol*/
                recog.base.set_state(603);
                recog.symbol()?;

                recog.base.set_state(604);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                recog.base.set_state(608);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == ParOpen {
                    {
                        {
                            /*InvokeRule sorted_var*/
                            recog.base.set_state(605);
                            recog.sorted_var()?;
                        }
                    }
                    recog.base.set_state(610);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(611);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;

                /*InvokeRule sort*/
                recog.base.set_state(612);
                recog.sort()?;

                recog.base.set_state(613);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- function_def ----------------
pub type Function_defContextAll<'input> = Function_defContext<'input>;

pub type Function_defContext<'input> =
    BaseParserRuleContext<'input, Function_defContextExt<'input>>;

#[derive(Clone)]
pub struct Function_defContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Function_defContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Function_defContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_function_def(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_function_def(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Function_defContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_function_def(self);
    }
}

impl<'input> CustomRuleContext<'input> for Function_defContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_function_def
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_function_def }
}
antlr_rust::type_id! {Function_defContextExt<'a>}

impl<'input> Function_defContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Function_defContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Function_defContextExt { ph: PhantomData },
        ))
    }
}

pub trait Function_defContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Function_defContextExt<'input>>
{
    fn symbol(&self) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn sort(&self) -> Option<Rc<SortContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn term(&self) -> Option<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn sorted_var_all(&self) -> Vec<Rc<Sorted_varContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn sorted_var(&self, i: usize) -> Option<Rc<Sorted_varContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Function_defContextAttrs<'input> for Function_defContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn function_def(&mut self) -> Result<Rc<Function_defContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Function_defContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 80, RULE_function_def);
        let mut _localctx: Rc<Function_defContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule symbol*/
                recog.base.set_state(615);
                recog.symbol()?;

                recog.base.set_state(616);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                recog.base.set_state(620);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == ParOpen {
                    {
                        {
                            /*InvokeRule sorted_var*/
                            recog.base.set_state(617);
                            recog.sorted_var()?;
                        }
                    }
                    recog.base.set_state(622);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(623);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;

                /*InvokeRule sort*/
                recog.base.set_state(624);
                recog.sort()?;

                /*InvokeRule term*/
                recog.base.set_state(625);
                recog.term()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- prop_literal ----------------
pub type Prop_literalContextAll<'input> = Prop_literalContext<'input>;

pub type Prop_literalContext<'input> =
    BaseParserRuleContext<'input, Prop_literalContextExt<'input>>;

#[derive(Clone)]
pub struct Prop_literalContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Prop_literalContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Prop_literalContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_prop_literal(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_prop_literal(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Prop_literalContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_prop_literal(self);
    }
}

impl<'input> CustomRuleContext<'input> for Prop_literalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_prop_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_prop_literal }
}
antlr_rust::type_id! {Prop_literalContextExt<'a>}

impl<'input> Prop_literalContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Prop_literalContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Prop_literalContextExt { ph: PhantomData },
        ))
    }
}

pub trait Prop_literalContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Prop_literalContextExt<'input>>
{
    fn symbol(&self) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Not
    /// Returns `None` if there is no child corresponding to token PS_Not
    fn PS_Not(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Not, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
}

impl<'input> Prop_literalContextAttrs<'input> for Prop_literalContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn prop_literal(&mut self) -> Result<Rc<Prop_literalContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Prop_literalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 82, RULE_prop_literal);
        let mut _localctx: Rc<Prop_literalContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(633);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                QuotedSymbol
                | PS_Not
                | PS_Bool
                | PS_ContinuedExecution
                | PS_Error
                | PS_False
                | PS_ImmediateExit
                | PS_Incomplete
                | PS_Logic
                | PS_Memout
                | PS_Sat
                | PS_Success
                | PS_Theory
                | PS_True
                | PS_Unknown
                | PS_Unsupported
                | PS_Unsat
                | UndefinedSymbol => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule symbol*/
                        recog.base.set_state(627);
                        recog.symbol()?;
                    }
                }

                ParOpen => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(628);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(629);
                        recog.base.match_token(PS_Not, &mut recog.err_handler)?;

                        /*InvokeRule symbol*/
                        recog.base.set_state(630);
                        recog.symbol()?;

                        recog.base.set_state(631);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- script ----------------
pub type ScriptContextAll<'input> = ScriptContext<'input>;

pub type ScriptContext<'input> = BaseParserRuleContext<'input, ScriptContextExt<'input>>;

#[derive(Clone)]
pub struct ScriptContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for ScriptContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for ScriptContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_script(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_script(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for ScriptContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_script(self);
    }
}

impl<'input> CustomRuleContext<'input> for ScriptContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_script
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_script }
}
antlr_rust::type_id! {ScriptContextExt<'a>}

impl<'input> ScriptContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ScriptContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ScriptContextExt { ph: PhantomData },
        ))
    }
}

pub trait ScriptContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<ScriptContextExt<'input>>
{
    fn command_all(&self) -> Vec<Rc<CommandContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn command(&self, i: usize) -> Option<Rc<CommandContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> ScriptContextAttrs<'input> for ScriptContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn script(&mut self) -> Result<Rc<ScriptContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ScriptContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_script);
        let mut _localctx: Rc<ScriptContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(638);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == ParOpen {
                    {
                        {
                            /*InvokeRule command*/
                            recog.base.set_state(635);
                            recog.command()?;
                        }
                    }
                    recog.base.set_state(640);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_assert ----------------
pub type Cmd_assertContextAll<'input> = Cmd_assertContext<'input>;

pub type Cmd_assertContext<'input> = BaseParserRuleContext<'input, Cmd_assertContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_assertContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_assertContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_assertContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_assert(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_assert(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_assertContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_assert(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_assertContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_assert
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_assert }
}
antlr_rust::type_id! {Cmd_assertContextExt<'a>}

impl<'input> Cmd_assertContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_assertContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_assertContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_assertContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_assertContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_Assert
    /// Returns `None` if there is no child corresponding to token CMD_Assert
    fn CMD_Assert(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_Assert, 0)
    }
}

impl<'input> Cmd_assertContextAttrs<'input> for Cmd_assertContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_assert(&mut self) -> Result<Rc<Cmd_assertContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Cmd_assertContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 86, RULE_cmd_assert);
        let mut _localctx: Rc<Cmd_assertContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(641);
                recog.base.match_token(CMD_Assert, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_checkSat ----------------
pub type Cmd_checkSatContextAll<'input> = Cmd_checkSatContext<'input>;

pub type Cmd_checkSatContext<'input> =
    BaseParserRuleContext<'input, Cmd_checkSatContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_checkSatContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_checkSatContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_checkSatContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_checkSat(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_checkSat(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_checkSatContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_checkSat(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_checkSatContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_checkSat
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_checkSat }
}
antlr_rust::type_id! {Cmd_checkSatContextExt<'a>}

impl<'input> Cmd_checkSatContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_checkSatContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_checkSatContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_checkSatContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_checkSatContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_CheckSat
    /// Returns `None` if there is no child corresponding to token CMD_CheckSat
    fn CMD_CheckSat(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_CheckSat, 0)
    }
}

impl<'input> Cmd_checkSatContextAttrs<'input> for Cmd_checkSatContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_checkSat(&mut self) -> Result<Rc<Cmd_checkSatContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Cmd_checkSatContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 88, RULE_cmd_checkSat);
        let mut _localctx: Rc<Cmd_checkSatContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(643);
                recog
                    .base
                    .match_token(CMD_CheckSat, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_checkSatAssuming ----------------
pub type Cmd_checkSatAssumingContextAll<'input> = Cmd_checkSatAssumingContext<'input>;

pub type Cmd_checkSatAssumingContext<'input> =
    BaseParserRuleContext<'input, Cmd_checkSatAssumingContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_checkSatAssumingContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_checkSatAssumingContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Cmd_checkSatAssumingContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_checkSatAssuming(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_checkSatAssuming(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a>
    for Cmd_checkSatAssumingContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_checkSatAssuming(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_checkSatAssumingContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_checkSatAssuming
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_checkSatAssuming }
}
antlr_rust::type_id! {Cmd_checkSatAssumingContextExt<'a>}

impl<'input> Cmd_checkSatAssumingContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_checkSatAssumingContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_checkSatAssumingContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_checkSatAssumingContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_checkSatAssumingContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_CheckSatAssuming
    /// Returns `None` if there is no child corresponding to token CMD_CheckSatAssuming
    fn CMD_CheckSatAssuming(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_CheckSatAssuming, 0)
    }
}

impl<'input> Cmd_checkSatAssumingContextAttrs<'input> for Cmd_checkSatAssumingContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_checkSatAssuming(
        &mut self,
    ) -> Result<Rc<Cmd_checkSatAssumingContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_checkSatAssumingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 90, RULE_cmd_checkSatAssuming);
        let mut _localctx: Rc<Cmd_checkSatAssumingContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(645);
                recog
                    .base
                    .match_token(CMD_CheckSatAssuming, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_declareConst ----------------
pub type Cmd_declareConstContextAll<'input> = Cmd_declareConstContext<'input>;

pub type Cmd_declareConstContext<'input> =
    BaseParserRuleContext<'input, Cmd_declareConstContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_declareConstContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_declareConstContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_declareConstContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_declareConst(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_declareConst(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_declareConstContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_declareConst(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_declareConstContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_declareConst
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_declareConst }
}
antlr_rust::type_id! {Cmd_declareConstContextExt<'a>}

impl<'input> Cmd_declareConstContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_declareConstContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_declareConstContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_declareConstContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_declareConstContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_DeclareConst
    /// Returns `None` if there is no child corresponding to token CMD_DeclareConst
    fn CMD_DeclareConst(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_DeclareConst, 0)
    }
}

impl<'input> Cmd_declareConstContextAttrs<'input> for Cmd_declareConstContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_declareConst(
        &mut self,
    ) -> Result<Rc<Cmd_declareConstContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_declareConstContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 92, RULE_cmd_declareConst);
        let mut _localctx: Rc<Cmd_declareConstContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(647);
                recog
                    .base
                    .match_token(CMD_DeclareConst, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_declareDatatype ----------------
pub type Cmd_declareDatatypeContextAll<'input> = Cmd_declareDatatypeContext<'input>;

pub type Cmd_declareDatatypeContext<'input> =
    BaseParserRuleContext<'input, Cmd_declareDatatypeContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_declareDatatypeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_declareDatatypeContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Cmd_declareDatatypeContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_declareDatatype(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_declareDatatype(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a>
    for Cmd_declareDatatypeContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_declareDatatype(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_declareDatatypeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_declareDatatype
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_declareDatatype }
}
antlr_rust::type_id! {Cmd_declareDatatypeContextExt<'a>}

impl<'input> Cmd_declareDatatypeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_declareDatatypeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_declareDatatypeContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_declareDatatypeContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_declareDatatypeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_DeclareDatatype
    /// Returns `None` if there is no child corresponding to token CMD_DeclareDatatype
    fn CMD_DeclareDatatype(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_DeclareDatatype, 0)
    }
}

impl<'input> Cmd_declareDatatypeContextAttrs<'input> for Cmd_declareDatatypeContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_declareDatatype(
        &mut self,
    ) -> Result<Rc<Cmd_declareDatatypeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_declareDatatypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 94, RULE_cmd_declareDatatype);
        let mut _localctx: Rc<Cmd_declareDatatypeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(649);
                recog
                    .base
                    .match_token(CMD_DeclareDatatype, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_declareDatatypes ----------------
pub type Cmd_declareDatatypesContextAll<'input> = Cmd_declareDatatypesContext<'input>;

pub type Cmd_declareDatatypesContext<'input> =
    BaseParserRuleContext<'input, Cmd_declareDatatypesContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_declareDatatypesContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_declareDatatypesContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Cmd_declareDatatypesContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_declareDatatypes(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_declareDatatypes(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a>
    for Cmd_declareDatatypesContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_declareDatatypes(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_declareDatatypesContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_declareDatatypes
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_declareDatatypes }
}
antlr_rust::type_id! {Cmd_declareDatatypesContextExt<'a>}

impl<'input> Cmd_declareDatatypesContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_declareDatatypesContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_declareDatatypesContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_declareDatatypesContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_declareDatatypesContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_DeclareDatatypes
    /// Returns `None` if there is no child corresponding to token CMD_DeclareDatatypes
    fn CMD_DeclareDatatypes(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_DeclareDatatypes, 0)
    }
}

impl<'input> Cmd_declareDatatypesContextAttrs<'input> for Cmd_declareDatatypesContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_declareDatatypes(
        &mut self,
    ) -> Result<Rc<Cmd_declareDatatypesContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_declareDatatypesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 96, RULE_cmd_declareDatatypes);
        let mut _localctx: Rc<Cmd_declareDatatypesContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(651);
                recog
                    .base
                    .match_token(CMD_DeclareDatatypes, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_declareFun ----------------
pub type Cmd_declareFunContextAll<'input> = Cmd_declareFunContext<'input>;

pub type Cmd_declareFunContext<'input> =
    BaseParserRuleContext<'input, Cmd_declareFunContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_declareFunContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_declareFunContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_declareFunContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_declareFun(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_declareFun(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_declareFunContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_declareFun(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_declareFunContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_declareFun
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_declareFun }
}
antlr_rust::type_id! {Cmd_declareFunContextExt<'a>}

impl<'input> Cmd_declareFunContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_declareFunContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_declareFunContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_declareFunContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_declareFunContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_DeclareFun
    /// Returns `None` if there is no child corresponding to token CMD_DeclareFun
    fn CMD_DeclareFun(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_DeclareFun, 0)
    }
}

impl<'input> Cmd_declareFunContextAttrs<'input> for Cmd_declareFunContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_declareFun(&mut self) -> Result<Rc<Cmd_declareFunContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_declareFunContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 98, RULE_cmd_declareFun);
        let mut _localctx: Rc<Cmd_declareFunContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(653);
                recog
                    .base
                    .match_token(CMD_DeclareFun, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_declareSort ----------------
pub type Cmd_declareSortContextAll<'input> = Cmd_declareSortContext<'input>;

pub type Cmd_declareSortContext<'input> =
    BaseParserRuleContext<'input, Cmd_declareSortContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_declareSortContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_declareSortContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_declareSortContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_declareSort(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_declareSort(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_declareSortContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_declareSort(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_declareSortContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_declareSort
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_declareSort }
}
antlr_rust::type_id! {Cmd_declareSortContextExt<'a>}

impl<'input> Cmd_declareSortContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_declareSortContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_declareSortContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_declareSortContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_declareSortContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_DeclareSort
    /// Returns `None` if there is no child corresponding to token CMD_DeclareSort
    fn CMD_DeclareSort(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_DeclareSort, 0)
    }
}

impl<'input> Cmd_declareSortContextAttrs<'input> for Cmd_declareSortContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_declareSort(&mut self) -> Result<Rc<Cmd_declareSortContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_declareSortContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 100, RULE_cmd_declareSort);
        let mut _localctx: Rc<Cmd_declareSortContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(655);
                recog
                    .base
                    .match_token(CMD_DeclareSort, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_defineFun ----------------
pub type Cmd_defineFunContextAll<'input> = Cmd_defineFunContext<'input>;

pub type Cmd_defineFunContext<'input> =
    BaseParserRuleContext<'input, Cmd_defineFunContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_defineFunContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_defineFunContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_defineFunContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_defineFun(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_defineFun(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_defineFunContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_defineFun(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_defineFunContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_defineFun
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_defineFun }
}
antlr_rust::type_id! {Cmd_defineFunContextExt<'a>}

impl<'input> Cmd_defineFunContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_defineFunContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_defineFunContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_defineFunContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_defineFunContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_DefineFun
    /// Returns `None` if there is no child corresponding to token CMD_DefineFun
    fn CMD_DefineFun(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_DefineFun, 0)
    }
}

impl<'input> Cmd_defineFunContextAttrs<'input> for Cmd_defineFunContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_defineFun(&mut self) -> Result<Rc<Cmd_defineFunContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_defineFunContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 102, RULE_cmd_defineFun);
        let mut _localctx: Rc<Cmd_defineFunContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(657);
                recog
                    .base
                    .match_token(CMD_DefineFun, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_defineFunRec ----------------
pub type Cmd_defineFunRecContextAll<'input> = Cmd_defineFunRecContext<'input>;

pub type Cmd_defineFunRecContext<'input> =
    BaseParserRuleContext<'input, Cmd_defineFunRecContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_defineFunRecContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_defineFunRecContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_defineFunRecContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_defineFunRec(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_defineFunRec(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_defineFunRecContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_defineFunRec(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_defineFunRecContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_defineFunRec
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_defineFunRec }
}
antlr_rust::type_id! {Cmd_defineFunRecContextExt<'a>}

impl<'input> Cmd_defineFunRecContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_defineFunRecContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_defineFunRecContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_defineFunRecContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_defineFunRecContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_DefineFunRec
    /// Returns `None` if there is no child corresponding to token CMD_DefineFunRec
    fn CMD_DefineFunRec(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_DefineFunRec, 0)
    }
}

impl<'input> Cmd_defineFunRecContextAttrs<'input> for Cmd_defineFunRecContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_defineFunRec(
        &mut self,
    ) -> Result<Rc<Cmd_defineFunRecContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_defineFunRecContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 104, RULE_cmd_defineFunRec);
        let mut _localctx: Rc<Cmd_defineFunRecContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(659);
                recog
                    .base
                    .match_token(CMD_DefineFunRec, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_defineFunsRec ----------------
pub type Cmd_defineFunsRecContextAll<'input> = Cmd_defineFunsRecContext<'input>;

pub type Cmd_defineFunsRecContext<'input> =
    BaseParserRuleContext<'input, Cmd_defineFunsRecContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_defineFunsRecContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_defineFunsRecContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Cmd_defineFunsRecContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_defineFunsRec(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_defineFunsRec(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_defineFunsRecContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_defineFunsRec(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_defineFunsRecContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_defineFunsRec
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_defineFunsRec }
}
antlr_rust::type_id! {Cmd_defineFunsRecContextExt<'a>}

impl<'input> Cmd_defineFunsRecContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_defineFunsRecContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_defineFunsRecContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_defineFunsRecContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_defineFunsRecContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_DefineFunsRec
    /// Returns `None` if there is no child corresponding to token CMD_DefineFunsRec
    fn CMD_DefineFunsRec(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_DefineFunsRec, 0)
    }
}

impl<'input> Cmd_defineFunsRecContextAttrs<'input> for Cmd_defineFunsRecContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_defineFunsRec(
        &mut self,
    ) -> Result<Rc<Cmd_defineFunsRecContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_defineFunsRecContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 106, RULE_cmd_defineFunsRec);
        let mut _localctx: Rc<Cmd_defineFunsRecContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(661);
                recog
                    .base
                    .match_token(CMD_DefineFunsRec, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_defineSort ----------------
pub type Cmd_defineSortContextAll<'input> = Cmd_defineSortContext<'input>;

pub type Cmd_defineSortContext<'input> =
    BaseParserRuleContext<'input, Cmd_defineSortContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_defineSortContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_defineSortContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_defineSortContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_defineSort(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_defineSort(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_defineSortContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_defineSort(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_defineSortContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_defineSort
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_defineSort }
}
antlr_rust::type_id! {Cmd_defineSortContextExt<'a>}

impl<'input> Cmd_defineSortContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_defineSortContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_defineSortContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_defineSortContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_defineSortContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_DefineSort
    /// Returns `None` if there is no child corresponding to token CMD_DefineSort
    fn CMD_DefineSort(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_DefineSort, 0)
    }
}

impl<'input> Cmd_defineSortContextAttrs<'input> for Cmd_defineSortContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_defineSort(&mut self) -> Result<Rc<Cmd_defineSortContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_defineSortContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 108, RULE_cmd_defineSort);
        let mut _localctx: Rc<Cmd_defineSortContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(663);
                recog
                    .base
                    .match_token(CMD_DefineSort, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_echo ----------------
pub type Cmd_echoContextAll<'input> = Cmd_echoContext<'input>;

pub type Cmd_echoContext<'input> = BaseParserRuleContext<'input, Cmd_echoContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_echoContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_echoContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_echoContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_echo(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_echo(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_echoContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_echo(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_echoContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_echo
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_echo }
}
antlr_rust::type_id! {Cmd_echoContextExt<'a>}

impl<'input> Cmd_echoContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_echoContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_echoContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_echoContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_echoContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_Echo
    /// Returns `None` if there is no child corresponding to token CMD_Echo
    fn CMD_Echo(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_Echo, 0)
    }
}

impl<'input> Cmd_echoContextAttrs<'input> for Cmd_echoContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_echo(&mut self) -> Result<Rc<Cmd_echoContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Cmd_echoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_cmd_echo);
        let mut _localctx: Rc<Cmd_echoContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(665);
                recog.base.match_token(CMD_Echo, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_exit ----------------
pub type Cmd_exitContextAll<'input> = Cmd_exitContext<'input>;

pub type Cmd_exitContext<'input> = BaseParserRuleContext<'input, Cmd_exitContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_exitContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_exitContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_exitContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_exit(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_exit(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_exitContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_exit(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_exitContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_exit
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_exit }
}
antlr_rust::type_id! {Cmd_exitContextExt<'a>}

impl<'input> Cmd_exitContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_exitContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_exitContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_exitContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_exitContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_Exit
    /// Returns `None` if there is no child corresponding to token CMD_Exit
    fn CMD_Exit(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_Exit, 0)
    }
}

impl<'input> Cmd_exitContextAttrs<'input> for Cmd_exitContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_exit(&mut self) -> Result<Rc<Cmd_exitContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Cmd_exitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_cmd_exit);
        let mut _localctx: Rc<Cmd_exitContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(667);
                recog.base.match_token(CMD_Exit, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_getAssertions ----------------
pub type Cmd_getAssertionsContextAll<'input> = Cmd_getAssertionsContext<'input>;

pub type Cmd_getAssertionsContext<'input> =
    BaseParserRuleContext<'input, Cmd_getAssertionsContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_getAssertionsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_getAssertionsContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Cmd_getAssertionsContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_getAssertions(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_getAssertions(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_getAssertionsContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_getAssertions(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_getAssertionsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_getAssertions
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_getAssertions }
}
antlr_rust::type_id! {Cmd_getAssertionsContextExt<'a>}

impl<'input> Cmd_getAssertionsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_getAssertionsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_getAssertionsContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_getAssertionsContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_getAssertionsContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_GetAssertions
    /// Returns `None` if there is no child corresponding to token CMD_GetAssertions
    fn CMD_GetAssertions(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_GetAssertions, 0)
    }
}

impl<'input> Cmd_getAssertionsContextAttrs<'input> for Cmd_getAssertionsContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_getAssertions(
        &mut self,
    ) -> Result<Rc<Cmd_getAssertionsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_getAssertionsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 114, RULE_cmd_getAssertions);
        let mut _localctx: Rc<Cmd_getAssertionsContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(669);
                recog
                    .base
                    .match_token(CMD_GetAssertions, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_getAssignment ----------------
pub type Cmd_getAssignmentContextAll<'input> = Cmd_getAssignmentContext<'input>;

pub type Cmd_getAssignmentContext<'input> =
    BaseParserRuleContext<'input, Cmd_getAssignmentContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_getAssignmentContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_getAssignmentContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Cmd_getAssignmentContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_getAssignment(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_getAssignment(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_getAssignmentContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_getAssignment(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_getAssignmentContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_getAssignment
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_getAssignment }
}
antlr_rust::type_id! {Cmd_getAssignmentContextExt<'a>}

impl<'input> Cmd_getAssignmentContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_getAssignmentContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_getAssignmentContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_getAssignmentContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_getAssignmentContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_GetAssignment
    /// Returns `None` if there is no child corresponding to token CMD_GetAssignment
    fn CMD_GetAssignment(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_GetAssignment, 0)
    }
}

impl<'input> Cmd_getAssignmentContextAttrs<'input> for Cmd_getAssignmentContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_getAssignment(
        &mut self,
    ) -> Result<Rc<Cmd_getAssignmentContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_getAssignmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 116, RULE_cmd_getAssignment);
        let mut _localctx: Rc<Cmd_getAssignmentContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(671);
                recog
                    .base
                    .match_token(CMD_GetAssignment, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_getInfo ----------------
pub type Cmd_getInfoContextAll<'input> = Cmd_getInfoContext<'input>;

pub type Cmd_getInfoContext<'input> = BaseParserRuleContext<'input, Cmd_getInfoContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_getInfoContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_getInfoContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_getInfoContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_getInfo(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_getInfo(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_getInfoContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_getInfo(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_getInfoContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_getInfo
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_getInfo }
}
antlr_rust::type_id! {Cmd_getInfoContextExt<'a>}

impl<'input> Cmd_getInfoContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_getInfoContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_getInfoContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_getInfoContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_getInfoContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_GetInfo
    /// Returns `None` if there is no child corresponding to token CMD_GetInfo
    fn CMD_GetInfo(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_GetInfo, 0)
    }
}

impl<'input> Cmd_getInfoContextAttrs<'input> for Cmd_getInfoContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_getInfo(&mut self) -> Result<Rc<Cmd_getInfoContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Cmd_getInfoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 118, RULE_cmd_getInfo);
        let mut _localctx: Rc<Cmd_getInfoContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(673);
                recog
                    .base
                    .match_token(CMD_GetInfo, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_getModel ----------------
pub type Cmd_getModelContextAll<'input> = Cmd_getModelContext<'input>;

pub type Cmd_getModelContext<'input> =
    BaseParserRuleContext<'input, Cmd_getModelContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_getModelContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_getModelContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_getModelContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_getModel(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_getModel(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_getModelContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_getModel(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_getModelContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_getModel
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_getModel }
}
antlr_rust::type_id! {Cmd_getModelContextExt<'a>}

impl<'input> Cmd_getModelContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_getModelContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_getModelContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_getModelContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_getModelContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_GetModel
    /// Returns `None` if there is no child corresponding to token CMD_GetModel
    fn CMD_GetModel(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_GetModel, 0)
    }
}

impl<'input> Cmd_getModelContextAttrs<'input> for Cmd_getModelContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_getModel(&mut self) -> Result<Rc<Cmd_getModelContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Cmd_getModelContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 120, RULE_cmd_getModel);
        let mut _localctx: Rc<Cmd_getModelContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(675);
                recog
                    .base
                    .match_token(CMD_GetModel, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_getOption ----------------
pub type Cmd_getOptionContextAll<'input> = Cmd_getOptionContext<'input>;

pub type Cmd_getOptionContext<'input> =
    BaseParserRuleContext<'input, Cmd_getOptionContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_getOptionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_getOptionContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_getOptionContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_getOption(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_getOption(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_getOptionContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_getOption(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_getOptionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_getOption
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_getOption }
}
antlr_rust::type_id! {Cmd_getOptionContextExt<'a>}

impl<'input> Cmd_getOptionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_getOptionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_getOptionContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_getOptionContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_getOptionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_GetOption
    /// Returns `None` if there is no child corresponding to token CMD_GetOption
    fn CMD_GetOption(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_GetOption, 0)
    }
}

impl<'input> Cmd_getOptionContextAttrs<'input> for Cmd_getOptionContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_getOption(&mut self) -> Result<Rc<Cmd_getOptionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_getOptionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 122, RULE_cmd_getOption);
        let mut _localctx: Rc<Cmd_getOptionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(677);
                recog
                    .base
                    .match_token(CMD_GetOption, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_getProof ----------------
pub type Cmd_getProofContextAll<'input> = Cmd_getProofContext<'input>;

pub type Cmd_getProofContext<'input> =
    BaseParserRuleContext<'input, Cmd_getProofContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_getProofContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_getProofContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_getProofContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_getProof(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_getProof(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_getProofContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_getProof(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_getProofContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_getProof
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_getProof }
}
antlr_rust::type_id! {Cmd_getProofContextExt<'a>}

impl<'input> Cmd_getProofContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_getProofContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_getProofContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_getProofContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_getProofContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_GetProof
    /// Returns `None` if there is no child corresponding to token CMD_GetProof
    fn CMD_GetProof(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_GetProof, 0)
    }
}

impl<'input> Cmd_getProofContextAttrs<'input> for Cmd_getProofContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_getProof(&mut self) -> Result<Rc<Cmd_getProofContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Cmd_getProofContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 124, RULE_cmd_getProof);
        let mut _localctx: Rc<Cmd_getProofContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(679);
                recog
                    .base
                    .match_token(CMD_GetProof, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_getUnsatAssumptions ----------------
pub type Cmd_getUnsatAssumptionsContextAll<'input> = Cmd_getUnsatAssumptionsContext<'input>;

pub type Cmd_getUnsatAssumptionsContext<'input> =
    BaseParserRuleContext<'input, Cmd_getUnsatAssumptionsContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_getUnsatAssumptionsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_getUnsatAssumptionsContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Cmd_getUnsatAssumptionsContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_getUnsatAssumptions(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_getUnsatAssumptions(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a>
    for Cmd_getUnsatAssumptionsContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_getUnsatAssumptions(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_getUnsatAssumptionsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_getUnsatAssumptions
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_getUnsatAssumptions }
}
antlr_rust::type_id! {Cmd_getUnsatAssumptionsContextExt<'a>}

impl<'input> Cmd_getUnsatAssumptionsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_getUnsatAssumptionsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_getUnsatAssumptionsContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_getUnsatAssumptionsContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_getUnsatAssumptionsContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_GetUnsatAssumptions
    /// Returns `None` if there is no child corresponding to token CMD_GetUnsatAssumptions
    fn CMD_GetUnsatAssumptions(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_GetUnsatAssumptions, 0)
    }
}

impl<'input> Cmd_getUnsatAssumptionsContextAttrs<'input>
    for Cmd_getUnsatAssumptionsContext<'input>
{
}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_getUnsatAssumptions(
        &mut self,
    ) -> Result<Rc<Cmd_getUnsatAssumptionsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_getUnsatAssumptionsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 126, RULE_cmd_getUnsatAssumptions);
        let mut _localctx: Rc<Cmd_getUnsatAssumptionsContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(681);
                recog
                    .base
                    .match_token(CMD_GetUnsatAssumptions, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_getUnsatCore ----------------
pub type Cmd_getUnsatCoreContextAll<'input> = Cmd_getUnsatCoreContext<'input>;

pub type Cmd_getUnsatCoreContext<'input> =
    BaseParserRuleContext<'input, Cmd_getUnsatCoreContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_getUnsatCoreContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_getUnsatCoreContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_getUnsatCoreContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_getUnsatCore(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_getUnsatCore(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_getUnsatCoreContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_getUnsatCore(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_getUnsatCoreContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_getUnsatCore
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_getUnsatCore }
}
antlr_rust::type_id! {Cmd_getUnsatCoreContextExt<'a>}

impl<'input> Cmd_getUnsatCoreContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_getUnsatCoreContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_getUnsatCoreContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_getUnsatCoreContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_getUnsatCoreContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_GetUnsatCore
    /// Returns `None` if there is no child corresponding to token CMD_GetUnsatCore
    fn CMD_GetUnsatCore(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_GetUnsatCore, 0)
    }
}

impl<'input> Cmd_getUnsatCoreContextAttrs<'input> for Cmd_getUnsatCoreContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_getUnsatCore(
        &mut self,
    ) -> Result<Rc<Cmd_getUnsatCoreContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_getUnsatCoreContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 128, RULE_cmd_getUnsatCore);
        let mut _localctx: Rc<Cmd_getUnsatCoreContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(683);
                recog
                    .base
                    .match_token(CMD_GetUnsatCore, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_getValue ----------------
pub type Cmd_getValueContextAll<'input> = Cmd_getValueContext<'input>;

pub type Cmd_getValueContext<'input> =
    BaseParserRuleContext<'input, Cmd_getValueContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_getValueContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_getValueContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_getValueContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_getValue(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_getValue(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_getValueContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_getValue(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_getValueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_getValue
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_getValue }
}
antlr_rust::type_id! {Cmd_getValueContextExt<'a>}

impl<'input> Cmd_getValueContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_getValueContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_getValueContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_getValueContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_getValueContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_GetValue
    /// Returns `None` if there is no child corresponding to token CMD_GetValue
    fn CMD_GetValue(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_GetValue, 0)
    }
}

impl<'input> Cmd_getValueContextAttrs<'input> for Cmd_getValueContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_getValue(&mut self) -> Result<Rc<Cmd_getValueContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Cmd_getValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 130, RULE_cmd_getValue);
        let mut _localctx: Rc<Cmd_getValueContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(685);
                recog
                    .base
                    .match_token(CMD_GetValue, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_pop ----------------
pub type Cmd_popContextAll<'input> = Cmd_popContext<'input>;

pub type Cmd_popContext<'input> = BaseParserRuleContext<'input, Cmd_popContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_popContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_popContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_popContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_pop(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_pop(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_popContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_pop(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_popContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_pop
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_pop }
}
antlr_rust::type_id! {Cmd_popContextExt<'a>}

impl<'input> Cmd_popContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_popContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_popContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_popContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_popContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_Pop
    /// Returns `None` if there is no child corresponding to token CMD_Pop
    fn CMD_Pop(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_Pop, 0)
    }
}

impl<'input> Cmd_popContextAttrs<'input> for Cmd_popContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_pop(&mut self) -> Result<Rc<Cmd_popContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Cmd_popContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_cmd_pop);
        let mut _localctx: Rc<Cmd_popContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(687);
                recog.base.match_token(CMD_Pop, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_push ----------------
pub type Cmd_pushContextAll<'input> = Cmd_pushContext<'input>;

pub type Cmd_pushContext<'input> = BaseParserRuleContext<'input, Cmd_pushContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_pushContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_pushContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_pushContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_push(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_push(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_pushContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_push(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_pushContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_push
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_push }
}
antlr_rust::type_id! {Cmd_pushContextExt<'a>}

impl<'input> Cmd_pushContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_pushContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_pushContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_pushContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_pushContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_Push
    /// Returns `None` if there is no child corresponding to token CMD_Push
    fn CMD_Push(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_Push, 0)
    }
}

impl<'input> Cmd_pushContextAttrs<'input> for Cmd_pushContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_push(&mut self) -> Result<Rc<Cmd_pushContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Cmd_pushContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_cmd_push);
        let mut _localctx: Rc<Cmd_pushContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(689);
                recog.base.match_token(CMD_Push, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_reset ----------------
pub type Cmd_resetContextAll<'input> = Cmd_resetContext<'input>;

pub type Cmd_resetContext<'input> = BaseParserRuleContext<'input, Cmd_resetContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_resetContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_resetContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_resetContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_reset(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_reset(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_resetContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_reset(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_resetContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_reset
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_reset }
}
antlr_rust::type_id! {Cmd_resetContextExt<'a>}

impl<'input> Cmd_resetContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_resetContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_resetContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_resetContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_resetContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_Reset
    /// Returns `None` if there is no child corresponding to token CMD_Reset
    fn CMD_Reset(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_Reset, 0)
    }
}

impl<'input> Cmd_resetContextAttrs<'input> for Cmd_resetContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_reset(&mut self) -> Result<Rc<Cmd_resetContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Cmd_resetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 136, RULE_cmd_reset);
        let mut _localctx: Rc<Cmd_resetContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(691);
                recog.base.match_token(CMD_Reset, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_resetAssertions ----------------
pub type Cmd_resetAssertionsContextAll<'input> = Cmd_resetAssertionsContext<'input>;

pub type Cmd_resetAssertionsContext<'input> =
    BaseParserRuleContext<'input, Cmd_resetAssertionsContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_resetAssertionsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_resetAssertionsContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Cmd_resetAssertionsContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_resetAssertions(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_resetAssertions(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a>
    for Cmd_resetAssertionsContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_resetAssertions(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_resetAssertionsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_resetAssertions
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_resetAssertions }
}
antlr_rust::type_id! {Cmd_resetAssertionsContextExt<'a>}

impl<'input> Cmd_resetAssertionsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_resetAssertionsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_resetAssertionsContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_resetAssertionsContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_resetAssertionsContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_ResetAssertions
    /// Returns `None` if there is no child corresponding to token CMD_ResetAssertions
    fn CMD_ResetAssertions(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_ResetAssertions, 0)
    }
}

impl<'input> Cmd_resetAssertionsContextAttrs<'input> for Cmd_resetAssertionsContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_resetAssertions(
        &mut self,
    ) -> Result<Rc<Cmd_resetAssertionsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_resetAssertionsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 138, RULE_cmd_resetAssertions);
        let mut _localctx: Rc<Cmd_resetAssertionsContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(693);
                recog
                    .base
                    .match_token(CMD_ResetAssertions, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_setInfo ----------------
pub type Cmd_setInfoContextAll<'input> = Cmd_setInfoContext<'input>;

pub type Cmd_setInfoContext<'input> = BaseParserRuleContext<'input, Cmd_setInfoContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_setInfoContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_setInfoContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_setInfoContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_setInfo(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_setInfo(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_setInfoContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_setInfo(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_setInfoContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_setInfo
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_setInfo }
}
antlr_rust::type_id! {Cmd_setInfoContextExt<'a>}

impl<'input> Cmd_setInfoContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_setInfoContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_setInfoContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_setInfoContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_setInfoContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_SetInfo
    /// Returns `None` if there is no child corresponding to token CMD_SetInfo
    fn CMD_SetInfo(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_SetInfo, 0)
    }
}

impl<'input> Cmd_setInfoContextAttrs<'input> for Cmd_setInfoContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_setInfo(&mut self) -> Result<Rc<Cmd_setInfoContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Cmd_setInfoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 140, RULE_cmd_setInfo);
        let mut _localctx: Rc<Cmd_setInfoContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(695);
                recog
                    .base
                    .match_token(CMD_SetInfo, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_setLogic ----------------
pub type Cmd_setLogicContextAll<'input> = Cmd_setLogicContext<'input>;

pub type Cmd_setLogicContext<'input> =
    BaseParserRuleContext<'input, Cmd_setLogicContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_setLogicContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_setLogicContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_setLogicContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_setLogic(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_setLogic(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_setLogicContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_setLogic(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_setLogicContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_setLogic
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_setLogic }
}
antlr_rust::type_id! {Cmd_setLogicContextExt<'a>}

impl<'input> Cmd_setLogicContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_setLogicContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_setLogicContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_setLogicContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_setLogicContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_SetLogic
    /// Returns `None` if there is no child corresponding to token CMD_SetLogic
    fn CMD_SetLogic(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_SetLogic, 0)
    }
}

impl<'input> Cmd_setLogicContextAttrs<'input> for Cmd_setLogicContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_setLogic(&mut self) -> Result<Rc<Cmd_setLogicContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Cmd_setLogicContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 142, RULE_cmd_setLogic);
        let mut _localctx: Rc<Cmd_setLogicContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(697);
                recog
                    .base
                    .match_token(CMD_SetLogic, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- cmd_setOption ----------------
pub type Cmd_setOptionContextAll<'input> = Cmd_setOptionContext<'input>;

pub type Cmd_setOptionContext<'input> =
    BaseParserRuleContext<'input, Cmd_setOptionContextExt<'input>>;

#[derive(Clone)]
pub struct Cmd_setOptionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Cmd_setOptionContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Cmd_setOptionContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cmd_setOption(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_cmd_setOption(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Cmd_setOptionContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_cmd_setOption(self);
    }
}

impl<'input> CustomRuleContext<'input> for Cmd_setOptionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_cmd_setOption
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_cmd_setOption }
}
antlr_rust::type_id! {Cmd_setOptionContextExt<'a>}

impl<'input> Cmd_setOptionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Cmd_setOptionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Cmd_setOptionContextExt { ph: PhantomData },
        ))
    }
}

pub trait Cmd_setOptionContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Cmd_setOptionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CMD_SetOption
    /// Returns `None` if there is no child corresponding to token CMD_SetOption
    fn CMD_SetOption(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_SetOption, 0)
    }
}

impl<'input> Cmd_setOptionContextAttrs<'input> for Cmd_setOptionContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn cmd_setOption(&mut self) -> Result<Rc<Cmd_setOptionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Cmd_setOptionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 144, RULE_cmd_setOption);
        let mut _localctx: Rc<Cmd_setOptionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(699);
                recog
                    .base
                    .match_token(CMD_SetOption, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- command ----------------
pub type CommandContextAll<'input> = CommandContext<'input>;

pub type CommandContext<'input> = BaseParserRuleContext<'input, CommandContextExt<'input>>;

#[derive(Clone)]
pub struct CommandContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for CommandContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for CommandContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_command(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_command(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for CommandContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_command(self);
    }
}

impl<'input> CustomRuleContext<'input> for CommandContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_command
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_command }
}
antlr_rust::type_id! {CommandContextExt<'a>}

impl<'input> CommandContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<CommandContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            CommandContextExt { ph: PhantomData },
        ))
    }
}

pub trait CommandContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<CommandContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token ParOpen in current rule
    fn ParOpen_all(&self) -> Vec<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ParOpen, starting from 0.
    /// Returns `None` if number of children corresponding to token ParOpen is less or equal than `i`.
    fn ParOpen(&self, i: usize) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, i)
    }
    fn cmd_assert(&self) -> Option<Rc<Cmd_assertContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn term_all(&self) -> Vec<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ParClose in current rule
    fn ParClose_all(&self) -> Vec<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ParClose, starting from 0.
    /// Returns `None` if number of children corresponding to token ParClose is less or equal than `i`.
    fn ParClose(&self, i: usize) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, i)
    }
    fn cmd_checkSat(&self) -> Option<Rc<Cmd_checkSatContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_checkSatAssuming(&self) -> Option<Rc<Cmd_checkSatAssumingContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_declareConst(&self) -> Option<Rc<Cmd_declareConstContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn symbol_all(&self) -> Vec<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn symbol(&self, i: usize) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn sort_all(&self) -> Vec<Rc<SortContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn sort(&self, i: usize) -> Option<Rc<SortContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn cmd_declareDatatype(&self) -> Option<Rc<Cmd_declareDatatypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn datatype_dec_all(&self) -> Vec<Rc<Datatype_decContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn datatype_dec(&self, i: usize) -> Option<Rc<Datatype_decContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn cmd_declareDatatypes(&self) -> Option<Rc<Cmd_declareDatatypesContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn sort_dec_all(&self) -> Vec<Rc<Sort_decContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn sort_dec(&self, i: usize) -> Option<Rc<Sort_decContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn cmd_declareFun(&self) -> Option<Rc<Cmd_declareFunContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_declareSort(&self) -> Option<Rc<Cmd_declareSortContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn numeral(&self) -> Option<Rc<NumeralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_defineFun(&self) -> Option<Rc<Cmd_defineFunContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn function_def(&self) -> Option<Rc<Function_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_defineFunRec(&self) -> Option<Rc<Cmd_defineFunRecContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_defineFunsRec(&self) -> Option<Rc<Cmd_defineFunsRecContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn function_dec_all(&self) -> Vec<Rc<Function_decContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn function_dec(&self, i: usize) -> Option<Rc<Function_decContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn cmd_defineSort(&self) -> Option<Rc<Cmd_defineSortContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_echo(&self) -> Option<Rc<Cmd_echoContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn string(&self) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_exit(&self) -> Option<Rc<Cmd_exitContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_getAssertions(&self) -> Option<Rc<Cmd_getAssertionsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_getAssignment(&self) -> Option<Rc<Cmd_getAssignmentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_getInfo(&self) -> Option<Rc<Cmd_getInfoContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn info_flag(&self) -> Option<Rc<Info_flagContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_getModel(&self) -> Option<Rc<Cmd_getModelContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_getOption(&self) -> Option<Rc<Cmd_getOptionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn keyword(&self) -> Option<Rc<KeywordContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_getProof(&self) -> Option<Rc<Cmd_getProofContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_getUnsatAssumptions(&self) -> Option<Rc<Cmd_getUnsatAssumptionsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_getUnsatCore(&self) -> Option<Rc<Cmd_getUnsatCoreContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_getValue(&self) -> Option<Rc<Cmd_getValueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_pop(&self) -> Option<Rc<Cmd_popContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_push(&self) -> Option<Rc<Cmd_pushContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_reset(&self) -> Option<Rc<Cmd_resetContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_resetAssertions(&self) -> Option<Rc<Cmd_resetAssertionsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_setInfo(&self) -> Option<Rc<Cmd_setInfoContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn attribute(&self) -> Option<Rc<AttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_setLogic(&self) -> Option<Rc<Cmd_setLogicContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn cmd_setOption(&self) -> Option<Rc<Cmd_setOptionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn option(&self) -> Option<Rc<OptionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> CommandContextAttrs<'input> for CommandContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn command(&mut self) -> Result<Rc<CommandContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = CommandContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_command);
        let mut _localctx: Rc<CommandContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(893);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(57, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(701);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_assert*/
                        recog.base.set_state(702);
                        recog.cmd_assert()?;

                        /*InvokeRule term*/
                        recog.base.set_state(703);
                        recog.term()?;

                        recog.base.set_state(704);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(706);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_checkSat*/
                        recog.base.set_state(707);
                        recog.cmd_checkSat()?;

                        recog.base.set_state(708);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(710);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_checkSatAssuming*/
                        recog.base.set_state(711);
                        recog.cmd_checkSatAssuming()?;

                        recog.base.set_state(712);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(714);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_declareConst*/
                        recog.base.set_state(715);
                        recog.cmd_declareConst()?;

                        /*InvokeRule symbol*/
                        recog.base.set_state(716);
                        recog.symbol()?;

                        /*InvokeRule sort*/
                        recog.base.set_state(717);
                        recog.sort()?;

                        recog.base.set_state(718);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        recog.base.set_state(720);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_declareDatatype*/
                        recog.base.set_state(721);
                        recog.cmd_declareDatatype()?;

                        /*InvokeRule symbol*/
                        recog.base.set_state(722);
                        recog.symbol()?;

                        /*InvokeRule datatype_dec*/
                        recog.base.set_state(723);
                        recog.datatype_dec()?;

                        recog.base.set_state(724);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        recog.base.set_state(726);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_declareDatatypes*/
                        recog.base.set_state(727);
                        recog.cmd_declareDatatypes()?;

                        recog.base.set_state(728);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(730);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule sort_dec*/
                                    recog.base.set_state(729);
                                    recog.sort_dec()?;
                                }
                            }
                            recog.base.set_state(732);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !(_la == ParOpen) {
                                break;
                            }
                        }
                        recog.base.set_state(734);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        recog.base.set_state(735);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(737);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule datatype_dec*/
                                    recog.base.set_state(736);
                                    recog.datatype_dec()?;
                                }
                            }
                            recog.base.set_state(739);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !(_la == ParOpen) {
                                break;
                            }
                        }
                        recog.base.set_state(741);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        recog.base.set_state(742);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        recog.base.set_state(744);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_declareFun*/
                        recog.base.set_state(745);
                        recog.cmd_declareFun()?;

                        /*InvokeRule symbol*/
                        recog.base.set_state(746);
                        recog.symbol()?;

                        recog.base.set_state(747);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(751);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while (((_la) & !0x3f) == 0
                            && ((1usize << _la)
                                & ((1usize << ParOpen)
                                    | (1usize << QuotedSymbol)
                                    | (1usize << PS_Not)
                                    | (1usize << PS_Bool)
                                    | (1usize << PS_ContinuedExecution)
                                    | (1usize << PS_Error)
                                    | (1usize << PS_False)
                                    | (1usize << PS_ImmediateExit)
                                    | (1usize << PS_Incomplete)
                                    | (1usize << PS_Logic)
                                    | (1usize << PS_Memout)
                                    | (1usize << PS_Sat)
                                    | (1usize << PS_Success)
                                    | (1usize << PS_Theory)
                                    | (1usize << PS_True)
                                    | (1usize << PS_Unknown)
                                    | (1usize << PS_Unsupported)
                                    | (1usize << PS_Unsat)))
                                != 0)
                            || _la == UndefinedSymbol
                        {
                            {
                                {
                                    /*InvokeRule sort*/
                                    recog.base.set_state(748);
                                    recog.sort()?;
                                }
                            }
                            recog.base.set_state(753);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(754);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        /*InvokeRule sort*/
                        recog.base.set_state(755);
                        recog.sort()?;

                        recog.base.set_state(756);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                8 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8);
                    recog.base.enter_outer_alt(None, 8);
                    {
                        recog.base.set_state(758);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_declareSort*/
                        recog.base.set_state(759);
                        recog.cmd_declareSort()?;

                        /*InvokeRule symbol*/
                        recog.base.set_state(760);
                        recog.symbol()?;

                        /*InvokeRule numeral*/
                        recog.base.set_state(761);
                        recog.numeral()?;

                        recog.base.set_state(762);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                9 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 9);
                    recog.base.enter_outer_alt(None, 9);
                    {
                        recog.base.set_state(764);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_defineFun*/
                        recog.base.set_state(765);
                        recog.cmd_defineFun()?;

                        /*InvokeRule function_def*/
                        recog.base.set_state(766);
                        recog.function_def()?;

                        recog.base.set_state(767);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                10 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 10);
                    recog.base.enter_outer_alt(None, 10);
                    {
                        recog.base.set_state(769);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_defineFunRec*/
                        recog.base.set_state(770);
                        recog.cmd_defineFunRec()?;

                        /*InvokeRule function_def*/
                        recog.base.set_state(771);
                        recog.function_def()?;

                        recog.base.set_state(772);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                11 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 11);
                    recog.base.enter_outer_alt(None, 11);
                    {
                        recog.base.set_state(774);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_defineFunsRec*/
                        recog.base.set_state(775);
                        recog.cmd_defineFunsRec()?;

                        recog.base.set_state(776);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(778);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule function_dec*/
                                    recog.base.set_state(777);
                                    recog.function_dec()?;
                                }
                            }
                            recog.base.set_state(780);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !(_la == ParOpen) {
                                break;
                            }
                        }
                        recog.base.set_state(782);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        recog.base.set_state(783);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(785);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule term*/
                                    recog.base.set_state(784);
                                    recog.term()?;
                                }
                            }
                            recog.base.set_state(787);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !((((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << ParOpen)
                                        | (1usize << String)
                                        | (1usize << QuotedSymbol)
                                        | (1usize << PS_Not)
                                        | (1usize << PS_Bool)
                                        | (1usize << PS_ContinuedExecution)
                                        | (1usize << PS_Error)
                                        | (1usize << PS_False)
                                        | (1usize << PS_ImmediateExit)
                                        | (1usize << PS_Incomplete)
                                        | (1usize << PS_Logic)
                                        | (1usize << PS_Memout)
                                        | (1usize << PS_Sat)
                                        | (1usize << PS_Success)
                                        | (1usize << PS_Theory)
                                        | (1usize << PS_True)
                                        | (1usize << PS_Unknown)
                                        | (1usize << PS_Unsupported)
                                        | (1usize << PS_Unsat)))
                                    != 0)
                                || (((_la - 66) & !0x3f) == 0
                                    && ((1usize << (_la - 66))
                                        & ((1usize << (Numeral - 66))
                                            | (1usize << (Binary - 66))
                                            | (1usize << (HexDecimal - 66))
                                            | (1usize << (Decimal - 66))
                                            | (1usize << (UndefinedSymbol - 66))))
                                        != 0))
                            {
                                break;
                            }
                        }
                        recog.base.set_state(789);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        recog.base.set_state(790);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                12 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 12);
                    recog.base.enter_outer_alt(None, 12);
                    {
                        recog.base.set_state(792);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_defineSort*/
                        recog.base.set_state(793);
                        recog.cmd_defineSort()?;

                        /*InvokeRule symbol*/
                        recog.base.set_state(794);
                        recog.symbol()?;

                        recog.base.set_state(795);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(799);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while (((_la) & !0x3f) == 0
                            && ((1usize << _la)
                                & ((1usize << QuotedSymbol)
                                    | (1usize << PS_Not)
                                    | (1usize << PS_Bool)
                                    | (1usize << PS_ContinuedExecution)
                                    | (1usize << PS_Error)
                                    | (1usize << PS_False)
                                    | (1usize << PS_ImmediateExit)
                                    | (1usize << PS_Incomplete)
                                    | (1usize << PS_Logic)
                                    | (1usize << PS_Memout)
                                    | (1usize << PS_Sat)
                                    | (1usize << PS_Success)
                                    | (1usize << PS_Theory)
                                    | (1usize << PS_True)
                                    | (1usize << PS_Unknown)
                                    | (1usize << PS_Unsupported)
                                    | (1usize << PS_Unsat)))
                                != 0)
                            || _la == UndefinedSymbol
                        {
                            {
                                {
                                    /*InvokeRule symbol*/
                                    recog.base.set_state(796);
                                    recog.symbol()?;
                                }
                            }
                            recog.base.set_state(801);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(802);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        /*InvokeRule sort*/
                        recog.base.set_state(803);
                        recog.sort()?;

                        recog.base.set_state(804);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                13 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 13);
                    recog.base.enter_outer_alt(None, 13);
                    {
                        recog.base.set_state(806);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_echo*/
                        recog.base.set_state(807);
                        recog.cmd_echo()?;

                        /*InvokeRule string*/
                        recog.base.set_state(808);
                        recog.string()?;

                        recog.base.set_state(809);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                14 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 14);
                    recog.base.enter_outer_alt(None, 14);
                    {
                        recog.base.set_state(811);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_exit*/
                        recog.base.set_state(812);
                        recog.cmd_exit()?;

                        recog.base.set_state(813);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                15 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 15);
                    recog.base.enter_outer_alt(None, 15);
                    {
                        recog.base.set_state(815);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_getAssertions*/
                        recog.base.set_state(816);
                        recog.cmd_getAssertions()?;

                        recog.base.set_state(817);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                16 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 16);
                    recog.base.enter_outer_alt(None, 16);
                    {
                        recog.base.set_state(819);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_getAssignment*/
                        recog.base.set_state(820);
                        recog.cmd_getAssignment()?;

                        recog.base.set_state(821);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                17 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 17);
                    recog.base.enter_outer_alt(None, 17);
                    {
                        recog.base.set_state(823);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_getInfo*/
                        recog.base.set_state(824);
                        recog.cmd_getInfo()?;

                        /*InvokeRule info_flag*/
                        recog.base.set_state(825);
                        recog.info_flag()?;

                        recog.base.set_state(826);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                18 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 18);
                    recog.base.enter_outer_alt(None, 18);
                    {
                        recog.base.set_state(828);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_getModel*/
                        recog.base.set_state(829);
                        recog.cmd_getModel()?;

                        recog.base.set_state(830);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                19 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 19);
                    recog.base.enter_outer_alt(None, 19);
                    {
                        recog.base.set_state(832);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_getOption*/
                        recog.base.set_state(833);
                        recog.cmd_getOption()?;

                        /*InvokeRule keyword*/
                        recog.base.set_state(834);
                        recog.keyword()?;

                        recog.base.set_state(835);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                20 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 20);
                    recog.base.enter_outer_alt(None, 20);
                    {
                        recog.base.set_state(837);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_getProof*/
                        recog.base.set_state(838);
                        recog.cmd_getProof()?;

                        recog.base.set_state(839);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                21 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 21);
                    recog.base.enter_outer_alt(None, 21);
                    {
                        recog.base.set_state(841);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_getUnsatAssumptions*/
                        recog.base.set_state(842);
                        recog.cmd_getUnsatAssumptions()?;

                        recog.base.set_state(843);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                22 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 22);
                    recog.base.enter_outer_alt(None, 22);
                    {
                        recog.base.set_state(845);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_getUnsatCore*/
                        recog.base.set_state(846);
                        recog.cmd_getUnsatCore()?;

                        recog.base.set_state(847);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                23 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 23);
                    recog.base.enter_outer_alt(None, 23);
                    {
                        recog.base.set_state(849);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_getValue*/
                        recog.base.set_state(850);
                        recog.cmd_getValue()?;

                        recog.base.set_state(851);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(853);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule term*/
                                    recog.base.set_state(852);
                                    recog.term()?;
                                }
                            }
                            recog.base.set_state(855);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !((((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << ParOpen)
                                        | (1usize << String)
                                        | (1usize << QuotedSymbol)
                                        | (1usize << PS_Not)
                                        | (1usize << PS_Bool)
                                        | (1usize << PS_ContinuedExecution)
                                        | (1usize << PS_Error)
                                        | (1usize << PS_False)
                                        | (1usize << PS_ImmediateExit)
                                        | (1usize << PS_Incomplete)
                                        | (1usize << PS_Logic)
                                        | (1usize << PS_Memout)
                                        | (1usize << PS_Sat)
                                        | (1usize << PS_Success)
                                        | (1usize << PS_Theory)
                                        | (1usize << PS_True)
                                        | (1usize << PS_Unknown)
                                        | (1usize << PS_Unsupported)
                                        | (1usize << PS_Unsat)))
                                    != 0)
                                || (((_la - 66) & !0x3f) == 0
                                    && ((1usize << (_la - 66))
                                        & ((1usize << (Numeral - 66))
                                            | (1usize << (Binary - 66))
                                            | (1usize << (HexDecimal - 66))
                                            | (1usize << (Decimal - 66))
                                            | (1usize << (UndefinedSymbol - 66))))
                                        != 0))
                            {
                                break;
                            }
                        }
                        recog.base.set_state(857);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        recog.base.set_state(858);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                24 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 24);
                    recog.base.enter_outer_alt(None, 24);
                    {
                        recog.base.set_state(860);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_pop*/
                        recog.base.set_state(861);
                        recog.cmd_pop()?;

                        /*InvokeRule numeral*/
                        recog.base.set_state(862);
                        recog.numeral()?;

                        recog.base.set_state(863);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                25 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 25);
                    recog.base.enter_outer_alt(None, 25);
                    {
                        recog.base.set_state(865);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_push*/
                        recog.base.set_state(866);
                        recog.cmd_push()?;

                        /*InvokeRule numeral*/
                        recog.base.set_state(867);
                        recog.numeral()?;

                        recog.base.set_state(868);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                26 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 26);
                    recog.base.enter_outer_alt(None, 26);
                    {
                        recog.base.set_state(870);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_reset*/
                        recog.base.set_state(871);
                        recog.cmd_reset()?;

                        recog.base.set_state(872);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                27 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 27);
                    recog.base.enter_outer_alt(None, 27);
                    {
                        recog.base.set_state(874);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_resetAssertions*/
                        recog.base.set_state(875);
                        recog.cmd_resetAssertions()?;

                        recog.base.set_state(876);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                28 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 28);
                    recog.base.enter_outer_alt(None, 28);
                    {
                        recog.base.set_state(878);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_setInfo*/
                        recog.base.set_state(879);
                        recog.cmd_setInfo()?;

                        /*InvokeRule attribute*/
                        recog.base.set_state(880);
                        recog.attribute()?;

                        recog.base.set_state(881);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                29 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 29);
                    recog.base.enter_outer_alt(None, 29);
                    {
                        recog.base.set_state(883);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_setLogic*/
                        recog.base.set_state(884);
                        recog.cmd_setLogic()?;

                        /*InvokeRule symbol*/
                        recog.base.set_state(885);
                        recog.symbol()?;

                        recog.base.set_state(886);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                30 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 30);
                    recog.base.enter_outer_alt(None, 30);
                    {
                        recog.base.set_state(888);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        /*InvokeRule cmd_setOption*/
                        recog.base.set_state(889);
                        recog.cmd_setOption()?;

                        /*InvokeRule option*/
                        recog.base.set_state(890);
                        recog.option()?;

                        recog.base.set_state(891);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- b_value ----------------
pub type B_valueContextAll<'input> = B_valueContext<'input>;

pub type B_valueContext<'input> = BaseParserRuleContext<'input, B_valueContextExt<'input>>;

#[derive(Clone)]
pub struct B_valueContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for B_valueContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for B_valueContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_b_value(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_b_value(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for B_valueContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_b_value(self);
    }
}

impl<'input> CustomRuleContext<'input> for B_valueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_b_value
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_b_value }
}
antlr_rust::type_id! {B_valueContextExt<'a>}

impl<'input> B_valueContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<B_valueContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            B_valueContextExt { ph: PhantomData },
        ))
    }
}

pub trait B_valueContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<B_valueContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PS_True
    /// Returns `None` if there is no child corresponding to token PS_True
    fn PS_True(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_True, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_False
    /// Returns `None` if there is no child corresponding to token PS_False
    fn PS_False(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_False, 0)
    }
}

impl<'input> B_valueContextAttrs<'input> for B_valueContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn b_value(&mut self) -> Result<Rc<B_valueContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = B_valueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_b_value);
        let mut _localctx: Rc<B_valueContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(895);
                _la = recog.base.input.la(1);
                if { !(_la == PS_False || _la == PS_True) } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- option ----------------
pub type OptionContextAll<'input> = OptionContext<'input>;

pub type OptionContext<'input> = BaseParserRuleContext<'input, OptionContextExt<'input>>;

#[derive(Clone)]
pub struct OptionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for OptionContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for OptionContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_option(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_option(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for OptionContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_option(self);
    }
}

impl<'input> CustomRuleContext<'input> for OptionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_option
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_option }
}
antlr_rust::type_id! {OptionContextExt<'a>}

impl<'input> OptionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OptionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OptionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OptionContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<OptionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PK_DiagnosticOutputChannel
    /// Returns `None` if there is no child corresponding to token PK_DiagnosticOutputChannel
    fn PK_DiagnosticOutputChannel(
        &self,
    ) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_DiagnosticOutputChannel, 0)
    }
    fn string(&self) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_GlobalDeclarations
    /// Returns `None` if there is no child corresponding to token PK_GlobalDeclarations
    fn PK_GlobalDeclarations(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_GlobalDeclarations, 0)
    }
    fn b_value(&self) -> Option<Rc<B_valueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_InteractiveMode
    /// Returns `None` if there is no child corresponding to token PK_InteractiveMode
    fn PK_InteractiveMode(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_InteractiveMode, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_PrintSuccess
    /// Returns `None` if there is no child corresponding to token PK_PrintSuccess
    fn PK_PrintSuccess(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_PrintSuccess, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ProduceAssertions
    /// Returns `None` if there is no child corresponding to token PK_ProduceAssertions
    fn PK_ProduceAssertions(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ProduceAssertions, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ProduceAssignments
    /// Returns `None` if there is no child corresponding to token PK_ProduceAssignments
    fn PK_ProduceAssignments(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ProduceAssignments, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ProduceModels
    /// Returns `None` if there is no child corresponding to token PK_ProduceModels
    fn PK_ProduceModels(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ProduceModels, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ProduceProofs
    /// Returns `None` if there is no child corresponding to token PK_ProduceProofs
    fn PK_ProduceProofs(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ProduceProofs, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ProduceUnsatAssumptions
    /// Returns `None` if there is no child corresponding to token PK_ProduceUnsatAssumptions
    fn PK_ProduceUnsatAssumptions(
        &self,
    ) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ProduceUnsatAssumptions, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ProduceUnsatCores
    /// Returns `None` if there is no child corresponding to token PK_ProduceUnsatCores
    fn PK_ProduceUnsatCores(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ProduceUnsatCores, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_RandomSeed
    /// Returns `None` if there is no child corresponding to token PK_RandomSeed
    fn PK_RandomSeed(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_RandomSeed, 0)
    }
    fn numeral(&self) -> Option<Rc<NumeralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_RegularOutputChannel
    /// Returns `None` if there is no child corresponding to token PK_RegularOutputChannel
    fn PK_RegularOutputChannel(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_RegularOutputChannel, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ReproducibleResourceLimit
    /// Returns `None` if there is no child corresponding to token PK_ReproducibleResourceLimit
    fn PK_ReproducibleResourceLimit(
        &self,
    ) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ReproducibleResourceLimit, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Verbosity
    /// Returns `None` if there is no child corresponding to token PK_Verbosity
    fn PK_Verbosity(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Verbosity, 0)
    }
    fn attribute(&self) -> Option<Rc<AttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OptionContextAttrs<'input> for OptionContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn option(&mut self) -> Result<Rc<OptionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OptionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_option);
        let mut _localctx: Rc<OptionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(926);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(58, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(897);
                        recog
                            .base
                            .match_token(PK_DiagnosticOutputChannel, &mut recog.err_handler)?;

                        /*InvokeRule string*/
                        recog.base.set_state(898);
                        recog.string()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(899);
                        recog
                            .base
                            .match_token(PK_GlobalDeclarations, &mut recog.err_handler)?;

                        /*InvokeRule b_value*/
                        recog.base.set_state(900);
                        recog.b_value()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(901);
                        recog
                            .base
                            .match_token(PK_InteractiveMode, &mut recog.err_handler)?;

                        /*InvokeRule b_value*/
                        recog.base.set_state(902);
                        recog.b_value()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(903);
                        recog
                            .base
                            .match_token(PK_PrintSuccess, &mut recog.err_handler)?;

                        /*InvokeRule b_value*/
                        recog.base.set_state(904);
                        recog.b_value()?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        recog.base.set_state(905);
                        recog
                            .base
                            .match_token(PK_ProduceAssertions, &mut recog.err_handler)?;

                        /*InvokeRule b_value*/
                        recog.base.set_state(906);
                        recog.b_value()?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        recog.base.set_state(907);
                        recog
                            .base
                            .match_token(PK_ProduceAssignments, &mut recog.err_handler)?;

                        /*InvokeRule b_value*/
                        recog.base.set_state(908);
                        recog.b_value()?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        recog.base.set_state(909);
                        recog
                            .base
                            .match_token(PK_ProduceModels, &mut recog.err_handler)?;

                        /*InvokeRule b_value*/
                        recog.base.set_state(910);
                        recog.b_value()?;
                    }
                }
                8 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8);
                    recog.base.enter_outer_alt(None, 8);
                    {
                        recog.base.set_state(911);
                        recog
                            .base
                            .match_token(PK_ProduceProofs, &mut recog.err_handler)?;

                        /*InvokeRule b_value*/
                        recog.base.set_state(912);
                        recog.b_value()?;
                    }
                }
                9 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 9);
                    recog.base.enter_outer_alt(None, 9);
                    {
                        recog.base.set_state(913);
                        recog
                            .base
                            .match_token(PK_ProduceUnsatAssumptions, &mut recog.err_handler)?;

                        /*InvokeRule b_value*/
                        recog.base.set_state(914);
                        recog.b_value()?;
                    }
                }
                10 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 10);
                    recog.base.enter_outer_alt(None, 10);
                    {
                        recog.base.set_state(915);
                        recog
                            .base
                            .match_token(PK_ProduceUnsatCores, &mut recog.err_handler)?;

                        /*InvokeRule b_value*/
                        recog.base.set_state(916);
                        recog.b_value()?;
                    }
                }
                11 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 11);
                    recog.base.enter_outer_alt(None, 11);
                    {
                        recog.base.set_state(917);
                        recog
                            .base
                            .match_token(PK_RandomSeed, &mut recog.err_handler)?;

                        /*InvokeRule numeral*/
                        recog.base.set_state(918);
                        recog.numeral()?;
                    }
                }
                12 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 12);
                    recog.base.enter_outer_alt(None, 12);
                    {
                        recog.base.set_state(919);
                        recog
                            .base
                            .match_token(PK_RegularOutputChannel, &mut recog.err_handler)?;

                        /*InvokeRule string*/
                        recog.base.set_state(920);
                        recog.string()?;
                    }
                }
                13 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 13);
                    recog.base.enter_outer_alt(None, 13);
                    {
                        recog.base.set_state(921);
                        recog
                            .base
                            .match_token(PK_ReproducibleResourceLimit, &mut recog.err_handler)?;

                        /*InvokeRule numeral*/
                        recog.base.set_state(922);
                        recog.numeral()?;
                    }
                }
                14 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 14);
                    recog.base.enter_outer_alt(None, 14);
                    {
                        recog.base.set_state(923);
                        recog
                            .base
                            .match_token(PK_Verbosity, &mut recog.err_handler)?;

                        /*InvokeRule numeral*/
                        recog.base.set_state(924);
                        recog.numeral()?;
                    }
                }
                15 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 15);
                    recog.base.enter_outer_alt(None, 15);
                    {
                        /*InvokeRule attribute*/
                        recog.base.set_state(925);
                        recog.attribute()?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- info_flag ----------------
pub type Info_flagContextAll<'input> = Info_flagContext<'input>;

pub type Info_flagContext<'input> = BaseParserRuleContext<'input, Info_flagContextExt<'input>>;

#[derive(Clone)]
pub struct Info_flagContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Info_flagContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Info_flagContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_info_flag(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_info_flag(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Info_flagContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_info_flag(self);
    }
}

impl<'input> CustomRuleContext<'input> for Info_flagContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_info_flag
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_info_flag }
}
antlr_rust::type_id! {Info_flagContextExt<'a>}

impl<'input> Info_flagContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Info_flagContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Info_flagContextExt { ph: PhantomData },
        ))
    }
}

pub trait Info_flagContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Info_flagContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PK_AllStatistics
    /// Returns `None` if there is no child corresponding to token PK_AllStatistics
    fn PK_AllStatistics(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_AllStatistics, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_AssertionStackLevels
    /// Returns `None` if there is no child corresponding to token PK_AssertionStackLevels
    fn PK_AssertionStackLevels(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_AssertionStackLevels, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Authors
    /// Returns `None` if there is no child corresponding to token PK_Authors
    fn PK_Authors(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Authors, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ErrorBehaviour
    /// Returns `None` if there is no child corresponding to token PK_ErrorBehaviour
    fn PK_ErrorBehaviour(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ErrorBehaviour, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Name
    /// Returns `None` if there is no child corresponding to token PK_Name
    fn PK_Name(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Name, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ReasonUnknown
    /// Returns `None` if there is no child corresponding to token PK_ReasonUnknown
    fn PK_ReasonUnknown(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ReasonUnknown, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Version
    /// Returns `None` if there is no child corresponding to token PK_Version
    fn PK_Version(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Version, 0)
    }
    fn keyword(&self) -> Option<Rc<KeywordContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Info_flagContextAttrs<'input> for Info_flagContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn info_flag(&mut self) -> Result<Rc<Info_flagContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Info_flagContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 152, RULE_info_flag);
        let mut _localctx: Rc<Info_flagContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(936);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(59, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(928);
                        recog
                            .base
                            .match_token(PK_AllStatistics, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(929);
                        recog
                            .base
                            .match_token(PK_AssertionStackLevels, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(930);
                        recog.base.match_token(PK_Authors, &mut recog.err_handler)?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(931);
                        recog
                            .base
                            .match_token(PK_ErrorBehaviour, &mut recog.err_handler)?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        recog.base.set_state(932);
                        recog.base.match_token(PK_Name, &mut recog.err_handler)?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        recog.base.set_state(933);
                        recog
                            .base
                            .match_token(PK_ReasonUnknown, &mut recog.err_handler)?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        recog.base.set_state(934);
                        recog.base.match_token(PK_Version, &mut recog.err_handler)?;
                    }
                }
                8 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8);
                    recog.base.enter_outer_alt(None, 8);
                    {
                        /*InvokeRule keyword*/
                        recog.base.set_state(935);
                        recog.keyword()?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- error_behaviour ----------------
pub type Error_behaviourContextAll<'input> = Error_behaviourContext<'input>;

pub type Error_behaviourContext<'input> =
    BaseParserRuleContext<'input, Error_behaviourContextExt<'input>>;

#[derive(Clone)]
pub struct Error_behaviourContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Error_behaviourContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Error_behaviourContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_error_behaviour(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_error_behaviour(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Error_behaviourContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_error_behaviour(self);
    }
}

impl<'input> CustomRuleContext<'input> for Error_behaviourContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_error_behaviour
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_error_behaviour }
}
antlr_rust::type_id! {Error_behaviourContextExt<'a>}

impl<'input> Error_behaviourContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Error_behaviourContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Error_behaviourContextExt { ph: PhantomData },
        ))
    }
}

pub trait Error_behaviourContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Error_behaviourContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PS_ImmediateExit
    /// Returns `None` if there is no child corresponding to token PS_ImmediateExit
    fn PS_ImmediateExit(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_ImmediateExit, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_ContinuedExecution
    /// Returns `None` if there is no child corresponding to token PS_ContinuedExecution
    fn PS_ContinuedExecution(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_ContinuedExecution, 0)
    }
}

impl<'input> Error_behaviourContextAttrs<'input> for Error_behaviourContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn error_behaviour(&mut self) -> Result<Rc<Error_behaviourContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Error_behaviourContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 154, RULE_error_behaviour);
        let mut _localctx: Rc<Error_behaviourContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(938);
                _la = recog.base.input.la(1);
                if { !(_la == PS_ContinuedExecution || _la == PS_ImmediateExit) } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- reason_unknown ----------------
pub type Reason_unknownContextAll<'input> = Reason_unknownContext<'input>;

pub type Reason_unknownContext<'input> =
    BaseParserRuleContext<'input, Reason_unknownContextExt<'input>>;

#[derive(Clone)]
pub struct Reason_unknownContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Reason_unknownContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Reason_unknownContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_reason_unknown(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_reason_unknown(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Reason_unknownContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_reason_unknown(self);
    }
}

impl<'input> CustomRuleContext<'input> for Reason_unknownContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_reason_unknown
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_reason_unknown }
}
antlr_rust::type_id! {Reason_unknownContextExt<'a>}

impl<'input> Reason_unknownContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Reason_unknownContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Reason_unknownContextExt { ph: PhantomData },
        ))
    }
}

pub trait Reason_unknownContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Reason_unknownContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PS_Memout
    /// Returns `None` if there is no child corresponding to token PS_Memout
    fn PS_Memout(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Memout, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Incomplete
    /// Returns `None` if there is no child corresponding to token PS_Incomplete
    fn PS_Incomplete(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Incomplete, 0)
    }
    fn s_expr(&self) -> Option<Rc<S_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Reason_unknownContextAttrs<'input> for Reason_unknownContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn reason_unknown(&mut self) -> Result<Rc<Reason_unknownContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Reason_unknownContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 156, RULE_reason_unknown);
        let mut _localctx: Rc<Reason_unknownContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(943);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(60, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(940);
                        recog.base.match_token(PS_Memout, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(941);
                        recog
                            .base
                            .match_token(PS_Incomplete, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule s_expr*/
                        recog.base.set_state(942);
                        recog.s_expr()?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- model_response ----------------
pub type Model_responseContextAll<'input> = Model_responseContext<'input>;

pub type Model_responseContext<'input> =
    BaseParserRuleContext<'input, Model_responseContextExt<'input>>;

#[derive(Clone)]
pub struct Model_responseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Model_responseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Model_responseContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_model_response(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_model_response(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Model_responseContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_model_response(self);
    }
}

impl<'input> CustomRuleContext<'input> for Model_responseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_model_response
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_model_response }
}
antlr_rust::type_id! {Model_responseContextExt<'a>}

impl<'input> Model_responseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Model_responseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Model_responseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Model_responseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Model_responseContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token ParOpen in current rule
    fn ParOpen_all(&self) -> Vec<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ParOpen, starting from 0.
    /// Returns `None` if number of children corresponding to token ParOpen is less or equal than `i`.
    fn ParOpen(&self, i: usize) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, i)
    }
    /// Retrieves first TerminalNode corresponding to token CMD_DefineFun
    /// Returns `None` if there is no child corresponding to token CMD_DefineFun
    fn CMD_DefineFun(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_DefineFun, 0)
    }
    fn function_def(&self) -> Option<Rc<Function_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ParClose in current rule
    fn ParClose_all(&self) -> Vec<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ParClose, starting from 0.
    /// Returns `None` if number of children corresponding to token ParClose is less or equal than `i`.
    fn ParClose(&self, i: usize) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, i)
    }
    /// Retrieves first TerminalNode corresponding to token CMD_DefineFunRec
    /// Returns `None` if there is no child corresponding to token CMD_DefineFunRec
    fn CMD_DefineFunRec(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_DefineFunRec, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CMD_DefineFunsRec
    /// Returns `None` if there is no child corresponding to token CMD_DefineFunsRec
    fn CMD_DefineFunsRec(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CMD_DefineFunsRec, 0)
    }
    fn function_dec_all(&self) -> Vec<Rc<Function_decContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn function_dec(&self, i: usize) -> Option<Rc<Function_decContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn term_all(&self) -> Vec<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Model_responseContextAttrs<'input> for Model_responseContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn model_response(&mut self) -> Result<Rc<Model_responseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Model_responseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 158, RULE_model_response);
        let mut _localctx: Rc<Model_responseContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(973);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(63, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(945);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(946);
                        recog
                            .base
                            .match_token(CMD_DefineFun, &mut recog.err_handler)?;

                        /*InvokeRule function_def*/
                        recog.base.set_state(947);
                        recog.function_def()?;

                        recog.base.set_state(948);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(950);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(951);
                        recog
                            .base
                            .match_token(CMD_DefineFunRec, &mut recog.err_handler)?;

                        /*InvokeRule function_def*/
                        recog.base.set_state(952);
                        recog.function_def()?;

                        recog.base.set_state(953);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(955);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(956);
                        recog
                            .base
                            .match_token(CMD_DefineFunsRec, &mut recog.err_handler)?;

                        recog.base.set_state(957);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(959);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule function_dec*/
                                    recog.base.set_state(958);
                                    recog.function_dec()?;
                                }
                            }
                            recog.base.set_state(961);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !(_la == ParOpen) {
                                break;
                            }
                        }
                        recog.base.set_state(963);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        recog.base.set_state(964);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(966);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule term*/
                                    recog.base.set_state(965);
                                    recog.term()?;
                                }
                            }
                            recog.base.set_state(968);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !((((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << ParOpen)
                                        | (1usize << String)
                                        | (1usize << QuotedSymbol)
                                        | (1usize << PS_Not)
                                        | (1usize << PS_Bool)
                                        | (1usize << PS_ContinuedExecution)
                                        | (1usize << PS_Error)
                                        | (1usize << PS_False)
                                        | (1usize << PS_ImmediateExit)
                                        | (1usize << PS_Incomplete)
                                        | (1usize << PS_Logic)
                                        | (1usize << PS_Memout)
                                        | (1usize << PS_Sat)
                                        | (1usize << PS_Success)
                                        | (1usize << PS_Theory)
                                        | (1usize << PS_True)
                                        | (1usize << PS_Unknown)
                                        | (1usize << PS_Unsupported)
                                        | (1usize << PS_Unsat)))
                                    != 0)
                                || (((_la - 66) & !0x3f) == 0
                                    && ((1usize << (_la - 66))
                                        & ((1usize << (Numeral - 66))
                                            | (1usize << (Binary - 66))
                                            | (1usize << (HexDecimal - 66))
                                            | (1usize << (Decimal - 66))
                                            | (1usize << (UndefinedSymbol - 66))))
                                        != 0))
                            {
                                break;
                            }
                        }
                        recog.base.set_state(970);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;

                        recog.base.set_state(971);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- info_response ----------------
pub type Info_responseContextAll<'input> = Info_responseContext<'input>;

pub type Info_responseContext<'input> =
    BaseParserRuleContext<'input, Info_responseContextExt<'input>>;

#[derive(Clone)]
pub struct Info_responseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Info_responseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Info_responseContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_info_response(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_info_response(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Info_responseContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_info_response(self);
    }
}

impl<'input> CustomRuleContext<'input> for Info_responseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_info_response
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_info_response }
}
antlr_rust::type_id! {Info_responseContextExt<'a>}

impl<'input> Info_responseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Info_responseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Info_responseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Info_responseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Info_responseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PK_AssertionStackLevels
    /// Returns `None` if there is no child corresponding to token PK_AssertionStackLevels
    fn PK_AssertionStackLevels(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_AssertionStackLevels, 0)
    }
    fn numeral(&self) -> Option<Rc<NumeralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Authors
    /// Returns `None` if there is no child corresponding to token PK_Authors
    fn PK_Authors(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Authors, 0)
    }
    fn string(&self) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ErrorBehaviour
    /// Returns `None` if there is no child corresponding to token PK_ErrorBehaviour
    fn PK_ErrorBehaviour(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ErrorBehaviour, 0)
    }
    fn error_behaviour(&self) -> Option<Rc<Error_behaviourContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Name
    /// Returns `None` if there is no child corresponding to token PK_Name
    fn PK_Name(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Name, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_ReasonUnknown
    /// Returns `None` if there is no child corresponding to token PK_ReasonUnknown
    fn PK_ReasonUnknown(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_ReasonUnknown, 0)
    }
    fn reason_unknown(&self) -> Option<Rc<Reason_unknownContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PK_Version
    /// Returns `None` if there is no child corresponding to token PK_Version
    fn PK_Version(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PK_Version, 0)
    }
    fn attribute(&self) -> Option<Rc<AttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Info_responseContextAttrs<'input> for Info_responseContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn info_response(&mut self) -> Result<Rc<Info_responseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Info_responseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 160, RULE_info_response);
        let mut _localctx: Rc<Info_responseContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(988);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(64, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(975);
                        recog
                            .base
                            .match_token(PK_AssertionStackLevels, &mut recog.err_handler)?;

                        /*InvokeRule numeral*/
                        recog.base.set_state(976);
                        recog.numeral()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(977);
                        recog.base.match_token(PK_Authors, &mut recog.err_handler)?;

                        /*InvokeRule string*/
                        recog.base.set_state(978);
                        recog.string()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(979);
                        recog
                            .base
                            .match_token(PK_ErrorBehaviour, &mut recog.err_handler)?;

                        /*InvokeRule error_behaviour*/
                        recog.base.set_state(980);
                        recog.error_behaviour()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(981);
                        recog.base.match_token(PK_Name, &mut recog.err_handler)?;

                        /*InvokeRule string*/
                        recog.base.set_state(982);
                        recog.string()?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        recog.base.set_state(983);
                        recog
                            .base
                            .match_token(PK_ReasonUnknown, &mut recog.err_handler)?;

                        /*InvokeRule reason_unknown*/
                        recog.base.set_state(984);
                        recog.reason_unknown()?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        recog.base.set_state(985);
                        recog.base.match_token(PK_Version, &mut recog.err_handler)?;

                        /*InvokeRule string*/
                        recog.base.set_state(986);
                        recog.string()?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        /*InvokeRule attribute*/
                        recog.base.set_state(987);
                        recog.attribute()?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- valuation_pair ----------------
pub type Valuation_pairContextAll<'input> = Valuation_pairContext<'input>;

pub type Valuation_pairContext<'input> =
    BaseParserRuleContext<'input, Valuation_pairContextExt<'input>>;

#[derive(Clone)]
pub struct Valuation_pairContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Valuation_pairContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Valuation_pairContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_valuation_pair(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_valuation_pair(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Valuation_pairContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_valuation_pair(self);
    }
}

impl<'input> CustomRuleContext<'input> for Valuation_pairContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_valuation_pair
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_valuation_pair }
}
antlr_rust::type_id! {Valuation_pairContextExt<'a>}

impl<'input> Valuation_pairContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Valuation_pairContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Valuation_pairContextExt { ph: PhantomData },
        ))
    }
}

pub trait Valuation_pairContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Valuation_pairContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    fn term_all(&self) -> Vec<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
}

impl<'input> Valuation_pairContextAttrs<'input> for Valuation_pairContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn valuation_pair(&mut self) -> Result<Rc<Valuation_pairContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Valuation_pairContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 162, RULE_valuation_pair);
        let mut _localctx: Rc<Valuation_pairContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(990);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                /*InvokeRule term*/
                recog.base.set_state(991);
                recog.term()?;

                /*InvokeRule term*/
                recog.base.set_state(992);
                recog.term()?;

                recog.base.set_state(993);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- t_valuation_pair ----------------
pub type T_valuation_pairContextAll<'input> = T_valuation_pairContext<'input>;

pub type T_valuation_pairContext<'input> =
    BaseParserRuleContext<'input, T_valuation_pairContextExt<'input>>;

#[derive(Clone)]
pub struct T_valuation_pairContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for T_valuation_pairContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for T_valuation_pairContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_t_valuation_pair(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_t_valuation_pair(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for T_valuation_pairContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_t_valuation_pair(self);
    }
}

impl<'input> CustomRuleContext<'input> for T_valuation_pairContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_t_valuation_pair
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_t_valuation_pair }
}
antlr_rust::type_id! {T_valuation_pairContextExt<'a>}

impl<'input> T_valuation_pairContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<T_valuation_pairContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            T_valuation_pairContextExt { ph: PhantomData },
        ))
    }
}

pub trait T_valuation_pairContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<T_valuation_pairContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    fn symbol(&self) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn b_value(&self) -> Option<Rc<B_valueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
}

impl<'input> T_valuation_pairContextAttrs<'input> for T_valuation_pairContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn t_valuation_pair(
        &mut self,
    ) -> Result<Rc<T_valuation_pairContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            T_valuation_pairContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 164, RULE_t_valuation_pair);
        let mut _localctx: Rc<T_valuation_pairContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(995);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                /*InvokeRule symbol*/
                recog.base.set_state(996);
                recog.symbol()?;

                /*InvokeRule b_value*/
                recog.base.set_state(997);
                recog.b_value()?;

                recog.base.set_state(998);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- check_sat_response ----------------
pub type Check_sat_responseContextAll<'input> = Check_sat_responseContext<'input>;

pub type Check_sat_responseContext<'input> =
    BaseParserRuleContext<'input, Check_sat_responseContextExt<'input>>;

#[derive(Clone)]
pub struct Check_sat_responseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Check_sat_responseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Check_sat_responseContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_check_sat_response(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_check_sat_response(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Check_sat_responseContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_check_sat_response(self);
    }
}

impl<'input> CustomRuleContext<'input> for Check_sat_responseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_check_sat_response
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_check_sat_response }
}
antlr_rust::type_id! {Check_sat_responseContextExt<'a>}

impl<'input> Check_sat_responseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Check_sat_responseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Check_sat_responseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Check_sat_responseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Check_sat_responseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PS_Sat
    /// Returns `None` if there is no child corresponding to token PS_Sat
    fn PS_Sat(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Sat, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Unsat
    /// Returns `None` if there is no child corresponding to token PS_Unsat
    fn PS_Unsat(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Unsat, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Unknown
    /// Returns `None` if there is no child corresponding to token PS_Unknown
    fn PS_Unknown(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Unknown, 0)
    }
}

impl<'input> Check_sat_responseContextAttrs<'input> for Check_sat_responseContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn check_sat_response(
        &mut self,
    ) -> Result<Rc<Check_sat_responseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Check_sat_responseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 166, RULE_check_sat_response);
        let mut _localctx: Rc<Check_sat_responseContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1000);
                _la = recog.base.input.la(1);
                if {
                    !(((_la) & !0x3f) == 0
                        && ((1usize << _la)
                            & ((1usize << PS_Sat) | (1usize << PS_Unknown) | (1usize << PS_Unsat)))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- echo_response ----------------
pub type Echo_responseContextAll<'input> = Echo_responseContext<'input>;

pub type Echo_responseContext<'input> =
    BaseParserRuleContext<'input, Echo_responseContextExt<'input>>;

#[derive(Clone)]
pub struct Echo_responseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Echo_responseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for Echo_responseContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_echo_response(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_echo_response(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Echo_responseContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_echo_response(self);
    }
}

impl<'input> CustomRuleContext<'input> for Echo_responseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_echo_response
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_echo_response }
}
antlr_rust::type_id! {Echo_responseContextExt<'a>}

impl<'input> Echo_responseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Echo_responseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Echo_responseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Echo_responseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Echo_responseContextExt<'input>>
{
    fn string(&self) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Echo_responseContextAttrs<'input> for Echo_responseContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn echo_response(&mut self) -> Result<Rc<Echo_responseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Echo_responseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 168, RULE_echo_response);
        let mut _localctx: Rc<Echo_responseContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule string*/
                recog.base.set_state(1002);
                recog.string()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- get_assertions_response ----------------
pub type Get_assertions_responseContextAll<'input> = Get_assertions_responseContext<'input>;

pub type Get_assertions_responseContext<'input> =
    BaseParserRuleContext<'input, Get_assertions_responseContextExt<'input>>;

#[derive(Clone)]
pub struct Get_assertions_responseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Get_assertions_responseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Get_assertions_responseContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_get_assertions_response(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_get_assertions_response(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a>
    for Get_assertions_responseContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_get_assertions_response(self);
    }
}

impl<'input> CustomRuleContext<'input> for Get_assertions_responseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_get_assertions_response
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_get_assertions_response }
}
antlr_rust::type_id! {Get_assertions_responseContextExt<'a>}

impl<'input> Get_assertions_responseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Get_assertions_responseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Get_assertions_responseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Get_assertions_responseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Get_assertions_responseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn term_all(&self) -> Vec<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Get_assertions_responseContextAttrs<'input>
    for Get_assertions_responseContext<'input>
{
}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_assertions_response(
        &mut self,
    ) -> Result<Rc<Get_assertions_responseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Get_assertions_responseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 170, RULE_get_assertions_response);
        let mut _localctx: Rc<Get_assertions_responseContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1004);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                recog.base.set_state(1008);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << ParOpen)
                            | (1usize << String)
                            | (1usize << QuotedSymbol)
                            | (1usize << PS_Not)
                            | (1usize << PS_Bool)
                            | (1usize << PS_ContinuedExecution)
                            | (1usize << PS_Error)
                            | (1usize << PS_False)
                            | (1usize << PS_ImmediateExit)
                            | (1usize << PS_Incomplete)
                            | (1usize << PS_Logic)
                            | (1usize << PS_Memout)
                            | (1usize << PS_Sat)
                            | (1usize << PS_Success)
                            | (1usize << PS_Theory)
                            | (1usize << PS_True)
                            | (1usize << PS_Unknown)
                            | (1usize << PS_Unsupported)
                            | (1usize << PS_Unsat)))
                        != 0)
                    || (((_la - 66) & !0x3f) == 0
                        && ((1usize << (_la - 66))
                            & ((1usize << (Numeral - 66))
                                | (1usize << (Binary - 66))
                                | (1usize << (HexDecimal - 66))
                                | (1usize << (Decimal - 66))
                                | (1usize << (UndefinedSymbol - 66))))
                            != 0)
                {
                    {
                        {
                            /*InvokeRule term*/
                            recog.base.set_state(1005);
                            recog.term()?;
                        }
                    }
                    recog.base.set_state(1010);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(1011);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- get_assignment_response ----------------
pub type Get_assignment_responseContextAll<'input> = Get_assignment_responseContext<'input>;

pub type Get_assignment_responseContext<'input> =
    BaseParserRuleContext<'input, Get_assignment_responseContextExt<'input>>;

#[derive(Clone)]
pub struct Get_assignment_responseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Get_assignment_responseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Get_assignment_responseContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_get_assignment_response(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_get_assignment_response(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a>
    for Get_assignment_responseContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_get_assignment_response(self);
    }
}

impl<'input> CustomRuleContext<'input> for Get_assignment_responseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_get_assignment_response
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_get_assignment_response }
}
antlr_rust::type_id! {Get_assignment_responseContextExt<'a>}

impl<'input> Get_assignment_responseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Get_assignment_responseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Get_assignment_responseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Get_assignment_responseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Get_assignment_responseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn t_valuation_pair_all(&self) -> Vec<Rc<T_valuation_pairContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn t_valuation_pair(&self, i: usize) -> Option<Rc<T_valuation_pairContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Get_assignment_responseContextAttrs<'input>
    for Get_assignment_responseContext<'input>
{
}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_assignment_response(
        &mut self,
    ) -> Result<Rc<Get_assignment_responseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Get_assignment_responseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 172, RULE_get_assignment_response);
        let mut _localctx: Rc<Get_assignment_responseContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1013);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                recog.base.set_state(1017);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == ParOpen {
                    {
                        {
                            /*InvokeRule t_valuation_pair*/
                            recog.base.set_state(1014);
                            recog.t_valuation_pair()?;
                        }
                    }
                    recog.base.set_state(1019);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(1020);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- get_info_response ----------------
pub type Get_info_responseContextAll<'input> = Get_info_responseContext<'input>;

pub type Get_info_responseContext<'input> =
    BaseParserRuleContext<'input, Get_info_responseContextExt<'input>>;

#[derive(Clone)]
pub struct Get_info_responseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Get_info_responseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Get_info_responseContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_get_info_response(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_get_info_response(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Get_info_responseContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_get_info_response(self);
    }
}

impl<'input> CustomRuleContext<'input> for Get_info_responseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_get_info_response
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_get_info_response }
}
antlr_rust::type_id! {Get_info_responseContextExt<'a>}

impl<'input> Get_info_responseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Get_info_responseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Get_info_responseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Get_info_responseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Get_info_responseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn info_response_all(&self) -> Vec<Rc<Info_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn info_response(&self, i: usize) -> Option<Rc<Info_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Get_info_responseContextAttrs<'input> for Get_info_responseContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_info_response(
        &mut self,
    ) -> Result<Rc<Get_info_responseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Get_info_responseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 174, RULE_get_info_response);
        let mut _localctx: Rc<Get_info_responseContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1022);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                recog.base.set_state(1024);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule info_response*/
                            recog.base.set_state(1023);
                            recog.info_response()?;
                        }
                    }
                    recog.base.set_state(1026);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(((_la - 70) & !0x3f) == 0
                        && ((1usize << (_la - 70))
                            & ((1usize << (Colon - 70))
                                | (1usize << (PK_AllStatistics - 70))
                                | (1usize << (PK_AssertionStackLevels - 70))
                                | (1usize << (PK_Authors - 70))
                                | (1usize << (PK_Category - 70))
                                | (1usize << (PK_Chainable - 70))
                                | (1usize << (PK_Definition - 70))
                                | (1usize << (PK_DiagnosticOutputChannel - 70))
                                | (1usize << (PK_ErrorBehaviour - 70))
                                | (1usize << (PK_Extension - 70))
                                | (1usize << (PK_Funs - 70))
                                | (1usize << (PK_FunsDescription - 70))
                                | (1usize << (PK_GlobalDeclarations - 70))
                                | (1usize << (PK_InteractiveMode - 70))
                                | (1usize << (PK_Language - 70))
                                | (1usize << (PK_LeftAssoc - 70))
                                | (1usize << (PK_License - 70))
                                | (1usize << (PK_Named - 70))
                                | (1usize << (PK_Name - 70))
                                | (1usize << (PK_Notes - 70))
                                | (1usize << (PK_Pattern - 70))
                                | (1usize << (PK_PrintSuccess - 70))
                                | (1usize << (PK_ProduceAssertions - 70))
                                | (1usize << (PK_ProduceAssignments - 70))
                                | (1usize << (PK_ProduceModels - 70))
                                | (1usize << (PK_ProduceProofs - 70))
                                | (1usize << (PK_ProduceUnsatAssumptions - 70))
                                | (1usize << (PK_ProduceUnsatCores - 70))
                                | (1usize << (PK_RandomSeed - 70))
                                | (1usize << (PK_ReasonUnknown - 70))
                                | (1usize << (PK_RegularOutputChannel - 70))
                                | (1usize << (PK_ReproducibleResourceLimit - 70))
                                | (1usize << (PK_RightAssoc - 70))
                                | (1usize << (PK_SmtLibVersion - 70))
                                | (1usize << (PK_Sorts - 70))
                                | (1usize << (PK_SortsDescription - 70))
                                | (1usize << (PK_Source - 70))
                                | (1usize << (PK_Status - 70))
                                | (1usize << (PK_Theories - 70))
                                | (1usize << (PK_Values - 70))
                                | (1usize << (PK_Verbosity - 70))
                                | (1usize << (PK_Version - 70))))
                            != 0)
                    {
                        break;
                    }
                }
                recog.base.set_state(1028);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- get_model_response ----------------
pub type Get_model_responseContextAll<'input> = Get_model_responseContext<'input>;

pub type Get_model_responseContext<'input> =
    BaseParserRuleContext<'input, Get_model_responseContextExt<'input>>;

#[derive(Clone)]
pub struct Get_model_responseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Get_model_responseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Get_model_responseContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_get_model_response(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_get_model_response(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Get_model_responseContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_get_model_response(self);
    }
}

impl<'input> CustomRuleContext<'input> for Get_model_responseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_get_model_response
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_get_model_response }
}
antlr_rust::type_id! {Get_model_responseContextExt<'a>}

impl<'input> Get_model_responseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Get_model_responseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Get_model_responseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Get_model_responseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Get_model_responseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn model_response_all(&self) -> Vec<Rc<Model_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn model_response(&self, i: usize) -> Option<Rc<Model_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Get_model_responseContextAttrs<'input> for Get_model_responseContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_model_response(
        &mut self,
    ) -> Result<Rc<Get_model_responseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Get_model_responseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 176, RULE_get_model_response);
        let mut _localctx: Rc<Get_model_responseContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1030);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                recog.base.set_state(1034);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == ParOpen {
                    {
                        {
                            /*InvokeRule model_response*/
                            recog.base.set_state(1031);
                            recog.model_response()?;
                        }
                    }
                    recog.base.set_state(1036);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(1037);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- get_option_response ----------------
pub type Get_option_responseContextAll<'input> = Get_option_responseContext<'input>;

pub type Get_option_responseContext<'input> =
    BaseParserRuleContext<'input, Get_option_responseContextExt<'input>>;

#[derive(Clone)]
pub struct Get_option_responseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Get_option_responseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Get_option_responseContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_get_option_response(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_get_option_response(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a>
    for Get_option_responseContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_get_option_response(self);
    }
}

impl<'input> CustomRuleContext<'input> for Get_option_responseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_get_option_response
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_get_option_response }
}
antlr_rust::type_id! {Get_option_responseContextExt<'a>}

impl<'input> Get_option_responseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Get_option_responseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Get_option_responseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Get_option_responseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Get_option_responseContextExt<'input>>
{
    fn attribute_value(&self) -> Option<Rc<Attribute_valueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Get_option_responseContextAttrs<'input> for Get_option_responseContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_option_response(
        &mut self,
    ) -> Result<Rc<Get_option_responseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Get_option_responseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 178, RULE_get_option_response);
        let mut _localctx: Rc<Get_option_responseContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule attribute_value*/
                recog.base.set_state(1039);
                recog.attribute_value()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- get_proof_response ----------------
pub type Get_proof_responseContextAll<'input> = Get_proof_responseContext<'input>;

pub type Get_proof_responseContext<'input> =
    BaseParserRuleContext<'input, Get_proof_responseContextExt<'input>>;

#[derive(Clone)]
pub struct Get_proof_responseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Get_proof_responseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Get_proof_responseContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_get_proof_response(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_get_proof_response(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Get_proof_responseContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_get_proof_response(self);
    }
}

impl<'input> CustomRuleContext<'input> for Get_proof_responseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_get_proof_response
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_get_proof_response }
}
antlr_rust::type_id! {Get_proof_responseContextExt<'a>}

impl<'input> Get_proof_responseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Get_proof_responseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Get_proof_responseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Get_proof_responseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Get_proof_responseContextExt<'input>>
{
    fn s_expr(&self) -> Option<Rc<S_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Get_proof_responseContextAttrs<'input> for Get_proof_responseContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_proof_response(
        &mut self,
    ) -> Result<Rc<Get_proof_responseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Get_proof_responseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 180, RULE_get_proof_response);
        let mut _localctx: Rc<Get_proof_responseContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule s_expr*/
                recog.base.set_state(1041);
                recog.s_expr()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- get_unsat_assump_response ----------------
pub type Get_unsat_assump_responseContextAll<'input> = Get_unsat_assump_responseContext<'input>;

pub type Get_unsat_assump_responseContext<'input> =
    BaseParserRuleContext<'input, Get_unsat_assump_responseContextExt<'input>>;

#[derive(Clone)]
pub struct Get_unsat_assump_responseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Get_unsat_assump_responseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Get_unsat_assump_responseContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_get_unsat_assump_response(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_get_unsat_assump_response(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a>
    for Get_unsat_assump_responseContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_get_unsat_assump_response(self);
    }
}

impl<'input> CustomRuleContext<'input> for Get_unsat_assump_responseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_get_unsat_assump_response
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_get_unsat_assump_response }
}
antlr_rust::type_id! {Get_unsat_assump_responseContextExt<'a>}

impl<'input> Get_unsat_assump_responseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Get_unsat_assump_responseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Get_unsat_assump_responseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Get_unsat_assump_responseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Get_unsat_assump_responseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn symbol_all(&self) -> Vec<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn symbol(&self, i: usize) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Get_unsat_assump_responseContextAttrs<'input>
    for Get_unsat_assump_responseContext<'input>
{
}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_unsat_assump_response(
        &mut self,
    ) -> Result<Rc<Get_unsat_assump_responseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Get_unsat_assump_responseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 182, RULE_get_unsat_assump_response);
        let mut _localctx: Rc<Get_unsat_assump_responseContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1043);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                recog.base.set_state(1047);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << QuotedSymbol)
                            | (1usize << PS_Not)
                            | (1usize << PS_Bool)
                            | (1usize << PS_ContinuedExecution)
                            | (1usize << PS_Error)
                            | (1usize << PS_False)
                            | (1usize << PS_ImmediateExit)
                            | (1usize << PS_Incomplete)
                            | (1usize << PS_Logic)
                            | (1usize << PS_Memout)
                            | (1usize << PS_Sat)
                            | (1usize << PS_Success)
                            | (1usize << PS_Theory)
                            | (1usize << PS_True)
                            | (1usize << PS_Unknown)
                            | (1usize << PS_Unsupported)
                            | (1usize << PS_Unsat)))
                        != 0)
                    || _la == UndefinedSymbol
                {
                    {
                        {
                            /*InvokeRule symbol*/
                            recog.base.set_state(1044);
                            recog.symbol()?;
                        }
                    }
                    recog.base.set_state(1049);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(1050);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- get_unsat_core_response ----------------
pub type Get_unsat_core_responseContextAll<'input> = Get_unsat_core_responseContext<'input>;

pub type Get_unsat_core_responseContext<'input> =
    BaseParserRuleContext<'input, Get_unsat_core_responseContextExt<'input>>;

#[derive(Clone)]
pub struct Get_unsat_core_responseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Get_unsat_core_responseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Get_unsat_core_responseContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_get_unsat_core_response(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_get_unsat_core_response(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a>
    for Get_unsat_core_responseContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_get_unsat_core_response(self);
    }
}

impl<'input> CustomRuleContext<'input> for Get_unsat_core_responseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_get_unsat_core_response
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_get_unsat_core_response }
}
antlr_rust::type_id! {Get_unsat_core_responseContextExt<'a>}

impl<'input> Get_unsat_core_responseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Get_unsat_core_responseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Get_unsat_core_responseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Get_unsat_core_responseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Get_unsat_core_responseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn symbol_all(&self) -> Vec<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn symbol(&self, i: usize) -> Option<Rc<SymbolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Get_unsat_core_responseContextAttrs<'input>
    for Get_unsat_core_responseContext<'input>
{
}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_unsat_core_response(
        &mut self,
    ) -> Result<Rc<Get_unsat_core_responseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Get_unsat_core_responseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 184, RULE_get_unsat_core_response);
        let mut _localctx: Rc<Get_unsat_core_responseContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1052);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                recog.base.set_state(1056);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << QuotedSymbol)
                            | (1usize << PS_Not)
                            | (1usize << PS_Bool)
                            | (1usize << PS_ContinuedExecution)
                            | (1usize << PS_Error)
                            | (1usize << PS_False)
                            | (1usize << PS_ImmediateExit)
                            | (1usize << PS_Incomplete)
                            | (1usize << PS_Logic)
                            | (1usize << PS_Memout)
                            | (1usize << PS_Sat)
                            | (1usize << PS_Success)
                            | (1usize << PS_Theory)
                            | (1usize << PS_True)
                            | (1usize << PS_Unknown)
                            | (1usize << PS_Unsupported)
                            | (1usize << PS_Unsat)))
                        != 0)
                    || _la == UndefinedSymbol
                {
                    {
                        {
                            /*InvokeRule symbol*/
                            recog.base.set_state(1053);
                            recog.symbol()?;
                        }
                    }
                    recog.base.set_state(1058);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(1059);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- get_value_response ----------------
pub type Get_value_responseContextAll<'input> = Get_value_responseContext<'input>;

pub type Get_value_responseContext<'input> =
    BaseParserRuleContext<'input, Get_value_responseContextExt<'input>>;

#[derive(Clone)]
pub struct Get_value_responseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Get_value_responseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Get_value_responseContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_get_value_response(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_get_value_response(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for Get_value_responseContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_get_value_response(self);
    }
}

impl<'input> CustomRuleContext<'input> for Get_value_responseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_get_value_response
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_get_value_response }
}
antlr_rust::type_id! {Get_value_responseContextExt<'a>}

impl<'input> Get_value_responseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Get_value_responseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Get_value_responseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Get_value_responseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Get_value_responseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
    fn valuation_pair_all(&self) -> Vec<Rc<Valuation_pairContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn valuation_pair(&self, i: usize) -> Option<Rc<Valuation_pairContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Get_value_responseContextAttrs<'input> for Get_value_responseContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_value_response(
        &mut self,
    ) -> Result<Rc<Get_value_responseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Get_value_responseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 186, RULE_get_value_response);
        let mut _localctx: Rc<Get_value_responseContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1061);
                recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                recog.base.set_state(1063);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule valuation_pair*/
                            recog.base.set_state(1062);
                            recog.valuation_pair()?;
                        }
                    }
                    recog.base.set_state(1065);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(_la == ParOpen) {
                        break;
                    }
                }
                recog.base.set_state(1067);
                recog.base.match_token(ParClose, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- specific_success_response ----------------
pub type Specific_success_responseContextAll<'input> = Specific_success_responseContext<'input>;

pub type Specific_success_responseContext<'input> =
    BaseParserRuleContext<'input, Specific_success_responseContextExt<'input>>;

#[derive(Clone)]
pub struct Specific_success_responseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for Specific_success_responseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a>
    for Specific_success_responseContext<'input>
{
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_specific_success_response(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_specific_success_response(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a>
    for Specific_success_responseContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_specific_success_response(self);
    }
}

impl<'input> CustomRuleContext<'input> for Specific_success_responseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_specific_success_response
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_specific_success_response }
}
antlr_rust::type_id! {Specific_success_responseContextExt<'a>}

impl<'input> Specific_success_responseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Specific_success_responseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Specific_success_responseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Specific_success_responseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<Specific_success_responseContextExt<'input>>
{
    fn check_sat_response(&self) -> Option<Rc<Check_sat_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn echo_response(&self) -> Option<Rc<Echo_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn get_assertions_response(&self) -> Option<Rc<Get_assertions_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn get_assignment_response(&self) -> Option<Rc<Get_assignment_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn get_info_response(&self) -> Option<Rc<Get_info_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn get_model_response(&self) -> Option<Rc<Get_model_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn get_option_response(&self) -> Option<Rc<Get_option_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn get_proof_response(&self) -> Option<Rc<Get_proof_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn get_unsat_assump_response(&self) -> Option<Rc<Get_unsat_assump_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn get_unsat_core_response(&self) -> Option<Rc<Get_unsat_core_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn get_value_response(&self) -> Option<Rc<Get_value_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Specific_success_responseContextAttrs<'input>
    for Specific_success_responseContext<'input>
{
}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn specific_success_response(
        &mut self,
    ) -> Result<Rc<Specific_success_responseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Specific_success_responseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 188, RULE_specific_success_response);
        let mut _localctx: Rc<Specific_success_responseContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(1080);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(72, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule check_sat_response*/
                        recog.base.set_state(1069);
                        recog.check_sat_response()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule echo_response*/
                        recog.base.set_state(1070);
                        recog.echo_response()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule get_assertions_response*/
                        recog.base.set_state(1071);
                        recog.get_assertions_response()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        /*InvokeRule get_assignment_response*/
                        recog.base.set_state(1072);
                        recog.get_assignment_response()?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        /*InvokeRule get_info_response*/
                        recog.base.set_state(1073);
                        recog.get_info_response()?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        /*InvokeRule get_model_response*/
                        recog.base.set_state(1074);
                        recog.get_model_response()?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        /*InvokeRule get_option_response*/
                        recog.base.set_state(1075);
                        recog.get_option_response()?;
                    }
                }
                8 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8);
                    recog.base.enter_outer_alt(None, 8);
                    {
                        /*InvokeRule get_proof_response*/
                        recog.base.set_state(1076);
                        recog.get_proof_response()?;
                    }
                }
                9 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 9);
                    recog.base.enter_outer_alt(None, 9);
                    {
                        /*InvokeRule get_unsat_assump_response*/
                        recog.base.set_state(1077);
                        recog.get_unsat_assump_response()?;
                    }
                }
                10 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 10);
                    recog.base.enter_outer_alt(None, 10);
                    {
                        /*InvokeRule get_unsat_core_response*/
                        recog.base.set_state(1078);
                        recog.get_unsat_core_response()?;
                    }
                }
                11 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 11);
                    recog.base.enter_outer_alt(None, 11);
                    {
                        /*InvokeRule get_value_response*/
                        recog.base.set_state(1079);
                        recog.get_value_response()?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- general_response ----------------
pub type General_responseContextAll<'input> = General_responseContext<'input>;

pub type General_responseContext<'input> =
    BaseParserRuleContext<'input, General_responseContextExt<'input>>;

#[derive(Clone)]
pub struct General_responseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SMTLIBv2ParserContext<'input> for General_responseContext<'input> {}

impl<'input, 'a> Listenable<dyn SMTLIBv2Listener<'input> + 'a> for General_responseContext<'input> {
    fn enter(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_general_response(self);
    }
    fn exit(&self, listener: &mut (dyn SMTLIBv2Listener<'input> + 'a)) {
        listener.exit_general_response(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SMTLIBv2Visitor<'input> + 'a> for General_responseContext<'input> {
    fn accept(&self, visitor: &mut (dyn SMTLIBv2Visitor<'input> + 'a)) {
        visitor.visit_general_response(self);
    }
}

impl<'input> CustomRuleContext<'input> for General_responseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SMTLIBv2ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_general_response
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_general_response }
}
antlr_rust::type_id! {General_responseContextExt<'a>}

impl<'input> General_responseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SMTLIBv2ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<General_responseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            General_responseContextExt { ph: PhantomData },
        ))
    }
}

pub trait General_responseContextAttrs<'input>:
    SMTLIBv2ParserContext<'input> + BorrowMut<General_responseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PS_Success
    /// Returns `None` if there is no child corresponding to token PS_Success
    fn PS_Success(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Success, 0)
    }
    fn specific_success_response(&self) -> Option<Rc<Specific_success_responseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Unsupported
    /// Returns `None` if there is no child corresponding to token PS_Unsupported
    fn PS_Unsupported(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Unsupported, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ParOpen
    /// Returns `None` if there is no child corresponding to token ParOpen
    fn ParOpen(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParOpen, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PS_Error
    /// Returns `None` if there is no child corresponding to token PS_Error
    fn PS_Error(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PS_Error, 0)
    }
    fn string(&self) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ParClose
    /// Returns `None` if there is no child corresponding to token ParClose
    fn ParClose(&self) -> Option<Rc<TerminalNode<'input, SMTLIBv2ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ParClose, 0)
    }
}

impl<'input> General_responseContextAttrs<'input> for General_responseContext<'input> {}

impl<'input, I, H> SMTLIBv2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn general_response(
        &mut self,
    ) -> Result<Rc<General_responseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            General_responseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 190, RULE_general_response);
        let mut _localctx: Rc<General_responseContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(1090);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(73, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(1082);
                        recog.base.match_token(PS_Success, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule specific_success_response*/
                        recog.base.set_state(1083);
                        recog.specific_success_response()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(1084);
                        recog
                            .base
                            .match_token(PS_Unsupported, &mut recog.err_handler)?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(1085);
                        recog.base.match_token(ParOpen, &mut recog.err_handler)?;

                        recog.base.set_state(1086);
                        recog.base.match_token(PS_Error, &mut recog.err_handler)?;

                        /*InvokeRule string*/
                        recog.base.set_state(1087);
                        recog.string()?;

                        recog.base.set_state(1088);
                        recog.base.match_token(ParClose, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x73\u{447}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x04\x42\x09\x42\x04\x43\x09\x43\x04\x44\x09\
	\x44\x04\x45\x09\x45\x04\x46\x09\x46\x04\x47\x09\x47\x04\x48\x09\x48\x04\
	\x49\x09\x49\x04\x4a\x09\x4a\x04\x4b\x09\x4b\x04\x4c\x09\x4c\x04\x4d\x09\
	\x4d\x04\x4e\x09\x4e\x04\x4f\x09\x4f\x04\x50\x09\x50\x04\x51\x09\x51\x04\
	\x52\x09\x52\x04\x53\x09\x53\x04\x54\x09\x54\x04\x55\x09\x55\x04\x56\x09\
	\x56\x04\x57\x09\x57\x04\x58\x09\x58\x04\x59\x09\x59\x04\x5a\x09\x5a\x04\
	\x5b\x09\x5b\x04\x5c\x09\x5c\x04\x5d\x09\x5d\x04\x5e\x09\x5e\x04\x5f\x09\
	\x5f\x04\x60\x09\x60\x04\x61\x09\x61\x03\x02\x03\x02\x03\x02\x03\x03\x03\
	\x03\x03\x03\x03\x04\x03\x04\x03\x05\x03\x05\x05\x05\u{cd}\x0a\x05\x03\x06\
	\x03\x06\x03\x07\x03\x07\x03\x08\x03\x08\x03\x09\x03\x09\x05\x09\u{d7}\x0a\
	\x09\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\
	\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x0f\x05\x0f\u{e6}\x0a\x0f\x03\x10\x03\x10\
	\x03\x10\x03\x10\x03\x10\x05\x10\u{ed}\x0a\x10\x03\x11\x03\x11\x03\x11\x03\
	\x11\x03\x11\x07\x11\u{f4}\x0a\x11\x0c\x11\x0e\x11\u{f7}\x0b\x11\x03\x11\
	\x05\x11\u{fa}\x0a\x11\x03\x12\x03\x12\x05\x12\u{fe}\x0a\x12\x03\x13\x03\
	\x13\x03\x13\x03\x13\x03\x13\x06\x13\u{105}\x0a\x13\x0d\x13\x0e\x13\u{106}\
	\x03\x13\x03\x13\x05\x13\u{10b}\x0a\x13\x03\x14\x03\x14\x03\x14\x03\x14\
	\x07\x14\u{111}\x0a\x14\x0c\x14\x0e\x14\u{114}\x0b\x14\x03\x14\x05\x14\u{117}\
	\x0a\x14\x03\x15\x03\x15\x03\x15\x03\x15\x05\x15\u{11d}\x0a\x15\x03\x16\
	\x03\x16\x03\x16\x03\x16\x06\x16\u{123}\x0a\x16\x0d\x16\x0e\x16\u{124}\x03\
	\x16\x03\x16\x05\x16\u{129}\x0a\x16\x03\x17\x03\x17\x03\x17\x03\x17\x03\
	\x17\x03\x17\x03\x17\x05\x17\u{132}\x0a\x17\x03\x18\x03\x18\x03\x18\x03\
	\x18\x03\x18\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x1a\x03\x1a\x03\
	\x1a\x03\x1a\x06\x1a\u{142}\x0a\x1a\x0d\x1a\x0e\x1a\u{143}\x03\x1a\x03\x1a\
	\x05\x1a\u{148}\x0a\x1a\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1c\
	\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x06\x1c\u{154}\x0a\x1c\x0d\x1c\x0e\x1c\
	\u{155}\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x06\x1c\u{15e}\x0a\
	\x1c\x0d\x1c\x0e\x1c\u{15f}\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\
	\x1c\x03\x1c\x03\x1c\x06\x1c\u{16a}\x0a\x1c\x0d\x1c\x0e\x1c\u{16b}\x03\x1c\
	\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x06\x1c\u{176}\
	\x0a\x1c\x0d\x1c\x0e\x1c\u{177}\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\
	\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x06\x1c\u{183}\x0a\x1c\x0d\x1c\x0e\x1c\
	\u{184}\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x06\x1c\
	\u{18e}\x0a\x1c\x0d\x1c\x0e\x1c\u{18f}\x03\x1c\x03\x1c\x05\x1c\u{194}\x0a\
	\x1c\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x07\x1d\u{19a}\x0a\x1d\x0c\x1d\x0e\
	\x1d\u{19d}\x0b\x1d\x03\x1d\x03\x1d\x03\x1e\x03\x1e\x03\x1f\x03\x1f\x03\
	\x1f\x03\x1f\x07\x1f\u{1a7}\x0a\x1f\x0c\x1f\x0e\x1f\u{1aa}\x0b\x1f\x03\x1f\
	\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x07\x1f\u{1b2}\x0a\x1f\x0c\x1f\
	\x0e\x1f\u{1b5}\x0b\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x06\x1f\
	\u{1bc}\x0a\x1f\x0d\x1f\x0e\x1f\u{1bd}\x03\x1f\x07\x1f\u{1c1}\x0a\x1f\x0c\
	\x1f\x0e\x1f\u{1c4}\x0b\x1f\x03\x1f\x03\x1f\x05\x1f\u{1c8}\x0a\x1f\x03\x20\
	\x03\x20\x03\x20\x03\x20\x03\x20\x06\x20\u{1cf}\x0a\x20\x0d\x20\x0e\x20\
	\u{1d0}\x03\x20\x03\x20\x03\x20\x03\x20\x06\x20\u{1d7}\x0a\x20\x0d\x20\x0e\
	\x20\u{1d8}\x03\x20\x07\x20\u{1dc}\x0a\x20\x0c\x20\x0e\x20\u{1df}\x0b\x20\
	\x03\x20\x03\x20\x03\x20\x05\x20\u{1e4}\x0a\x20\x03\x21\x03\x21\x03\x21\
	\x06\x21\u{1e9}\x0a\x21\x0d\x21\x0e\x21\u{1ea}\x03\x21\x03\x21\x03\x21\x03\
	\x21\x03\x21\x06\x21\u{1f2}\x0a\x21\x0d\x21\x0e\x21\u{1f3}\x03\x21\x03\x21\
	\x03\x21\x03\x21\x03\x21\x03\x21\x03\x21\x03\x21\x03\x21\x03\x21\x03\x21\
	\x03\x21\x03\x21\x05\x21\u{203}\x0a\x21\x03\x22\x03\x22\x03\x22\x03\x22\
	\x06\x22\u{209}\x0a\x22\x0d\x22\x0e\x22\u{20a}\x03\x22\x03\x22\x03\x23\x03\
	\x23\x03\x23\x06\x23\u{212}\x0a\x23\x0d\x23\x0e\x23\u{213}\x03\x23\x03\x23\
	\x03\x23\x03\x23\x03\x23\x03\x23\x03\x23\x03\x23\x03\x23\x03\x23\x03\x23\
	\x05\x23\u{221}\x0a\x23\x03\x24\x03\x24\x03\x24\x03\x24\x06\x24\u{227}\x0a\
	\x24\x0d\x24\x0e\x24\u{228}\x03\x24\x03\x24\x03\x25\x03\x25\x03\x25\x03\
	\x25\x03\x25\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x03\x27\x03\x27\x03\
	\x27\x07\x27\u{23a}\x0a\x27\x0c\x27\x0e\x27\u{23d}\x0b\x27\x03\x27\x03\x27\
	\x03\x28\x03\x28\x06\x28\u{243}\x0a\x28\x0d\x28\x0e\x28\u{244}\x03\x28\x03\
	\x28\x03\x28\x03\x28\x03\x28\x03\x28\x06\x28\u{24d}\x0a\x28\x0d\x28\x0e\
	\x28\u{24e}\x03\x28\x03\x28\x03\x28\x06\x28\u{254}\x0a\x28\x0d\x28\x0e\x28\
	\u{255}\x03\x28\x03\x28\x03\x28\x05\x28\u{25b}\x0a\x28\x03\x29\x03\x29\x03\
	\x29\x03\x29\x07\x29\u{261}\x0a\x29\x0c\x29\x0e\x29\u{264}\x0b\x29\x03\x29\
	\x03\x29\x03\x29\x03\x29\x03\x2a\x03\x2a\x03\x2a\x07\x2a\u{26d}\x0a\x2a\
	\x0c\x2a\x0e\x2a\u{270}\x0b\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2b\
	\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x05\x2b\u{27c}\x0a\x2b\x03\x2c\
	\x07\x2c\u{27f}\x0a\x2c\x0c\x2c\x0e\x2c\u{282}\x0b\x2c\x03\x2d\x03\x2d\x03\
	\x2e\x03\x2e\x03\x2f\x03\x2f\x03\x30\x03\x30\x03\x31\x03\x31\x03\x32\x03\
	\x32\x03\x33\x03\x33\x03\x34\x03\x34\x03\x35\x03\x35\x03\x36\x03\x36\x03\
	\x37\x03\x37\x03\x38\x03\x38\x03\x39\x03\x39\x03\x3a\x03\x3a\x03\x3b\x03\
	\x3b\x03\x3c\x03\x3c\x03\x3d\x03\x3d\x03\x3e\x03\x3e\x03\x3f\x03\x3f\x03\
	\x40\x03\x40\x03\x41\x03\x41\x03\x42\x03\x42\x03\x43\x03\x43\x03\x44\x03\
	\x44\x03\x45\x03\x45\x03\x46\x03\x46\x03\x47\x03\x47\x03\x48\x03\x48\x03\
	\x49\x03\x49\x03\x4a\x03\x4a\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x06\x4b\u{2dd}\x0a\x4b\x0d\
	\x4b\x0e\x4b\u{2de}\x03\x4b\x03\x4b\x03\x4b\x06\x4b\u{2e4}\x0a\x4b\x0d\x4b\
	\x0e\x4b\u{2e5}\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\
	\x03\x4b\x07\x4b\u{2f0}\x0a\x4b\x0c\x4b\x0e\x4b\u{2f3}\x0b\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x06\x4b\u{30d}\x0a\x4b\x0d\x4b\x0e\
	\x4b\u{30e}\x03\x4b\x03\x4b\x03\x4b\x06\x4b\u{314}\x0a\x4b\x0d\x4b\x0e\x4b\
	\u{315}\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\
	\x07\x4b\u{320}\x0a\x4b\x0c\x4b\x0e\x4b\u{323}\x0b\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x03\x4b\x03\x4b\x03\x4b\x06\x4b\u{358}\x0a\x4b\x0d\x4b\x0e\x4b\u{359}\
	\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\
	\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\
	\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\
	\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\
	\x05\x4b\u{380}\x0a\x4b\x03\x4c\x03\x4c\x03\x4d\x03\x4d\x03\x4d\x03\x4d\
	\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\
	\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\
	\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x05\x4d\u{3a1}\
	\x0a\x4d\x03\x4e\x03\x4e\x03\x4e\x03\x4e\x03\x4e\x03\x4e\x03\x4e\x03\x4e\
	\x05\x4e\u{3ab}\x0a\x4e\x03\x4f\x03\x4f\x03\x50\x03\x50\x03\x50\x05\x50\
	\u{3b2}\x0a\x50\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\
	\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x06\x51\u{3c2}\
	\x0a\x51\x0d\x51\x0e\x51\u{3c3}\x03\x51\x03\x51\x03\x51\x06\x51\u{3c9}\x0a\
	\x51\x0d\x51\x0e\x51\u{3ca}\x03\x51\x03\x51\x03\x51\x05\x51\u{3d0}\x0a\x51\
	\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\
	\x03\x52\x03\x52\x03\x52\x03\x52\x05\x52\u{3df}\x0a\x52\x03\x53\x03\x53\
	\x03\x53\x03\x53\x03\x53\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x55\
	\x03\x55\x03\x56\x03\x56\x03\x57\x03\x57\x07\x57\u{3f1}\x0a\x57\x0c\x57\
	\x0e\x57\u{3f4}\x0b\x57\x03\x57\x03\x57\x03\x58\x03\x58\x07\x58\u{3fa}\x0a\
	\x58\x0c\x58\x0e\x58\u{3fd}\x0b\x58\x03\x58\x03\x58\x03\x59\x03\x59\x06\
	\x59\u{403}\x0a\x59\x0d\x59\x0e\x59\u{404}\x03\x59\x03\x59\x03\x5a\x03\x5a\
	\x07\x5a\u{40b}\x0a\x5a\x0c\x5a\x0e\x5a\u{40e}\x0b\x5a\x03\x5a\x03\x5a\x03\
	\x5b\x03\x5b\x03\x5c\x03\x5c\x03\x5d\x03\x5d\x07\x5d\u{418}\x0a\x5d\x0c\
	\x5d\x0e\x5d\u{41b}\x0b\x5d\x03\x5d\x03\x5d\x03\x5e\x03\x5e\x07\x5e\u{421}\
	\x0a\x5e\x0c\x5e\x0e\x5e\u{424}\x0b\x5e\x03\x5e\x03\x5e\x03\x5f\x03\x5f\
	\x06\x5f\u{42a}\x0a\x5f\x0d\x5f\x0e\x5f\u{42b}\x03\x5f\x03\x5f\x03\x60\x03\
	\x60\x03\x60\x03\x60\x03\x60\x03\x60\x03\x60\x03\x60\x03\x60\x03\x60\x03\
	\x60\x05\x60\u{43b}\x0a\x60\x03\x61\x03\x61\x03\x61\x03\x61\x03\x61\x03\
	\x61\x03\x61\x03\x61\x05\x61\u{445}\x0a\x61\x03\x61\x02\x02\x62\x02\x04\
	\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\
	\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\
	\x4e\x50\x52\x54\x56\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\x70\
	\x72\x74\x76\x78\x7a\x7c\x7e\u{80}\u{82}\u{84}\u{86}\u{88}\u{8a}\u{8c}\u{8e}\
	\u{90}\u{92}\u{94}\u{96}\u{98}\u{9a}\u{9c}\u{9e}\u{a0}\u{a2}\u{a4}\u{a6}\
	\u{a8}\u{aa}\u{ac}\u{ae}\u{b0}\u{b2}\u{b4}\u{b6}\u{b8}\u{ba}\u{bc}\u{be}\
	\u{c0}\x02\x09\x03\x02\x37\x43\x03\x02\x09\x18\x03\x02\x49\x71\x05\x02\x3b\
	\x3b\x41\x41\x43\x43\x04\x02\x0d\x0d\x15\x15\x04\x02\x0b\x0b\x0e\x0e\x05\
	\x02\x12\x12\x16\x16\x18\x18\x02\u{488}\x02\u{c2}\x03\x02\x02\x02\x04\u{c5}\
	\x03\x02\x02\x02\x06\u{c8}\x03\x02\x02\x02\x08\u{cc}\x03\x02\x02\x02\x0a\
	\u{ce}\x03\x02\x02\x02\x0c\u{d0}\x03\x02\x02\x02\x0e\u{d2}\x03\x02\x02\x02\
	\x10\u{d6}\x03\x02\x02\x02\x12\u{d8}\x03\x02\x02\x02\x14\u{da}\x03\x02\x02\
	\x02\x16\u{dc}\x03\x02\x02\x02\x18\u{de}\x03\x02\x02\x02\x1a\u{e0}\x03\x02\
	\x02\x02\x1c\u{e5}\x03\x02\x02\x02\x1e\u{ec}\x03\x02\x02\x02\x20\u{f9}\x03\
	\x02\x02\x02\x22\u{fd}\x03\x02\x02\x02\x24\u{10a}\x03\x02\x02\x02\x26\u{116}\
	\x03\x02\x02\x02\x28\u{11c}\x03\x02\x02\x02\x2a\u{128}\x03\x02\x02\x02\x2c\
	\u{131}\x03\x02\x02\x02\x2e\u{133}\x03\x02\x02\x02\x30\u{138}\x03\x02\x02\
	\x02\x32\u{147}\x03\x02\x02\x02\x34\u{149}\x03\x02\x02\x02\x36\u{193}\x03\
	\x02\x02\x02\x38\u{195}\x03\x02\x02\x02\x3a\u{1a0}\x03\x02\x02\x02\x3c\u{1c7}\
	\x03\x02\x02\x02\x3e\u{1e3}\x03\x02\x02\x02\x40\u{202}\x03\x02\x02\x02\x42\
	\u{204}\x03\x02\x02\x02\x44\u{220}\x03\x02\x02\x02\x46\u{222}\x03\x02\x02\
	\x02\x48\u{22c}\x03\x02\x02\x02\x4a\u{231}\x03\x02\x02\x02\x4c\u{236}\x03\
	\x02\x02\x02\x4e\u{25a}\x03\x02\x02\x02\x50\u{25c}\x03\x02\x02\x02\x52\u{269}\
	\x03\x02\x02\x02\x54\u{27b}\x03\x02\x02\x02\x56\u{280}\x03\x02\x02\x02\x58\
	\u{283}\x03\x02\x02\x02\x5a\u{285}\x03\x02\x02\x02\x5c\u{287}\x03\x02\x02\
	\x02\x5e\u{289}\x03\x02\x02\x02\x60\u{28b}\x03\x02\x02\x02\x62\u{28d}\x03\
	\x02\x02\x02\x64\u{28f}\x03\x02\x02\x02\x66\u{291}\x03\x02\x02\x02\x68\u{293}\
	\x03\x02\x02\x02\x6a\u{295}\x03\x02\x02\x02\x6c\u{297}\x03\x02\x02\x02\x6e\
	\u{299}\x03\x02\x02\x02\x70\u{29b}\x03\x02\x02\x02\x72\u{29d}\x03\x02\x02\
	\x02\x74\u{29f}\x03\x02\x02\x02\x76\u{2a1}\x03\x02\x02\x02\x78\u{2a3}\x03\
	\x02\x02\x02\x7a\u{2a5}\x03\x02\x02\x02\x7c\u{2a7}\x03\x02\x02\x02\x7e\u{2a9}\
	\x03\x02\x02\x02\u{80}\u{2ab}\x03\x02\x02\x02\u{82}\u{2ad}\x03\x02\x02\x02\
	\u{84}\u{2af}\x03\x02\x02\x02\u{86}\u{2b1}\x03\x02\x02\x02\u{88}\u{2b3}\
	\x03\x02\x02\x02\u{8a}\u{2b5}\x03\x02\x02\x02\u{8c}\u{2b7}\x03\x02\x02\x02\
	\u{8e}\u{2b9}\x03\x02\x02\x02\u{90}\u{2bb}\x03\x02\x02\x02\u{92}\u{2bd}\
	\x03\x02\x02\x02\u{94}\u{37f}\x03\x02\x02\x02\u{96}\u{381}\x03\x02\x02\x02\
	\u{98}\u{3a0}\x03\x02\x02\x02\u{9a}\u{3aa}\x03\x02\x02\x02\u{9c}\u{3ac}\
	\x03\x02\x02\x02\u{9e}\u{3b1}\x03\x02\x02\x02\u{a0}\u{3cf}\x03\x02\x02\x02\
	\u{a2}\u{3de}\x03\x02\x02\x02\u{a4}\u{3e0}\x03\x02\x02\x02\u{a6}\u{3e5}\
	\x03\x02\x02\x02\u{a8}\u{3ea}\x03\x02\x02\x02\u{aa}\u{3ec}\x03\x02\x02\x02\
	\u{ac}\u{3ee}\x03\x02\x02\x02\u{ae}\u{3f7}\x03\x02\x02\x02\u{b0}\u{400}\
	\x03\x02\x02\x02\u{b2}\u{408}\x03\x02\x02\x02\u{b4}\u{411}\x03\x02\x02\x02\
	\u{b6}\u{413}\x03\x02\x02\x02\u{b8}\u{415}\x03\x02\x02\x02\u{ba}\u{41e}\
	\x03\x02\x02\x02\u{bc}\u{427}\x03\x02\x02\x02\u{be}\u{43a}\x03\x02\x02\x02\
	\u{c0}\u{444}\x03\x02\x02\x02\u{c2}\u{c3}\x05\x56\x2c\x02\u{c3}\u{c4}\x07\
	\x02\x02\x03\u{c4}\x03\x03\x02\x02\x02\u{c5}\u{c6}\x05\u{c0}\x61\x02\u{c6}\
	\u{c7}\x07\x02\x02\x03\u{c7}\x05\x03\x02\x02\x02\u{c8}\u{c9}\x09\x02\x02\
	\x02\u{c9}\x07\x03\x02\x02\x02\u{ca}\u{cd}\x05\x0c\x07\x02\u{cb}\u{cd}\x07\
	\x72\x02\x02\u{cc}\u{ca}\x03\x02\x02\x02\u{cc}\u{cb}\x03\x02\x02\x02\u{cd}\
	\x09\x03\x02\x02\x02\u{ce}\u{cf}\x07\x08\x02\x02\u{cf}\x0b\x03\x02\x02\x02\
	\u{d0}\u{d1}\x09\x03\x02\x02\u{d1}\x0d\x03\x02\x02\x02\u{d2}\u{d3}\x09\x04\
	\x02\x02\u{d3}\x0f\x03\x02\x02\x02\u{d4}\u{d7}\x05\x08\x05\x02\u{d5}\u{d7}\
	\x05\x0a\x06\x02\u{d6}\u{d4}\x03\x02\x02\x02\u{d6}\u{d5}\x03\x02\x02\x02\
	\u{d7}\x11\x03\x02\x02\x02\u{d8}\u{d9}\x07\x44\x02\x02\u{d9}\x13\x03\x02\
	\x02\x02\u{da}\u{db}\x07\x47\x02\x02\u{db}\x15\x03\x02\x02\x02\u{dc}\u{dd}\
	\x07\x46\x02\x02\u{dd}\x17\x03\x02\x02\x02\u{de}\u{df}\x07\x45\x02\x02\u{df}\
	\x19\x03\x02\x02\x02\u{e0}\u{e1}\x07\x07\x02\x02\u{e1}\x1b\x03\x02\x02\x02\
	\u{e2}\u{e6}\x05\x0e\x08\x02\u{e3}\u{e4}\x07\x48\x02\x02\u{e4}\u{e6}\x05\
	\x08\x05\x02\u{e5}\u{e2}\x03\x02\x02\x02\u{e5}\u{e3}\x03\x02\x02\x02\u{e6}\
	\x1d\x03\x02\x02\x02\u{e7}\u{ed}\x05\x12\x0a\x02\u{e8}\u{ed}\x05\x14\x0b\
	\x02\u{e9}\u{ed}\x05\x16\x0c\x02\u{ea}\u{ed}\x05\x18\x0d\x02\u{eb}\u{ed}\
	\x05\x1a\x0e\x02\u{ec}\u{e7}\x03\x02\x02\x02\u{ec}\u{e8}\x03\x02\x02\x02\
	\u{ec}\u{e9}\x03\x02\x02\x02\u{ec}\u{ea}\x03\x02\x02\x02\u{ec}\u{eb}\x03\
	\x02\x02\x02\u{ed}\x1f\x03\x02\x02\x02\u{ee}\u{fa}\x05\x1e\x10\x02\u{ef}\
	\u{fa}\x05\x10\x09\x02\u{f0}\u{fa}\x05\x1c\x0f\x02\u{f1}\u{f5}\x07\x04\x02\
	\x02\u{f2}\u{f4}\x05\x20\x11\x02\u{f3}\u{f2}\x03\x02\x02\x02\u{f4}\u{f7}\
	\x03\x02\x02\x02\u{f5}\u{f3}\x03\x02\x02\x02\u{f5}\u{f6}\x03\x02\x02\x02\
	\u{f6}\u{f8}\x03\x02\x02\x02\u{f7}\u{f5}\x03\x02\x02\x02\u{f8}\u{fa}\x07\
	\x05\x02\x02\u{f9}\u{ee}\x03\x02\x02\x02\u{f9}\u{ef}\x03\x02\x02\x02\u{f9}\
	\u{f0}\x03\x02\x02\x02\u{f9}\u{f1}\x03\x02\x02\x02\u{fa}\x21\x03\x02\x02\
	\x02\u{fb}\u{fe}\x05\x12\x0a\x02\u{fc}\u{fe}\x05\x10\x09\x02\u{fd}\u{fb}\
	\x03\x02\x02\x02\u{fd}\u{fc}\x03\x02\x02\x02\u{fe}\x23\x03\x02\x02\x02\u{ff}\
	\u{10b}\x05\x10\x09\x02\u{100}\u{101}\x07\x04\x02\x02\u{101}\u{102}\x07\
	\x38\x02\x02\u{102}\u{104}\x05\x10\x09\x02\u{103}\u{105}\x05\x22\x12\x02\
	\u{104}\u{103}\x03\x02\x02\x02\u{105}\u{106}\x03\x02\x02\x02\u{106}\u{104}\
	\x03\x02\x02\x02\u{106}\u{107}\x03\x02\x02\x02\u{107}\u{108}\x03\x02\x02\
	\x02\u{108}\u{109}\x07\x05\x02\x02\u{109}\u{10b}\x03\x02\x02\x02\u{10a}\
	\u{ff}\x03\x02\x02\x02\u{10a}\u{100}\x03\x02\x02\x02\u{10b}\x25\x03\x02\
	\x02\x02\u{10c}\u{117}\x05\x1e\x10\x02\u{10d}\u{117}\x05\x10\x09\x02\u{10e}\
	\u{112}\x07\x04\x02\x02\u{10f}\u{111}\x05\x20\x11\x02\u{110}\u{10f}\x03\
	\x02\x02\x02\u{111}\u{114}\x03\x02\x02\x02\u{112}\u{110}\x03\x02\x02\x02\
	\u{112}\u{113}\x03\x02\x02\x02\u{113}\u{115}\x03\x02\x02\x02\u{114}\u{112}\
	\x03\x02\x02\x02\u{115}\u{117}\x07\x05\x02\x02\u{116}\u{10c}\x03\x02\x02\
	\x02\u{116}\u{10d}\x03\x02\x02\x02\u{116}\u{10e}\x03\x02\x02\x02\u{117}\
	\x27\x03\x02\x02\x02\u{118}\u{11d}\x05\x1c\x0f\x02\u{119}\u{11a}\x05\x1c\
	\x0f\x02\u{11a}\u{11b}\x05\x26\x14\x02\u{11b}\u{11d}\x03\x02\x02\x02\u{11c}\
	\u{118}\x03\x02\x02\x02\u{11c}\u{119}\x03\x02\x02\x02\u{11d}\x29\x03\x02\
	\x02\x02\u{11e}\u{129}\x05\x24\x13\x02\u{11f}\u{120}\x07\x04\x02\x02\u{120}\
	\u{122}\x05\x24\x13\x02\u{121}\u{123}\x05\x2a\x16\x02\u{122}\u{121}\x03\
	\x02\x02\x02\u{123}\u{124}\x03\x02\x02\x02\u{124}\u{122}\x03\x02\x02\x02\
	\u{124}\u{125}\x03\x02\x02\x02\u{125}\u{126}\x03\x02\x02\x02\u{126}\u{127}\
	\x07\x05\x02\x02\u{127}\u{129}\x03\x02\x02\x02\u{128}\u{11e}\x03\x02\x02\
	\x02\u{128}\u{11f}\x03\x02\x02\x02\u{129}\x2b\x03\x02\x02\x02\u{12a}\u{132}\
	\x05\x24\x13\x02\u{12b}\u{12c}\x07\x04\x02\x02\u{12c}\u{12d}\x07\x39\x02\
	\x02\u{12d}\u{12e}\x05\x24\x13\x02\u{12e}\u{12f}\x05\x2a\x16\x02\u{12f}\
	\u{130}\x07\x05\x02\x02\u{130}\u{132}\x03\x02\x02\x02\u{131}\u{12a}\x03\
	\x02\x02\x02\u{131}\u{12b}\x03\x02\x02\x02\u{132}\x2d\x03\x02\x02\x02\u{133}\
	\u{134}\x07\x04\x02\x02\u{134}\u{135}\x05\x10\x09\x02\u{135}\u{136}\x05\
	\x36\x1c\x02\u{136}\u{137}\x07\x05\x02\x02\u{137}\x2f\x03\x02\x02\x02\u{138}\
	\u{139}\x07\x04\x02\x02\u{139}\u{13a}\x05\x10\x09\x02\u{13a}\u{13b}\x05\
	\x2a\x16\x02\u{13b}\u{13c}\x07\x05\x02\x02\u{13c}\x31\x03\x02\x02\x02\u{13d}\
	\u{148}\x05\x10\x09\x02\u{13e}\u{13f}\x07\x04\x02\x02\u{13f}\u{141}\x05\
	\x10\x09\x02\u{140}\u{142}\x05\x10\x09\x02\u{141}\u{140}\x03\x02\x02\x02\
	\u{142}\u{143}\x03\x02\x02\x02\u{143}\u{141}\x03\x02\x02\x02\u{143}\u{144}\
	\x03\x02\x02\x02\u{144}\u{145}\x03\x02\x02\x02\u{145}\u{146}\x07\x05\x02\
	\x02\u{146}\u{148}\x03\x02\x02\x02\u{147}\u{13d}\x03\x02\x02\x02\u{147}\
	\u{13e}\x03\x02\x02\x02\u{148}\x33\x03\x02\x02\x02\u{149}\u{14a}\x07\x04\
	\x02\x02\u{14a}\u{14b}\x05\x32\x1a\x02\u{14b}\u{14c}\x05\x36\x1c\x02\u{14c}\
	\u{14d}\x07\x05\x02\x02\u{14d}\x35\x03\x02\x02\x02\u{14e}\u{194}\x05\x1e\
	\x10\x02\u{14f}\u{194}\x05\x2c\x17\x02\u{150}\u{151}\x07\x04\x02\x02\u{151}\
	\u{153}\x05\x2c\x17\x02\u{152}\u{154}\x05\x36\x1c\x02\u{153}\u{152}\x03\
	\x02\x02\x02\u{154}\u{155}\x03\x02\x02\x02\u{155}\u{153}\x03\x02\x02\x02\
	\u{155}\u{156}\x03\x02\x02\x02\u{156}\u{157}\x03\x02\x02\x02\u{157}\u{158}\
	\x07\x05\x02\x02\u{158}\u{194}\x03\x02\x02\x02\u{159}\u{15a}\x07\x04\x02\
	\x02\u{15a}\u{15b}\x07\x3f\x02\x02\u{15b}\u{15d}\x07\x04\x02\x02\u{15c}\
	\u{15e}\x05\x2e\x18\x02\u{15d}\u{15c}\x03\x02\x02\x02\u{15e}\u{15f}\x03\
	\x02\x02\x02\u{15f}\u{15d}\x03\x02\x02\x02\u{15f}\u{160}\x03\x02\x02\x02\
	\u{160}\u{161}\x03\x02\x02\x02\u{161}\u{162}\x07\x05\x02\x02\u{162}\u{163}\
	\x05\x36\x1c\x02\u{163}\u{164}\x07\x05\x02\x02\u{164}\u{194}\x03\x02\x02\
	\x02\u{165}\u{166}\x07\x04\x02\x02\u{166}\u{167}\x07\x3e\x02\x02\u{167}\
	\u{169}\x07\x04\x02\x02\u{168}\u{16a}\x05\x30\x19\x02\u{169}\u{168}\x03\
	\x02\x02\x02\u{16a}\u{16b}\x03\x02\x02\x02\u{16b}\u{169}\x03\x02\x02\x02\
	\u{16b}\u{16c}\x03\x02\x02\x02\u{16c}\u{16d}\x03\x02\x02\x02\u{16d}\u{16e}\
	\x07\x05\x02\x02\u{16e}\u{16f}\x05\x36\x1c\x02\u{16f}\u{170}\x07\x05\x02\
	\x02\u{170}\u{194}\x03\x02\x02\x02\u{171}\u{172}\x07\x04\x02\x02\u{172}\
	\u{173}\x07\x3c\x02\x02\u{173}\u{175}\x07\x04\x02\x02\u{174}\u{176}\x05\
	\x30\x19\x02\u{175}\u{174}\x03\x02\x02\x02\u{176}\u{177}\x03\x02\x02\x02\
	\u{177}\u{175}\x03\x02\x02\x02\u{177}\u{178}\x03\x02\x02\x02\u{178}\u{179}\
	\x03\x02\x02\x02\u{179}\u{17a}\x07\x05\x02\x02\u{17a}\u{17b}\x05\x36\x1c\
	\x02\u{17b}\u{17c}\x07\x05\x02\x02\u{17c}\u{194}\x03\x02\x02\x02\u{17d}\
	\u{17e}\x07\x04\x02\x02\u{17e}\u{17f}\x07\x40\x02\x02\u{17f}\u{180}\x05\
	\x36\x1c\x02\u{180}\u{182}\x07\x04\x02\x02\u{181}\u{183}\x05\x34\x1b\x02\
	\u{182}\u{181}\x03\x02\x02\x02\u{183}\u{184}\x03\x02\x02\x02\u{184}\u{182}\
	\x03\x02\x02\x02\u{184}\u{185}\x03\x02\x02\x02\u{185}\u{186}\x03\x02\x02\
	\x02\u{186}\u{187}\x07\x05\x02\x02\u{187}\u{188}\x07\x05\x02\x02\u{188}\
	\u{194}\x03\x02\x02\x02\u{189}\u{18a}\x07\x04\x02\x02\u{18a}\u{18b}\x07\
	\x37\x02\x02\u{18b}\u{18d}\x05\x36\x1c\x02\u{18c}\u{18e}\x05\x28\x15\x02\
	\u{18d}\u{18c}\x03\x02\x02\x02\u{18e}\u{18f}\x03\x02\x02\x02\u{18f}\u{18d}\
	\x03\x02\x02\x02\u{18f}\u{190}\x03\x02\x02\x02\u{190}\u{191}\x03\x02\x02\
	\x02\u{191}\u{192}\x07\x05\x02\x02\u{192}\u{194}\x03\x02\x02\x02\u{193}\
	\u{14e}\x03\x02\x02\x02\u{193}\u{14f}\x03\x02\x02\x02\u{193}\u{150}\x03\
	\x02\x02\x02\u{193}\u{159}\x03\x02\x02\x02\u{193}\u{165}\x03\x02\x02\x02\
	\u{193}\u{171}\x03\x02\x02\x02\u{193}\u{17d}\x03\x02\x02\x02\u{193}\u{189}\
	\x03\x02\x02\x02\u{194}\x37\x03\x02\x02\x02\u{195}\u{196}\x07\x04\x02\x02\
	\u{196}\u{197}\x05\x24\x13\x02\u{197}\u{19b}\x05\x12\x0a\x02\u{198}\u{19a}\
	\x05\x28\x15\x02\u{199}\u{198}\x03\x02\x02\x02\u{19a}\u{19d}\x03\x02\x02\
	\x02\u{19b}\u{199}\x03\x02\x02\x02\u{19b}\u{19c}\x03\x02\x02\x02\u{19c}\
	\u{19e}\x03\x02\x02\x02\u{19d}\u{19b}\x03\x02\x02\x02\u{19e}\u{19f}\x07\
	\x05\x02\x02\u{19f}\x39\x03\x02\x02\x02\u{1a0}\u{1a1}\x09\x05\x02\x02\u{1a1}\
	\x3b\x03\x02\x02\x02\u{1a2}\u{1a3}\x07\x04\x02\x02\u{1a3}\u{1a4}\x05\x1e\
	\x10\x02\u{1a4}\u{1a8}\x05\x2a\x16\x02\u{1a5}\u{1a7}\x05\x28\x15\x02\u{1a6}\
	\u{1a5}\x03\x02\x02\x02\u{1a7}\u{1aa}\x03\x02\x02\x02\u{1a8}\u{1a6}\x03\
	\x02\x02\x02\u{1a8}\u{1a9}\x03\x02\x02\x02\u{1a9}\u{1ab}\x03\x02\x02\x02\
	\u{1aa}\u{1a8}\x03\x02\x02\x02\u{1ab}\u{1ac}\x07\x05\x02\x02\u{1ac}\u{1c8}\
	\x03\x02\x02\x02\u{1ad}\u{1ae}\x07\x04\x02\x02\u{1ae}\u{1af}\x05\x3a\x1e\
	\x02\u{1af}\u{1b3}\x05\x2a\x16\x02\u{1b0}\u{1b2}\x05\x28\x15\x02\u{1b1}\
	\u{1b0}\x03\x02\x02\x02\u{1b2}\u{1b5}\x03\x02\x02\x02\u{1b3}\u{1b1}\x03\
	\x02\x02\x02\u{1b3}\u{1b4}\x03\x02\x02\x02\u{1b4}\u{1b6}\x03\x02\x02\x02\
	\u{1b5}\u{1b3}\x03\x02\x02\x02\u{1b6}\u{1b7}\x07\x05\x02\x02\u{1b7}\u{1c8}\
	\x03\x02\x02\x02\u{1b8}\u{1b9}\x07\x04\x02\x02\u{1b9}\u{1bb}\x05\x24\x13\
	\x02\u{1ba}\u{1bc}\x05\x2a\x16\x02\u{1bb}\u{1ba}\x03\x02\x02\x02\u{1bc}\
	\u{1bd}\x03\x02\x02\x02\u{1bd}\u{1bb}\x03\x02\x02\x02\u{1bd}\u{1be}\x03\
	\x02\x02\x02\u{1be}\u{1c2}\x03\x02\x02\x02\u{1bf}\u{1c1}\x05\x28\x15\x02\
	\u{1c0}\u{1bf}\x03\x02\x02\x02\u{1c1}\u{1c4}\x03\x02\x02\x02\u{1c2}\u{1c0}\
	\x03\x02\x02\x02\u{1c2}\u{1c3}\x03\x02\x02\x02\u{1c3}\u{1c5}\x03\x02\x02\
	\x02\u{1c4}\u{1c2}\x03\x02\x02\x02\u{1c5}\u{1c6}\x07\x05\x02\x02\u{1c6}\
	\u{1c8}\x03\x02\x02\x02\u{1c7}\u{1a2}\x03\x02\x02\x02\u{1c7}\u{1ad}\x03\
	\x02\x02\x02\u{1c7}\u{1b8}\x03\x02\x02\x02\u{1c8}\x3d\x03\x02\x02\x02\u{1c9}\
	\u{1e4}\x05\x3c\x1f\x02\u{1ca}\u{1cb}\x07\x04\x02\x02\u{1cb}\u{1cc}\x07\
	\x42\x02\x02\u{1cc}\u{1ce}\x07\x04\x02\x02\u{1cd}\u{1cf}\x05\x10\x09\x02\
	\u{1ce}\u{1cd}\x03\x02\x02\x02\u{1cf}\u{1d0}\x03\x02\x02\x02\u{1d0}\u{1ce}\
	\x03\x02\x02\x02\u{1d0}\u{1d1}\x03\x02\x02\x02\u{1d1}\u{1d2}\x03\x02\x02\
	\x02\u{1d2}\u{1d3}\x07\x05\x02\x02\u{1d3}\u{1d4}\x07\x04\x02\x02\u{1d4}\
	\u{1d6}\x05\x24\x13\x02\u{1d5}\u{1d7}\x05\x2a\x16\x02\u{1d6}\u{1d5}\x03\
	\x02\x02\x02\u{1d7}\u{1d8}\x03\x02\x02\x02\u{1d8}\u{1d6}\x03\x02\x02\x02\
	\u{1d8}\u{1d9}\x03\x02\x02\x02\u{1d9}\u{1dd}\x03\x02\x02\x02\u{1da}\u{1dc}\
	\x05\x28\x15\x02\u{1db}\u{1da}\x03\x02\x02\x02\u{1dc}\u{1df}\x03\x02\x02\
	\x02\u{1dd}\u{1db}\x03\x02\x02\x02\u{1dd}\u{1de}\x03\x02\x02\x02\u{1de}\
	\u{1e0}\x03\x02\x02\x02\u{1df}\u{1dd}\x03\x02\x02\x02\u{1e0}\u{1e1}\x07\
	\x05\x02\x02\u{1e1}\u{1e2}\x07\x05\x02\x02\u{1e2}\u{1e4}\x03\x02\x02\x02\
	\u{1e3}\u{1c9}\x03\x02\x02\x02\u{1e3}\u{1ca}\x03\x02\x02\x02\u{1e4}\x3f\
	\x03\x02\x02\x02\u{1e5}\u{1e6}\x07\x6a\x02\x02\u{1e6}\u{1e8}\x07\x04\x02\
	\x02\u{1e7}\u{1e9}\x05\x38\x1d\x02\u{1e8}\u{1e7}\x03\x02\x02\x02\u{1e9}\
	\u{1ea}\x03\x02\x02\x02\u{1ea}\u{1e8}\x03\x02\x02\x02\u{1ea}\u{1eb}\x03\
	\x02\x02\x02\u{1eb}\u{1ec}\x03\x02\x02\x02\u{1ec}\u{1ed}\x07\x05\x02\x02\
	\u{1ed}\u{203}\x03\x02\x02\x02\u{1ee}\u{1ef}\x07\x52\x02\x02\u{1ef}\u{1f1}\
	\x07\x04\x02\x02\u{1f0}\u{1f2}\x05\x3e\x20\x02\u{1f1}\u{1f0}\x03\x02\x02\
	\x02\u{1f2}\u{1f3}\x03\x02\x02\x02\u{1f3}\u{1f1}\x03\x02\x02\x02\u{1f3}\
	\u{1f4}\x03\x02\x02\x02\u{1f4}\u{1f5}\x03\x02\x02\x02\u{1f5}\u{1f6}\x07\
	\x05\x02\x02\u{1f6}\u{203}\x03\x02\x02\x02\u{1f7}\u{1f8}\x07\x6b\x02\x02\
	\u{1f8}\u{203}\x05\x1a\x0e\x02\u{1f9}\u{1fa}\x07\x53\x02\x02\u{1fa}\u{203}\
	\x05\x1a\x0e\x02\u{1fb}\u{1fc}\x07\x4e\x02\x02\u{1fc}\u{203}\x05\x1a\x0e\
	\x02\u{1fd}\u{1fe}\x07\x6f\x02\x02\u{1fe}\u{203}\x05\x1a\x0e\x02\u{1ff}\
	\u{200}\x07\x5b\x02\x02\u{200}\u{203}\x05\x1a\x0e\x02\u{201}\u{203}\x05\
	\x28\x15\x02\u{202}\u{1e5}\x03\x02\x02\x02\u{202}\u{1ee}\x03\x02\x02\x02\
	\u{202}\u{1f7}\x03\x02\x02\x02\u{202}\u{1f9}\x03\x02\x02\x02\u{202}\u{1fb}\
	\x03\x02\x02\x02\u{202}\u{1fd}\x03\x02\x02\x02\u{202}\u{1ff}\x03\x02\x02\
	\x02\u{202}\u{201}\x03\x02\x02\x02\u{203}\x41\x03\x02\x02\x02\u{204}\u{205}\
	\x07\x04\x02\x02\u{205}\u{206}\x07\x14\x02\x02\u{206}\u{208}\x05\x10\x09\
	\x02\u{207}\u{209}\x05\x40\x21\x02\u{208}\u{207}\x03\x02\x02\x02\u{209}\
	\u{20a}\x03\x02\x02\x02\u{20a}\u{208}\x03\x02\x02\x02\u{20a}\u{20b}\x03\
	\x02\x02\x02\u{20b}\u{20c}\x03\x02\x02\x02\u{20c}\u{20d}\x07\x05\x02\x02\
	\u{20d}\x43\x03\x02\x02\x02\u{20e}\u{20f}\x07\x6e\x02\x02\u{20f}\u{211}\
	\x07\x04\x02\x02\u{210}\u{212}\x05\x10\x09\x02\u{211}\u{210}\x03\x02\x02\
	\x02\u{212}\u{213}\x03\x02\x02\x02\u{213}\u{211}\x03\x02\x02\x02\u{213}\
	\u{214}\x03\x02\x02\x02\u{214}\u{215}\x03\x02\x02\x02\u{215}\u{216}\x07\
	\x05\x02\x02\u{216}\u{221}\x03\x02\x02\x02\u{217}\u{218}\x07\x56\x02\x02\
	\u{218}\u{221}\x05\x1a\x0e\x02\u{219}\u{21a}\x07\x51\x02\x02\u{21a}\u{221}\
	\x05\x1a\x0e\x02\u{21b}\u{21c}\x07\x6f\x02\x02\u{21c}\u{221}\x05\x1a\x0e\
	\x02\u{21d}\u{21e}\x07\x5b\x02\x02\u{21e}\u{221}\x05\x1a\x0e\x02\u{21f}\
	\u{221}\x05\x28\x15\x02\u{220}\u{20e}\x03\x02\x02\x02\u{220}\u{217}\x03\
	\x02\x02\x02\u{220}\u{219}\x03\x02\x02\x02\u{220}\u{21b}\x03\x02\x02\x02\
	\u{220}\u{21d}\x03\x02\x02\x02\u{220}\u{21f}\x03\x02\x02\x02\u{221}\x45\
	\x03\x02\x02\x02\u{222}\u{223}\x07\x04\x02\x02\u{223}\u{224}\x07\x10\x02\
	\x02\u{224}\u{226}\x05\x10\x09\x02\u{225}\u{227}\x05\x44\x23\x02\u{226}\
	\u{225}\x03\x02\x02\x02\u{227}\u{228}\x03\x02\x02\x02\u{228}\u{226}\x03\
	\x02\x02\x02\u{228}\u{229}\x03\x02\x02\x02\u{229}\u{22a}\x03\x02\x02\x02\
	\u{22a}\u{22b}\x07\x05\x02\x02\u{22b}\x47\x03\x02\x02\x02\u{22c}\u{22d}\
	\x07\x04\x02\x02\u{22d}\u{22e}\x05\x10\x09\x02\u{22e}\u{22f}\x05\x12\x0a\
	\x02\u{22f}\u{230}\x07\x05\x02\x02\u{230}\x49\x03\x02\x02\x02\u{231}\u{232}\
	\x07\x04\x02\x02\u{232}\u{233}\x05\x10\x09\x02\u{233}\u{234}\x05\x2a\x16\
	\x02\u{234}\u{235}\x07\x05\x02\x02\u{235}\x4b\x03\x02\x02\x02\u{236}\u{237}\
	\x07\x04\x02\x02\u{237}\u{23b}\x05\x10\x09\x02\u{238}\u{23a}\x05\x4a\x26\
	\x02\u{239}\u{238}\x03\x02\x02\x02\u{23a}\u{23d}\x03\x02\x02\x02\u{23b}\
	\u{239}\x03\x02\x02\x02\u{23b}\u{23c}\x03\x02\x02\x02\u{23c}\u{23e}\x03\
	\x02\x02\x02\u{23d}\u{23b}\x03\x02\x02\x02\u{23e}\u{23f}\x07\x05\x02\x02\
	\u{23f}\x4d\x03\x02\x02\x02\u{240}\u{242}\x07\x04\x02\x02\u{241}\u{243}\
	\x05\x4c\x27\x02\u{242}\u{241}\x03\x02\x02\x02\u{243}\u{244}\x03\x02\x02\
	\x02\u{244}\u{242}\x03\x02\x02\x02\u{244}\u{245}\x03\x02\x02\x02\u{245}\
	\u{246}\x03\x02\x02\x02\u{246}\u{247}\x07\x05\x02\x02\u{247}\u{25b}\x03\
	\x02\x02\x02\u{248}\u{249}\x07\x04\x02\x02\u{249}\u{24a}\x07\x42\x02\x02\
	\u{24a}\u{24c}\x07\x04\x02\x02\u{24b}\u{24d}\x05\x10\x09\x02\u{24c}\u{24b}\
	\x03\x02\x02\x02\u{24d}\u{24e}\x03\x02\x02\x02\u{24e}\u{24c}\x03\x02\x02\
	\x02\u{24e}\u{24f}\x03\x02\x02\x02\u{24f}\u{250}\x03\x02\x02\x02\u{250}\
	\u{251}\x07\x05\x02\x02\u{251}\u{253}\x07\x04\x02\x02\u{252}\u{254}\x05\
	\x4c\x27\x02\u{253}\u{252}\x03\x02\x02\x02\u{254}\u{255}\x03\x02\x02\x02\
	\u{255}\u{253}\x03\x02\x02\x02\u{255}\u{256}\x03\x02\x02\x02\u{256}\u{257}\
	\x03\x02\x02\x02\u{257}\u{258}\x07\x05\x02\x02\u{258}\u{259}\x07\x05\x02\
	\x02\u{259}\u{25b}\x03\x02\x02\x02\u{25a}\u{240}\x03\x02\x02\x02\u{25a}\
	\u{248}\x03\x02\x02\x02\u{25b}\x4f\x03\x02\x02\x02\u{25c}\u{25d}\x07\x04\
	\x02\x02\u{25d}\u{25e}\x05\x10\x09\x02\u{25e}\u{262}\x07\x04\x02\x02\u{25f}\
	\u{261}\x05\x30\x19\x02\u{260}\u{25f}\x03\x02\x02\x02\u{261}\u{264}\x03\
	\x02\x02\x02\u{262}\u{260}\x03\x02\x02\x02\u{262}\u{263}\x03\x02\x02\x02\
	\u{263}\u{265}\x03\x02\x02\x02\u{264}\u{262}\x03\x02\x02\x02\u{265}\u{266}\
	\x07\x05\x02\x02\u{266}\u{267}\x05\x2a\x16\x02\u{267}\u{268}\x07\x05\x02\
	\x02\u{268}\x51\x03\x02\x02\x02\u{269}\u{26a}\x05\x10\x09\x02\u{26a}\u{26e}\
	\x07\x04\x02\x02\u{26b}\u{26d}\x05\x30\x19\x02\u{26c}\u{26b}\x03\x02\x02\
	\x02\u{26d}\u{270}\x03\x02\x02\x02\u{26e}\u{26c}\x03\x02\x02\x02\u{26e}\
	\u{26f}\x03\x02\x02\x02\u{26f}\u{271}\x03\x02\x02\x02\u{270}\u{26e}\x03\
	\x02\x02\x02\u{271}\u{272}\x07\x05\x02\x02\u{272}\u{273}\x05\x2a\x16\x02\
	\u{273}\u{274}\x05\x36\x1c\x02\u{274}\x53\x03\x02\x02\x02\u{275}\u{27c}\
	\x05\x10\x09\x02\u{276}\u{277}\x07\x04\x02\x02\u{277}\u{278}\x07\x09\x02\
	\x02\u{278}\u{279}\x05\x10\x09\x02\u{279}\u{27a}\x07\x05\x02\x02\u{27a}\
	\u{27c}\x03\x02\x02\x02\u{27b}\u{275}\x03\x02\x02\x02\u{27b}\u{276}\x03\
	\x02\x02\x02\u{27c}\x55\x03\x02\x02\x02\u{27d}\u{27f}\x05\u{94}\x4b\x02\
	\u{27e}\u{27d}\x03\x02\x02\x02\u{27f}\u{282}\x03\x02\x02\x02\u{280}\u{27e}\
	\x03\x02\x02\x02\u{280}\u{281}\x03\x02\x02\x02\u{281}\x57\x03\x02\x02\x02\
	\u{282}\u{280}\x03\x02\x02\x02\u{283}\u{284}\x07\x19\x02\x02\u{284}\x59\
	\x03\x02\x02\x02\u{285}\u{286}\x07\x1a\x02\x02\u{286}\x5b\x03\x02\x02\x02\
	\u{287}\u{288}\x07\x1b\x02\x02\u{288}\x5d\x03\x02\x02\x02\u{289}\u{28a}\
	\x07\x1c\x02\x02\u{28a}\x5f\x03\x02\x02\x02\u{28b}\u{28c}\x07\x1d\x02\x02\
	\u{28c}\x61\x03\x02\x02\x02\u{28d}\u{28e}\x07\x1e\x02\x02\u{28e}\x63\x03\
	\x02\x02\x02\u{28f}\u{290}\x07\x1f\x02\x02\u{290}\x65\x03\x02\x02\x02\u{291}\
	\u{292}\x07\x20\x02\x02\u{292}\x67\x03\x02\x02\x02\u{293}\u{294}\x07\x21\
	\x02\x02\u{294}\x69\x03\x02\x02\x02\u{295}\u{296}\x07\x22\x02\x02\u{296}\
	\x6b\x03\x02\x02\x02\u{297}\u{298}\x07\x23\x02\x02\u{298}\x6d\x03\x02\x02\
	\x02\u{299}\u{29a}\x07\x24\x02\x02\u{29a}\x6f\x03\x02\x02\x02\u{29b}\u{29c}\
	\x07\x25\x02\x02\u{29c}\x71\x03\x02\x02\x02\u{29d}\u{29e}\x07\x26\x02\x02\
	\u{29e}\x73\x03\x02\x02\x02\u{29f}\u{2a0}\x07\x27\x02\x02\u{2a0}\x75\x03\
	\x02\x02\x02\u{2a1}\u{2a2}\x07\x28\x02\x02\u{2a2}\x77\x03\x02\x02\x02\u{2a3}\
	\u{2a4}\x07\x29\x02\x02\u{2a4}\x79\x03\x02\x02\x02\u{2a5}\u{2a6}\x07\x2a\
	\x02\x02\u{2a6}\x7b\x03\x02\x02\x02\u{2a7}\u{2a8}\x07\x2b\x02\x02\u{2a8}\
	\x7d\x03\x02\x02\x02\u{2a9}\u{2aa}\x07\x2c\x02\x02\u{2aa}\x7f\x03\x02\x02\
	\x02\u{2ab}\u{2ac}\x07\x2d\x02\x02\u{2ac}\u{81}\x03\x02\x02\x02\u{2ad}\u{2ae}\
	\x07\x2e\x02\x02\u{2ae}\u{83}\x03\x02\x02\x02\u{2af}\u{2b0}\x07\x2f\x02\
	\x02\u{2b0}\u{85}\x03\x02\x02\x02\u{2b1}\u{2b2}\x07\x30\x02\x02\u{2b2}\u{87}\
	\x03\x02\x02\x02\u{2b3}\u{2b4}\x07\x31\x02\x02\u{2b4}\u{89}\x03\x02\x02\
	\x02\u{2b5}\u{2b6}\x07\x32\x02\x02\u{2b6}\u{8b}\x03\x02\x02\x02\u{2b7}\u{2b8}\
	\x07\x33\x02\x02\u{2b8}\u{8d}\x03\x02\x02\x02\u{2b9}\u{2ba}\x07\x34\x02\
	\x02\u{2ba}\u{8f}\x03\x02\x02\x02\u{2bb}\u{2bc}\x07\x35\x02\x02\u{2bc}\u{91}\
	\x03\x02\x02\x02\u{2bd}\u{2be}\x07\x36\x02\x02\u{2be}\u{93}\x03\x02\x02\
	\x02\u{2bf}\u{2c0}\x07\x04\x02\x02\u{2c0}\u{2c1}\x05\x58\x2d\x02\u{2c1}\
	\u{2c2}\x05\x36\x1c\x02\u{2c2}\u{2c3}\x07\x05\x02\x02\u{2c3}\u{380}\x03\
	\x02\x02\x02\u{2c4}\u{2c5}\x07\x04\x02\x02\u{2c5}\u{2c6}\x05\x5a\x2e\x02\
	\u{2c6}\u{2c7}\x07\x05\x02\x02\u{2c7}\u{380}\x03\x02\x02\x02\u{2c8}\u{2c9}\
	\x07\x04\x02\x02\u{2c9}\u{2ca}\x05\x5c\x2f\x02\u{2ca}\u{2cb}\x07\x05\x02\
	\x02\u{2cb}\u{380}\x03\x02\x02\x02\u{2cc}\u{2cd}\x07\x04\x02\x02\u{2cd}\
	\u{2ce}\x05\x5e\x30\x02\u{2ce}\u{2cf}\x05\x10\x09\x02\u{2cf}\u{2d0}\x05\
	\x2a\x16\x02\u{2d0}\u{2d1}\x07\x05\x02\x02\u{2d1}\u{380}\x03\x02\x02\x02\
	\u{2d2}\u{2d3}\x07\x04\x02\x02\u{2d3}\u{2d4}\x05\x60\x31\x02\u{2d4}\u{2d5}\
	\x05\x10\x09\x02\u{2d5}\u{2d6}\x05\x4e\x28\x02\u{2d6}\u{2d7}\x07\x05\x02\
	\x02\u{2d7}\u{380}\x03\x02\x02\x02\u{2d8}\u{2d9}\x07\x04\x02\x02\u{2d9}\
	\u{2da}\x05\x62\x32\x02\u{2da}\u{2dc}\x07\x04\x02\x02\u{2db}\u{2dd}\x05\
	\x48\x25\x02\u{2dc}\u{2db}\x03\x02\x02\x02\u{2dd}\u{2de}\x03\x02\x02\x02\
	\u{2de}\u{2dc}\x03\x02\x02\x02\u{2de}\u{2df}\x03\x02\x02\x02\u{2df}\u{2e0}\
	\x03\x02\x02\x02\u{2e0}\u{2e1}\x07\x05\x02\x02\u{2e1}\u{2e3}\x07\x04\x02\
	\x02\u{2e2}\u{2e4}\x05\x4e\x28\x02\u{2e3}\u{2e2}\x03\x02\x02\x02\u{2e4}\
	\u{2e5}\x03\x02\x02\x02\u{2e5}\u{2e3}\x03\x02\x02\x02\u{2e5}\u{2e6}\x03\
	\x02\x02\x02\u{2e6}\u{2e7}\x03\x02\x02\x02\u{2e7}\u{2e8}\x07\x05\x02\x02\
	\u{2e8}\u{2e9}\x07\x05\x02\x02\u{2e9}\u{380}\x03\x02\x02\x02\u{2ea}\u{2eb}\
	\x07\x04\x02\x02\u{2eb}\u{2ec}\x05\x64\x33\x02\u{2ec}\u{2ed}\x05\x10\x09\
	\x02\u{2ed}\u{2f1}\x07\x04\x02\x02\u{2ee}\u{2f0}\x05\x2a\x16\x02\u{2ef}\
	\u{2ee}\x03\x02\x02\x02\u{2f0}\u{2f3}\x03\x02\x02\x02\u{2f1}\u{2ef}\x03\
	\x02\x02\x02\u{2f1}\u{2f2}\x03\x02\x02\x02\u{2f2}\u{2f4}\x03\x02\x02\x02\
	\u{2f3}\u{2f1}\x03\x02\x02\x02\u{2f4}\u{2f5}\x07\x05\x02\x02\u{2f5}\u{2f6}\
	\x05\x2a\x16\x02\u{2f6}\u{2f7}\x07\x05\x02\x02\u{2f7}\u{380}\x03\x02\x02\
	\x02\u{2f8}\u{2f9}\x07\x04\x02\x02\u{2f9}\u{2fa}\x05\x66\x34\x02\u{2fa}\
	\u{2fb}\x05\x10\x09\x02\u{2fb}\u{2fc}\x05\x12\x0a\x02\u{2fc}\u{2fd}\x07\
	\x05\x02\x02\u{2fd}\u{380}\x03\x02\x02\x02\u{2fe}\u{2ff}\x07\x04\x02\x02\
	\u{2ff}\u{300}\x05\x68\x35\x02\u{300}\u{301}\x05\x52\x2a\x02\u{301}\u{302}\
	\x07\x05\x02\x02\u{302}\u{380}\x03\x02\x02\x02\u{303}\u{304}\x07\x04\x02\
	\x02\u{304}\u{305}\x05\x6a\x36\x02\u{305}\u{306}\x05\x52\x2a\x02\u{306}\
	\u{307}\x07\x05\x02\x02\u{307}\u{380}\x03\x02\x02\x02\u{308}\u{309}\x07\
	\x04\x02\x02\u{309}\u{30a}\x05\x6c\x37\x02\u{30a}\u{30c}\x07\x04\x02\x02\
	\u{30b}\u{30d}\x05\x50\x29\x02\u{30c}\u{30b}\x03\x02\x02\x02\u{30d}\u{30e}\
	\x03\x02\x02\x02\u{30e}\u{30c}\x03\x02\x02\x02\u{30e}\u{30f}\x03\x02\x02\
	\x02\u{30f}\u{310}\x03\x02\x02\x02\u{310}\u{311}\x07\x05\x02\x02\u{311}\
	\u{313}\x07\x04\x02\x02\u{312}\u{314}\x05\x36\x1c\x02\u{313}\u{312}\x03\
	\x02\x02\x02\u{314}\u{315}\x03\x02\x02\x02\u{315}\u{313}\x03\x02\x02\x02\
	\u{315}\u{316}\x03\x02\x02\x02\u{316}\u{317}\x03\x02\x02\x02\u{317}\u{318}\
	\x07\x05\x02\x02\u{318}\u{319}\x07\x05\x02\x02\u{319}\u{380}\x03\x02\x02\
	\x02\u{31a}\u{31b}\x07\x04\x02\x02\u{31b}\u{31c}\x05\x6e\x38\x02\u{31c}\
	\u{31d}\x05\x10\x09\x02\u{31d}\u{321}\x07\x04\x02\x02\u{31e}\u{320}\x05\
	\x10\x09\x02\u{31f}\u{31e}\x03\x02\x02\x02\u{320}\u{323}\x03\x02\x02\x02\
	\u{321}\u{31f}\x03\x02\x02\x02\u{321}\u{322}\x03\x02\x02\x02\u{322}\u{324}\
	\x03\x02\x02\x02\u{323}\u{321}\x03\x02\x02\x02\u{324}\u{325}\x07\x05\x02\
	\x02\u{325}\u{326}\x05\x2a\x16\x02\u{326}\u{327}\x07\x05\x02\x02\u{327}\
	\u{380}\x03\x02\x02\x02\u{328}\u{329}\x07\x04\x02\x02\u{329}\u{32a}\x05\
	\x70\x39\x02\u{32a}\u{32b}\x05\x1a\x0e\x02\u{32b}\u{32c}\x07\x05\x02\x02\
	\u{32c}\u{380}\x03\x02\x02\x02\u{32d}\u{32e}\x07\x04\x02\x02\u{32e}\u{32f}\
	\x05\x72\x3a\x02\u{32f}\u{330}\x07\x05\x02\x02\u{330}\u{380}\x03\x02\x02\
	\x02\u{331}\u{332}\x07\x04\x02\x02\u{332}\u{333}\x05\x74\x3b\x02\u{333}\
	\u{334}\x07\x05\x02\x02\u{334}\u{380}\x03\x02\x02\x02\u{335}\u{336}\x07\
	\x04\x02\x02\u{336}\u{337}\x05\x76\x3c\x02\u{337}\u{338}\x07\x05\x02\x02\
	\u{338}\u{380}\x03\x02\x02\x02\u{339}\u{33a}\x07\x04\x02\x02\u{33a}\u{33b}\
	\x05\x78\x3d\x02\u{33b}\u{33c}\x05\u{9a}\x4e\x02\u{33c}\u{33d}\x07\x05\x02\
	\x02\u{33d}\u{380}\x03\x02\x02\x02\u{33e}\u{33f}\x07\x04\x02\x02\u{33f}\
	\u{340}\x05\x7a\x3e\x02\u{340}\u{341}\x07\x05\x02\x02\u{341}\u{380}\x03\
	\x02\x02\x02\u{342}\u{343}\x07\x04\x02\x02\u{343}\u{344}\x05\x7c\x3f\x02\
	\u{344}\u{345}\x05\x1c\x0f\x02\u{345}\u{346}\x07\x05\x02\x02\u{346}\u{380}\
	\x03\x02\x02\x02\u{347}\u{348}\x07\x04\x02\x02\u{348}\u{349}\x05\x7e\x40\
	\x02\u{349}\u{34a}\x07\x05\x02\x02\u{34a}\u{380}\x03\x02\x02\x02\u{34b}\
	\u{34c}\x07\x04\x02\x02\u{34c}\u{34d}\x05\u{80}\x41\x02\u{34d}\u{34e}\x07\
	\x05\x02\x02\u{34e}\u{380}\x03\x02\x02\x02\u{34f}\u{350}\x07\x04\x02\x02\
	\u{350}\u{351}\x05\u{82}\x42\x02\u{351}\u{352}\x07\x05\x02\x02\u{352}\u{380}\
	\x03\x02\x02\x02\u{353}\u{354}\x07\x04\x02\x02\u{354}\u{355}\x05\u{84}\x43\
	\x02\u{355}\u{357}\x07\x04\x02\x02\u{356}\u{358}\x05\x36\x1c\x02\u{357}\
	\u{356}\x03\x02\x02\x02\u{358}\u{359}\x03\x02\x02\x02\u{359}\u{357}\x03\
	\x02\x02\x02\u{359}\u{35a}\x03\x02\x02\x02\u{35a}\u{35b}\x03\x02\x02\x02\
	\u{35b}\u{35c}\x07\x05\x02\x02\u{35c}\u{35d}\x07\x05\x02\x02\u{35d}\u{380}\
	\x03\x02\x02\x02\u{35e}\u{35f}\x07\x04\x02\x02\u{35f}\u{360}\x05\u{86}\x44\
	\x02\u{360}\u{361}\x05\x12\x0a\x02\u{361}\u{362}\x07\x05\x02\x02\u{362}\
	\u{380}\x03\x02\x02\x02\u{363}\u{364}\x07\x04\x02\x02\u{364}\u{365}\x05\
	\u{88}\x45\x02\u{365}\u{366}\x05\x12\x0a\x02\u{366}\u{367}\x07\x05\x02\x02\
	\u{367}\u{380}\x03\x02\x02\x02\u{368}\u{369}\x07\x04\x02\x02\u{369}\u{36a}\
	\x05\u{8a}\x46\x02\u{36a}\u{36b}\x07\x05\x02\x02\u{36b}\u{380}\x03\x02\x02\
	\x02\u{36c}\u{36d}\x07\x04\x02\x02\u{36d}\u{36e}\x05\u{8c}\x47\x02\u{36e}\
	\u{36f}\x07\x05\x02\x02\u{36f}\u{380}\x03\x02\x02\x02\u{370}\u{371}\x07\
	\x04\x02\x02\u{371}\u{372}\x05\u{8e}\x48\x02\u{372}\u{373}\x05\x28\x15\x02\
	\u{373}\u{374}\x07\x05\x02\x02\u{374}\u{380}\x03\x02\x02\x02\u{375}\u{376}\
	\x07\x04\x02\x02\u{376}\u{377}\x05\u{90}\x49\x02\u{377}\u{378}\x05\x10\x09\
	\x02\u{378}\u{379}\x07\x05\x02\x02\u{379}\u{380}\x03\x02\x02\x02\u{37a}\
	\u{37b}\x07\x04\x02\x02\u{37b}\u{37c}\x05\u{92}\x4a\x02\u{37c}\u{37d}\x05\
	\u{98}\x4d\x02\u{37d}\u{37e}\x07\x05\x02\x02\u{37e}\u{380}\x03\x02\x02\x02\
	\u{37f}\u{2bf}\x03\x02\x02\x02\u{37f}\u{2c4}\x03\x02\x02\x02\u{37f}\u{2c8}\
	\x03\x02\x02\x02\u{37f}\u{2cc}\x03\x02\x02\x02\u{37f}\u{2d2}\x03\x02\x02\
	\x02\u{37f}\u{2d8}\x03\x02\x02\x02\u{37f}\u{2ea}\x03\x02\x02\x02\u{37f}\
	\u{2f8}\x03\x02\x02\x02\u{37f}\u{2fe}\x03\x02\x02\x02\u{37f}\u{303}\x03\
	\x02\x02\x02\u{37f}\u{308}\x03\x02\x02\x02\u{37f}\u{31a}\x03\x02\x02\x02\
	\u{37f}\u{328}\x03\x02\x02\x02\u{37f}\u{32d}\x03\x02\x02\x02\u{37f}\u{331}\
	\x03\x02\x02\x02\u{37f}\u{335}\x03\x02\x02\x02\u{37f}\u{339}\x03\x02\x02\
	\x02\u{37f}\u{33e}\x03\x02\x02\x02\u{37f}\u{342}\x03\x02\x02\x02\u{37f}\
	\u{347}\x03\x02\x02\x02\u{37f}\u{34b}\x03\x02\x02\x02\u{37f}\u{34f}\x03\
	\x02\x02\x02\u{37f}\u{353}\x03\x02\x02\x02\u{37f}\u{35e}\x03\x02\x02\x02\
	\u{37f}\u{363}\x03\x02\x02\x02\u{37f}\u{368}\x03\x02\x02\x02\u{37f}\u{36c}\
	\x03\x02\x02\x02\u{37f}\u{370}\x03\x02\x02\x02\u{37f}\u{375}\x03\x02\x02\
	\x02\u{37f}\u{37a}\x03\x02\x02\x02\u{380}\u{95}\x03\x02\x02\x02\u{381}\u{382}\
	\x09\x06\x02\x02\u{382}\u{97}\x03\x02\x02\x02\u{383}\u{384}\x07\x4f\x02\
	\x02\u{384}\u{3a1}\x05\x1a\x0e\x02\u{385}\u{386}\x07\x54\x02\x02\u{386}\
	\u{3a1}\x05\u{96}\x4c\x02\u{387}\u{388}\x07\x55\x02\x02\u{388}\u{3a1}\x05\
	\u{96}\x4c\x02\u{389}\u{38a}\x07\x5d\x02\x02\u{38a}\u{3a1}\x05\u{96}\x4c\
	\x02\u{38b}\u{38c}\x07\x5e\x02\x02\u{38c}\u{3a1}\x05\u{96}\x4c\x02\u{38d}\
	\u{38e}\x07\x5f\x02\x02\u{38e}\u{3a1}\x05\u{96}\x4c\x02\u{38f}\u{390}\x07\
	\x60\x02\x02\u{390}\u{3a1}\x05\u{96}\x4c\x02\u{391}\u{392}\x07\x61\x02\x02\
	\u{392}\u{3a1}\x05\u{96}\x4c\x02\u{393}\u{394}\x07\x62\x02\x02\u{394}\u{3a1}\
	\x05\u{96}\x4c\x02\u{395}\u{396}\x07\x63\x02\x02\u{396}\u{3a1}\x05\u{96}\
	\x4c\x02\u{397}\u{398}\x07\x64\x02\x02\u{398}\u{3a1}\x05\x12\x0a\x02\u{399}\
	\u{39a}\x07\x66\x02\x02\u{39a}\u{3a1}\x05\x1a\x0e\x02\u{39b}\u{39c}\x07\
	\x67\x02\x02\u{39c}\u{3a1}\x05\x12\x0a\x02\u{39d}\u{39e}\x07\x70\x02\x02\
	\u{39e}\u{3a1}\x05\x12\x0a\x02\u{39f}\u{3a1}\x05\x28\x15\x02\u{3a0}\u{383}\
	\x03\x02\x02\x02\u{3a0}\u{385}\x03\x02\x02\x02\u{3a0}\u{387}\x03\x02\x02\
	\x02\u{3a0}\u{389}\x03\x02\x02\x02\u{3a0}\u{38b}\x03\x02\x02\x02\u{3a0}\
	\u{38d}\x03\x02\x02\x02\u{3a0}\u{38f}\x03\x02\x02\x02\u{3a0}\u{391}\x03\
	\x02\x02\x02\u{3a0}\u{393}\x03\x02\x02\x02\u{3a0}\u{395}\x03\x02\x02\x02\
	\u{3a0}\u{397}\x03\x02\x02\x02\u{3a0}\u{399}\x03\x02\x02\x02\u{3a0}\u{39b}\
	\x03\x02\x02\x02\u{3a0}\u{39d}\x03\x02\x02\x02\u{3a0}\u{39f}\x03\x02\x02\
	\x02\u{3a1}\u{99}\x03\x02\x02\x02\u{3a2}\u{3ab}\x07\x49\x02\x02\u{3a3}\u{3ab}\
	\x07\x4a\x02\x02\u{3a4}\u{3ab}\x07\x4b\x02\x02\u{3a5}\u{3ab}\x07\x50\x02\
	\x02\u{3a6}\u{3ab}\x07\x5a\x02\x02\u{3a7}\u{3ab}\x07\x65\x02\x02\u{3a8}\
	\u{3ab}\x07\x71\x02\x02\u{3a9}\u{3ab}\x05\x1c\x0f\x02\u{3aa}\u{3a2}\x03\
	\x02\x02\x02\u{3aa}\u{3a3}\x03\x02\x02\x02\u{3aa}\u{3a4}\x03\x02\x02\x02\
	\u{3aa}\u{3a5}\x03\x02\x02\x02\u{3aa}\u{3a6}\x03\x02\x02\x02\u{3aa}\u{3a7}\
	\x03\x02\x02\x02\u{3aa}\u{3a8}\x03\x02\x02\x02\u{3aa}\u{3a9}\x03\x02\x02\
	\x02\u{3ab}\u{9b}\x03\x02\x02\x02\u{3ac}\u{3ad}\x09\x07\x02\x02\u{3ad}\u{9d}\
	\x03\x02\x02\x02\u{3ae}\u{3b2}\x07\x11\x02\x02\u{3af}\u{3b2}\x07\x0f\x02\
	\x02\u{3b0}\u{3b2}\x05\x20\x11\x02\u{3b1}\u{3ae}\x03\x02\x02\x02\u{3b1}\
	\u{3af}\x03\x02\x02\x02\u{3b1}\u{3b0}\x03\x02\x02\x02\u{3b2}\u{9f}\x03\x02\
	\x02\x02\u{3b3}\u{3b4}\x07\x04\x02\x02\u{3b4}\u{3b5}\x07\x21\x02\x02\u{3b5}\
	\u{3b6}\x05\x52\x2a\x02\u{3b6}\u{3b7}\x07\x05\x02\x02\u{3b7}\u{3d0}\x03\
	\x02\x02\x02\u{3b8}\u{3b9}\x07\x04\x02\x02\u{3b9}\u{3ba}\x07\x22\x02\x02\
	\u{3ba}\u{3bb}\x05\x52\x2a\x02\u{3bb}\u{3bc}\x07\x05\x02\x02\u{3bc}\u{3d0}\
	\x03\x02\x02\x02\u{3bd}\u{3be}\x07\x04\x02\x02\u{3be}\u{3bf}\x07\x23\x02\
	\x02\u{3bf}\u{3c1}\x07\x04\x02\x02\u{3c0}\u{3c2}\x05\x50\x29\x02\u{3c1}\
	\u{3c0}\x03\x02\x02\x02\u{3c2}\u{3c3}\x03\x02\x02\x02\u{3c3}\u{3c1}\x03\
	\x02\x02\x02\u{3c3}\u{3c4}\x03\x02\x02\x02\u{3c4}\u{3c5}\x03\x02\x02\x02\
	\u{3c5}\u{3c6}\x07\x05\x02\x02\u{3c6}\u{3c8}\x07\x04\x02\x02\u{3c7}\u{3c9}\
	\x05\x36\x1c\x02\u{3c8}\u{3c7}\x03\x02\x02\x02\u{3c9}\u{3ca}\x03\x02\x02\
	\x02\u{3ca}\u{3c8}\x03\x02\x02\x02\u{3ca}\u{3cb}\x03\x02\x02\x02\u{3cb}\
	\u{3cc}\x03\x02\x02\x02\u{3cc}\u{3cd}\x07\x05\x02\x02\u{3cd}\u{3ce}\x07\
	\x05\x02\x02\u{3ce}\u{3d0}\x03\x02\x02\x02\u{3cf}\u{3b3}\x03\x02\x02\x02\
	\u{3cf}\u{3b8}\x03\x02\x02\x02\u{3cf}\u{3bd}\x03\x02\x02\x02\u{3d0}\u{a1}\
	\x03\x02\x02\x02\u{3d1}\u{3d2}\x07\x4a\x02\x02\u{3d2}\u{3df}\x05\x12\x0a\
	\x02\u{3d3}\u{3d4}\x07\x4b\x02\x02\u{3d4}\u{3df}\x05\x1a\x0e\x02\u{3d5}\
	\u{3d6}\x07\x50\x02\x02\u{3d6}\u{3df}\x05\u{9c}\x4f\x02\u{3d7}\u{3d8}\x07\
	\x5a\x02\x02\u{3d8}\u{3df}\x05\x1a\x0e\x02\u{3d9}\u{3da}\x07\x65\x02\x02\
	\u{3da}\u{3df}\x05\u{9e}\x50\x02\u{3db}\u{3dc}\x07\x71\x02\x02\u{3dc}\u{3df}\
	\x05\x1a\x0e\x02\u{3dd}\u{3df}\x05\x28\x15\x02\u{3de}\u{3d1}\x03\x02\x02\
	\x02\u{3de}\u{3d3}\x03\x02\x02\x02\u{3de}\u{3d5}\x03\x02\x02\x02\u{3de}\
	\u{3d7}\x03\x02\x02\x02\u{3de}\u{3d9}\x03\x02\x02\x02\u{3de}\u{3db}\x03\
	\x02\x02\x02\u{3de}\u{3dd}\x03\x02\x02\x02\u{3df}\u{a3}\x03\x02\x02\x02\
	\u{3e0}\u{3e1}\x07\x04\x02\x02\u{3e1}\u{3e2}\x05\x36\x1c\x02\u{3e2}\u{3e3}\
	\x05\x36\x1c\x02\u{3e3}\u{3e4}\x07\x05\x02\x02\u{3e4}\u{a5}\x03\x02\x02\
	\x02\u{3e5}\u{3e6}\x07\x04\x02\x02\u{3e6}\u{3e7}\x05\x10\x09\x02\u{3e7}\
	\u{3e8}\x05\u{96}\x4c\x02\u{3e8}\u{3e9}\x07\x05\x02\x02\u{3e9}\u{a7}\x03\
	\x02\x02\x02\u{3ea}\u{3eb}\x09\x08\x02\x02\u{3eb}\u{a9}\x03\x02\x02\x02\
	\u{3ec}\u{3ed}\x05\x1a\x0e\x02\u{3ed}\u{ab}\x03\x02\x02\x02\u{3ee}\u{3f2}\
	\x07\x04\x02\x02\u{3ef}\u{3f1}\x05\x36\x1c\x02\u{3f0}\u{3ef}\x03\x02\x02\
	\x02\u{3f1}\u{3f4}\x03\x02\x02\x02\u{3f2}\u{3f0}\x03\x02\x02\x02\u{3f2}\
	\u{3f3}\x03\x02\x02\x02\u{3f3}\u{3f5}\x03\x02\x02\x02\u{3f4}\u{3f2}\x03\
	\x02\x02\x02\u{3f5}\u{3f6}\x07\x05\x02\x02\u{3f6}\u{ad}\x03\x02\x02\x02\
	\u{3f7}\u{3fb}\x07\x04\x02\x02\u{3f8}\u{3fa}\x05\u{a6}\x54\x02\u{3f9}\u{3f8}\
	\x03\x02\x02\x02\u{3fa}\u{3fd}\x03\x02\x02\x02\u{3fb}\u{3f9}\x03\x02\x02\
	\x02\u{3fb}\u{3fc}\x03\x02\x02\x02\u{3fc}\u{3fe}\x03\x02\x02\x02\u{3fd}\
	\u{3fb}\x03\x02\x02\x02\u{3fe}\u{3ff}\x07\x05\x02\x02\u{3ff}\u{af}\x03\x02\
	\x02\x02\u{400}\u{402}\x07\x04\x02\x02\u{401}\u{403}\x05\u{a2}\x52\x02\u{402}\
	\u{401}\x03\x02\x02\x02\u{403}\u{404}\x03\x02\x02\x02\u{404}\u{402}\x03\
	\x02\x02\x02\u{404}\u{405}\x03\x02\x02\x02\u{405}\u{406}\x03\x02\x02\x02\
	\u{406}\u{407}\x07\x05\x02\x02\u{407}\u{b1}\x03\x02\x02\x02\u{408}\u{40c}\
	\x07\x04\x02\x02\u{409}\u{40b}\x05\u{a0}\x51\x02\u{40a}\u{409}\x03\x02\x02\
	\x02\u{40b}\u{40e}\x03\x02\x02\x02\u{40c}\u{40a}\x03\x02\x02\x02\u{40c}\
	\u{40d}\x03\x02\x02\x02\u{40d}\u{40f}\x03\x02\x02\x02\u{40e}\u{40c}\x03\
	\x02\x02\x02\u{40f}\u{410}\x07\x05\x02\x02\u{410}\u{b3}\x03\x02\x02\x02\
	\u{411}\u{412}\x05\x26\x14\x02\u{412}\u{b5}\x03\x02\x02\x02\u{413}\u{414}\
	\x05\x20\x11\x02\u{414}\u{b7}\x03\x02\x02\x02\u{415}\u{419}\x07\x04\x02\
	\x02\u{416}\u{418}\x05\x10\x09\x02\u{417}\u{416}\x03\x02\x02\x02\u{418}\
	\u{41b}\x03\x02\x02\x02\u{419}\u{417}\x03\x02\x02\x02\u{419}\u{41a}\x03\
	\x02\x02\x02\u{41a}\u{41c}\x03\x02\x02\x02\u{41b}\u{419}\x03\x02\x02\x02\
	\u{41c}\u{41d}\x07\x05\x02\x02\u{41d}\u{b9}\x03\x02\x02\x02\u{41e}\u{422}\
	\x07\x04\x02\x02\u{41f}\u{421}\x05\x10\x09\x02\u{420}\u{41f}\x03\x02\x02\
	\x02\u{421}\u{424}\x03\x02\x02\x02\u{422}\u{420}\x03\x02\x02\x02\u{422}\
	\u{423}\x03\x02\x02\x02\u{423}\u{425}\x03\x02\x02\x02\u{424}\u{422}\x03\
	\x02\x02\x02\u{425}\u{426}\x07\x05\x02\x02\u{426}\u{bb}\x03\x02\x02\x02\
	\u{427}\u{429}\x07\x04\x02\x02\u{428}\u{42a}\x05\u{a4}\x53\x02\u{429}\u{428}\
	\x03\x02\x02\x02\u{42a}\u{42b}\x03\x02\x02\x02\u{42b}\u{429}\x03\x02\x02\
	\x02\u{42b}\u{42c}\x03\x02\x02\x02\u{42c}\u{42d}\x03\x02\x02\x02\u{42d}\
	\u{42e}\x07\x05\x02\x02\u{42e}\u{bd}\x03\x02\x02\x02\u{42f}\u{43b}\x05\u{a8}\
	\x55\x02\u{430}\u{43b}\x05\u{aa}\x56\x02\u{431}\u{43b}\x05\u{ac}\x57\x02\
	\u{432}\u{43b}\x05\u{ae}\x58\x02\u{433}\u{43b}\x05\u{b0}\x59\x02\u{434}\
	\u{43b}\x05\u{b2}\x5a\x02\u{435}\u{43b}\x05\u{b4}\x5b\x02\u{436}\u{43b}\
	\x05\u{b6}\x5c\x02\u{437}\u{43b}\x05\u{b8}\x5d\x02\u{438}\u{43b}\x05\u{ba}\
	\x5e\x02\u{439}\u{43b}\x05\u{bc}\x5f\x02\u{43a}\u{42f}\x03\x02\x02\x02\u{43a}\
	\u{430}\x03\x02\x02\x02\u{43a}\u{431}\x03\x02\x02\x02\u{43a}\u{432}\x03\
	\x02\x02\x02\u{43a}\u{433}\x03\x02\x02\x02\u{43a}\u{434}\x03\x02\x02\x02\
	\u{43a}\u{435}\x03\x02\x02\x02\u{43a}\u{436}\x03\x02\x02\x02\u{43a}\u{437}\
	\x03\x02\x02\x02\u{43a}\u{438}\x03\x02\x02\x02\u{43a}\u{439}\x03\x02\x02\
	\x02\u{43b}\u{bf}\x03\x02\x02\x02\u{43c}\u{445}\x07\x13\x02\x02\u{43d}\u{445}\
	\x05\u{be}\x60\x02\u{43e}\u{445}\x07\x17\x02\x02\u{43f}\u{440}\x07\x04\x02\
	\x02\u{440}\u{441}\x07\x0c\x02\x02\u{441}\u{442}\x05\x1a\x0e\x02\u{442}\
	\u{443}\x07\x05\x02\x02\u{443}\u{445}\x03\x02\x02\x02\u{444}\u{43c}\x03\
	\x02\x02\x02\u{444}\u{43d}\x03\x02\x02\x02\u{444}\u{43e}\x03\x02\x02\x02\
	\u{444}\u{43f}\x03\x02\x02\x02\u{445}\u{c1}\x03\x02\x02\x02\x4c\u{cc}\u{d6}\
	\u{e5}\u{ec}\u{f5}\u{f9}\u{fd}\u{106}\u{10a}\u{112}\u{116}\u{11c}\u{124}\
	\u{128}\u{131}\u{143}\u{147}\u{155}\u{15f}\u{16b}\u{177}\u{184}\u{18f}\u{193}\
	\u{19b}\u{1a8}\u{1b3}\u{1bd}\u{1c2}\u{1c7}\u{1d0}\u{1d8}\u{1dd}\u{1e3}\u{1ea}\
	\u{1f3}\u{202}\u{20a}\u{213}\u{220}\u{228}\u{23b}\u{244}\u{24e}\u{255}\u{25a}\
	\u{262}\u{26e}\u{27b}\u{280}\u{2de}\u{2e5}\u{2f1}\u{30e}\u{315}\u{321}\u{359}\
	\u{37f}\u{3a0}\u{3aa}\u{3b1}\u{3c3}\u{3ca}\u{3cf}\u{3de}\u{3f2}\u{3fb}\u{404}\
	\u{40c}\u{419}\u{422}\u{42b}\u{43a}\u{444}";
