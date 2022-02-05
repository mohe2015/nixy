// antlr4 -o antlr Nix.g4
// whereis antlr
// javac -cp /nix/store/0h2al86yb3gh59h4lckwsprc5vavirmr-antlr-4.8/share/java/antlr-4.8-complete.jar *.java
// grun Nix r -gui
// write code then ctrl+D

// antlr4 NixLexer.g4 && antlr4 NixParser.g4 && javac -cp /nix/store/0h2al86yb3gh59h4lckwsprc5vavirmr-antlr-4.8/share/java/antlr-4.8-complete.jar *.java && grun Nix tokens -tokens

// antlr4 NixLexer.g4 && antlr4 NixParser.g4 && javac -cp /nix/store/0h2al86yb3gh59h4lckwsprc5vavirmr-antlr-4.8/share/java/antlr-4.8-complete.jar *.java && grun Nix start -gui
parser grammar NixParser;

options {
	tokenVocab = NixLexer;
}

// https://github.com/antlr/antlr4/blob/master/doc/parser-rules.md
// https://github.com/antlr/antlr4/blob/master/doc/lexer-rules.md
// https://github.com/antlr/antlr4/blob/master/doc/wildcard.md
// https://github.com/antlr/antlr4/blob/master/doc/interpreters.md
// https://github.com/antlr/antlr4/blob/master/doc/cpp-target.md

// https://github.com/NixOS/nix/blob/master/src/libexpr/lexer.l
// https://github.com/NixOS/nix/blob/master/src/libexpr/parser.y

// TODO FIXME operator precedence

start: expr EOF;
expr: expr_function;
expr_function
  : ID ':' expr_function
  | '{' formals '}' ':' expr_function
  | '{' formals '}' '@' ID ':' expr_function
  | ID '@' '{' formals '}' ':' expr_function
  | ASSERT expr ';' expr_function
  | WITH expr ';' expr_function
  | LET binds IN expr_function
  | expr_if
  ;

expr_if
  : IF expr THEN expr ELSE expr
  | expr_op
  ;

expr_op
  : '!' expr_op
  | '-' expr_op
  | expr_op EQ expr_op
  | expr_op NEQ expr_op
  | expr_op '<' expr_op
  | expr_op LEQ expr_op
  | expr_op '>' expr_op
  | expr_op GEQ expr_op
  | expr_op AND expr_op
  | expr_op OR expr_op
  | expr_op IMPL expr_op
  | expr_op UPDATE expr_op
  | expr_op '?' attrpath
  | expr_op '+' expr_op
  | expr_op '-' expr_op
  | expr_op '*' expr_op
  | expr_op '/' expr_op
  | expr_op CONCAT expr_op
  | expr_app
  ;

expr_app
  : expr_app expr_select
  | expr_select
  ;

expr_select
  : expr_simple '.' attrpath
  | expr_simple '.' attrpath OR_KW expr_select
  | /* Backwards compatibility: because Nixpkgs has a rarely used
       function named ‘or’, allow stuff like ‘map or [...]’. */
    expr_simple OR_KW
  | expr_simple
  ;

expr_simple
  : ID
  | INT
  | FLOAT
  | STRING_OPEN string_parts STRING_CLOSE
  | IND_STRING_OPEN ind_string_parts IND_STRING_CLOSE
  | path_start PATH_END
  | path_start string_parts_interpolated PATH_END
  | SPATH
  | URI
  | '(' expr ')'
  /* Let expressions `let {..., body = ...}' are just desugared
     into `(rec {..., body = ...}).body'. */
  | LET '{' binds '}'
  | REC '{' binds '}'
  | '{' binds '}'
  | '[' expr_list ']'
  ;

string_parts
  : STR
  | string_parts_interpolated
  ;

string_parts_interpolated
  : string_parts_interpolated STR
  | string_parts_interpolated DOLLAR_CURLY expr '}'
  | DOLLAR_CURLY expr '}'
  | STR DOLLAR_CURLY expr '}'
  ;

path_start
  : PATH
  | HPATH
  ;

ind_string_parts
  : ind_string_parts IND_STR
  | ind_string_parts DOLLAR_CURLY expr '}'
  | 
  ;

binds
  : binds attrpath '=' expr ';'
  | binds INHERIT attrs ';'
  | binds INHERIT '(' expr ')' attrs ';'
  |
  ;

attrs
  : attrs attr
  | attrs string_attr
  |
  ;

attrpath
  : attrpath '.' attr
  | attrpath '.' string_attr
  | attr
  | string_attr
  ;

attr
  : ID
  | OR_KW
  ;

string_attr:
  STRING_OPEN string_parts STRING_CLOSE
  | DOLLAR_CURLY expr '}'
  ;

expr_list
  : expr_list expr_select
  |
  ;

formals
  : formal ',' formals
  | formal
  | ELLIPSIS
  ;

formal
  : ID
  | ID '?' expr
  ;
