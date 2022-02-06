// A Bison parser, made by GNU Bison 3.8.2.

// Skeleton interface for Bison GLR parsers in C++

// Copyright (C) 2002-2015, 2018-2021 Free Software Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// As a special exception, you may create a larger work that contains
// part or all of the Bison parser skeleton and distribute that work
// under terms of your choice, so long as that work isn't itself a
// parser generator using the skeleton or a modified version thereof
// as a parser skeleton.  Alternatively, if you modify or redistribute
// the parser skeleton itself, you may (at your option) remove this
// special exception, which will cause the skeleton and the resulting
// Bison output files to be licensed under the GNU General Public
// License without this special exception.

// This special exception was added by the Free Software Foundation in
// version 2.2 of Bison.

// C++ GLR parser skeleton written by Valentin Tolmer.

// DO NOT RELY ON FEATURES THAT ARE NOT DOCUMENTED in the manual,
// especially those whose name start with YY_ or yy_.  They are
// private implementation details that can be changed or removed.

#ifndef YY_YY_SRC_PARSERS_NIX_PARSER_TAB_HH_INCLUDED
# define YY_YY_SRC_PARSERS_NIX_PARSER_TAB_HH_INCLUDED
// "%code requires" blocks.
#line 19 "src/parsers/nix/parser.y"


#ifndef BISON_HEADER
#define BISON_HEADER

#include <variant>


    struct ParseData
    {
        EvalState & state;
        SymbolTable & symbols;
        Expr * result;
        Path basePath;
        Symbol file;
        FileOrigin origin;
        std::optional<ErrorInfo> error;
        ParseData(EvalState & state)
            : state(state)
            , symbols(state.symbols)
            { };
    };

    struct ParserFormals {
        std::vector<Formal> formals;
        bool ellipsis = false;
    };


// using C a struct allows us to avoid having to define the special
// members that using string_view here would implicitly delete.
struct StringToken {
  const char * p;
  size_t l;
  bool hasIndentation;
  operator std::string_view() const { return {p, l}; }
};

#define YY_DECL int yylex \
    (YYSTYPE * yylval_param, YYLTYPE * yylloc_param, yyscan_t yyscanner, nix::ParseData * data)

#endif


#line 88 "src/parsers/nix/parser-tab.hh"

#include <algorithm>
#include <cstddef> // ptrdiff_t
#include <cstring> // memcpy
#include <iostream>
#include <iomanip>
#include <limits>
#include <stdexcept>
#include <stdint.h>
#include <string>
#include <vector>

#if defined __cplusplus
# define YY_CPLUSPLUS __cplusplus
#else
# define YY_CPLUSPLUS 199711L
#endif

// Support move semantics when possible.
#if 201103L <= YY_CPLUSPLUS
# define YY_MOVE           std::move
# define YY_MOVE_OR_COPY   move
# define YY_MOVE_REF(Type) Type&&
# define YY_RVREF(Type)    Type&&
# define YY_COPY(Type)     Type
#else
# define YY_MOVE
# define YY_MOVE_OR_COPY   copy
# define YY_MOVE_REF(Type) Type&
# define YY_RVREF(Type)    const Type&
# define YY_COPY(Type)     const Type&
#endif

// Support noexcept when possible.
#if 201103L <= YY_CPLUSPLUS
# define YY_NOEXCEPT noexcept
# define YY_NOTHROW
#else
# define YY_NOEXCEPT
# define YY_NOTHROW throw ()
#endif

// Support constexpr when possible.
#if 201703 <= YY_CPLUSPLUS
# define YY_CONSTEXPR constexpr
#else
# define YY_CONSTEXPR
#endif
# include "location.hh"


/* Debug traces.  */
#ifndef YYDEBUG
# define YYDEBUG 0
#endif

namespace yy {
#line 146 "src/parsers/nix/parser-tab.hh"




  /// A Bison parser.
  class parser
  {
  public:
    /// Symbol semantic values.
    union value_type
    {
#line 306 "src/parsers/nix/parser.y"

  // !!! We're probably leaking stuff here.
  nix::Expr * e;
  nix::ExprList * list;
  nix::ExprAttrs * attrs;
  nix::ParserFormals * formals;
  nix::Formal * formal;
  nix::NixInt n;
  nix::NixFloat nf;
  StringToken id; // !!! -> Symbol
  StringToken path;
  StringToken uri;
  StringToken str;
  std::vector<nix::AttrName> * attrNames;
  std::vector<std::pair<nix::Pos, nix::Expr *> > * string_parts;
  std::vector<std::pair<nix::Pos, std::variant<nix::Expr *, StringToken> > > * ind_string_parts;

#line 176 "src/parsers/nix/parser-tab.hh"

    };
    /// Symbol locations.
    typedef location location_type;

