#![allow(nonstandard_style)]
#![allow(warnings)]
// Generated from src/parser/SMTLIBv2.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::smtlibv2parser::*;

pub trait SMTLIBv2Listener<'input> : ParseTreeListener<'input,SMTLIBv2ParserContextType>{

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#start}.
 * @param ctx the parse tree
 */
fn enter_start(&mut self, _ctx: &StartContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#start}.
 * @param ctx the parse tree
 */
fn exit_start(&mut self, _ctx: &StartContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#response}.
 * @param ctx the parse tree
 */
fn enter_response(&mut self, _ctx: &ResponseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#response}.
 * @param ctx the parse tree
 */
fn exit_response(&mut self, _ctx: &ResponseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#generalReservedWord}.
 * @param ctx the parse tree
 */
fn enter_generalReservedWord(&mut self, _ctx: &GeneralReservedWordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#generalReservedWord}.
 * @param ctx the parse tree
 */
fn exit_generalReservedWord(&mut self, _ctx: &GeneralReservedWordContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#simpleSymbol}.
 * @param ctx the parse tree
 */
fn enter_simpleSymbol(&mut self, _ctx: &SimpleSymbolContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#simpleSymbol}.
 * @param ctx the parse tree
 */
fn exit_simpleSymbol(&mut self, _ctx: &SimpleSymbolContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#quotedSymbol}.
 * @param ctx the parse tree
 */
fn enter_quotedSymbol(&mut self, _ctx: &QuotedSymbolContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#quotedSymbol}.
 * @param ctx the parse tree
 */
fn exit_quotedSymbol(&mut self, _ctx: &QuotedSymbolContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#predefSymbol}.
 * @param ctx the parse tree
 */
fn enter_predefSymbol(&mut self, _ctx: &PredefSymbolContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#predefSymbol}.
 * @param ctx the parse tree
 */
fn exit_predefSymbol(&mut self, _ctx: &PredefSymbolContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#predefKeyword}.
 * @param ctx the parse tree
 */
fn enter_predefKeyword(&mut self, _ctx: &PredefKeywordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#predefKeyword}.
 * @param ctx the parse tree
 */
fn exit_predefKeyword(&mut self, _ctx: &PredefKeywordContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#symbol}.
 * @param ctx the parse tree
 */
fn enter_symbol(&mut self, _ctx: &SymbolContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#symbol}.
 * @param ctx the parse tree
 */
fn exit_symbol(&mut self, _ctx: &SymbolContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#numeral}.
 * @param ctx the parse tree
 */
fn enter_numeral(&mut self, _ctx: &NumeralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#numeral}.
 * @param ctx the parse tree
 */
fn exit_numeral(&mut self, _ctx: &NumeralContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#decimal}.
 * @param ctx the parse tree
 */
fn enter_decimal(&mut self, _ctx: &DecimalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#decimal}.
 * @param ctx the parse tree
 */
fn exit_decimal(&mut self, _ctx: &DecimalContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#hexadecimal}.
 * @param ctx the parse tree
 */
fn enter_hexadecimal(&mut self, _ctx: &HexadecimalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#hexadecimal}.
 * @param ctx the parse tree
 */
fn exit_hexadecimal(&mut self, _ctx: &HexadecimalContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#binary}.
 * @param ctx the parse tree
 */
fn enter_binary(&mut self, _ctx: &BinaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#binary}.
 * @param ctx the parse tree
 */
fn exit_binary(&mut self, _ctx: &BinaryContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#string}.
 * @param ctx the parse tree
 */
fn enter_string(&mut self, _ctx: &StringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#string}.
 * @param ctx the parse tree
 */
fn exit_string(&mut self, _ctx: &StringContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#keyword}.
 * @param ctx the parse tree
 */
fn enter_keyword(&mut self, _ctx: &KeywordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#keyword}.
 * @param ctx the parse tree
 */
fn exit_keyword(&mut self, _ctx: &KeywordContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#spec_constant}.
 * @param ctx the parse tree
 */
fn enter_spec_constant(&mut self, _ctx: &Spec_constantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#spec_constant}.
 * @param ctx the parse tree
 */
fn exit_spec_constant(&mut self, _ctx: &Spec_constantContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#s_expr}.
 * @param ctx the parse tree
 */
fn enter_s_expr(&mut self, _ctx: &S_exprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#s_expr}.
 * @param ctx the parse tree
 */
fn exit_s_expr(&mut self, _ctx: &S_exprContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#index}.
 * @param ctx the parse tree
 */
fn enter_index(&mut self, _ctx: &IndexContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#index}.
 * @param ctx the parse tree
 */
fn exit_index(&mut self, _ctx: &IndexContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#identifier}.
 * @param ctx the parse tree
 */
fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#identifier}.
 * @param ctx the parse tree
 */
fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#attribute_value}.
 * @param ctx the parse tree
 */
fn enter_attribute_value(&mut self, _ctx: &Attribute_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#attribute_value}.
 * @param ctx the parse tree
 */
fn exit_attribute_value(&mut self, _ctx: &Attribute_valueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#attribute}.
 * @param ctx the parse tree
 */
fn enter_attribute(&mut self, _ctx: &AttributeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#attribute}.
 * @param ctx the parse tree
 */
fn exit_attribute(&mut self, _ctx: &AttributeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#sort}.
 * @param ctx the parse tree
 */
fn enter_sort(&mut self, _ctx: &SortContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#sort}.
 * @param ctx the parse tree
 */
fn exit_sort(&mut self, _ctx: &SortContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#qual_identifier}.
 * @param ctx the parse tree
 */
fn enter_qual_identifier(&mut self, _ctx: &Qual_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#qual_identifier}.
 * @param ctx the parse tree
 */
fn exit_qual_identifier(&mut self, _ctx: &Qual_identifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#var_binding}.
 * @param ctx the parse tree
 */
fn enter_var_binding(&mut self, _ctx: &Var_bindingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#var_binding}.
 * @param ctx the parse tree
 */
fn exit_var_binding(&mut self, _ctx: &Var_bindingContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#sorted_var}.
 * @param ctx the parse tree
 */
fn enter_sorted_var(&mut self, _ctx: &Sorted_varContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#sorted_var}.
 * @param ctx the parse tree
 */
fn exit_sorted_var(&mut self, _ctx: &Sorted_varContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#pattern}.
 * @param ctx the parse tree
 */
fn enter_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#pattern}.
 * @param ctx the parse tree
 */
fn exit_pattern(&mut self, _ctx: &PatternContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#match_case}.
 * @param ctx the parse tree
 */
fn enter_match_case(&mut self, _ctx: &Match_caseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#match_case}.
 * @param ctx the parse tree
 */
fn exit_match_case(&mut self, _ctx: &Match_caseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#term}.
 * @param ctx the parse tree
 */
fn enter_term(&mut self, _ctx: &TermContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#term}.
 * @param ctx the parse tree
 */
fn exit_term(&mut self, _ctx: &TermContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#sort_symbol_decl}.
 * @param ctx the parse tree
 */
fn enter_sort_symbol_decl(&mut self, _ctx: &Sort_symbol_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#sort_symbol_decl}.
 * @param ctx the parse tree
 */
fn exit_sort_symbol_decl(&mut self, _ctx: &Sort_symbol_declContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#meta_spec_constant}.
 * @param ctx the parse tree
 */
fn enter_meta_spec_constant(&mut self, _ctx: &Meta_spec_constantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#meta_spec_constant}.
 * @param ctx the parse tree
 */
fn exit_meta_spec_constant(&mut self, _ctx: &Meta_spec_constantContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#fun_symbol_decl}.
 * @param ctx the parse tree
 */
fn enter_fun_symbol_decl(&mut self, _ctx: &Fun_symbol_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#fun_symbol_decl}.
 * @param ctx the parse tree
 */
fn exit_fun_symbol_decl(&mut self, _ctx: &Fun_symbol_declContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#par_fun_symbol_decl}.
 * @param ctx the parse tree
 */
fn enter_par_fun_symbol_decl(&mut self, _ctx: &Par_fun_symbol_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#par_fun_symbol_decl}.
 * @param ctx the parse tree
 */
fn exit_par_fun_symbol_decl(&mut self, _ctx: &Par_fun_symbol_declContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#theory_attribute}.
 * @param ctx the parse tree
 */
fn enter_theory_attribute(&mut self, _ctx: &Theory_attributeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#theory_attribute}.
 * @param ctx the parse tree
 */
fn exit_theory_attribute(&mut self, _ctx: &Theory_attributeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#theory_decl}.
 * @param ctx the parse tree
 */
fn enter_theory_decl(&mut self, _ctx: &Theory_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#theory_decl}.
 * @param ctx the parse tree
 */
fn exit_theory_decl(&mut self, _ctx: &Theory_declContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#logic_attribue}.
 * @param ctx the parse tree
 */
fn enter_logic_attribue(&mut self, _ctx: &Logic_attribueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#logic_attribue}.
 * @param ctx the parse tree
 */
fn exit_logic_attribue(&mut self, _ctx: &Logic_attribueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#logic}.
 * @param ctx the parse tree
 */
fn enter_logic(&mut self, _ctx: &LogicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#logic}.
 * @param ctx the parse tree
 */
fn exit_logic(&mut self, _ctx: &LogicContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#sort_dec}.
 * @param ctx the parse tree
 */
fn enter_sort_dec(&mut self, _ctx: &Sort_decContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#sort_dec}.
 * @param ctx the parse tree
 */
fn exit_sort_dec(&mut self, _ctx: &Sort_decContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#selector_dec}.
 * @param ctx the parse tree
 */
fn enter_selector_dec(&mut self, _ctx: &Selector_decContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#selector_dec}.
 * @param ctx the parse tree
 */
fn exit_selector_dec(&mut self, _ctx: &Selector_decContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#constructor_dec}.
 * @param ctx the parse tree
 */
fn enter_constructor_dec(&mut self, _ctx: &Constructor_decContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#constructor_dec}.
 * @param ctx the parse tree
 */
fn exit_constructor_dec(&mut self, _ctx: &Constructor_decContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#datatype_dec}.
 * @param ctx the parse tree
 */
fn enter_datatype_dec(&mut self, _ctx: &Datatype_decContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#datatype_dec}.
 * @param ctx the parse tree
 */
fn exit_datatype_dec(&mut self, _ctx: &Datatype_decContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#function_dec}.
 * @param ctx the parse tree
 */
fn enter_function_dec(&mut self, _ctx: &Function_decContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#function_dec}.
 * @param ctx the parse tree
 */
fn exit_function_dec(&mut self, _ctx: &Function_decContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#function_def}.
 * @param ctx the parse tree
 */
fn enter_function_def(&mut self, _ctx: &Function_defContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#function_def}.
 * @param ctx the parse tree
 */
fn exit_function_def(&mut self, _ctx: &Function_defContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#prop_literal}.
 * @param ctx the parse tree
 */
fn enter_prop_literal(&mut self, _ctx: &Prop_literalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#prop_literal}.
 * @param ctx the parse tree
 */
fn exit_prop_literal(&mut self, _ctx: &Prop_literalContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#script}.
 * @param ctx the parse tree
 */
fn enter_script(&mut self, _ctx: &ScriptContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#script}.
 * @param ctx the parse tree
 */
fn exit_script(&mut self, _ctx: &ScriptContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_assert}.
 * @param ctx the parse tree
 */
fn enter_cmd_assert(&mut self, _ctx: &Cmd_assertContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_assert}.
 * @param ctx the parse tree
 */
fn exit_cmd_assert(&mut self, _ctx: &Cmd_assertContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_checkSat}.
 * @param ctx the parse tree
 */
fn enter_cmd_checkSat(&mut self, _ctx: &Cmd_checkSatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_checkSat}.
 * @param ctx the parse tree
 */
fn exit_cmd_checkSat(&mut self, _ctx: &Cmd_checkSatContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_checkSatAssuming}.
 * @param ctx the parse tree
 */
fn enter_cmd_checkSatAssuming(&mut self, _ctx: &Cmd_checkSatAssumingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_checkSatAssuming}.
 * @param ctx the parse tree
 */
fn exit_cmd_checkSatAssuming(&mut self, _ctx: &Cmd_checkSatAssumingContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_declareConst}.
 * @param ctx the parse tree
 */
fn enter_cmd_declareConst(&mut self, _ctx: &Cmd_declareConstContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_declareConst}.
 * @param ctx the parse tree
 */
fn exit_cmd_declareConst(&mut self, _ctx: &Cmd_declareConstContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_declareDatatype}.
 * @param ctx the parse tree
 */
fn enter_cmd_declareDatatype(&mut self, _ctx: &Cmd_declareDatatypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_declareDatatype}.
 * @param ctx the parse tree
 */
fn exit_cmd_declareDatatype(&mut self, _ctx: &Cmd_declareDatatypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_declareDatatypes}.
 * @param ctx the parse tree
 */
fn enter_cmd_declareDatatypes(&mut self, _ctx: &Cmd_declareDatatypesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_declareDatatypes}.
 * @param ctx the parse tree
 */
fn exit_cmd_declareDatatypes(&mut self, _ctx: &Cmd_declareDatatypesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_declareFun}.
 * @param ctx the parse tree
 */
fn enter_cmd_declareFun(&mut self, _ctx: &Cmd_declareFunContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_declareFun}.
 * @param ctx the parse tree
 */
fn exit_cmd_declareFun(&mut self, _ctx: &Cmd_declareFunContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_declareSort}.
 * @param ctx the parse tree
 */
fn enter_cmd_declareSort(&mut self, _ctx: &Cmd_declareSortContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_declareSort}.
 * @param ctx the parse tree
 */
fn exit_cmd_declareSort(&mut self, _ctx: &Cmd_declareSortContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_defineFun}.
 * @param ctx the parse tree
 */
fn enter_cmd_defineFun(&mut self, _ctx: &Cmd_defineFunContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_defineFun}.
 * @param ctx the parse tree
 */
fn exit_cmd_defineFun(&mut self, _ctx: &Cmd_defineFunContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_defineFunRec}.
 * @param ctx the parse tree
 */
fn enter_cmd_defineFunRec(&mut self, _ctx: &Cmd_defineFunRecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_defineFunRec}.
 * @param ctx the parse tree
 */
fn exit_cmd_defineFunRec(&mut self, _ctx: &Cmd_defineFunRecContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_defineFunsRec}.
 * @param ctx the parse tree
 */
fn enter_cmd_defineFunsRec(&mut self, _ctx: &Cmd_defineFunsRecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_defineFunsRec}.
 * @param ctx the parse tree
 */
fn exit_cmd_defineFunsRec(&mut self, _ctx: &Cmd_defineFunsRecContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_defineSort}.
 * @param ctx the parse tree
 */
fn enter_cmd_defineSort(&mut self, _ctx: &Cmd_defineSortContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_defineSort}.
 * @param ctx the parse tree
 */
fn exit_cmd_defineSort(&mut self, _ctx: &Cmd_defineSortContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_echo}.
 * @param ctx the parse tree
 */
fn enter_cmd_echo(&mut self, _ctx: &Cmd_echoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_echo}.
 * @param ctx the parse tree
 */
fn exit_cmd_echo(&mut self, _ctx: &Cmd_echoContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_exit}.
 * @param ctx the parse tree
 */
fn enter_cmd_exit(&mut self, _ctx: &Cmd_exitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_exit}.
 * @param ctx the parse tree
 */
fn exit_cmd_exit(&mut self, _ctx: &Cmd_exitContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getAssertions}.
 * @param ctx the parse tree
 */
fn enter_cmd_getAssertions(&mut self, _ctx: &Cmd_getAssertionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getAssertions}.
 * @param ctx the parse tree
 */
fn exit_cmd_getAssertions(&mut self, _ctx: &Cmd_getAssertionsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getAssignment}.
 * @param ctx the parse tree
 */
fn enter_cmd_getAssignment(&mut self, _ctx: &Cmd_getAssignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getAssignment}.
 * @param ctx the parse tree
 */
fn exit_cmd_getAssignment(&mut self, _ctx: &Cmd_getAssignmentContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getInfo}.
 * @param ctx the parse tree
 */
fn enter_cmd_getInfo(&mut self, _ctx: &Cmd_getInfoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getInfo}.
 * @param ctx the parse tree
 */
fn exit_cmd_getInfo(&mut self, _ctx: &Cmd_getInfoContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getModel}.
 * @param ctx the parse tree
 */
fn enter_cmd_getModel(&mut self, _ctx: &Cmd_getModelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getModel}.
 * @param ctx the parse tree
 */
fn exit_cmd_getModel(&mut self, _ctx: &Cmd_getModelContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getOption}.
 * @param ctx the parse tree
 */
fn enter_cmd_getOption(&mut self, _ctx: &Cmd_getOptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getOption}.
 * @param ctx the parse tree
 */
fn exit_cmd_getOption(&mut self, _ctx: &Cmd_getOptionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getProof}.
 * @param ctx the parse tree
 */
fn enter_cmd_getProof(&mut self, _ctx: &Cmd_getProofContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getProof}.
 * @param ctx the parse tree
 */
fn exit_cmd_getProof(&mut self, _ctx: &Cmd_getProofContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getUnsatAssumptions}.
 * @param ctx the parse tree
 */
fn enter_cmd_getUnsatAssumptions(&mut self, _ctx: &Cmd_getUnsatAssumptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getUnsatAssumptions}.
 * @param ctx the parse tree
 */
fn exit_cmd_getUnsatAssumptions(&mut self, _ctx: &Cmd_getUnsatAssumptionsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getUnsatCore}.
 * @param ctx the parse tree
 */
fn enter_cmd_getUnsatCore(&mut self, _ctx: &Cmd_getUnsatCoreContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getUnsatCore}.
 * @param ctx the parse tree
 */
fn exit_cmd_getUnsatCore(&mut self, _ctx: &Cmd_getUnsatCoreContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getValue}.
 * @param ctx the parse tree
 */
fn enter_cmd_getValue(&mut self, _ctx: &Cmd_getValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getValue}.
 * @param ctx the parse tree
 */
fn exit_cmd_getValue(&mut self, _ctx: &Cmd_getValueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_pop}.
 * @param ctx the parse tree
 */
fn enter_cmd_pop(&mut self, _ctx: &Cmd_popContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_pop}.
 * @param ctx the parse tree
 */
fn exit_cmd_pop(&mut self, _ctx: &Cmd_popContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_push}.
 * @param ctx the parse tree
 */
fn enter_cmd_push(&mut self, _ctx: &Cmd_pushContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_push}.
 * @param ctx the parse tree
 */
fn exit_cmd_push(&mut self, _ctx: &Cmd_pushContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_reset}.
 * @param ctx the parse tree
 */
fn enter_cmd_reset(&mut self, _ctx: &Cmd_resetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_reset}.
 * @param ctx the parse tree
 */
fn exit_cmd_reset(&mut self, _ctx: &Cmd_resetContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_resetAssertions}.
 * @param ctx the parse tree
 */
fn enter_cmd_resetAssertions(&mut self, _ctx: &Cmd_resetAssertionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_resetAssertions}.
 * @param ctx the parse tree
 */
fn exit_cmd_resetAssertions(&mut self, _ctx: &Cmd_resetAssertionsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_setInfo}.
 * @param ctx the parse tree
 */
fn enter_cmd_setInfo(&mut self, _ctx: &Cmd_setInfoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_setInfo}.
 * @param ctx the parse tree
 */
fn exit_cmd_setInfo(&mut self, _ctx: &Cmd_setInfoContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_setLogic}.
 * @param ctx the parse tree
 */
fn enter_cmd_setLogic(&mut self, _ctx: &Cmd_setLogicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_setLogic}.
 * @param ctx the parse tree
 */
fn exit_cmd_setLogic(&mut self, _ctx: &Cmd_setLogicContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_setOption}.
 * @param ctx the parse tree
 */
fn enter_cmd_setOption(&mut self, _ctx: &Cmd_setOptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_setOption}.
 * @param ctx the parse tree
 */
fn exit_cmd_setOption(&mut self, _ctx: &Cmd_setOptionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#command}.
 * @param ctx the parse tree
 */
fn enter_command(&mut self, _ctx: &CommandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#command}.
 * @param ctx the parse tree
 */
fn exit_command(&mut self, _ctx: &CommandContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#b_value}.
 * @param ctx the parse tree
 */
fn enter_b_value(&mut self, _ctx: &B_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#b_value}.
 * @param ctx the parse tree
 */
fn exit_b_value(&mut self, _ctx: &B_valueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#option}.
 * @param ctx the parse tree
 */
fn enter_option(&mut self, _ctx: &OptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#option}.
 * @param ctx the parse tree
 */
fn exit_option(&mut self, _ctx: &OptionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#info_flag}.
 * @param ctx the parse tree
 */
fn enter_info_flag(&mut self, _ctx: &Info_flagContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#info_flag}.
 * @param ctx the parse tree
 */
fn exit_info_flag(&mut self, _ctx: &Info_flagContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#error_behaviour}.
 * @param ctx the parse tree
 */
fn enter_error_behaviour(&mut self, _ctx: &Error_behaviourContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#error_behaviour}.
 * @param ctx the parse tree
 */
fn exit_error_behaviour(&mut self, _ctx: &Error_behaviourContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#reason_unknown}.
 * @param ctx the parse tree
 */
fn enter_reason_unknown(&mut self, _ctx: &Reason_unknownContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#reason_unknown}.
 * @param ctx the parse tree
 */
fn exit_reason_unknown(&mut self, _ctx: &Reason_unknownContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#model_response}.
 * @param ctx the parse tree
 */
fn enter_model_response(&mut self, _ctx: &Model_responseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#model_response}.
 * @param ctx the parse tree
 */
fn exit_model_response(&mut self, _ctx: &Model_responseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#info_response}.
 * @param ctx the parse tree
 */
fn enter_info_response(&mut self, _ctx: &Info_responseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#info_response}.
 * @param ctx the parse tree
 */
fn exit_info_response(&mut self, _ctx: &Info_responseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#valuation_pair}.
 * @param ctx the parse tree
 */
fn enter_valuation_pair(&mut self, _ctx: &Valuation_pairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#valuation_pair}.
 * @param ctx the parse tree
 */
fn exit_valuation_pair(&mut self, _ctx: &Valuation_pairContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#t_valuation_pair}.
 * @param ctx the parse tree
 */
fn enter_t_valuation_pair(&mut self, _ctx: &T_valuation_pairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#t_valuation_pair}.
 * @param ctx the parse tree
 */
fn exit_t_valuation_pair(&mut self, _ctx: &T_valuation_pairContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#check_sat_response}.
 * @param ctx the parse tree
 */
fn enter_check_sat_response(&mut self, _ctx: &Check_sat_responseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#check_sat_response}.
 * @param ctx the parse tree
 */
fn exit_check_sat_response(&mut self, _ctx: &Check_sat_responseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#echo_response}.
 * @param ctx the parse tree
 */
fn enter_echo_response(&mut self, _ctx: &Echo_responseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#echo_response}.
 * @param ctx the parse tree
 */
fn exit_echo_response(&mut self, _ctx: &Echo_responseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#get_assertions_response}.
 * @param ctx the parse tree
 */
fn enter_get_assertions_response(&mut self, _ctx: &Get_assertions_responseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#get_assertions_response}.
 * @param ctx the parse tree
 */
fn exit_get_assertions_response(&mut self, _ctx: &Get_assertions_responseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#get_assignment_response}.
 * @param ctx the parse tree
 */
fn enter_get_assignment_response(&mut self, _ctx: &Get_assignment_responseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#get_assignment_response}.
 * @param ctx the parse tree
 */
fn exit_get_assignment_response(&mut self, _ctx: &Get_assignment_responseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#get_info_response}.
 * @param ctx the parse tree
 */
fn enter_get_info_response(&mut self, _ctx: &Get_info_responseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#get_info_response}.
 * @param ctx the parse tree
 */
fn exit_get_info_response(&mut self, _ctx: &Get_info_responseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#get_model_response}.
 * @param ctx the parse tree
 */
fn enter_get_model_response(&mut self, _ctx: &Get_model_responseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#get_model_response}.
 * @param ctx the parse tree
 */
fn exit_get_model_response(&mut self, _ctx: &Get_model_responseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#get_option_response}.
 * @param ctx the parse tree
 */
fn enter_get_option_response(&mut self, _ctx: &Get_option_responseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#get_option_response}.
 * @param ctx the parse tree
 */
fn exit_get_option_response(&mut self, _ctx: &Get_option_responseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#get_proof_response}.
 * @param ctx the parse tree
 */
fn enter_get_proof_response(&mut self, _ctx: &Get_proof_responseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#get_proof_response}.
 * @param ctx the parse tree
 */
fn exit_get_proof_response(&mut self, _ctx: &Get_proof_responseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#get_unsat_assump_response}.
 * @param ctx the parse tree
 */
fn enter_get_unsat_assump_response(&mut self, _ctx: &Get_unsat_assump_responseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#get_unsat_assump_response}.
 * @param ctx the parse tree
 */
fn exit_get_unsat_assump_response(&mut self, _ctx: &Get_unsat_assump_responseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#get_unsat_core_response}.
 * @param ctx the parse tree
 */
fn enter_get_unsat_core_response(&mut self, _ctx: &Get_unsat_core_responseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#get_unsat_core_response}.
 * @param ctx the parse tree
 */
fn exit_get_unsat_core_response(&mut self, _ctx: &Get_unsat_core_responseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#get_value_response}.
 * @param ctx the parse tree
 */
fn enter_get_value_response(&mut self, _ctx: &Get_value_responseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#get_value_response}.
 * @param ctx the parse tree
 */
fn exit_get_value_response(&mut self, _ctx: &Get_value_responseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#specific_success_response}.
 * @param ctx the parse tree
 */
fn enter_specific_success_response(&mut self, _ctx: &Specific_success_responseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#specific_success_response}.
 * @param ctx the parse tree
 */
fn exit_specific_success_response(&mut self, _ctx: &Specific_success_responseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SMTLIBv2Parser#general_response}.
 * @param ctx the parse tree
 */
fn enter_general_response(&mut self, _ctx: &General_responseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SMTLIBv2Parser#general_response}.
 * @param ctx the parse tree
 */
fn exit_general_response(&mut self, _ctx: &General_responseContext<'input>) { }

}
