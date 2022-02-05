// antlr4 -Dlanguage=C++ 

lexer grammar Nix;

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


IF: 'if';          
THEN: 'then';
ELSE: 'else';
ASSERT: 'assert';
WITH: 'with';
LET: 'let';
IN: 'in';
REC: 'rec';
INERHIT: 'inherit';
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

IND_STRING_OPEN: '\'\'' (' '*'\n')?;
IND_STRING_CLOSE: '\'\'';



WS: [ \t\r\n]+ -> skip ;


mode test;
