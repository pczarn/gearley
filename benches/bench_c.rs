#![feature(test)]

extern crate test;
extern crate cfg;
extern crate gearley;
extern crate c_lexer_logos;

macro_rules! trace(($($tt:tt)*) => ());

#[path = "../tests/helpers/mod.rs"]
mod helpers;

use cfg::sequence::Separator::Proper;
use cfg::earley::Grammar;
use gearley::forest::{Bocage, NullForest};
use gearley::grammar::InternalGrammar;
use gearley::recognizer::Recognizer;
use gearley::memory_use::MemoryUse;

use helpers::Parse;

const SYM_NAMES: &'static [&'static str] = &[
    "term", "identifier", "signed", "const_", "inline", "auto", "break_", "case", "char_", "continue_", "default",
    "do_", "double", "else_", "enum_", "extern_", "float", "for_", "goto", "if_", "int", "long", "register", "return_",
    "short", "sizeof_", "static_", "struct_", "switch", "typedef", "union", "unsigned", "void", "volatile", "while_",
    "constant", "string_literal", "right_assign", "left_assign", "add_assign", "sub_assign", "mul_assign",
    "div_assign", "mod_assign", "and_assign", "xor_assign", "or_assign", "right_op", "left_op", "inc_op", "dec_op",
    "ptr_op", "and_op", "or_op", "le_op", "ge_op", "eq_op", "ne_op", "elipsis", "restrict", "bool_", "complex", "imaginary",
    "lparen", "rparen", "lbracket", "rbracket", "lbrace", "rbrace", "dot", "colon", "semicolon", "comma", "ampersand",
    "star", "plus", "minus", "tilde", "exclamation", "slash", "percent", "langle", "rangle", "xor", "pipe", "question",
    "equal",
    "start", "primary_expression", "postfix_expression",
    "argument_expression_list_opt", "argument_expression_list", "unary_expression", "unary_operator",
    "cast_expression", "multiplicative_expression", "additive_expression", "shift_expression",
    "relational_expression", "equality_expression", "AND_expression", "exclusive_OR_expression",
    "inclusive_OR_expression", "logical_AND_expression", "logical_OR_expression",
    "conditional_expression", "assignment_expression", "assignment_operator", "expression",
    "constant_expression", "declaration", "init_declarator_list_opt", "declaration_specifiers",
    "declaration_specifiers_opt", "init_declarator_list", "init_declarator", "storage_class_specifier",
    "type_specifier", "struct_or_union_specifier", "identifier_opt", "struct_or_union",
    "struct_declaration_list", "struct_declaration", "specifier_qualifier_list",
    "specifier_qualifier_list_opt", "struct_declarator_list", "struct_declarator", "declarator_opt",
    "enum_specifier", "enumerator_list", "enumerator", "type_qualifier", "function_specifier", "declarator",
    "pointer_opt", "direct_declarator", "type_qualifier_list_opt", "identifier_list_opt", "pointer",
    "type_qualifier_list", "parameter_type_list", "parameter_list", "parameter_declaration",
    "abstract_declarator_opt", "identifier_list", "abstract_declarator", "direct_abstract_declarator",
    "direct_abstract_declarator_opt", "assignment_expression_opt", "parameter_type_list_opt",
    "typedef_name", "initializer", "initializer_list", "designation_opt", "designation", "designator_list",
    "designator", "statement", "labeled_statement", "compound_statement", "block_item_list_opt",
    "block_item_list", "block_item", "expression_statement", "expression_opt", "selection_statement",
    "iteration_statement", "jump_statement", "translation_unit", "external_declaration",
    "function_definition", "declaration_list_opt", "declaration_list", "enumeration_constant",
    "type_name", "error",
];

