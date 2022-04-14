#![allow(nonstandard_style)]
// Generated from src/parser/SMTLIBv2.g4 by ANTLR 4.8
use super::smtlibv2parser::*;
use antlr_rust::tree::ParseTreeVisitor;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link SMTLIBv2Parser}.
 */
pub trait SMTLIBv2Visitor<'input>: ParseTreeVisitor<'input, SMTLIBv2ParserContextType> {
    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#start}.
     * @param ctx the parse tree
     */
    fn visit_start(&mut self, ctx: &StartContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#response}.
     * @param ctx the parse tree
     */
    fn visit_response(&mut self, ctx: &ResponseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#generalReservedWord}.
     * @param ctx the parse tree
     */
    fn visit_generalReservedWord(&mut self, ctx: &GeneralReservedWordContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#simpleSymbol}.
     * @param ctx the parse tree
     */
    fn visit_simpleSymbol(&mut self, ctx: &SimpleSymbolContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#quotedSymbol}.
     * @param ctx the parse tree
     */
    fn visit_quotedSymbol(&mut self, ctx: &QuotedSymbolContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#predefSymbol}.
     * @param ctx the parse tree
     */
    fn visit_predefSymbol(&mut self, ctx: &PredefSymbolContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#predefKeyword}.
     * @param ctx the parse tree
     */
    fn visit_predefKeyword(&mut self, ctx: &PredefKeywordContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#symbol}.
     * @param ctx the parse tree
     */
    fn visit_symbol(&mut self, ctx: &SymbolContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#numeral}.
     * @param ctx the parse tree
     */
    fn visit_numeral(&mut self, ctx: &NumeralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#decimal}.
     * @param ctx the parse tree
     */
    fn visit_decimal(&mut self, ctx: &DecimalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#hexadecimal}.
     * @param ctx the parse tree
     */
    fn visit_hexadecimal(&mut self, ctx: &HexadecimalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#binary}.
     * @param ctx the parse tree
     */
    fn visit_binary(&mut self, ctx: &BinaryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#string}.
     * @param ctx the parse tree
     */
    fn visit_string(&mut self, ctx: &StringContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#keyword}.
     * @param ctx the parse tree
     */
    fn visit_keyword(&mut self, ctx: &KeywordContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#spec_constant}.
     * @param ctx the parse tree
     */
    fn visit_spec_constant(&mut self, ctx: &Spec_constantContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#s_expr}.
     * @param ctx the parse tree
     */
    fn visit_s_expr(&mut self, ctx: &S_exprContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#index}.
     * @param ctx the parse tree
     */
    fn visit_index(&mut self, ctx: &IndexContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#identifier}.
     * @param ctx the parse tree
     */
    fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#attribute_value}.
     * @param ctx the parse tree
     */
    fn visit_attribute_value(&mut self, ctx: &Attribute_valueContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#attribute}.
     * @param ctx the parse tree
     */
    fn visit_attribute(&mut self, ctx: &AttributeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#sort}.
     * @param ctx the parse tree
     */
    fn visit_sort(&mut self, ctx: &SortContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#qual_identifier}.
     * @param ctx the parse tree
     */
    fn visit_qual_identifier(&mut self, ctx: &Qual_identifierContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#var_binding}.
     * @param ctx the parse tree
     */
    fn visit_var_binding(&mut self, ctx: &Var_bindingContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#sorted_var}.
     * @param ctx the parse tree
     */
    fn visit_sorted_var(&mut self, ctx: &Sorted_varContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#pattern}.
     * @param ctx the parse tree
     */
    fn visit_pattern(&mut self, ctx: &PatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#match_case}.
     * @param ctx the parse tree
     */
    fn visit_match_case(&mut self, ctx: &Match_caseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#term}.
     * @param ctx the parse tree
     */
    fn visit_term(&mut self, ctx: &TermContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#sort_symbol_decl}.
     * @param ctx the parse tree
     */
    fn visit_sort_symbol_decl(&mut self, ctx: &Sort_symbol_declContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#meta_spec_constant}.
     * @param ctx the parse tree
     */
    fn visit_meta_spec_constant(&mut self, ctx: &Meta_spec_constantContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#fun_symbol_decl}.
     * @param ctx the parse tree
     */
    fn visit_fun_symbol_decl(&mut self, ctx: &Fun_symbol_declContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#par_fun_symbol_decl}.
     * @param ctx the parse tree
     */
    fn visit_par_fun_symbol_decl(&mut self, ctx: &Par_fun_symbol_declContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#theory_attribute}.
     * @param ctx the parse tree
     */
    fn visit_theory_attribute(&mut self, ctx: &Theory_attributeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#theory_decl}.
     * @param ctx the parse tree
     */
    fn visit_theory_decl(&mut self, ctx: &Theory_declContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#logic_attribue}.
     * @param ctx the parse tree
     */
    fn visit_logic_attribue(&mut self, ctx: &Logic_attribueContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#logic}.
     * @param ctx the parse tree
     */
    fn visit_logic(&mut self, ctx: &LogicContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#sort_dec}.
     * @param ctx the parse tree
     */
    fn visit_sort_dec(&mut self, ctx: &Sort_decContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#selector_dec}.
     * @param ctx the parse tree
     */
    fn visit_selector_dec(&mut self, ctx: &Selector_decContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#constructor_dec}.
     * @param ctx the parse tree
     */
    fn visit_constructor_dec(&mut self, ctx: &Constructor_decContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#datatype_dec}.
     * @param ctx the parse tree
     */
    fn visit_datatype_dec(&mut self, ctx: &Datatype_decContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#function_dec}.
     * @param ctx the parse tree
     */
    fn visit_function_dec(&mut self, ctx: &Function_decContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#function_def}.
     * @param ctx the parse tree
     */
    fn visit_function_def(&mut self, ctx: &Function_defContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#prop_literal}.
     * @param ctx the parse tree
     */
    fn visit_prop_literal(&mut self, ctx: &Prop_literalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#script}.
     * @param ctx the parse tree
     */
    fn visit_script(&mut self, ctx: &ScriptContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_assert}.
     * @param ctx the parse tree
     */
    fn visit_cmd_assert(&mut self, ctx: &Cmd_assertContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_checkSat}.
     * @param ctx the parse tree
     */
    fn visit_cmd_checkSat(&mut self, ctx: &Cmd_checkSatContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_checkSatAssuming}.
     * @param ctx the parse tree
     */
    fn visit_cmd_checkSatAssuming(&mut self, ctx: &Cmd_checkSatAssumingContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_declareConst}.
     * @param ctx the parse tree
     */
    fn visit_cmd_declareConst(&mut self, ctx: &Cmd_declareConstContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_declareDatatype}.
     * @param ctx the parse tree
     */
    fn visit_cmd_declareDatatype(&mut self, ctx: &Cmd_declareDatatypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_declareDatatypes}.
     * @param ctx the parse tree
     */
    fn visit_cmd_declareDatatypes(&mut self, ctx: &Cmd_declareDatatypesContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_declareFun}.
     * @param ctx the parse tree
     */
    fn visit_cmd_declareFun(&mut self, ctx: &Cmd_declareFunContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_declareSort}.
     * @param ctx the parse tree
     */
    fn visit_cmd_declareSort(&mut self, ctx: &Cmd_declareSortContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_defineFun}.
     * @param ctx the parse tree
     */
    fn visit_cmd_defineFun(&mut self, ctx: &Cmd_defineFunContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_defineFunRec}.
     * @param ctx the parse tree
     */
    fn visit_cmd_defineFunRec(&mut self, ctx: &Cmd_defineFunRecContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_defineFunsRec}.
     * @param ctx the parse tree
     */
    fn visit_cmd_defineFunsRec(&mut self, ctx: &Cmd_defineFunsRecContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_defineSort}.
     * @param ctx the parse tree
     */
    fn visit_cmd_defineSort(&mut self, ctx: &Cmd_defineSortContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_echo}.
     * @param ctx the parse tree
     */
    fn visit_cmd_echo(&mut self, ctx: &Cmd_echoContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_exit}.
     * @param ctx the parse tree
     */
    fn visit_cmd_exit(&mut self, ctx: &Cmd_exitContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_getAssertions}.
     * @param ctx the parse tree
     */
    fn visit_cmd_getAssertions(&mut self, ctx: &Cmd_getAssertionsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_getAssignment}.
     * @param ctx the parse tree
     */
    fn visit_cmd_getAssignment(&mut self, ctx: &Cmd_getAssignmentContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_getInfo}.
     * @param ctx the parse tree
     */
    fn visit_cmd_getInfo(&mut self, ctx: &Cmd_getInfoContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_getModel}.
     * @param ctx the parse tree
     */
    fn visit_cmd_getModel(&mut self, ctx: &Cmd_getModelContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_getOption}.
     * @param ctx the parse tree
     */
    fn visit_cmd_getOption(&mut self, ctx: &Cmd_getOptionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_getProof}.
     * @param ctx the parse tree
     */
    fn visit_cmd_getProof(&mut self, ctx: &Cmd_getProofContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_getUnsatAssumptions}.
     * @param ctx the parse tree
     */
    fn visit_cmd_getUnsatAssumptions(&mut self, ctx: &Cmd_getUnsatAssumptionsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_getUnsatCore}.
     * @param ctx the parse tree
     */
    fn visit_cmd_getUnsatCore(&mut self, ctx: &Cmd_getUnsatCoreContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_getValue}.
     * @param ctx the parse tree
     */
    fn visit_cmd_getValue(&mut self, ctx: &Cmd_getValueContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_pop}.
     * @param ctx the parse tree
     */
    fn visit_cmd_pop(&mut self, ctx: &Cmd_popContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_push}.
     * @param ctx the parse tree
     */
    fn visit_cmd_push(&mut self, ctx: &Cmd_pushContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_reset}.
     * @param ctx the parse tree
     */
    fn visit_cmd_reset(&mut self, ctx: &Cmd_resetContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_resetAssertions}.
     * @param ctx the parse tree
     */
    fn visit_cmd_resetAssertions(&mut self, ctx: &Cmd_resetAssertionsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_setInfo}.
     * @param ctx the parse tree
     */
    fn visit_cmd_setInfo(&mut self, ctx: &Cmd_setInfoContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_setLogic}.
     * @param ctx the parse tree
     */
    fn visit_cmd_setLogic(&mut self, ctx: &Cmd_setLogicContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#cmd_setOption}.
     * @param ctx the parse tree
     */
    fn visit_cmd_setOption(&mut self, ctx: &Cmd_setOptionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#command}.
     * @param ctx the parse tree
     */
    fn visit_command(&mut self, ctx: &CommandContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#b_value}.
     * @param ctx the parse tree
     */
    fn visit_b_value(&mut self, ctx: &B_valueContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#option}.
     * @param ctx the parse tree
     */
    fn visit_option(&mut self, ctx: &OptionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#info_flag}.
     * @param ctx the parse tree
     */
    fn visit_info_flag(&mut self, ctx: &Info_flagContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#error_behaviour}.
     * @param ctx the parse tree
     */
    fn visit_error_behaviour(&mut self, ctx: &Error_behaviourContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#reason_unknown}.
     * @param ctx the parse tree
     */
    fn visit_reason_unknown(&mut self, ctx: &Reason_unknownContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#model_response}.
     * @param ctx the parse tree
     */
    fn visit_model_response(&mut self, ctx: &Model_responseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#info_response}.
     * @param ctx the parse tree
     */
    fn visit_info_response(&mut self, ctx: &Info_responseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#valuation_pair}.
     * @param ctx the parse tree
     */
    fn visit_valuation_pair(&mut self, ctx: &Valuation_pairContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#t_valuation_pair}.
     * @param ctx the parse tree
     */
    fn visit_t_valuation_pair(&mut self, ctx: &T_valuation_pairContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#check_sat_response}.
     * @param ctx the parse tree
     */
    fn visit_check_sat_response(&mut self, ctx: &Check_sat_responseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#echo_response}.
     * @param ctx the parse tree
     */
    fn visit_echo_response(&mut self, ctx: &Echo_responseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#get_assertions_response}.
     * @param ctx the parse tree
     */
    fn visit_get_assertions_response(&mut self, ctx: &Get_assertions_responseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#get_assignment_response}.
     * @param ctx the parse tree
     */
    fn visit_get_assignment_response(&mut self, ctx: &Get_assignment_responseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#get_info_response}.
     * @param ctx the parse tree
     */
    fn visit_get_info_response(&mut self, ctx: &Get_info_responseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#get_model_response}.
     * @param ctx the parse tree
     */
    fn visit_get_model_response(&mut self, ctx: &Get_model_responseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#get_option_response}.
     * @param ctx the parse tree
     */
    fn visit_get_option_response(&mut self, ctx: &Get_option_responseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#get_proof_response}.
     * @param ctx the parse tree
     */
    fn visit_get_proof_response(&mut self, ctx: &Get_proof_responseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#get_unsat_assump_response}.
     * @param ctx the parse tree
     */
    fn visit_get_unsat_assump_response(&mut self, ctx: &Get_unsat_assump_responseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#get_unsat_core_response}.
     * @param ctx the parse tree
     */
    fn visit_get_unsat_core_response(&mut self, ctx: &Get_unsat_core_responseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#get_value_response}.
     * @param ctx the parse tree
     */
    fn visit_get_value_response(&mut self, ctx: &Get_value_responseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#specific_success_response}.
     * @param ctx the parse tree
     */
    fn visit_specific_success_response(&mut self, ctx: &Specific_success_responseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SMTLIBv2Parser#general_response}.
     * @param ctx the parse tree
     */
    fn visit_general_response(&mut self, ctx: &General_responseContext<'input>) {
        self.visit_children(ctx)
    }
}