    /// Syntax errors thrown from user actions.
    struct syntax_error : std::runtime_error
    {
      syntax_error (const location_type& l, const std::string& m)
        : std::runtime_error (m)
        , location (l)
      {}

      syntax_error (const syntax_error& s)
        : std::runtime_error (s.what ())
        , location (s.location)
      {}

      ~syntax_error () YY_NOEXCEPT YY_NOTHROW;

      location_type location;
    };

    /// Token kinds.
    struct token
    {
      enum token_kind_type
      {
        YYEMPTY = -2,
    YYEOF = 0,                     // "end of file"
    YYerror = 256,                 // error
    YYUNDEF = 257,                 // "invalid token"
    ID = 258,                      // ID
    ATTRPATH = 259,                // ATTRPATH
    STR = 260,                     // STR
    IND_STR = 261,                 // IND_STR
    INT = 262,                     // INT
    FLOAT = 263,                   // FLOAT
    PATH = 264,                    // PATH
    HPATH = 265,                   // HPATH
    SPATH = 266,                   // SPATH
    PATH_END = 267,                // PATH_END
    URI = 268,                     // URI
    IF = 269,                      // IF
    THEN = 270,                    // THEN
    ELSE = 271,                    // ELSE
    ASSERT = 272,                  // ASSERT
    WITH = 273,                    // WITH
    LET = 274,                     // LET
    IN = 275,                      // IN
    REC = 276,                     // REC
    INHERIT = 277,                 // INHERIT
    EQ = 278,                      // EQ
    NEQ = 279,                     // NEQ
    AND = 280,                     // AND
    OR = 281,                      // OR
    IMPL = 282,                    // IMPL
    OR_KW = 283,                   // OR_KW
    DOLLAR_CURLY = 284,            // DOLLAR_CURLY
    IND_STRING_OPEN = 285,         // IND_STRING_OPEN
    IND_STRING_CLOSE = 286,        // IND_STRING_CLOSE
    ELLIPSIS = 287,                // ELLIPSIS
    LEQ = 288,                     // LEQ
    GEQ = 289,                     // GEQ
    UPDATE = 290,                  // UPDATE
    NOT = 291,                     // NOT
    CONCAT = 292,                  // CONCAT
    NEGATE = 293                   // NEGATE
      };
    };

    /// Token kind, as returned by yylex.
    typedef token::token_kind_type token_kind_type;

    /// Symbol kinds.
    struct symbol_kind
    {
      enum symbol_kind_type
      {
        YYNTOKENS = 60, ///< Number of tokens.
        S_YYEMPTY = -2,
        S_YYEOF = 0,                             // "end of file"
        S_YYerror = 1,                           // error
        S_YYUNDEF = 2,                           // "invalid token"
        S_ID = 3,                                // ID
        S_ATTRPATH = 4,                          // ATTRPATH
        S_STR = 5,                               // STR
        S_IND_STR = 6,                           // IND_STR
        S_INT = 7,                               // INT
        S_FLOAT = 8,                             // FLOAT
        S_PATH = 9,                              // PATH
        S_HPATH = 10,                            // HPATH
        S_SPATH = 11,                            // SPATH
        S_PATH_END = 12,                         // PATH_END
        S_URI = 13,                              // URI
        S_IF = 14,                               // IF
        S_THEN = 15,                             // THEN
        S_ELSE = 16,                             // ELSE
        S_ASSERT = 17,                           // ASSERT
        S_WITH = 18,                             // WITH
        S_LET = 19,                              // LET
        S_IN = 20,                               // IN
        S_REC = 21,                              // REC
        S_INHERIT = 22,                          // INHERIT
        S_EQ = 23,                               // EQ
        S_NEQ = 24,                              // NEQ
        S_AND = 25,                              // AND
        S_OR = 26,                               // OR
        S_IMPL = 27,                             // IMPL
        S_OR_KW = 28,                            // OR_KW
        S_DOLLAR_CURLY = 29,                     // DOLLAR_CURLY
        S_IND_STRING_OPEN = 30,                  // IND_STRING_OPEN
        S_IND_STRING_CLOSE = 31,                 // IND_STRING_CLOSE
        S_ELLIPSIS = 32,                         // ELLIPSIS
        S_33_ = 33,                              // '<'
        S_34_ = 34,                              // '>'
        S_LEQ = 35,                              // LEQ
        S_GEQ = 36,                              // GEQ
        S_UPDATE = 37,                           // UPDATE
        S_NOT = 38,                              // NOT
        S_39_ = 39,                              // '+'
        S_40_ = 40,                              // '-'
        S_41_ = 41,                              // '*'
        S_42_ = 42,                              // '/'
        S_CONCAT = 43,                           // CONCAT
        S_44_ = 44,                              // '?'
        S_NEGATE = 45,                           // NEGATE
        S_46_ = 46,                              // ':'
        S_47_ = 47,                              // '{'
        S_48_ = 48,                              // '}'
        S_49_ = 49,                              // '@'
        S_50_ = 50,                              // ';'
        S_51_ = 51,                              // '!'
        S_52_ = 52,                              // '.'
        S_53_ = 53,                              // '"'
        S_54_ = 54,                              // '('
        S_55_ = 55,                              // ')'
        S_56_ = 56,                              // '['
        S_57_ = 57,                              // ']'
        S_58_ = 58,                              // '='
        S_59_ = 59,                              // ','
        S_YYACCEPT = 60,                         // $accept
        S_start = 61,                            // start
        S_expr = 62,                             // expr
        S_expr_function = 63,                    // expr_function
        S_expr_if = 64,                          // expr_if
        S_expr_op = 65,                          // expr_op
        S_expr_app = 66,                         // expr_app
        S_expr_select = 67,                      // expr_select
        S_expr_simple = 68,                      // expr_simple
        S_string_parts = 69,                     // string_parts
        S_string_parts_interpolated = 70,        // string_parts_interpolated
        S_path_start = 71,                       // path_start
        S_ind_string_parts = 72,                 // ind_string_parts
        S_binds = 73,                            // binds
        S_attrs = 74,                            // attrs
        S_attrpath = 75,                         // attrpath
        S_attr = 76,                             // attr
        S_string_attr = 77,                      // string_attr
        S_expr_list = 78,                        // expr_list
        S_formals = 79,                          // formals
        S_formal = 80                            // formal
      };
    };

