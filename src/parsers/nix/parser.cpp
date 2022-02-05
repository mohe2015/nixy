// gcc11
// g++ -c -fmodules-ts -x c++-system-header -std=c++20 iostream string vector
// g++ -std=c++20 -fmodules-ts src/parsers/nix/parser.cpp

// https://cor3ntin.github.io/posts/iouring/
// https://cor3ntin.github.io/posts/executors/
// https://en.cppreference.com/w/cpp/language/coroutines
// https://en.cppreference.com/w/cpp/language/modules
export module parser;
import <iostream>;

export int main() {
    std::cout << "Hello world!" << std::endl;
    return 0;
}