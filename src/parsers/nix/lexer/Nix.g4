// antlr4 -Dlanguage=C++ 

// TODO FIXME default mode is DEFAULT_MODE

lexer grammar Nix;
// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l

/*
expr : expr '^'<assoc=right> expr
 	| expr '*' expr // match subexpressions joined with '*' operator
 	| expr '+' expr // match subexpressions joined with '+' operator
 	| INT // matches simple integer atom
 	;
*/

tokens { STRING }

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

// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L192
IND_STRING_OPEN: '\'\'' (' '*'\n')? -> pushMode(IND_STRING);




// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L217
PATH1: ({PATH_SEG} '${' | {HPATH_START} '${') -> pushMode(PATH_START);


// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L238
PATH2: PATH -> pushMode(INPATH); // TODO FIXME bruh
PATH3: HPATH -> pushMode(INPATH); // TODO FIXME bruh




// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L289
WS: [ \t\r\n]+ -> skip;
SINGLE_LINE_COMMENT: '#' [^\r\n]* -> channel(COMMENTS_CHANNEL);
MULTILINE_COMMENT: '/*' ([^*]|'*'+[^*/])*'*'+'/' -> channel(COMMENTS_CHANNEL);

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



// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L224
mode PATH_START;
PATH_START1: PATH_SEG -> mode(INPATH_SLASH);
PATH_START2: HPATH_START -> mode(INPATH_SLASH);


// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L255
mode INPATH;
INPATH1: '${' -> mode(INPATH), pushMode(DEFAULT), type(DOLLAR_CURLY);
INPATH2: (PATH | PATH_SEG | PATH_CHAR+) -> mode(INPATH), type(STR); // TODO FIXME
INPATH3: (ANY | EOF) -> popMode, type(PATH_END);

// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L255
mode INPATH_SLASH;
INPATH_SLASH1: '${' -> mode(INPATH), pushMode(DEFAULT), type(DOLLAR_CURLY);
INPATH_SLASH2: (PATH | PATH_SEG | PATH_CHAR+) -> mode(INPATH), type(STR); // TODO FIXME
INPATH_SLASH3: (ANY | EOF); // TODO FIXME error