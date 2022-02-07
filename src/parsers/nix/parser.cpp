// gcc11
// g++ -c -fmodules-ts -x c++-system-header -std=c++20 iostream string vector format
// g++ -std=c++20 -fmodules-ts src/parsers/nix/parser.cpp

// clang_13
// clang++ -std=c++20 -fmodules src/parsers/nix2/parser.cpp

// https://cor3ntin.github.io/posts/iouring/
// http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2020/p2052r0.pdf
// https://unixism.net/2020/04/io-uring-by-example-part-1-introduction/
// https://cor3ntin.github.io/posts/executors/
// https://en.cppreference.com/w/cpp/language/coroutines
// https://en.cppreference.com/w/cpp/language/modules

// https://github.com/NixOS/nix/blob/master/src/libexpr/parser.y

// https://en.wikipedia.org/wiki/Operator-precedence_parser

// https://en.wikipedia.org/wiki/Comparison_of_parser_generators

// https://github.com/antlr/grammars-v4/

// bison or antlr or probably just hand-written

// https://www.gnu.org/software/bison/manual/bison.html#GLR-Parsers

// antlr

export module parser;
import <iostream>;
import <fstream>;
import <sstream>;
//import <format>;

// https://nixos.wiki/wiki/Nix_Expression_Language

export void parse(std::string data) {

}

export int main() {
    std::cout << "Hello world!" << std::endl;

    std::ifstream f("/etc/nixos/nixpkgs/flake.nix");
    if(f) {
        std::ostringstream ss;
        ss << f.rdbuf(); // reading data
        std::string str = ss.str();
        parse(str);
    }

    return 0;
}
