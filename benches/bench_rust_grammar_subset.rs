#![feature(test)]

extern crate test;
extern crate cfg;
extern crate gearley;

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

macro_rules! rhs_elem {
    (use) => (0);
    (as) => (1);
    (::) => (2);
    (*) => (3);
    (,) => (4);
    (;) => (5);
    ('{') => (6);
    ('}') => (7);
    (pub) => (8);
    ($i:ident) => (9);
}

macro_rules! rhs {
    ($($e:tt)+) => (
        &[$(rhs_elem!($e) + 9,)+]
    )
}

const TOKENS: &'static [u32] = rhs!(
    use gearley::events::'{' PredictionEvents, MedialEvents, CompletionEvents '}';
    use gearley::util::slice_builder::SliceBuilder;
    use gearley::forest::depth_first::'{'
        NullOrder, FastEvaluator, ArrayStore, ClosureActionEvaluator
    '}';
    pub use self::PathParsingMode::*;

    use abi :: '{' self, Abi '}';
    use ast::BareFnTy;
    use ast :: '{' RegionTyParamBound, TraitTyParamBound, TraitBoundModifier '}';
    use ast::Unsafety;
    use ast :: '{' Mod, Arg, Arm, Attribute, BindingMode, TraitItemKind '}';
    use ast::Block;
    use ast :: '{' BlockCheckMode, CaptureBy '}';
    use ast :: '{' Constness, Crate, CrateConfig '}';
    use ast :: '{' Decl, DeclKind '}';
    use ast :: '{' EMPTY_CTXT, EnumDef, ExplicitSelf '}';
    use ast :: '{' Expr, ExprKind '}';
    use ast :: '{' Field, FnDecl '}';
    use ast :: '{' ForeignItem, ForeignItemKind, FunctionRetTy '}';
    use ast :: '{' Ident, ImplItem, Item, ItemKind '}';
    use ast :: '{' Lit, LitKind, UintTy '}';
    use ast::Local;
    use ast::MacStmtStyle;
    use ast::Mac_;
    use ast :: '{' MutTy, Mutability '}';
    use ast::NamedField;
    use ast :: '{' Pat, PatKind '}';
    use ast :: '{' PolyTraitRef, QSelf '}';
    use ast :: '{' Stmt, StmtKind '}';
    use ast :: '{' VariantData, StructField '}';
    use ast::StrStyle;
    use ast::SelfKind;
    use ast :: '{' Delimited, SequenceRepetition, TokenTree, TraitItem, TraitRef '}';
    use ast :: '{' Ty, TyKind, TypeBinding, TyParam, TyParamBounds '}';
    use ast::UnnamedField;
    use ast :: '{' ViewPath, ViewPathGlob, ViewPathList, ViewPathSimple '}';
    use ast :: '{' Visibility, WhereClause '}';
    use attr :: '{' ThinAttributes, ThinAttributesExt, AttributesExt '}';
    use ast :: '{' BinOpKind, UnOp '}';
    use ast;
    use ast_util :: '{' self, ident_to_path '}';
    use codemap :: '{' self, Span, BytePos, Spanned, spanned, mk_sp, CodeMap '}';
    use errors :: '{' self, DiagnosticBuilder '}';
    use ext::tt::macro_parser;
    use parse;
    use parse::classify;
    use parse::common::SeqSep;
    use parse::lexer :: '{' Reader, TokenAndSpan '}';
    use parse::obsolete :: '{' ParserObsoleteMethods, ObsoleteSyntax '}';
    use parse::token :: '{' self, intern, MatchNt, SubstNt, SpecialVarNt, InternedString '}';
    use parse::token :: '{' keywords, special_idents, SpecialMacroVar '}';
    use parse :: '{' new_sub_parser_from_file, ParseSess '}';
    use util::parser :: '{' AssocOp, Fixity '}';
    use print::pprust;
    use ptr::P;
    use parse::PResult;

    use std::collections::HashSet;
    use std::io::prelude::*;
    use std::mem;
    use std::path :: '{' Path, PathBuf '}';
    use std::rc::Rc;
    use std::slice;
);

const _TOKEN_NAMES: &'static [&'static str] = &[
"start", "use_decls", "use_decl", "segments", "segment", "import_mod", "import_seq", "import",
"pub_opt",
"use_tok", "as_tok", "mod_sep", "star", "comma", "semi", "lbrace", "rbrace", "pub_tok", "ident"
];

fn grammar() -> Grammar {
    let mut external = Grammar::new();
    let (start, use_decls, use_decl, segments, segment, import_mod, import_seq, import, pub_opt) = external.sym();
    let (use_tok, as_tok, mod_sep, star, comma, semi, lbrace, rbrace, pub_tok, ident) = external.sym();
    external
            .sequence(segments).inclusive(0, None).rhs(segment)
            .sequence(import_seq).separator(Proper(comma)).inclusive(1, None).rhs(import)
            .sequence(use_decls).inclusive(0, None).rhs(use_decl)
            ;
    external.rule(start).rhs([use_decls])
            .rule(use_decl).rhs([pub_opt, use_tok, segments, import_mod, semi])
            .rule(segment).rhs([ident, mod_sep])
            .rule(import_mod).rhs([lbrace, import_seq, rbrace])
                          .rhs([import])
                          .rhs([star])
            .rule(import).rhs([ident])
                         .rhs([ident, as_tok, ident])
            .rule(pub_opt).rhs([pub_tok])
                          .rhs([])
            ;
    external.set_start(start);
    external
}

#[bench]
fn bench_recognize_decl_use(b: &mut test::Bencher) {
    let external = grammar();
    let cfg = InternalGrammar::from_grammar(&external);

    b.iter(|| {
        let mut rec: Recognizer<NullForest> = Recognizer::new_with_limit(&cfg, 2_000_000);
        rec.parse(TOKENS);
        test::black_box(&rec);
    })
}

#[bench]
fn bench_parse_decl_use(b: &mut test::Bencher) {
    let external = grammar();
    let cfg = InternalGrammar::from_grammar(&external);

    b.iter(|| {
        let mut rec: Recognizer<Bocage<&'_ InternalGrammar>> = Recognizer::new_with_limit(&cfg, 2_000_000);
        let finished = rec.parse(TOKENS);
        assert!(finished);
        test::black_box(&rec.forest);
    })
}
