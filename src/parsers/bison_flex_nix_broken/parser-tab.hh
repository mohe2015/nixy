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
#line 28 "src/parsers/nix/parser.y"


#ifndef BISON_HEADER
#define BISON_HEADER

import <string>;

typedef struct YYLTYPE
{
  int first_line;
  int first_column;
  int last_line;
  int last_column;
} YYLTYPE;

    struct ParseData
    {
       /* EvalState & state;
        SymbolTable & symbols;
        Expr * result;
        Path basePath;
        Symbol file;
        FileOrigin origin;
        std::optional<ErrorInfo> error;
        ParseData(EvalState & state)
            : state(state)
            , symbols(state.symbols)
            { };*/
    };

    struct ParserFormals {
      /*  std::vector<Formal> formals;
        bool ellipsis = false;*/
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
    (YYSTYPE * yylval_param, yy::location * yylloc_param, yyscan_t yyscanner, nix::ParseData * data)

#endif


#line 95 "src/parsers/nix/parser-tab.hh"

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



/* Debug traces.  */
#ifndef YYDEBUG
# define YYDEBUG 0
#endif

namespace yy {
#line 153 "src/parsers/nix/parser-tab.hh"




  /// A Bison parser.
  class parser
  {
  public:
  /// A buffer to store and retrieve objects.
  ///
  /// Sort of a variant, but does not keep track of the nature
  /// of the stored data, since that knowledge is available
  /// via the current parser state.
  class value_type
  {
  public:
    /// Type of *this.
    typedef value_type self_type;

    /// Empty construction.
    value_type () YY_NOEXCEPT
      : yyraw_ ()
    {}

    /// Construct and fill.
    template <typename T>
    value_type (YY_RVREF (T) t)
    {
      new (yyas_<T> ()) T (YY_MOVE (t));
    }

#if 201103L <= YY_CPLUSPLUS
    /// Non copyable.
    value_type (const self_type&) = delete;
    /// Non copyable.
    self_type& operator= (const self_type&) = delete;
#endif

    /// Destruction, allowed only if empty.
    ~value_type () YY_NOEXCEPT
    {}

# if 201103L <= YY_CPLUSPLUS
    /// Instantiate a \a T in here from \a t.
    template <typename T, typename... U>
    T&
    emplace (U&&... u)
    {
      return *new (yyas_<T> ()) T (std::forward <U>(u)...);
    }
# else
    /// Instantiate an empty \a T in here.
    template <typename T>
    T&
    emplace ()
    {
      return *new (yyas_<T> ()) T ();
    }

    /// Instantiate a \a T in here from \a t.
    template <typename T>
    T&
    emplace (const T& t)
    {
      return *new (yyas_<T> ()) T (t);
    }
# endif

    /// Instantiate an empty \a T in here.
    /// Obsolete, use emplace.
    template <typename T>
    T&
    build ()
    {
      return emplace<T> ();
    }

    /// Instantiate a \a T in here from \a t.
    /// Obsolete, use emplace.
    template <typename T>
    T&
    build (const T& t)
    {
      return emplace<T> (t);
    }

    /// Accessor to a built \a T.
    template <typename T>
    T&
    as () YY_NOEXCEPT
    {
      return *yyas_<T> ();
    }

    /// Const accessor to a built \a T (for %printer).
    template <typename T>
    const T&
    as () const YY_NOEXCEPT
    {
      return *yyas_<T> ();
    }

    /// Swap the content with \a that, of same type.
    ///
    /// Both variants must be built beforehand, because swapping the actual
    /// data requires reading it (with as()), and this is not possible on
    /// unconstructed variants: it would require some dynamic testing, which
    /// should not be the variant's responsibility.
    /// Swapping between built and (possibly) non-built is done with
    /// self_type::move ().
    template <typename T>
    void
    swap (self_type& that) YY_NOEXCEPT
    {
      std::swap (as<T> (), that.as<T> ());
    }

    /// Move the content of \a that to this.
    ///
    /// Destroys \a that.
    template <typename T>
    void
    move (self_type& that)
    {
# if 201103L <= YY_CPLUSPLUS
      emplace<T> (std::move (that.as<T> ()));
# else
      emplace<T> ();
      swap<T> (that);
# endif
      that.destroy<T> ();
    }

# if 201103L <= YY_CPLUSPLUS
    /// Move the content of \a that to this.
    template <typename T>
    void
    move (self_type&& that)
    {
      emplace<T> (std::move (that.as<T> ()));
      that.destroy<T> ();
    }
#endif

    /// Copy the content of \a that to this.
    template <typename T>
    void
    copy (const self_type& that)
    {
      emplace<T> (that.as<T> ());
    }

    /// Destroy the stored \a T.
    template <typename T>
    void
    destroy ()
    {
      as<T> ().~T ();
    }

  private:
#if YY_CPLUSPLUS < 201103L
    /// Non copyable.
    value_type (const self_type&);
    /// Non copyable.
    self_type& operator= (const self_type&);
#endif

    /// Accessor to raw memory as \a T.
    template <typename T>
    T*
    yyas_ () YY_NOEXCEPT
    {
      void *yyp = yyraw_;
      return static_cast<T*> (yyp);
     }

    /// Const accessor to raw memory as \a T.
    template <typename T>
    const T*
    yyas_ () const YY_NOEXCEPT
    {
      const void *yyp = yyraw_;
      return static_cast<const T*> (yyp);
     }

    /// An auxiliary type to compute the largest semantic type.
    union union_type
    {
      // attrs
      // attrpath
      char dummy1[sizeof (attrNames)];

      // binds
      char dummy2[sizeof (attrs)];

      // start
      // expr
      // expr_function
      // expr_if
      // expr_op
      // expr_app
      // expr_select
      // expr_simple
      // string_parts
      // path_start
      // string_attr
      char dummy3[sizeof (e)];

      // formal
      char dummy4[sizeof (formal)];

      // formals
      char dummy5[sizeof (formals)];

      // ID
      // ATTRPATH
      // attr
      char dummy6[sizeof (id)];

      // ind_string_parts
      char dummy7[sizeof (ind_string_parts)];

      // expr_list
      char dummy8[sizeof (list)];

      // INT
      char dummy9[sizeof (n)];

      // FLOAT
      char dummy10[sizeof (nf)];

      // PATH
      // HPATH
      // SPATH
      // PATH_END
      char dummy11[sizeof (path)];

      // STR
      // IND_STR
      char dummy12[sizeof (str)];

      // string_parts_interpolated
      char dummy13[sizeof (string_parts)];

      // URI
      char dummy14[sizeof (uri)];
    };

    /// The size of the largest semantic type.
    enum { size = sizeof (union_type) };

    /// A buffer to store semantic values.
    union
    {
      /// Strongest alignment constraints.
      long double yyalign_me_;
      /// A buffer large enough to store any of the semantic values.
      char yyraw_[size];
    };
  };

    /// Symbol locations.
    typedef YYLTYPE location_type;

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
        , value ()
        , location (std::move (that.location))
      {
        switch (this->kind ())
    {
      case symbol_kind::S_attrs: // attrs
      case symbol_kind::S_attrpath: // attrpath
        value.move< attrNames > (std::move (that.value));
        break;

      case symbol_kind::S_binds: // binds
        value.move< attrs > (std::move (that.value));
        break;

      case symbol_kind::S_start: // start
      case symbol_kind::S_expr: // expr
      case symbol_kind::S_expr_function: // expr_function
      case symbol_kind::S_expr_if: // expr_if
      case symbol_kind::S_expr_op: // expr_op
      case symbol_kind::S_expr_app: // expr_app
      case symbol_kind::S_expr_select: // expr_select
      case symbol_kind::S_expr_simple: // expr_simple
      case symbol_kind::S_string_parts: // string_parts
      case symbol_kind::S_path_start: // path_start
      case symbol_kind::S_string_attr: // string_attr
        value.move< e > (std::move (that.value));
        break;

      case symbol_kind::S_formal: // formal
        value.move< formal > (std::move (that.value));
        break;

      case symbol_kind::S_formals: // formals
        value.move< formals > (std::move (that.value));
        break;

      case symbol_kind::S_ID: // ID
      case symbol_kind::S_ATTRPATH: // ATTRPATH
      case symbol_kind::S_attr: // attr
        value.move< id > (std::move (that.value));
        break;

      case symbol_kind::S_ind_string_parts: // ind_string_parts
        value.move< ind_string_parts > (std::move (that.value));
        break;

      case symbol_kind::S_expr_list: // expr_list
        value.move< list > (std::move (that.value));
        break;

      case symbol_kind::S_INT: // INT
        value.move< n > (std::move (that.value));
        break;

      case symbol_kind::S_FLOAT: // FLOAT
        value.move< nf > (std::move (that.value));
        break;

      case symbol_kind::S_PATH: // PATH
      case symbol_kind::S_HPATH: // HPATH
      case symbol_kind::S_SPATH: // SPATH
      case symbol_kind::S_PATH_END: // PATH_END
        value.move< path > (std::move (that.value));
        break;

      case symbol_kind::S_STR: // STR
      case symbol_kind::S_IND_STR: // IND_STR
        value.move< str > (std::move (that.value));
        break;

      case symbol_kind::S_string_parts_interpolated: // string_parts_interpolated
        value.move< string_parts > (std::move (that.value));
        break;

      case symbol_kind::S_URI: // URI
        value.move< uri > (std::move (that.value));
        break;

      default:
        break;
    }

      }
#endif

      /// Copy constructor.
      basic_symbol (const basic_symbol& that);

      /// Constructors for typed symbols.
#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, location_type&& l)
        : Base (t)
        , location (std::move (l))
      {}
#else
      basic_symbol (typename Base::kind_type t, const location_type& l)
        : Base (t)
        , location (l)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, attrNames&& v, location_type&& l)
        : Base (t)
        , value (std::move (v))
        , location (std::move (l))
      {}
#else
      basic_symbol (typename Base::kind_type t, const attrNames& v, const location_type& l)
        : Base (t)
        , value (v)
        , location (l)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, attrs&& v, location_type&& l)
        : Base (t)
        , value (std::move (v))
        , location (std::move (l))
      {}
#else
      basic_symbol (typename Base::kind_type t, const attrs& v, const location_type& l)
        : Base (t)
        , value (v)
        , location (l)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, e&& v, location_type&& l)
        : Base (t)
        , value (std::move (v))
        , location (std::move (l))
      {}
