use core::fmt::Debug;
use std::vec;

use crate::{
    lexer::NixTokenType,
    parser::{
        BindType, BUILTIN_IF, BUILTIN_PATH_CONCATENATE, BUILTIN_STRING_CONCATENATE,
        BUILTIN_UNARY_MINUS, BUILTIN_UNARY_NOT,
    },
    visitor::{ASTVisitor, WithOrLet},
};

#[derive(PartialEq, Debug)]
pub struct Identifier<'a>(pub &'a str);

#[derive(PartialEq, Debug)]
pub struct NixFunctionParameter<'a> {
    pub name: Identifier<'a>,
    pub default: Option<AST<'a>>,
}

#[derive(PartialEq, Debug)]
pub struct Bind<'a> {
    pub path: Vec<AST<'a>>, 
    pub value: Box<AST<'a>>
}

#[derive(PartialEq, Debug)]
pub struct Attrset<'a>(pub Vec<Bind<'a>>); // list of bound values

#[derive(PartialEq, Debug)]
pub struct Formals<'a> {
    pub parameters: Vec<NixFunctionParameter<'a>>,
    pub at_identifier: Option<Identifier<'a>>,
    pub ellipsis: bool,
}

#[derive(PartialEq, Debug)]
pub enum AST<'a> {
    Identifier(Identifier<'a>),
    String(&'a str),
    PathSegment(&'a str),
    Integer(i64),
    Float(f64),
    Attrset(Attrset<'a>),
    Array(Vec<AST<'a>>),
    Inherit(Vec<AST<'a>>), // TODO do we need this
    Call(Box<AST<'a>>, Box<AST<'a>>),
    Function(Formals<'a>, Box<AST<'a>>),
    WithOrLet(WithOrLet, Box<AST<'a>>, Box<AST<'a>>),
    Select(Box<AST<'a>>, Vec<AST<'a>>, Option<Box<AST<'a>>>,),
    Builtins
}

/*
Ich fang erstmal ein stückchen weiter vorne an, beim AST erstellen:
```nix
{ a = { b = 1; }; a.c = 1; }
```

Wenn ich https://github.com/NixOS/nix/blob/8ad485ea893862029e02cb560a15fd276753b04f/src/libexpr/parser.y#L132 richtig verstehe, bauen die sich ein a mit dem b = 1 drin und ein anderes a mit c drin (der syntax sollte ja äquivalent sein). dann merken die, dass das gemerged werden muss.

Problem 1: ${"trolled"} you know. (Ich hab das gefühl ich muss da wohl wirklich const evaluation machen).

Problem 2: Entweder ich mach das mergen beim parsing, dann hab ich ja vmtl etliche scoping probleme, weil ich da dann ja tracken muss aus welchem rec / inherit was kam.

Oder ich mach das mergen zur Laufzeit, wobei ich dann ja expressions und die vorher erstellen attrsets unterscheiden müsste, weil ich random expressions ja nicht mergen sollte.

You see its kacke

*/
/*
let a = "Hi"; in { ${"hi"+"jo"+a} = 1; }

let a = "Hi"; in { ${"hi"+"jo"+a} = 1; hijoHi = 1; }

let a = "Hi"; in { ${"hi"+"jo"+a} = {}; hijoHi.test = 1; }
*/

// https://github.com/NixOS/nix/blob/8ad485ea893862029e02cb560a15fd276753b04f/src/libexpr/parser.y#L534

// https://github.com/nix-community/rnix-parser

// hard think to decide how to implement for now are nested attrsets and how { a = { b = 1; }; a.c = 1; } is implemented

// https://github.com/NixOS/nix/blob/8ad485ea893862029e02cb560a15fd276753b04f/src/libexpr/parser.y#L132 I suspect they go the merge way (the question is how rec works then)

// convert all attrsets into nested attrpath syntax (because there can be dynamic attributes everywhere this is the only sane way)

// then merge them by applying one after another (like I already did I think)

// the problem is that we need to differentiate between these and expressions (which we shouldn't merge) (we could also ignore that for now as that would be wrong use)

// I think that would be the path to go for now

// leaves the scoping issues. (well actually I realize I think we could also keep the current way (so not flattening) and then we should have less scoping issues. Then just merge recursively attrsets which would break if you override existing ones but I don't care)

/*
let a = { hi = 1; }; in { inherit a; a.jo = 2; } 
let b = { hi = 1; }; in { a = b; a.jo = 2; }
just allow that for now and care about the scoping which is way more important first.

{ a = { hi = 1; }; a.jo = 2; }
*/

pub struct ASTBuilder;

impl<'a> ASTVisitor<'a, AST<'a>, Formals<'a>, Bind<'a>> for ASTBuilder {
    fn visit_identifier(&mut self, id: &'a [u8]) -> AST<'a> {
        AST::Identifier(Identifier(std::str::from_utf8(id).unwrap()))
    }

    fn visit_integer(&mut self, integer: i64) -> AST<'a> {
        AST::Integer(integer)
    }

    fn visit_float(&mut self, float: f64) -> AST<'a> {
        AST::Float(float)
    }

    fn visit_todo(&mut self) -> AST<'a> {
        todo!()
    }

    fn visit_select(
        &mut self,
        expr: AST<'a>,
        attrpath: Vec<AST<'a>>,
        default: Option<AST<'a>>,
    ) -> AST<'a> {
        AST::Select(Box::new(expr), attrpath, default.map(Box::new))
    }

    fn visit_infix_lhs(&mut self, _operator: NixTokenType<'a>, _left: &AST<'a>) {}

    fn visit_infix_operation(
        &mut self,
        left: AST<'a>,
        right: AST<'a>,
        operator: NixTokenType<'a>,
    ) -> AST<'a> {
        AST::Call(
            Box::new(AST::Call(
                Box::new(AST::Identifier(Identifier(match operator {
                    NixTokenType::If => "if",
                    NixTokenType::Then => "then",
                    NixTokenType::Else => "else",
                    NixTokenType::Assert => "assert",
                    NixTokenType::With => "with",
                    NixTokenType::Let => "let",
                    NixTokenType::In => "in",
                    NixTokenType::Rec => "rec",
                    NixTokenType::Inherit => "inherit",
                    NixTokenType::Or => "or",
                    NixTokenType::Ellipsis => "ellipsis",
                    NixTokenType::Equals => "equals",
                    NixTokenType::NotEquals => "notequals",
                    NixTokenType::LessThanOrEqual => "lessthanorequal",
                    NixTokenType::GreaterThanOrEqual => "greaterthanorequal",
                    NixTokenType::LessThan => "lessthan",
                    NixTokenType::GreaterThan => "greaterthan",
                    NixTokenType::And => "and",
                    NixTokenType::Implies => "implies",
                    NixTokenType::Update => "update",
                    NixTokenType::Concatenate => "concatenate",
                    NixTokenType::Assign => "assign",
                    NixTokenType::Semicolon => "semicolon",
                    NixTokenType::Colon => "colon",
                    NixTokenType::Select => "select",
                    NixTokenType::Comma => "comman",
                    NixTokenType::AtSign => "atsign",
                    NixTokenType::QuestionMark => "questionmark",
                    NixTokenType::ExclamationMark => "exclamationmark",
                    NixTokenType::Addition => "addition",
                    NixTokenType::Subtraction => "subtractoin",
                    NixTokenType::Multiplication => "multiplication",
                    NixTokenType::Division => "division",
                    _ => todo!(),
                }))),
                Box::new(left),
            )),
            Box::new(right),
        )
    }

    fn visit_prefix_operation(&mut self, operator: NixTokenType<'a>, expr: AST<'a>) -> AST<'a> {
        AST::Call(
            Box::new(AST::Identifier(Identifier(match operator {
                NixTokenType::Subtraction => BUILTIN_UNARY_MINUS,
                NixTokenType::ExclamationMark => BUILTIN_UNARY_NOT,
                _ => todo!(),
            }))),
            Box::new(expr),
        )
    }

    fn visit_if(&mut self, condition: AST<'a>, true_case: AST<'a>, false_case: AST<'a>) -> AST<'a> {
        AST::Call(
            Box::new(AST::Call(
                Box::new(AST::Call(
                    Box::new(AST::Identifier(Identifier(BUILTIN_IF))),
                    Box::new(condition),
                )),
                Box::new(true_case),
            )),
            Box::new(false_case),
        )
    }

    fn visit_path_concatenate(&mut self, begin: AST<'a>, last: AST<'a>) -> AST<'a> {
        AST::Call(
            Box::new(AST::Call(
                Box::new(AST::Identifier(Identifier(BUILTIN_PATH_CONCATENATE))),
                Box::new(begin),
            )),
            Box::new(last),
        )
    }

    fn visit_path_segment(&mut self, segment: &'a [u8]) -> AST<'a> {
        AST::PathSegment(std::str::from_utf8(segment).unwrap())
    }

    fn visit_string(&mut self, string: &'a [u8]) -> AST<'a> {
        AST::String(std::str::from_utf8(string).unwrap())
    }

    fn visit_string_concatenate(&mut self, begin: Option<AST<'a>>, last: AST<'a>) -> AST<'a> {
        match begin {
            Some(begin) => AST::Call(
                Box::new(AST::Call(
                    Box::new(AST::Identifier(Identifier(BUILTIN_STRING_CONCATENATE))),
                    Box::new(begin),
                )),
                Box::new(last),
            ),
            None => last,
        }
    }

    fn visit_array_end(&mut self, array: Vec<AST<'a>>) -> AST<'a> {
        //AST::Call(Box::new(array), Box::new(AST::Identifier(b"nil")))
        AST::Array(array)
    }

    fn visit_call(&mut self, function: AST<'a>, parameter: AST<'a>) -> AST<'a> {
        AST::Call(Box::new(function), Box::new(parameter))
    }

    fn visit_function_exit(&mut self, arg: &'a [u8], body: AST<'a>) -> AST<'a> {
        AST::Function(Formals {
            at_identifier: Some(Identifier(std::str::from_utf8(arg).unwrap())),
            parameters: vec![],
            ellipsis: true
        }, Box::new(body))
    }

    fn visit_bind_after(
        &mut self,
        _bind_type: BindType,
        attrpath: Vec<AST<'a>>,
        expr: AST<'a>,
    ) -> Bind<'a> {
        Bind { path: attrpath, value: Box::new(expr)}
    }
    fn visit_string_concatenate_end(&mut self, result: Option<AST<'a>>) -> AST<'a> {
        match result {
            Some(result) => result,
            None => AST::String(""),
        }
    }

    fn visit_formals(
        &mut self,
        parameters: Vec<(&'a [u8], Option<AST<'a>>)>,
        at_identifier: Option<&'a [u8]>,
        ellipsis: bool,
    ) -> Formals<'a> {
        Formals {
            parameters: parameters.into_iter().map(|(a,b)| NixFunctionParameter { name: Identifier(std::str::from_utf8(a).unwrap()), default: b }).collect(),
            at_identifier: at_identifier.map(|s| Identifier(std::str::from_utf8(s).unwrap())),
            ellipsis,
        }
    }

    fn visit_inherit(&mut self, attrs: Vec<AST<'a>>) -> AST<'a> {
        AST::Inherit(attrs)
    }

    fn visit_with_or_let(&mut self, with_or_let: WithOrLet, with_expr: AST<'a>, expr: AST<'a>) -> AST<'a> {
        AST::WithOrLet(WithOrLet::With, Box::new(with_expr), Box::new(expr))
    }

    fn visit_attrset(&mut self, binds: Vec<Bind<'a>>) -> AST<'a> {
        AST::Attrset(Attrset(binds))
    }
}

// cargo test ast::test_java_transpiler -- --nocapture
#[test]
fn test_ast() {}