    /// (Internal) symbol kind.
    typedef symbol_kind::symbol_kind_type symbol_kind_type;

    /// The number of tokens.
    static const symbol_kind_type YYNTOKENS = symbol_kind::YYNTOKENS;

    /// A complete symbol.
    ///
    /// Expects its Base type to provide access to the symbol kind
    /// via kind ().
    ///
    /// Provide access to semantic value and location.
    template <typename Base>
    struct basic_symbol : Base
    {
      /// Alias to Base.
      typedef Base super_type;

      /// Default constructor.
      basic_symbol () YY_NOEXCEPT
        : value ()
        , location ()
      {}

#if 201103L <= YY_CPLUSPLUS
      /// Move constructor.
      basic_symbol (basic_symbol&& that)
        : Base (std::move (that))
        , value (std::move (that.value))
        , location (std::move (that.location))
      {}
#endif

      /// Copy constructor.
      basic_symbol (const basic_symbol& that);
      /// Constructor for valueless symbols.
      basic_symbol (typename Base::kind_type t,
                    YY_MOVE_REF (location_type) l);

      /// Constructor for symbols with semantic value.
      basic_symbol (typename Base::kind_type t,
                    YY_RVREF (value_type) v,
                    YY_RVREF (location_type) l);

      /// Destroy the symbol.
      ~basic_symbol ()
      {
        clear ();
      }


      /// Copy assignment.
      basic_symbol& operator= (const basic_symbol& that)
      {
        Base::operator= (that);
        value = that.value;
        location = that.location;
        return *this;
      }

      /// Move assignment.
      basic_symbol& operator= (basic_symbol&& that)
      {
        Base::operator= (std::move (that));
        value = std::move (that.value);
        location = std::move (that.location);
        return *this;
      }


      /// Destroy contents, and record that is empty.
      void clear () YY_NOEXCEPT
      {
        Base::clear ();
      }

      /// The user-facing name of this symbol.
      std::string name () const YY_NOEXCEPT
      {
        return parser::symbol_name (this->kind ());
      }

      /// Whether empty.
      bool empty () const YY_NOEXCEPT;

      /// Destructive move, \a s is emptied into this.
      void move (basic_symbol& s);

      /// The semantic value.
      value_type value;

      /// The location.
      location_type location;

    private:
#if YY_CPLUSPLUS < 201103L
      /// Assignment operator.
      basic_symbol& operator= (const basic_symbol& that);
#endif
    };

    /// Type access provider for token (enum) based symbols.
    struct by_kind
    {
      /// The symbol kind as needed by the constructor.
      typedef token_kind_type kind_type;

      /// Default constructor.
      by_kind () YY_NOEXCEPT;

#if 201103L <= YY_CPLUSPLUS
      /// Move constructor.
      by_kind (by_kind&& that) YY_NOEXCEPT;
#endif

      /// Copy constructor.
      by_kind (const by_kind& that) YY_NOEXCEPT;

      /// Constructor from (external) token numbers.
      by_kind (kind_type t) YY_NOEXCEPT;


      /// Copy assignment.
      by_kind& operator= (const by_kind& that);