fn grammar() -> Grammar {
    let mut grammar = Grammar::new();
    let (
        term, identifier, signed, const_, inline, auto, break_, case, char_, continue_, default,
        do_, double, else_, enum_, extern_, float, for_, goto, if_, int, long, register, return_,
        short, sizeof_, static_, struct_, switch, typedef, union, unsigned, void, volatile, while_,
        constant, string_literal, right_assign, left_assign, add_assign, sub_assign, mul_assign,
        div_assign, mod_assign, and_assign, xor_assign, or_assign, right_op, left_op, inc_op, dec_op,
        ptr_op, and_op, or_op, le_op, ge_op, eq_op, ne_op, elipsis, restrict, bool_, complex, imaginary,
        lparen, rparen, lbracket, rbracket, lbrace, rbrace, dot, colon, semicolon, comma, ampersand,
        star, plus, minus, tilde, exclamation, slash, percent, langle, rangle, xor, pipe, question,
        equal
    ) = grammar.sym();

    let (
        start, primary_expression, postfix_expression,
        argument_expression_list_opt, argument_expression_list, unary_expression, unary_operator,
        cast_expression, multiplicative_expression, additive_expression, shift_expression,
        relational_expression, equality_expression, AND_expression, exclusive_OR_expression,
        inclusive_OR_expression, logical_AND_expression, logical_OR_expression,
        conditional_expression, assignment_expression, assignment_operator, expression,
        constant_expression, declaration, init_declarator_list_opt, declaration_specifiers,
        declaration_specifiers_opt, init_declarator_list, init_declarator, storage_class_specifier,
        type_specifier, struct_or_union_specifier, identifier_opt, struct_or_union,
        struct_declaration_list, struct_declaration, specifier_qualifier_list,
        specifier_qualifier_list_opt, struct_declarator_list, struct_declarator, declarator_opt,
        enum_specifier, enumerator_list, enumerator, type_qualifier, function_specifier, declarator,
        pointer_opt, direct_declarator, type_qualifier_list_opt, identifier_list_opt, pointer,
        type_qualifier_list, parameter_type_list, parameter_list, parameter_declaration,
        abstract_declarator_opt, identifier_list, abstract_declarator, direct_abstract_declarator,
        direct_abstract_declarator_opt, assignment_expression_opt, parameter_type_list_opt,
        typedef_name, initializer, initializer_list, designation_opt, designation, designator_list,
        designator, statement, labeled_statement, compound_statement, block_item_list_opt,
        block_item_list, block_item, expression_statement, expression_opt, selection_statement,
        iteration_statement, jump_statement, translation_unit, external_declaration,
        function_definition, declaration_list_opt, declaration_list, enumeration_constant,
        type_name, error,
    ) = grammar.sym();

    grammar.rule(start).rhs([translation_unit]);
    grammar.rule(primary_expression).rhs([identifier])
                    .rhs([constant])
                    .rhs([string_literal])
                    .rhs([lparen, expression, rparen]);
    grammar.rule(postfix_expression).rhs([primary_expression])
                    .rhs([postfix_expression, lbracket, expression, rbracket])
                    .rhs([postfix_expression, lparen, argument_expression_list_opt, rparen])
                    .rhs([postfix_expression, dot, identifier])
                    .rhs([postfix_expression, ptr_op, identifier])
                    .rhs([postfix_expression, inc_op])
                    .rhs([postfix_expression, dec_op])
                    .rhs([lparen, type_name, rparen, lbrace, initializer_list, rbrace])
                    .rhs([lparen, type_name, rparen, lbrace, initializer_list, comma, rbrace]);
    grammar.rule(argument_expression_list_opt).rhs([])
                                .rhs([argument_expression_list]);
    grammar.rule(argument_expression_list).rhs([assignment_expression])
                            .rhs([argument_expression_list, comma, assignment_expression]);
    grammar.rule(unary_expression).rhs([postfix_expression])
                    .rhs([inc_op, unary_expression])
                    .rhs([dec_op, unary_expression])
                    .rhs([unary_operator, cast_expression])
                    .rhs([sizeof_, unary_expression])
                    .rhs([sizeof_, lparen, type_name, rparen]);
    grammar.rule(unary_operator).rhs([ampersand])
                .rhs([star])
                .rhs([plus])
                .rhs([minus])
                .rhs([tilde])
                .rhs([exclamation]);
    grammar.rule(cast_expression).rhs([unary_expression])
                    .rhs([lparen, type_name, rparen, cast_expression]);
    grammar.rule(multiplicative_expression).rhs([cast_expression])
                            .rhs([multiplicative_expression, star, cast_expression])
                            .rhs([multiplicative_expression, slash, cast_expression])
                            .rhs([multiplicative_expression, percent, cast_expression]);
    grammar.rule(additive_expression).rhs([multiplicative_expression])
                        .rhs([additive_expression, plus, multiplicative_expression])
                        .rhs([additive_expression, minus, multiplicative_expression]);
    grammar.rule(shift_expression).rhs([additive_expression])
                    .rhs([shift_expression, left_op, additive_expression])
                    .rhs([shift_expression, right_op, additive_expression]);
    grammar.rule(relational_expression).rhs([shift_expression])
                        .rhs([relational_expression, langle, shift_expression])
                        .rhs([relational_expression, rangle, shift_expression])
                        .rhs([relational_expression, le_op, shift_expression])
                        .rhs([relational_expression, ge_op, shift_expression]);
    grammar.rule(equality_expression).rhs([relational_expression])
                        .rhs([equality_expression, eq_op, relational_expression])
                        .rhs([equality_expression, ne_op, relational_expression]);
    grammar.rule(AND_expression).rhs([equality_expression])
                .rhs([AND_expression, ampersand, equality_expression]);
    grammar.rule(exclusive_OR_expression).rhs([AND_expression])
                            .rhs([exclusive_OR_expression, xor, AND_expression]);
    grammar.rule(inclusive_OR_expression).rhs([exclusive_OR_expression])
                            .rhs([inclusive_OR_expression, pipe, exclusive_OR_expression]);
    grammar.rule(logical_AND_expression).rhs([inclusive_OR_expression])
                        .rhs([logical_AND_expression, and_op, inclusive_OR_expression]);
    grammar.rule(logical_OR_expression).rhs([logical_AND_expression])
                        .rhs([logical_OR_expression, or_op, logical_AND_expression]);
    grammar.rule(conditional_expression).rhs([logical_OR_expression])
                        .rhs([logical_OR_expression, question, expression, colon, conditional_expression]);
    grammar.rule(assignment_expression).rhs([conditional_expression])
                        .rhs([unary_expression, assignment_operator, assignment_expression]);
    grammar.rule(assignment_operator).rhs([equal])
                        .rhs([mul_assign])
                        .rhs([div_assign])
                        .rhs([mod_assign])
                        .rhs([add_assign])
                        .rhs([sub_assign])
                        .rhs([left_assign])
                        .rhs([right_assign])
                        .rhs([and_assign])
                        .rhs([xor_assign])
                        .rhs([or_assign]);
    grammar.rule(expression).rhs([assignment_expression])
            .rhs([expression, comma, assignment_expression])
            .rhs([error]);
    grammar.rule(constant_expression).rhs([conditional_expression]);
                
    grammar.rule(declaration).rhs([declaration_specifiers, init_declarator_list_opt, semicolon])
                .rhs([error]);
    grammar.rule(init_declarator_list_opt).rhs([])
                            .rhs([init_declarator_list]);
    grammar.rule(declaration_specifiers).rhs([storage_class_specifier, declaration_specifiers_opt])
                    .rhs([type_specifier, declaration_specifiers_opt])
                        .rhs([type_qualifier, declaration_specifiers_opt])
                        .rhs([function_specifier, declaration_specifiers_opt]);
    grammar.rule(declaration_specifiers_opt).rhs([])
                            .rhs([declaration_specifiers]);
    grammar.rule(init_declarator_list).rhs([init_declarator])
                        .rhs([init_declarator_list, comma, init_declarator]);
    grammar.rule(init_declarator).rhs([declarator])
                    .rhs([declarator, equal, initializer]);
    grammar.rule(storage_class_specifier).rhs([typedef])
                            .rhs([extern_])
                            .rhs([static_])
                            .rhs([auto])
                        .rhs([register]);
    grammar.rule(type_specifier).rhs([void])
                .rhs([char_])
                .rhs([short])
                .rhs([int])
                .rhs([long])
                .rhs([float])
                .rhs([double])
                .rhs([signed])
                .rhs([unsigned])
                .rhs([bool_])
                .rhs([complex])
                .rhs([imaginary])
                .rhs([struct_or_union_specifier])
                .rhs([enum_specifier])
                .rhs([typedef_name]);
    grammar.rule(struct_or_union_specifier).rhs([struct_or_union, identifier_opt, lbrace, struct_declaration_list, rbrace])
                            .rhs([struct_or_union, identifier]);
    grammar.rule(identifier_opt).rhs([])
                .rhs([identifier]);
    grammar.rule(struct_or_union).rhs([struct_])
                    .rhs([union]);
    grammar.rule(struct_declaration_list).rhs([struct_declaration])
                            .rhs([struct_declaration_list, struct_declaration]);
    grammar.rule(struct_declaration).rhs([specifier_qualifier_list, struct_declarator_list, semicolon]);
    grammar.rule(specifier_qualifier_list).rhs([type_specifier, specifier_qualifier_list_opt])
                            .rhs([type_qualifier, specifier_qualifier_list_opt]);
    grammar.rule(specifier_qualifier_list_opt).rhs([])
                                .rhs([specifier_qualifier_list]);
    grammar.rule(struct_declarator_list).rhs([struct_declarator])
                        .rhs([struct_declarator_list, comma, struct_declarator]);
    grammar.rule(struct_declarator).rhs([declarator])
                    .rhs([declarator_opt, colon, constant_expression]);
    grammar.rule(declarator_opt).rhs([])
                .rhs([declarator]);
    grammar.rule(enum_specifier).rhs([enum_, identifier_opt, lbrace, enumerator_list, rbrace])
                .rhs([enum_, identifier_opt, lbrace, enumerator_list, comma, rbrace])
                .rhs([enum_, identifier]);
    grammar.rule(enumerator_list).rhs([enumerator])
                    .rhs([enumerator_list, comma, enumerator]);
    grammar.rule(enumerator).rhs([enumeration_constant])
            .rhs([enumeration_constant, equal, constant_expression]);
    grammar.rule(type_qualifier).rhs([const_])
                .rhs([restrict])
                .rhs([volatile]);
    grammar.rule(function_specifier).rhs([inline]);
    grammar.rule(declarator).rhs([pointer_opt, direct_declarator]);
    grammar.rule(pointer_opt).rhs([])
                .rhs([pointer]);
    grammar.rule(direct_declarator).rhs([identifier])
                    .rhs([lparen, declarator, rparen])
                    .rhs([direct_declarator, lbracket, type_qualifier_list_opt, assignment_expression_opt, rbracket])
                    .rhs([direct_declarator, lbracket, static_, type_qualifier_list_opt, assignment_expression, rbracket])
                    .rhs([direct_declarator, lbracket, type_qualifier_list, static_, assignment_expression, rbracket])
                    .rhs([direct_declarator, lbracket, type_qualifier_list_opt, star, rbracket])
                    .rhs([direct_declarator, lparen, parameter_type_list, rparen])
                    .rhs([direct_declarator, lparen, identifier_list_opt, rparen]);
    grammar.rule(type_qualifier_list_opt).rhs([])
                            .rhs([type_qualifier_list]);
    grammar.rule(identifier_list_opt).rhs([])
                        .rhs([identifier_list]);
    grammar.rule(pointer).rhs([star, type_qualifier_list_opt])
            .rhs([star, type_qualifier_list_opt, pointer]);
    grammar.rule(type_qualifier_list).rhs([type_qualifier])
                        .rhs([type_qualifier_list, type_qualifier]);
    grammar.rule(parameter_type_list).rhs([parameter_list])
                        .rhs([parameter_list, comma, elipsis]);
    grammar.rule(parameter_list).rhs([parameter_declaration])
                .rhs([parameter_list, comma, parameter_declaration]);
    grammar.rule(parameter_declaration).rhs([declaration_specifiers, declarator])
                        .rhs([declaration_specifiers, abstract_declarator_opt]);
    grammar.rule(abstract_declarator_opt).rhs([])
                            .rhs([abstract_declarator]);
    grammar.rule(identifier_list).rhs([identifier])
                    .rhs([identifier_list, comma, identifier]);
    grammar.rule(type_name).rhs([specifier_qualifier_list, abstract_declarator_opt]);
    grammar.rule(abstract_declarator).rhs([pointer])
                        .rhs([pointer_opt, direct_abstract_declarator]);
    grammar.rule(direct_abstract_declarator).rhs([lparen, abstract_declarator, rparen])
                            .rhs([direct_abstract_declarator_opt, lbracket, assignment_expression_opt, rbracket])
                            .rhs([direct_abstract_declarator_opt, lbracket, star, rbracket])
                            .rhs([direct_abstract_declarator_opt, lparen, parameter_type_list_opt, rparen]);
    grammar.rule(direct_abstract_declarator_opt).rhs([])
                                .rhs([direct_abstract_declarator]);
    grammar.rule(assignment_expression_opt).rhs([])
                            .rhs([assignment_expression]);
    grammar.rule(parameter_type_list_opt).rhs([])
                            .rhs([parameter_type_list]);
    grammar.rule(typedef_name).rhs([identifier]);
    grammar.rule(initializer).rhs([assignment_expression])
                .rhs([lbrace, initializer_list, rbrace])
                .rhs([lbrace, initializer_list, comma, rbrace]);
    grammar.rule(initializer_list).rhs([designation_opt, initializer])
                    .rhs([initializer_list, comma, designation_opt, initializer]);
    grammar.rule(designation_opt).rhs([])
                    .rhs([designation]);
    grammar.rule(designation).rhs([designator_list, equal]);
    grammar.rule(designator_list).rhs([designator])
                    .rhs([designator_list, designator]);
    grammar.rule(designator).rhs([rbracket, constant_expression, rbracket])
            .rhs([dot, identifier]);
    grammar.rule(statement).rhs([labeled_statement])
            .rhs([compound_statement])
            .rhs([expression_statement])
            .rhs([selection_statement])
            .rhs([iteration_statement])
            .rhs([jump_statement])
            .rhs([error]);
    grammar.rule(labeled_statement).rhs([identifier, colon, statement])
                    .rhs([case, constant_expression, colon, statement])
                    .rhs([default, colon, statement]);
    grammar.rule(compound_statement).rhs([lbrace, block_item_list_opt, rbrace]);
    grammar.rule(block_item_list_opt).rhs([])
                        .rhs([block_item_list]);
    grammar.rule(block_item_list).rhs([block_item])
                    .rhs([block_item_list, block_item]);
    grammar.rule(block_item).rhs([declaration])
            .rhs([statement]);
    grammar.rule(expression_statement).rhs([expression_opt, semicolon]);
    grammar.rule(expression_opt).rhs([])
                .rhs([expression]);
    grammar.rule(selection_statement).rhs([if_, lparen, expression, rparen, statement])
                        .rhs([if_, lparen, expression, rparen, statement, else_, statement])
                        .rhs([switch, lparen, expression, rparen, statement]);
    grammar.rule(iteration_statement).rhs([while_, lparen, expression, rparen, statement])
                        .rhs([do_, statement, while_, lparen, expression, rparen, semicolon])
                        .rhs([for_, lparen, expression_opt, semicolon, expression_opt, semicolon, expression_opt, rparen, statement])
                        .rhs([for_, lparen, declaration, expression_opt, semicolon, expression_opt, rparen, statement]);
    grammar.rule(jump_statement).rhs([goto, identifier, semicolon])
                .rhs([continue_, semicolon])
                .rhs([break_, semicolon])
                .rhs([return_, expression_opt, semicolon]);
    grammar.rule(translation_unit).rhs([external_declaration])
                    .rhs([translation_unit, external_declaration]);
    grammar.rule(external_declaration).rhs([function_definition])
                        .rhs([declaration]);
    grammar.rule(function_definition).rhs([declaration_specifiers, declarator, declaration_list_opt, compound_statement]);
    grammar.rule(declaration_list_opt).rhs([])
                        .rhs([declaration_list]);
    grammar.rule(declaration_list).rhs([declaration])
                    .rhs([declaration_list, declaration]);
    grammar.rule(enumeration_constant).rhs([identifier]);

    grammar.set_start(start);
    grammar
}