#else
      basic_symbol (typename Base::kind_type t, const e& v, const location_type& l)
        : Base (t)
        , value (v)
        , location (l)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, formal&& v, location_type&& l)
        : Base (t)
        , value (std::move (v))
        , location (std::move (l))
      {}
#else
      basic_symbol (typename Base::kind_type t, const formal& v, const location_type& l)
        : Base (t)
        , value (v)
        , location (l)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, formals&& v, location_type&& l)
        : Base (t)
        , value (std::move (v))
        , location (std::move (l))
      {}
#else
      basic_symbol (typename Base::kind_type t, const formals& v, const location_type& l)
        : Base (t)
        , value (v)
        , location (l)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, id&& v, location_type&& l)
        : Base (t)
        , value (std::move (v))
        , location (std::move (l))
      {}
#else
      basic_symbol (typename Base::kind_type t, const id& v, const location_type& l)
        : Base (t)
        , value (v)
        , location (l)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, ind_string_parts&& v, location_type&& l)
        : Base (t)
        , value (std::move (v))
        , location (std::move (l))
      {}
#else
      basic_symbol (typename Base::kind_type t, const ind_string_parts& v, const location_type& l)
        : Base (t)
        , value (v)
        , location (l)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, list&& v, location_type&& l)
        : Base (t)
        , value (std::move (v))
        , location (std::move (l))
      {}
