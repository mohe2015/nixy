// antlr4 -o antlr Nix.g4
// whereis antlr
// javac -cp /nix/store/0h2al86yb3gh59h4lckwsprc5vavirmr-antlr-4.8/share/java/antlr-4.8-complete.jar *.java
// grun Nix r -gui
// write code then ctrl+D
grammar Nix;

r  : 'hello' ID ;         // match keyword hello followed by an identifier
ID : [a-z]+ ;             // match lower-case identifiers
WS : [ \t\r\n]+ -> skip ; // skip spaces, tabs, newlines

