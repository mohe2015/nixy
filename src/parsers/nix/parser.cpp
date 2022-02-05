// gcc11
// g++ -c -fmodules-ts -x c++-system-header -std=c++20 iostream string vector format
// g++ -std=c++20 -fmodules-ts src/parsers/nix/parser.cpp

// clang_13
// clang++ -std=c++20 -fmodules src/parsers/nix/parser.cpp

// https://cor3ntin.github.io/posts/iouring/
// https://cor3ntin.github.io/posts/executors/
// https://en.cppreference.com/w/cpp/language/coroutines
// https://en.cppreference.com/w/cpp/language/modules

// https://github.com/NixOS/nix/blob/master/src/libexpr/parser.y

export module parser;
import <iostream>;
//import <format>;

export int main() {
    std::cout << "Hello world!" << std::endl;
    return 0;
}