      /// Move assignment.
      by_kind& operator= (by_kind&& that);


      /// Record that this symbol is empty.
      void clear () YY_NOEXCEPT;

      /// Steal the symbol kind from \a that.
      void move (by_kind& that);

      /// The (internal) type number (corresponding to \a type).
      /// \a empty when empty.
      symbol_kind_type kind () const YY_NOEXCEPT;

      /// The symbol kind.
      /// \a S_YYEMPTY when empty.
      symbol_kind_type kind_;
    };

    /// "External" symbols: returned by the scanner.
    struct symbol_type : basic_symbol<by_kind>
    {};


    // FIXME: should be private eventually.
    class glr_stack;
    class glr_state;

    /// Build a parser object.
    parser (void * scanner_yyarg, nix::ParseData * data_yyarg);
    ~parser ();

    /// Parse.  An alias for parse ().
    /// \returns  0 iff parsing succeeded.
    int operator() ();

    /// Parse.
    /// \returns  0 iff parsing succeeded.
    int parse ();

#if YYDEBUG
    /// The current debugging stream.
    std::ostream& debug_stream () const;
    /// Set the current debugging stream.
    void set_debug_stream (std::ostream &);

    /// Type for debugging levels.
    using debug_level_type = int;
    /// The current debugging level.
    debug_level_type debug_level () const;
    /// Set the current debugging level.
    void set_debug_level (debug_level_type l);
#endif

    /// Report a syntax error.
    /// \param loc    where the syntax error is found.
    /// \param msg    a description of the syntax error.
    void error (const location_type& loc, const std::string& msg);

    /// The user-facing name of the symbol whose (internal) number is
    /// YYSYMBOL.  No bounds checking.
    static std::string symbol_name (symbol_kind_type yysymbol);



    class context
    {
    public:
      context (glr_stack& yystack, const symbol_type& yyla);
      const symbol_type& lookahead () const YY_NOEXCEPT { return yyla_; }
      symbol_kind_type token () const YY_NOEXCEPT { return yyla_.kind (); }
      const location_type& location () const YY_NOEXCEPT { return yyla_.location; }

      /// Put in YYARG at most YYARGN of the expected tokens, and return the
      /// number of tokens stored in YYARG.  If YYARG is null, return the
      /// number of expected tokens (guaranteed to be less than YYNTOKENS).
      int expected_tokens (symbol_kind_type yyarg[], int yyargn) const;

    private:
      glr_stack& yystack_;
      const symbol_type& yyla_;
    };

# if YYDEBUG
  public:
    /// \brief Report a symbol value on the debug stream.
    /// \param yykind   The symbol kind.
    /// \param yyval    Its semantic value.
    /// \param yyloc    Its location.
    void yy_symbol_value_print_ (symbol_kind_type yykind,
                                 const value_type& yyval,
                                 const location_type& yyloc) const;
    /// \brief Report a symbol on the debug stream.
    /// \param yykind   The symbol kind.
    /// \param yyval    Its semantic value.
    /// \param yyloc    Its location.
    void yy_symbol_print_ (symbol_kind_type yykind,
                           const value_type& yyval,
                           const location_type& yyloc) const;
  private:
    /// Debug stream.
    std::ostream* yycdebug_;
#endif


  private:
    /// The arguments of the error message.
    int yy_syntax_error_arguments_ (const context& yyctx,
                                    symbol_kind_type yyarg[], int yyargn) const;

    /// Generate an error message.
    /// \param yyctx     the context in which the error occurred.
    virtual std::string yysyntax_error_ (const context& yyctx) const;

    /// Convert a scanner token kind \a t to a symbol kind.
    /// In theory \a t should be a token_kind_type, but character literals
    /// are valid, yet not members of the token_kind_type enum.
    static symbol_kind_type yytranslate_ (int t) YY_NOEXCEPT;

    /// Convert the symbol name \a n to a form suitable for a diagnostic.
    static std::string yytnamerr_ (const char *yystr);

    /// For a symbol, its name in clear.
    static const char* const yytname_[];


    /// \brief Reclaim the memory associated to a symbol.
    /// \param yymsg     Why this token is reclaimed.
    ///                  If null, print nothing.
    /// \param yykind    The symbol kind.
    void yy_destroy_ (const char* yymsg, symbol_kind_type yykind,
                      value_type& yyval,
                      location_type& yyloc);


    // User arguments.
    void * scanner;
    nix::ParseData * data;
    // Needs access to yy_destroy_, report_syntax_error, etc.
    friend glr_stack;
  };


} // yy
#line 611 "src/parsers/nix/parser-tab.hh"




#endif // !YY_YY_SRC_PARSERS_NIX_PARSER_TAB_HH_INCLUDED
