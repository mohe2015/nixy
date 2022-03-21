use core::fmt::Debug;
use std::vec;

use crate::{
    lexer::NixTokenType,
    parser::{
        BindType, BUILTIN_IF, BUILTIN_PATH_CONCATENATE, BUILTIN_SELECT, BUILTIN_STRING_CONCATENATE,
        BUILTIN_UNARY_MINUS, BUILTIN_UNARY_NOT,
    },
    visitor::ASTVisitor,
};

#[derive(PartialEq, Debug)]
pub struct NixFunctionParameter<'a> {
    name: &'a [u8],
    default: Option<AST<'a>>,
}

#[derive(PartialEq, Debug)]
enum WithOrLet {
    With, Let
}

#[derive(PartialEq, Debug)]
pub struct Bind<'a> {
    path: Vec<AST<'a>>, 
    value: Box<AST<'a>>
}

#[derive(PartialEq, Debug)]
pub struct Attrset<'a>(Vec<Bind<'a>>); // list of bound values

#[derive(PartialEq, Debug)]
pub struct Formals<'a> {
    parameters: Vec<NixFunctionParameter<'a>>,
    at_identifier: Option<&'a str>,
    ellipsis: bool,
}

#[derive(PartialEq, Debug)]
pub enum AST<'a> {
    Identifier(&'a str),
    String(&'a str),
    PathSegment(&'a str),
    Integer(i64),
    Float(f64),
    Attrset(Attrset<'a>),
    Array(Vec<AST<'a>>),
    Inherit(Vec<AST<'a>>), // TODO do we need this
    Call(Box<AST<'a>>, Box<AST<'a>>),
    Function(Formals<'a>, Box<AST<'a>>),
    WithOrLet(WithOrLet, Box<Attrset<'a>>, Box<AST<'a>>),
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

impl<'a> ASTVisitor<'a, AST<'a>, Formals<'a>, Bind<'a>, &'a [u8]> for ASTBuilder {
    fn visit_identifier(&mut self, id: &'a [u8]) -> AST<'a> {
        AST::Identifier(std::str::from_utf8(id).unwrap())
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
        attrpath: AST<'a>,
        default: Option<AST<'a>>,
    ) -> AST<'a> {
        let value = AST::Call(
            Box::new(AST::Call(
                Box::new(AST::Identifier(BUILTIN_SELECT)),
                Box::new(expr),
            )),
            Box::new(attrpath),
        );
        match default {
            Some(default) => AST::Call(
                Box::new(AST::Call(
                    Box::new(AST::Identifier("__value_or_default")),
                    Box::new(value),
                )),
                Box::new(default),
            ),
            None => AST::Call(
                Box::new(AST::Identifier("__abort_invalid_attrpath")),
                Box::new(value),
            ),
        }
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
                Box::new(AST::Identifier(match operator {
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
                })),
                Box::new(left),
            )),
            Box::new(right),
        )
    }

    fn visit_prefix_operation(&mut self, operator: NixTokenType<'a>, expr: AST<'a>) -> AST<'a> {
        AST::Call(
            Box::new(AST::Identifier(match operator {
                NixTokenType::Subtraction => BUILTIN_UNARY_MINUS,
                NixTokenType::ExclamationMark => BUILTIN_UNARY_NOT,
                _ => todo!(),
            })),
            Box::new(expr),
        )
    }

    fn visit_if(&mut self, condition: AST<'a>, true_case: AST<'a>, false_case: AST<'a>) -> AST<'a> {
        AST::Call(
            Box::new(AST::Call(
                Box::new(AST::Call(
                    Box::new(AST::Identifier(BUILTIN_IF)),
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
                Box::new(AST::Identifier(BUILTIN_PATH_CONCATENATE)),
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
                    Box::new(AST::Identifier(BUILTIN_STRING_CONCATENATE)),
                    Box::new(begin),
                )),
                Box::new(last),
            ),
            None => last,
        }
    }
    fn visit_array_start(&mut self) {}

    fn visit_array_push_before(&mut self, _begin: &[AST<'a>]) {}

    fn visit_array_push(&mut self, _begin: &[AST<'a>], last: AST<'a>) -> AST<'a> {
        /*match begin {
            Some(begin) => AST::Call(
                Box::new(AST::Identifier(b"cons")),
                Box::new(AST::Call(Box::new(begin), Box::new(last))),
            ),
            None => AST::Call(Box::new(AST::Identifier(b"cons")), Box::new(last)),
        }*/
        last
    }

    fn visit_array_end(&mut self, array: Vec<AST<'a>>) -> AST<'a> {
        //AST::Call(Box::new(array), Box::new(AST::Identifier(b"nil")))
        AST::Array(array)
    }

    fn visit_call(&mut self, function: AST<'a>, parameter: AST<'a>) -> AST<'a> {
        AST::Call(Box::new(function), Box::new(parameter))
    }

    fn visit_attrset_bind_push(&mut self, _binds: &[AST<'a>], bind: AST<'a>) -> AST<'a> {
        bind
    }

    fn visit_function_enter(&mut self, _arg: &AST<'a>) {}

    fn visit_function_exit(&mut self, arg: AST<'a>, body: AST<'a>) -> AST<'a> {
        AST::Function(Formals {
            at_identifier: arg,
            parameters: vec![],
            ellipsis: true
        }, Box::new(body))
    }

    fn visit_function_before(&mut self) {}

    fn visit_if_before(&mut self) {}

    fn visit_if_after_condition(&mut self, _condition: &AST<'a>) {}

    fn visit_if_after_true_case(&mut self, _condition: &AST<'a>, _true_case: &AST<'a>) {}

    fn visit_call_maybe(&mut self, _expr: &Option<AST<'a>>) {}

    fn visit_call_maybe_not(&mut self) {}

    fn visit_bind_before(&mut self, _bind_type: BindType) {}

    fn visit_bind_between(&mut self, _bind_type: BindType, _attrpath: &AST<'a>) {}

    fn visit_bind_after(
        &mut self,
        _bind_type: BindType,
        attrpath: AST<'a>,
        expr: AST<'a>,
    ) -> Bind<'a> {
        Bind { path: vec![attrpath], value: Box::new(expr)}
    }

    fn visit_let_bind_push(&mut self, _binds: &[AST<'a>], bind: AST<'a>) -> AST<'a> {
        bind
    }

    fn visit_let_or_attrset(&mut self, binds: Vec<AST<'a>>, body: Option<AST<'a>>) -> AST<'a> {
        match body {
            Some(body) => AST::WithOrLet(WithOrLet::Let, binds, Box::new(body)),
            None => AST::Attrset(binds),
        }
    }

    fn visit_let_before_body(&mut self, _binds: &[AST<'a>]) {}

    fn visit_let_or_attrset_before(&mut self, _binds: &[AST<'a>]) {}

    fn visit_string_concatenate_end(&mut self, result: Option<AST<'a>>) -> AST<'a> {
        match result {
            Some(result) => result,
            None => AST::String(""),
        }
    }

    fn visit_formal(
        &mut self,
        formals: Option<AST<'a>>,
        identifier: &'a [u8],
        default: Option<AST<'a>>,
    ) -> AST<'a> {
        let formal = NixFunctionParameter {
            name: identifier,
            default,
        };
        match formals {
            Some(AST::Formals {
                mut parameters,
                at_identifier,
                ellipsis,
            }) => {
                parameters.push(formal);
                AST::Formals {
                    parameters,
                    at_identifier,
                    ellipsis,
                }
            }
            None => AST::Formals {
                parameters: vec![formal],
                at_identifier: None,
                ellipsis: false,
            },
            _ => panic!(),
        }
    }

    fn visit_formals(
        &mut self,
        formals: Option<AST<'a>>,
        at_identifier: Option<&'a [u8]>,
        ellipsis: bool,
    ) -> AST<'a> {
        match formals {
            Some(AST::Formals { parameters, .. }) => AST::Formals {
                parameters,
                at_identifier: at_identifier.map(|s| std::str::from_utf8(s).unwrap()),
                ellipsis,
            },
            None => AST::Formals {
                parameters: vec![],
                at_identifier: at_identifier.map(|s| std::str::from_utf8(s).unwrap()),
                ellipsis,
            },
            _ => panic!(),
        }
    }

    fn visit_inherit(&mut self, attrs: Vec<AST<'a>>) -> AST<'a> {
        AST::Inherit(attrs)
    }

    fn visit_with(&mut self, with_expr: AST<'a>, expr: AST<'a>) -> AST<'a> {
        AST::With(Box::new(with_expr), Box::new(expr))
    }

    fn visit_attrpath_between(&mut self) {}

    fn visit_select_before(&mut self) {}
}

// cargo test ast::test_java_transpiler -- --nocapture
#[test]
fn test_ast() {}
