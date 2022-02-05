// antlr4 -Dlanguage=C++ 
// antlr4 Nix.g4
// whereis antlr
// javac -cp /nix/store/0h2al86yb3gh59h4lckwsprc5vavirmr-antlr-4.8/share/java/antlr-4.8-complete.jar *.java
// grun Nix tokens -tokens
// write code then ctrl+D

// antlr4 NixLexer.g4 && antlr4 NixParser.g4 && javac -cp /nix/store/0h2al86yb3gh59h4lckwsprc5vavirmr-antlr-4.8/share/java/antlr-4.8-complete.jar *.java && grun Nix tokens -tokens
// https://github.com/antlr/antlr4/tree/master/runtime/Cpp/demo

// https://github.com/antlr/antlr4/blob/master/doc/lexer-rules.md

lexer grammar NixLexer;
// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l

/*
expr : expr '^'<assoc=right> expr
 	| expr '*' expr // match subexpressions joined with '*' operator
 	| expr '+' expr // match subexpressions joined with '+' operator
 	| INT // matches simple integer atom
 	;
*/

tokens { IND_STR, STR, IND_STRING_CLOSE, PATH_END }

channels {
  WHITESPACE_CHANNEL,
  COMMENTS_CHANNEL
}

// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L289
WS: [ \t\r\n]+ -> skip;
SINGLE_LINE_COMMENT: '#' ~[\r\n]* -> channel(COMMENTS_CHANNEL);
MULTILINE_COMMENT: '/*' (~[*]|'*'+~[*/])*'*'+'/' -> channel(COMMENTS_CHANNEL);

// TODO FIXME lexer context
// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L107
// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L145
fragment ANY:         .|'\n';
ID:          [a-zA-Z_][a-zA-Z0-9_'\-]*;
INT:         [0-9]+;
FLOAT:       (([1-9][0-9]*'.'[0-9]*) | ('0'?'.'[0-9]+)) ([Ee][+-]?[0-9]+)?;
PATH_CHAR:   [a-zA-Z0-9._\-+];
PATH:        PATH_CHAR*('/'PATH_CHAR+)+'/'?;
PATH_SEG:    PATH_CHAR*'/';
HPATH:       '~'('/' PATH_CHAR+)+'/'?;
HPATH_START: '~' '/';
SPATH:       '<'PATH_CHAR+('/'PATH_CHAR+)*'>';
URI:         [a-zA-Z][a-zA-Z0-9+\-.]*':'[a-zA-Z0-9%/?:@&=+$,\-_.!~*']+;

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

DOLLAR_CURLY: '${' -> pushMode(DEFAULT_MODE);
CURLY_OPEN: '{' -> pushMode(DEFAULT_MODE);
CURLY_CLOSE: '}' -> popMode;
STRING_OPEN: DOUBLE_QUOTES -> pushMode(STRING_MODE);

// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L192
IND_STRING_OPEN: '\'\'' (' '*'\n')? -> pushMode(IND_STRING_MODE);




// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L217
PATH1: (PATH_SEG '${' | HPATH_START '${') -> pushMode(PATH_START_MODE);


// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L238
PATH2: PATH -> pushMode(INPATH_MODE); // TODO FIXME bruh
PATH3: HPATH -> pushMode(INPATH_MODE); // TODO FIXME bruh



// additional tokens
COLON: ':';
AT_SIGN: '@';
SEMICOLON: ';';
EXCLAMATION_MARK: '!';
MINUS: '-';
LESS_THAN: '<';
GREATER_THAN: '>';
PLUS: '+';
STAR: '*';
FORWARD_SLASH: '/';
QUESTION_MARK: '?';
DOT: '.';
DOUBLE_QUOTES: '"';
PAREN_OPEN: '(';
PAREN_CLOSE: ')';
BRACKET_OPEN: '[';
BRACKET_CLOSE: ']';
EQUALS: '=';
COMMA: ',';




// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L174
mode STRING_MODE;

// interpolated variables in string
STRING1: (~[$"\\]|'$'~[{"\\]| '\\' ANY|'$\\'ANY)*'$\\"' -> type(STR);
STRING2: (~[$"\\]|'$'~[{"\\]| '\\' ANY|'$\\'ANY)+ -> type(STR);
STRING3: '${' -> pushMode(DEFAULT_MODE), type(DOLLAR_CURLY);
STRING_CLOSE: DOUBLE_QUOTES -> popMode;



// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L193
mode IND_STRING_MODE;
IND_STRING1: (~[$']| '$' ~[{']| '\'' ~['$])+;
IND_STRING2: '\'\'$' -> type(IND_STR);
IND_STRING3: '$' -> type(IND_STR);
IND_STRING4: '\'\'\'' -> type(IND_STR);
IND_STRING5: '\'\'\\' ANY -> type(IND_STR);
IND_STRING6: '${' -> pushMode(DEFAULT_MODE), type(DOLLAR_CURLY);
IND_STRING7: '\'\'' -> popMode, type(IND_STRING_CLOSE);
IND_STRING: '\'' -> type(IND_STR);



// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L224
mode PATH_START_MODE;
PATH_START1: PATH_SEG -> mode(INPATH_SLASH_MODE);
PATH_START2: HPATH_START -> mode(INPATH_SLASH_MODE);


// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L255
mode INPATH_MODE;
INPATH1: '${' -> mode(INPATH_MODE), pushMode(DEFAULT_MODE), type(DOLLAR_CURLY);
INPATH2: (PATH | PATH_SEG | PATH_CHAR+) -> mode(INPATH_MODE), type(STR); // TODO FIXME
INPATH3: (ANY | EOF) -> popMode, type(PATH_END);

// https://github.com/NixOS/nix/blob/0a7746603eda58c4b368e977e87d0aa4db397f5b/src/libexpr/lexer.l#L255
mode INPATH_SLASH_MODE;
INPATH_SLASH1: '${' -> mode(INPATH_MODE), pushMode(DEFAULT_MODE), type(DOLLAR_CURLY);
INPATH_SLASH2: (PATH | PATH_SEG | PATH_CHAR+) -> mode(INPATH_MODE), type(STR); // TODO FIXME
INPATH_SLASH3: (ANY | EOF); // TODO FIXME error