#else
      basic_symbol (typename Base::kind_type t, const list& v, const location_type& l)
        : Base (t)
        , value (v)
        , location (l)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, n&& v, location_type&& l)
        : Base (t)
        , value (std::move (v))
        , location (std::move (l))
      {}
#else
      basic_symbol (typename Base::kind_type t, const n& v, const location_type& l)
        : Base (t)
        , value (v)
        , location (l)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, nf&& v, location_type&& l)
        : Base (t)
        , value (std::move (v))
        , location (std::move (l))
      {}
#else
      basic_symbol (typename Base::kind_type t, const nf& v, const location_type& l)
        : Base (t)
        , value (v)
        , location (l)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, path&& v, location_type&& l)
        : Base (t)
        , value (std::move (v))
        , location (std::move (l))
      {}
#else
      basic_symbol (typename Base::kind_type t, const path& v, const location_type& l)
        : Base (t)
        , value (v)
        , location (l)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, str&& v, location_type&& l)
        : Base (t)
        , value (std::move (v))
        , location (std::move (l))
      {}
#else
      basic_symbol (typename Base::kind_type t, const str& v, const location_type& l)
        : Base (t)
        , value (v)
        , location (l)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, string_parts&& v, location_type&& l)
        : Base (t)
        , value (std::move (v))
        , location (std::move (l))
      {}
