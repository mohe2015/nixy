// antlr4 -Dlanguage=C++ 

lexer grammar Nix;
// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l

/*
expr : expr '^'<assoc=right> expr
 	| expr '*' expr // match subexpressions joined with '*' operator
 	| expr '+' expr // match subexpressions joined with '+' operator
 	| INT // matches simple integer atom
 	;
*/

channels {
  WHITESPACE_CHANNEL,
  COMMENTS_CHANNEL
}

// TODO FIXME lexer context
// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L107
ANY:         .|'\n';
ID:          [a-zA-Z\_][a-zA-Z0-9\_\'\-]*;
INT:         [0-9]+;
FLOAT:       (([1-9][0-9]*'.'[0-9]*) | ('0'?'.'[0-9]+)) ([Ee][+-]?[0-9]+)?;
PATH_CHAR:   [a-zA-Z0-9\.\_\-\+];
PATH:        PATH_CHAR*('/'PATH_CHAR+)+'/'?;
PATH_SEG:    PATH_CHAR*'/';
HPATH:       '~'('/' PATH_CHAR+)+'/'?;
HPATH_START: '~' '/';
SPATH:       '<'PATH_CHAR+('/'PATH_CHAR+)*'>';
URI:         [a-zA-Z][a-zA-Z0-9\+\-\.]*':'[a-zA-Z0-9\%\/\?\:\@\&\=\+\$\,\-\_\.\!\~\*\']+;

// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L123
IF: 'if';          
THEN: 'then';
ELSE: 'else';
ASSERT: 'assert';
WITH: 'with';
LET: 'let';
IN: 'in';
REC: 'rec';
INHERIT: 'inherit';
OR_KW: 'or';
ELLIPSIS: '...';
EQ: '==';
NEQ: '!=';
LEQ: '<=';
GEQ: '>=';
AND: '&&';
OR: '||';
IMPL: '->';
UPDATE: '//';
CONCAT: '++';

// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L145

DOLLAR_CURLY: '${' -> pushMode(DEFAULT);
CURLY_OPEN: '{' -> pushMode(DEFAULT);
CURLY_CLOSE: '}' -> popMode;
STRING_OPEN: '"' -> pushMode(STRING);

WS: [ \t\r\n]+ -> skip;

// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L192
IND_STRING_OPEN: '\'\'' (' '*'\n')? -> pushMode(IND_STRING);




// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L217



// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L174
mode STRING;

STRING1: ([^$"\\]|'$'[^{"\\]| '\\' {ANY}|'$\\'{ANY})*'$\\"' -> type(STR);
STRING2: ([^$"\\]|'$'[^{"\\]| '\\' {ANY}|'$\\'{ANY})+ -> type(STR);
STRING3: '${' -> pushMode(DEFAULT), type(DOLLAR_CURLY);
STRING4: '"' -> popMode;



// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L193
mode IND_STRING;
IND_STRING1: ([^$']| '$' [^{']| '\'' [^'$])+;
IND_STRING2: '\'\'$' -> type(IND_STR);
IND_STRING3: '$' -> type(IND_STR);
IND_STRING4: '\'\'\'' -> type(IND_STR);
IND_STRING5: '\'\'\\' ANY -> type(IND_STR);
IND_STRING6: '${' -> pushMode(DEFAULT), type(DOLLAR_CURLY);
IND_STRING7: '\'\'' -> popMode, type(IND_STRING_CLOSE);
IND_STRING: '\'' -> type(IND_STR);
