// gcc11
// g++ -c -fmodules-ts -x c++-system-header -std=c++20 iostream string vector format
// g++ -std=c++20 -fmodules-ts src/parsers/nix/lexer.cpp

// clang_13
// clang++ -std=c++20 -fmodules src/parsers/nix/lexer.cpp

// https://cor3ntin.github.io/posts/iouring/
// http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2020/p2052r0.pdf
// https://unixism.net/2020/04/io-uring-by-example-part-1-introduction/
// https://cor3ntin.github.io/posts/executors/
// https://en.cppreference.com/w/cpp/language/coroutines
// https://en.cppreference.com/w/cpp/language/modules

// https://github.com/NixOS/nix/blob/master/src/libexpr/lexer.l

// https://en.wikipedia.org/wiki/Comparison_of_parser_generators

export module parser;
import <iostream>;
import <fstream>;
import <sstream>;
import <iterator>;
import <concepts>;
//import <format>;

// https://nixos.wiki/wiki/Nix_Expression_Language

/*
DESIGN:
We need to read in the file somehow so either in blocks or the whole file (this memory allocation can not be prevented)
For memory locality it may make sense to then do one reallocation which removes all comments, whitespace etc. (and also unescapes?)
*/

// https://en.cppreference.com/w/cpp/iterator#C.2B.2B20_iterator_concepts
// https://en.cppreference.com/w/cpp/named_req/ForwardIterator

export struct Token {

};

export struct Iterator 
{
    using iterator_category = std::input_iterator_tag;
    using value_type        = Token;
    using reference         = const Token&;

    Iterator(std::ifstream input) : input(std::move(input)) {}

    reference operator*() const { return currentToken; }
    Token operator->() { return currentToken; }
    Iterator& operator++() { return *this; }  
    friend bool operator== (const Iterator& a, const Iterator& b) { return false; };
    Iterator& begin() { return *this; }
    Iterator& end() { return *this; }

private:
    std::ifstream input;
    Token currentToken;
};

export int main() {
    std::cout << "Hello world!" << std::endl;

    std::ifstream f("/etc/nixos/nixpkgs/flake.nix");
        
    for (auto& elem : Iterator(std::move(f))) {
        //std::cout << elem << std::endl;
    }

    return 0;
}
