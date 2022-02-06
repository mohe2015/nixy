all: src/parsers/nix/lexer-tab.o

src/parsers/nix/lexer-tab.cc src/parsers/nix/lexer-tab.hh: src/parsers/nix/lexer.l
	flex --outfile=src/parsers/nix/lexer-tab.cc --header-file=src/parsers/nix/lexer-tab.hh src/parsers/nix/lexer.l

src/parsers/nix/lexer-tab.o: src/parsers/nix/lexer-tab.cc src/parsers/nix/lexer-tab.hh src/parsers/nix/parser-tab.hh
	clang++ -std=c++20 -fmodules src/parsers/nix/lexer-tab.cc

src/parsers/nix/parser-tab.cc src/parsers/nix/parser-tab.hh src/parsers/nix/location.hh: src/parsers/nix/parser.y
	bison --output=src/parsers/nix/parser-tab.cc src/parsers/nix/parser.y

clean:
	rm -f src/parsers/nix/lexer-tab.cc src/parsers/nix/lexer-tab.hh src/parsers/nix/parser-tab.cc src/parsers/nix/parser-tab.hh src/parsers/nix/location.hh

diff:
	clear
	diff --color /tmp/nix/src/libexpr/parser.y src/parsers/nix/parser.y || exit 0
	diff --color /tmp/nix/src/libexpr/lexer.l src/parsers/nix/lexer.l || exit 0