#[bench]
fn bench_parse_c(b: &mut test::Bencher) {
    use c_lexer_logos::Lexer;
    use c_lexer_logos::token::Token::*;
    let external = grammar();
    let mut grammar = Grammar::new();
    let (
        term, identifier, signed, const_, inline, auto, break_, case, char_, continue_, default,
        do_, double, else_, enum_, extern_, float, for_, goto, if_, int, long, register, return_,
        short, sizeof_, static_, struct_, switch, typedef, union, unsigned, void, volatile, while_,
        constant, string_literal, right_assign, left_assign, add_assign, sub_assign, mul_assign,
        div_assign, mod_assign, and_assign, xor_assign, or_assign, right_op, left_op, inc_op, dec_op,
        ptr_op, and_op, or_op, le_op, ge_op, eq_op, ne_op, elipsis, restrict, bool_, complex, imaginary,
        lparen, rparen, lbracket, rbracket, lbrace, rbrace, dot, colon, semicolon, comma, ampersand,
        star, plus, minus, tilde, exclamation, slash, percent, langle, rangle, xor, pipe, question,
        equal
    ) = grammar.sym();

    let contents = include_str!("part_gcc_test.i");
    let tokens: Vec<_> = Lexer::lex(&contents[..]).unwrap().into_iter().filter_map(|token| {
        // println!("{:?}", token);
        let tok = match token {
            LBrace => Some(lbrace),
            RBrace => Some(rbrace),
            LParen => Some(lparen),
            RParen => Some(rparen),
            LBracket => Some(lbracket),
            RBracket => Some(rbracket),
            Semicolon => Some(semicolon),
            Assign => Some(equal),
            Lt => Some(langle),
            Gt => Some(rangle),
            Minus => Some(minus),
            Tilde => Some(tilde),
            Exclamation => Some(exclamation),
            Plus => Some(plus),
            Multi => Some(star),
            Slash => Some(slash),
            Colon => Some(colon),
            QuestionMark => Some(question),
            Comma => Some(comma),
            Dot => Some(dot),
            SingleAnd => Some(ampersand),
            InclusiveOr => Some(pipe),
            ExclusiveOr => Some(xor),
            Mod => Some(percent),
            Identifier(i_str) => Some(identifier),
            NumericLiteral(num) => Some(constant),
            StringLiteral(s) => Some(string_literal),
            FuncName => None,
            SIZEOF => Some(sizeof_),
            PtrOp => Some(ptr_op),
            IncOp => Some(inc_op),
            DecOp => Some(dec_op),
            LeftOp => Some(left_op),
            RightOp => Some(right_op),
            LeOp => Some(le_op),
            GeOp => Some(ge_op),
            EqOp => Some(eq_op),
            NeOp => Some(ne_op),
            AndOp => Some(and_op),
            OrOp => Some(or_op),
            MulAssign => Some(mul_assign),
            DivAssign => Some(div_assign),
            ModAssign => Some(mod_assign),
            AddAssign => Some(add_assign),
            SubAssign => Some(sub_assign),
            LeftAssign => Some(left_assign),
            RightAssign => Some(right_assign),
            AndAssign => Some(and_assign),
            XorAssign => Some(xor_assign),
            OrAssign => Some(or_assign),
            // TODO: this should be done when we found this is a typedef name,
            //       typedef LL int, then LL is typedef_name
            TypedefName => Some(identifier),
            ELLIPSIS => Some(elipsis),                    // ...
            EnumerationConstant(..) => None, // TODO: add check
            LineTerminator => None,
            EOF => None,

            TYPEDEF => Some(typedef),
            EXTERN => Some(extern_),
            STATIC => Some(static_),
            // AUTO => Some(auto_),
            REGISTER => Some(register),
            INLINE => Some(inline),
            CONST => Some(const_),
            RESTRICT => Some(restrict),
            VOLATILE => Some(volatile),
            BOOL => Some(bool_),
            CHAR => Some(char_),
            SHORT => Some(short),
            INT => Some(int),
            LONG => Some(long),
            SIGNED => Some(signed),
            UNSIGNED => Some(unsigned),
            FLOAT => Some(float),
            DOUBLE => Some(double),
            VOID => Some(void),
            COMPLEX => Some(complex),
            IMAGINARY => Some(imaginary),
            STRUCT => Some(struct_),
            UNION => Some(union),
            ENUM => Some(enum_),
            CASE => Some(case),
            DEFAULT => Some(default),
            IF => Some(if_),
            ELSE => Some(else_),
            SWITCH => Some(switch),
            WHILE => Some(while_),
            DO => Some(do_),
            FOR => Some(for_),
            GOTO => Some(goto),
            CONTINUE => Some(continue_),
            BREAK => Some(break_),
            RETURN => Some(return_),
            // ALIGNAS => Some(alignas),
            // ALIGNOF => Some(alignof),
            // ATOMIC => Some(atomic),
            // GENERIC => Some(generic),
            // NORETURN,
            // StaticAssert,
            // ThreadLocal,
            _ => None,
        };
        // tok.map(|t| (t.usize() as u32, start, end))
        tok.map(|t| t.usize() as u32)
    }).collect();
    let mut first = true;
    b.iter(|| {
        let cfg = InternalGrammar::from_grammar(&external);
        let bocage = Bocage::new(&cfg);
        let mut rec: Recognizer<Bocage<&'_ InternalGrammar>> = Recognizer::new_with_limit(&cfg, 2_00_000);
        rec.forest = bocage;
        let finished = rec.parse(&tokens[..]);
        assert!(finished);
        if first {
            println!("memory use: all:{} forest:{}", rec.memory_use(), rec.forest.memory_use());
            first = false;
        }
        test::black_box(&rec.forest);
    });
}
