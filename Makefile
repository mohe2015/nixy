all: src/parsers/nix/lexer-tab.cc

src/parsers/nix/lexer-tab.cc: src/parsers/nix/lexer.l
	flex --outfile=src/parsers/nix/lexer-tab.cc src/parsers/nix/lexer.l