#else
      basic_symbol (typename Base::kind_type t, const string_parts& v, const location_type& l)
        : Base (t)
        , value (v)
        , location (l)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, uri&& v, location_type&& l)
        : Base (t)
        , value (std::move (v))
        , location (std::move (l))
      {}
#else
      basic_symbol (typename Base::kind_type t, const uri& v, const location_type& l)
        : Base (t)
        , value (v)
        , location (l)
      {}
#endif

      /// Destroy the symbol.
      ~basic_symbol ()
      {
        clear ();
      }


      /// Copy assignment.
      basic_symbol& operator= (const basic_symbol& that)
      {
        Base::operator= (that);
        switch (this->kind ())
    {
      case symbol_kind::S_attrs: // attrs
      case symbol_kind::S_attrpath: // attrpath
        value.copy< attrNames > (that.value);
        break;

      case symbol_kind::S_binds: // binds
        value.copy< attrs > (that.value);
        break;

      case symbol_kind::S_start: // start
      case symbol_kind::S_expr: // expr
      case symbol_kind::S_expr_function: // expr_function
      case symbol_kind::S_expr_if: // expr_if
      case symbol_kind::S_expr_op: // expr_op
      case symbol_kind::S_expr_app: // expr_app
      case symbol_kind::S_expr_select: // expr_select
      case symbol_kind::S_expr_simple: // expr_simple
      case symbol_kind::S_string_parts: // string_parts
      case symbol_kind::S_path_start: // path_start
      case symbol_kind::S_string_attr: // string_attr
        value.copy< e > (that.value);
        break;

      case symbol_kind::S_formal: // formal
        value.copy< formal > (that.value);
        break;

      case symbol_kind::S_formals: // formals
        value.copy< formals > (that.value);
        break;

      case symbol_kind::S_ID: // ID
      case symbol_kind::S_ATTRPATH: // ATTRPATH
      case symbol_kind::S_attr: // attr
        value.copy< id > (that.value);
        break;

      case symbol_kind::S_ind_string_parts: // ind_string_parts
        value.copy< ind_string_parts > (that.value);
        break;

      case symbol_kind::S_expr_list: // expr_list
        value.copy< list > (that.value);
        break;

      case symbol_kind::S_INT: // INT
        value.copy< n > (that.value);
        break;

      case symbol_kind::S_FLOAT: // FLOAT
        value.copy< nf > (that.value);
        break;

      case symbol_kind::S_PATH: // PATH
      case symbol_kind::S_HPATH: // HPATH
      case symbol_kind::S_SPATH: // SPATH
      case symbol_kind::S_PATH_END: // PATH_END
        value.copy< path > (that.value);
        break;

      case symbol_kind::S_STR: // STR
      case symbol_kind::S_IND_STR: // IND_STR
        value.copy< str > (that.value);
        break;

      case symbol_kind::S_string_parts_interpolated: // string_parts_interpolated
        value.copy< string_parts > (that.value);
        break;

      case symbol_kind::S_URI: // URI
        value.copy< uri > (that.value);
        break;

      default:
        break;
    }
;
        location = that.location;
        return *this;
      }

      /// Move assignment.
      basic_symbol& operator= (basic_symbol&& that)
      {
        Base::operator= (std::move (that));
        switch (this->kind ())
    {
      case symbol_kind::S_attrs: // attrs
      case symbol_kind::S_attrpath: // attrpath
        value.move< attrNames > (std::move (that.value));
        break;

      case symbol_kind::S_binds: // binds
        value.move< attrs > (std::move (that.value));
        break;

      case symbol_kind::S_start: // start
      case symbol_kind::S_expr: // expr
      case symbol_kind::S_expr_function: // expr_function
      case symbol_kind::S_expr_if: // expr_if
      case symbol_kind::S_expr_op: // expr_op
      case symbol_kind::S_expr_app: // expr_app
      case symbol_kind::S_expr_select: // expr_select
      case symbol_kind::S_expr_simple: // expr_simple
      case symbol_kind::S_string_parts: // string_parts
      case symbol_kind::S_path_start: // path_start
      case symbol_kind::S_string_attr: // string_attr
        value.move< e > (std::move (that.value));
        break;

      case symbol_kind::S_formal: // formal
        value.move< formal > (std::move (that.value));
        break;

      case symbol_kind::S_formals: // formals
        value.move< formals > (std::move (that.value));
        break;

      case symbol_kind::S_ID: // ID
      case symbol_kind::S_ATTRPATH: // ATTRPATH
      case symbol_kind::S_attr: // attr
        value.move< id > (std::move (that.value));
        break;

      case symbol_kind::S_ind_string_parts: // ind_string_parts
        value.move< ind_string_parts > (std::move (that.value));
        break;

      case symbol_kind::S_expr_list: // expr_list
        value.move< list > (std::move (that.value));
        break;

      case symbol_kind::S_INT: // INT
        value.move< n > (std::move (that.value));
        break;

      case symbol_kind::S_FLOAT: // FLOAT
        value.move< nf > (std::move (that.value));
        break;

      case symbol_kind::S_PATH: // PATH
      case symbol_kind::S_HPATH: // HPATH
      case symbol_kind::S_SPATH: // SPATH
      case symbol_kind::S_PATH_END: // PATH_END
        value.move< path > (std::move (that.value));
        break;

      case symbol_kind::S_STR: // STR
      case symbol_kind::S_IND_STR: // IND_STR
        value.move< str > (std::move (that.value));
        break;

      case symbol_kind::S_string_parts_interpolated: // string_parts_interpolated
        value.move< string_parts > (std::move (that.value));
        break;

      case symbol_kind::S_URI: // URI
        value.move< uri > (std::move (that.value));
        break;

      default:
        break;
    }
;
        location = std::move (that.location);
        return *this;
      }


      /// Destroy contents, and record that is empty.
      void clear () YY_NOEXCEPT
      {
        // User destructor.
        symbol_kind_type yykind = this->kind ();
        basic_symbol<Base>& yysym = *this;
        (void) yysym;
        switch (yykind)
        {
       default:
          break;
        }

        // Value type destructor.
switch (yykind)
    {
      case symbol_kind::S_attrs: // attrs
      case symbol_kind::S_attrpath: // attrpath
        value.template destroy< attrNames > ();
        break;

      case symbol_kind::S_binds: // binds
        value.template destroy< attrs > ();
        break;

      case symbol_kind::S_start: // start
      case symbol_kind::S_expr: // expr
      case symbol_kind::S_expr_function: // expr_function
      case symbol_kind::S_expr_if: // expr_if
      case symbol_kind::S_expr_op: // expr_op
      case symbol_kind::S_expr_app: // expr_app
      case symbol_kind::S_expr_select: // expr_select
      case symbol_kind::S_expr_simple: // expr_simple
      case symbol_kind::S_string_parts: // string_parts
      case symbol_kind::S_path_start: // path_start
      case symbol_kind::S_string_attr: // string_attr
        value.template destroy< e > ();
        break;

      case symbol_kind::S_formal: // formal
        value.template destroy< formal > ();
        break;

      case symbol_kind::S_formals: // formals
        value.template destroy< formals > ();
        break;

      case symbol_kind::S_ID: // ID
      case symbol_kind::S_ATTRPATH: // ATTRPATH
      case symbol_kind::S_attr: // attr
        value.template destroy< id > ();
        break;

      case symbol_kind::S_ind_string_parts: // ind_string_parts
        value.template destroy< ind_string_parts > ();
        break;

      case symbol_kind::S_expr_list: // expr_list
        value.template destroy< list > ();
        break;

      case symbol_kind::S_INT: // INT
        value.template destroy< n > ();
        break;

      case symbol_kind::S_FLOAT: // FLOAT
        value.template destroy< nf > ();
        break;

      case symbol_kind::S_PATH: // PATH
      case symbol_kind::S_HPATH: // HPATH
      case symbol_kind::S_SPATH: // SPATH
      case symbol_kind::S_PATH_END: // PATH_END
        value.template destroy< path > ();
        break;

      case symbol_kind::S_STR: // STR
      case symbol_kind::S_IND_STR: // IND_STR
        value.template destroy< str > ();
        break;

      case symbol_kind::S_string_parts_interpolated: // string_parts_interpolated
        value.template destroy< string_parts > ();
        break;

      case symbol_kind::S_URI: // URI
        value.template destroy< uri > ();
        break;

      default:
        break;
    }

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
    {
      /// Superclass.
      typedef basic_symbol<by_kind> super_type;

      /// Empty symbol.
      symbol_type () YY_NOEXCEPT {}

      /// Constructor for valueless symbols, and symbols from each type.
#if 201103L <= YY_CPLUSPLUS
      symbol_type (int tok, location_type l)
        : super_type (token_kind_type (tok), std::move (l))
#else
      symbol_type (int tok, const location_type& l)
        : super_type (token_kind_type (tok), l)
#endif
      {}
#if 201103L <= YY_CPLUSPLUS
      symbol_type (int tok, id v, location_type l)
        : super_type (token_kind_type (tok), std::move (v), std::move (l))
#else
      symbol_type (int tok, const id& v, const location_type& l)
        : super_type (token_kind_type (tok), v, l)
#endif
      {}
#if 201103L <= YY_CPLUSPLUS
      symbol_type (int tok, n v, location_type l)
        : super_type (token_kind_type (tok), std::move (v), std::move (l))
#else
      symbol_type (int tok, const n& v, const location_type& l)
        : super_type (token_kind_type (tok), v, l)
#endif
      {}
#if 201103L <= YY_CPLUSPLUS
      symbol_type (int tok, nf v, location_type l)
        : super_type (token_kind_type (tok), std::move (v), std::move (l))
#else
      symbol_type (int tok, const nf& v, const location_type& l)
        : super_type (token_kind_type (tok), v, l)
#endif
      {}
#if 201103L <= YY_CPLUSPLUS
      symbol_type (int tok, path v, location_type l)
        : super_type (token_kind_type (tok), std::move (v), std::move (l))
#else
      symbol_type (int tok, const path& v, const location_type& l)
        : super_type (token_kind_type (tok), v, l)
#endif
      {}
#if 201103L <= YY_CPLUSPLUS
      symbol_type (int tok, str v, location_type l)
        : super_type (token_kind_type (tok), std::move (v), std::move (l))
#else
      symbol_type (int tok, const str& v, const location_type& l)
        : super_type (token_kind_type (tok), v, l)
#endif
      {}
#if 201103L <= YY_CPLUSPLUS
      symbol_type (int tok, uri v, location_type l)
        : super_type (token_kind_type (tok), std::move (v), std::move (l))
#else
      symbol_type (int tok, const uri& v, const location_type& l)
        : super_type (token_kind_type (tok), v, l)
#endif
      {}
    };


    // FIXME: should be private eventually.
    class glr_stack;
    class glr_state;

    /// Build a parser object.
    parser (void * scanner_yyarg, ParseData * data_yyarg);
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

    // Implementation of make_symbol for each token kind.
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_YYEOF (location_type l)
      {
        return symbol_type (token::YYEOF, std::move (l));
      }
#else
      static
      symbol_type
      make_YYEOF (const location_type& l)
      {
        return symbol_type (token::YYEOF, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_YYerror (location_type l)
      {
        return symbol_type (token::YYerror, std::move (l));
      }
#else
      static
      symbol_type
      make_YYerror (const location_type& l)
      {
        return symbol_type (token::YYerror, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_YYUNDEF (location_type l)
      {
        return symbol_type (token::YYUNDEF, std::move (l));
      }
#else
      static
      symbol_type
      make_YYUNDEF (const location_type& l)
      {
        return symbol_type (token::YYUNDEF, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_ID (id v, location_type l)
      {
        return symbol_type (token::ID, std::move (v), std::move (l));
      }
#else
      static
      symbol_type
      make_ID (const id& v, const location_type& l)
      {
        return symbol_type (token::ID, v, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_ATTRPATH (id v, location_type l)
      {
        return symbol_type (token::ATTRPATH, std::move (v), std::move (l));
      }
#else
      static
      symbol_type
      make_ATTRPATH (const id& v, const location_type& l)
      {
        return symbol_type (token::ATTRPATH, v, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_STR (str v, location_type l)
      {
        return symbol_type (token::STR, std::move (v), std::move (l));
      }
#else
      static
      symbol_type
      make_STR (const str& v, const location_type& l)
      {
        return symbol_type (token::STR, v, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_IND_STR (str v, location_type l)
      {
        return symbol_type (token::IND_STR, std::move (v), std::move (l));
      }
#else
      static
      symbol_type
      make_IND_STR (const str& v, const location_type& l)
      {
        return symbol_type (token::IND_STR, v, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_INT (n v, location_type l)
      {
        return symbol_type (token::INT, std::move (v), std::move (l));
      }
#else
      static
      symbol_type
      make_INT (const n& v, const location_type& l)
      {
        return symbol_type (token::INT, v, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_FLOAT (nf v, location_type l)
      {
        return symbol_type (token::FLOAT, std::move (v), std::move (l));
      }
#else
      static
      symbol_type
      make_FLOAT (const nf& v, const location_type& l)
      {
        return symbol_type (token::FLOAT, v, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_PATH (path v, location_type l)
      {
        return symbol_type (token::PATH, std::move (v), std::move (l));
      }
#else
      static
      symbol_type
      make_PATH (const path& v, const location_type& l)
      {
        return symbol_type (token::PATH, v, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_HPATH (path v, location_type l)
      {
        return symbol_type (token::HPATH, std::move (v), std::move (l));
      }
#else
      static
      symbol_type
      make_HPATH (const path& v, const location_type& l)
      {
        return symbol_type (token::HPATH, v, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_SPATH (path v, location_type l)
      {
        return symbol_type (token::SPATH, std::move (v), std::move (l));
      }
#else
      static
      symbol_type
      make_SPATH (const path& v, const location_type& l)
      {
        return symbol_type (token::SPATH, v, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_PATH_END (path v, location_type l)
      {
        return symbol_type (token::PATH_END, std::move (v), std::move (l));
      }
#else
      static
      symbol_type
      make_PATH_END (const path& v, const location_type& l)
      {
        return symbol_type (token::PATH_END, v, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_URI (uri v, location_type l)
      {
        return symbol_type (token::URI, std::move (v), std::move (l));
      }
#else
      static
      symbol_type
      make_URI (const uri& v, const location_type& l)
      {
        return symbol_type (token::URI, v, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_IF (location_type l)
      {
        return symbol_type (token::IF, std::move (l));
      }
#else
      static
      symbol_type
      make_IF (const location_type& l)
      {
        return symbol_type (token::IF, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_THEN (location_type l)
      {
        return symbol_type (token::THEN, std::move (l));
      }
#else
      static
      symbol_type
      make_THEN (const location_type& l)
      {
        return symbol_type (token::THEN, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_ELSE (location_type l)
      {
        return symbol_type (token::ELSE, std::move (l));
      }
#else
      static
      symbol_type
      make_ELSE (const location_type& l)
      {
        return symbol_type (token::ELSE, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_ASSERT (location_type l)
      {
        return symbol_type (token::ASSERT, std::move (l));
      }
#else
      static
      symbol_type
      make_ASSERT (const location_type& l)
      {
        return symbol_type (token::ASSERT, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_WITH (location_type l)
      {
        return symbol_type (token::WITH, std::move (l));
      }
#else
      static
      symbol_type
      make_WITH (const location_type& l)
      {
        return symbol_type (token::WITH, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_LET (location_type l)
      {
        return symbol_type (token::LET, std::move (l));
      }
#else
      static
      symbol_type
      make_LET (const location_type& l)
      {
        return symbol_type (token::LET, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_IN (location_type l)
      {
        return symbol_type (token::IN, std::move (l));
      }
#else
      static
      symbol_type
      make_IN (const location_type& l)
      {
        return symbol_type (token::IN, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_REC (location_type l)
      {
        return symbol_type (token::REC, std::move (l));
      }
#else
      static
      symbol_type
      make_REC (const location_type& l)
      {
        return symbol_type (token::REC, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_INHERIT (location_type l)
      {
        return symbol_type (token::INHERIT, std::move (l));
      }
#else
      static
      symbol_type
      make_INHERIT (const location_type& l)
      {
        return symbol_type (token::INHERIT, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_EQ (location_type l)
      {
        return symbol_type (token::EQ, std::move (l));
      }
#else
      static
      symbol_type
      make_EQ (const location_type& l)
      {
        return symbol_type (token::EQ, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_NEQ (location_type l)
      {
        return symbol_type (token::NEQ, std::move (l));
      }
#else
      static
      symbol_type
      make_NEQ (const location_type& l)
      {
        return symbol_type (token::NEQ, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_AND (location_type l)
      {
        return symbol_type (token::AND, std::move (l));
      }
#else
      static
      symbol_type
      make_AND (const location_type& l)
      {
        return symbol_type (token::AND, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_OR (location_type l)
      {
        return symbol_type (token::OR, std::move (l));
      }
#else
      static
      symbol_type
      make_OR (const location_type& l)
      {
        return symbol_type (token::OR, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_IMPL (location_type l)
      {
        return symbol_type (token::IMPL, std::move (l));
      }
#else
      static
      symbol_type
      make_IMPL (const location_type& l)
      {
        return symbol_type (token::IMPL, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_OR_KW (location_type l)
      {
        return symbol_type (token::OR_KW, std::move (l));
      }
#else
      static
      symbol_type
      make_OR_KW (const location_type& l)
      {
        return symbol_type (token::OR_KW, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_DOLLAR_CURLY (location_type l)
      {
        return symbol_type (token::DOLLAR_CURLY, std::move (l));
      }
#else
      static
      symbol_type
      make_DOLLAR_CURLY (const location_type& l)
      {
        return symbol_type (token::DOLLAR_CURLY, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_IND_STRING_OPEN (location_type l)
      {
        return symbol_type (token::IND_STRING_OPEN, std::move (l));
      }
#else
      static
      symbol_type
      make_IND_STRING_OPEN (const location_type& l)
      {
        return symbol_type (token::IND_STRING_OPEN, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_IND_STRING_CLOSE (location_type l)
      {
        return symbol_type (token::IND_STRING_CLOSE, std::move (l));
      }
#else
      static
      symbol_type
      make_IND_STRING_CLOSE (const location_type& l)
      {
        return symbol_type (token::IND_STRING_CLOSE, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_ELLIPSIS (location_type l)
      {
        return symbol_type (token::ELLIPSIS, std::move (l));
      }
#else
      static
      symbol_type
      make_ELLIPSIS (const location_type& l)
      {
        return symbol_type (token::ELLIPSIS, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_LEQ (location_type l)
      {
        return symbol_type (token::LEQ, std::move (l));
      }
#else
      static
      symbol_type
      make_LEQ (const location_type& l)
      {
        return symbol_type (token::LEQ, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_GEQ (location_type l)
      {
        return symbol_type (token::GEQ, std::move (l));
      }
#else
      static
      symbol_type
      make_GEQ (const location_type& l)
      {
        return symbol_type (token::GEQ, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_UPDATE (location_type l)
      {
        return symbol_type (token::UPDATE, std::move (l));
      }
#else
      static
      symbol_type
      make_UPDATE (const location_type& l)
      {
        return symbol_type (token::UPDATE, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_NOT (location_type l)
      {
        return symbol_type (token::NOT, std::move (l));
      }
#else
      static
      symbol_type
      make_NOT (const location_type& l)
      {
        return symbol_type (token::NOT, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_CONCAT (location_type l)
      {
        return symbol_type (token::CONCAT, std::move (l));
      }
#else
      static
      symbol_type
      make_CONCAT (const location_type& l)
      {
        return symbol_type (token::CONCAT, l);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_NEGATE (location_type l)
      {
        return symbol_type (token::NEGATE, std::move (l));
      }
#else
      static
      symbol_type
      make_NEGATE (const location_type& l)
      {
        return symbol_type (token::NEGATE, l);
      }
#endif


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
    ParseData * data;
    // Needs access to yy_destroy_, report_syntax_error, etc.
    friend glr_stack;
  };


} // yy
#line 2025 "src/parsers/nix/parser-tab.hh"




#endif // !YY_YY_SRC_PARSERS_NIX_PARSER_TAB_HH_INCLUDED
