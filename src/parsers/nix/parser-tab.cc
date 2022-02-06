// A Bison parser, made by GNU Bison 3.8.2.

// Skeleton implementation for Bison GLR parsers in C

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

/* Identify Bison output, and Bison version.  */
#define YYBISON 30802

/* Bison version string.  */
#define YYBISON_VERSION "3.8.2"

/* Skeleton name.  */
#define YYSKELETON_NAME "glr2.cc"





// First part of user prologue.
#line 64 "src/parsers/nix/parser.y"


#include "parser-tab.hh"
#include "lexer-tab.hh"

YY_DECL;

using namespace nix;


namespace nix {


static void dupAttr(const AttrPath & attrPath, const Pos & pos, const Pos & prevPos)
{
    throw ParseError({
         .msg = hintfmt("attribute '%1%' already defined at %2%",
             showAttrPath(attrPath), prevPos),
         .errPos = pos
    });
}

static void dupAttr(Symbol attr, const Pos & pos, const Pos & prevPos)
{
    throw ParseError({
        .msg = hintfmt("attribute '%1%' already defined at %2%", attr, prevPos),
        .errPos = pos
    });
}


static void addAttr(ExprAttrs * attrs, AttrPath & attrPath,
    Expr * e, const Pos & pos)
{
    AttrPath::iterator i;
    // All attrpaths have at least one attr
    assert(!attrPath.empty());
    // Checking attrPath validity.
    // ===========================
    for (i = attrPath.begin(); i + 1 < attrPath.end(); i++) {
        if (i->symbol.set()) {
            ExprAttrs::AttrDefs::iterator j = attrs->attrs.find(i->symbol);
            if (j != attrs->attrs.end()) {
                if (!j->second.inherited) {
                    ExprAttrs * attrs2 = dynamic_cast<ExprAttrs *>(j->second.e);
                    if (!attrs2) dupAttr(attrPath, pos, j->second.pos);
                    attrs = attrs2;
                } else
                    dupAttr(attrPath, pos, j->second.pos);
            } else {
                ExprAttrs * nested = new ExprAttrs;
                attrs->attrs[i->symbol] = ExprAttrs::AttrDef(nested, pos);
                attrs = nested;
            }
        } else {
            ExprAttrs *nested = new ExprAttrs;
            attrs->dynamicAttrs.push_back(ExprAttrs::DynamicAttrDef(i->expr, nested, pos));
            attrs = nested;
        }
    }
    // Expr insertion.
    // ==========================
    if (i->symbol.set()) {
        ExprAttrs::AttrDefs::iterator j = attrs->attrs.find(i->symbol);
        if (j != attrs->attrs.end()) {
            // This attr path is already defined. However, if both
            // e and the expr pointed by the attr path are two attribute sets,
            // we want to merge them.
            // Otherwise, throw an error.
            auto ae = dynamic_cast<ExprAttrs *>(e);
            auto jAttrs = dynamic_cast<ExprAttrs *>(j->second.e);
            if (jAttrs && ae) {
                for (auto & ad : ae->attrs) {
                    auto j2 = jAttrs->attrs.find(ad.first);
                    if (j2 != jAttrs->attrs.end()) // Attr already defined in iAttrs, error.
                        dupAttr(ad.first, j2->second.pos, ad.second.pos);
                    jAttrs->attrs.emplace(ad.first, ad.second);
                }
            } else {
                dupAttr(attrPath, pos, j->second.pos);
            }
        } else {
            // This attr path is not defined. Let's create it.
            attrs->attrs.emplace(i->symbol, ExprAttrs::AttrDef(e, pos));
            e->setName(i->symbol);
        }
    } else {
        attrs->dynamicAttrs.push_back(ExprAttrs::DynamicAttrDef(i->expr, e, pos));
    }
}


static Formals * toFormals(ParseData & data, ParserFormals * formals,
    Pos pos = noPos, Symbol arg = {})
{
    std::sort(formals->formals.begin(), formals->formals.end(),
        [] (const auto & a, const auto & b) {
            return std::tie(a.name, a.pos) < std::tie(b.name, b.pos);
        });

    std::optional<std::pair<Symbol, Pos>> duplicate;
    for (size_t i = 0; i + 1 < formals->formals.size(); i++) {
        if (formals->formals[i].name != formals->formals[i + 1].name)
            continue;
        std::pair thisDup{formals->formals[i].name, formals->formals[i + 1].pos};
        duplicate = std::min(thisDup, duplicate.value_or(thisDup));
    }
    if (duplicate)
        throw ParseError({
            .msg = hintfmt("duplicate formal function argument '%1%'", duplicate->first),
            .errPos = duplicate->second
        });

    Formals result;
    result.ellipsis = formals->ellipsis;
    result.formals = std::move(formals->formals);

    if (arg.set() && result.has(arg))
        throw ParseError({
            .msg = hintfmt("duplicate formal function argument '%1%'", arg),
            .errPos = pos
        });

    delete formals;
    return new Formals(std::move(result));
}


static Expr * stripIndentation(const Pos & pos, SymbolTable & symbols,
    vector<std::pair<Pos, std::variant<Expr *, StringToken> > > & es)
{
    if (es.empty()) return new ExprString("");

    /* Figure out the minimum indentation.  Note that by design
       whitespace-only final lines are not taken into account.  (So
       the " " in "\n ''" is ignored, but the " " in "\n foo''" is.) */
    bool atStartOfLine = true; /* = seen only whitespace in the current line */
    size_t minIndent = 1000000;
    size_t curIndent = 0;
    for (auto & [i_pos, i] : es) {
        auto * str = std::get_if<StringToken>(&i);
        if (!str || !str->hasIndentation) {
            /* Anti-quotations and escaped characters end the current start-of-line whitespace. */
            if (atStartOfLine) {
                atStartOfLine = false;
                if (curIndent < minIndent) minIndent = curIndent;
            }
            continue;
        }
        for (size_t j = 0; j < str->l; ++j) {
            if (atStartOfLine) {
                if (str->p[j] == ' ')
                    curIndent++;
                else if (str->p[j] == '\n') {
                    /* Empty line, doesn't influence minimum
                       indentation. */
                    curIndent = 0;
                } else {
                    atStartOfLine = false;
                    if (curIndent < minIndent) minIndent = curIndent;
                }
            } else if (str->p[j] == '\n') {
                atStartOfLine = true;
                curIndent = 0;
            }
        }
    }

    /* Strip spaces from each line. */
    vector<std::pair<Pos, Expr *> > * es2 = new vector<std::pair<Pos, Expr *> >;
    atStartOfLine = true;
    size_t curDropped = 0;
    size_t n = es.size();
    auto i = es.begin();
    const auto trimExpr = [&] (Expr * e) {
        atStartOfLine = false;
        curDropped = 0;
        es2->emplace_back(i->first, e);
    };
    const auto trimString = [&] (const StringToken & t) {
        string s2;
        for (size_t j = 0; j < t.l; ++j) {
            if (atStartOfLine) {
                if (t.p[j] == ' ') {
                    if (curDropped++ >= minIndent)
                        s2 += t.p[j];
                }
                else if (t.p[j] == '\n') {
                    curDropped = 0;
                    s2 += t.p[j];
                } else {
                    atStartOfLine = false;
                    curDropped = 0;
                    s2 += t.p[j];
                }
            } else {
                s2 += t.p[j];
                if (t.p[j] == '\n') atStartOfLine = true;
            }
        }

        /* Remove the last line if it is empty and consists only of
           spaces. */
        if (n == 1) {
            string::size_type p = s2.find_last_of('\n');
            if (p != string::npos && s2.find_first_not_of(' ', p + 1) == string::npos)
                s2 = string(s2, 0, p + 1);
        }

        es2->emplace_back(i->first, new ExprString(s2));
    };
    for (; i != es.end(); ++i, --n) {
        std::visit(overloaded { trimExpr, trimString }, i->second);
    }

    /* If this is a single string, then don't do a concatenation. */
    return es2->size() == 1 && dynamic_cast<ExprString *>((*es2)[0].second) ? (*es2)[0].second : new ExprConcatStrings(pos, true, es2);
}


static inline Pos makeCurPos(const YYLTYPE & loc, ParseData * data)
{
    return Pos(data->origin, data->file, loc.first_line, loc.first_column);
}

#define CUR_POS makeCurPos(*yylocp, data)


}


void yyerror(YYLTYPE * loc, yyscan_t scanner, ParseData * data, const char * error)
{
    data->error = {
        .msg = hintfmt(error),
        .errPos = makeCurPos(*loc, data)
    };
}



#line 296 "src/parsers/nix/parser-tab.cc"


# ifndef YY_NULLPTR
#  if defined __cplusplus
#   if 201103L <= __cplusplus
#    define YY_NULLPTR nullptr
#   else
#    define YY_NULLPTR 0
#   endif
#  else
#   define YY_NULLPTR ((void*)0)
#  endif
# endif

#include "parser-tab.hh"

namespace
{
  /* Default (constant) value used for initialization for null
     right-hand sides.  Unlike the standard yacc.c template, here we set
     the default value of $$ to a zeroed-out value.  Since the default
     value is undefined, this behavior is technically correct.  */
  yy::parser::value_type yyval_default;
}




#include <cstdio>
#include <cstdlib>

#ifndef YY_
# if defined YYENABLE_NLS && YYENABLE_NLS
#  if ENABLE_NLS
#   include <libintl.h> /* INFRINGES ON USER NAME SPACE */
#   define YY_(Msgid) dgettext ("bison-runtime", Msgid)
#  endif
# endif
# ifndef YY_
#  define YY_(Msgid) Msgid
# endif
#endif

// Whether we are compiled with exception support.
#ifndef YY_EXCEPTIONS
# if defined __GNUC__ && !defined __EXCEPTIONS
#  define YY_EXCEPTIONS 0
# else
#  define YY_EXCEPTIONS 1
# endif
#endif

#ifndef YYFREE
# define YYFREE free
#endif
#ifndef YYMALLOC
# define YYMALLOC malloc
#endif

#ifndef YYSETJMP
# include <setjmp.h>
# define YYJMP_BUF jmp_buf
# define YYSETJMP(Env) setjmp (Env)
/* Pacify Clang and ICC.  */
# define YYLONGJMP(Env, Val)                    \
 do {                                           \
   longjmp (Env, Val);                          \
   YYASSERT (0);                                \
 } while (false)
#endif

#ifndef YY_ATTRIBUTE_PURE
# if defined __GNUC__ && 2 < __GNUC__ + (96 <= __GNUC_MINOR__)
#  define YY_ATTRIBUTE_PURE __attribute__ ((__pure__))
# else
#  define YY_ATTRIBUTE_PURE
# endif
#endif

#ifndef YY_ATTRIBUTE_UNUSED
# if defined __GNUC__ && 2 < __GNUC__ + (7 <= __GNUC_MINOR__)
#  define YY_ATTRIBUTE_UNUSED __attribute__ ((__unused__))
# else
#  define YY_ATTRIBUTE_UNUSED
# endif
#endif

/* The _Noreturn keyword of C11.  */
#ifndef _Noreturn
# if (defined __cplusplus \
      && ((201103 <= __cplusplus && !(__GNUC__ == 4 && __GNUC_MINOR__ == 7)) \
          || (defined _MSC_VER && 1900 <= _MSC_VER)))
#  define _Noreturn [[noreturn]]
# elif ((!defined __cplusplus || defined __clang__) \
        && (201112 <= (defined __STDC_VERSION__ ? __STDC_VERSION__ : 0) \
            || (!defined __STRICT_ANSI__ \
                && (4 < __GNUC__ + (7 <= __GNUC_MINOR__) \
                    || (defined __apple_build_version__ \
                        ? 6000000 <= __apple_build_version__ \
                        : 3 < __clang_major__ + (5 <= __clang_minor__))))))
   /* _Noreturn works as-is.  */
# elif (2 < __GNUC__ + (8 <= __GNUC_MINOR__) || defined __clang__ \
        || 0x5110 <= __SUNPRO_C)
#  define _Noreturn __attribute__ ((__noreturn__))
# elif 1200 <= (defined _MSC_VER ? _MSC_VER : 0)
#  define _Noreturn __declspec (noreturn)
# else
#  define _Noreturn
# endif
#endif

/* Suppress unused-variable warnings by "using" E.  */
#if ! defined lint || defined __GNUC__
# define YY_USE(E) ((void) (E))
#else
# define YY_USE(E) /* empty */
#endif

/* Suppress an incorrect diagnostic about yylval being uninitialized.  */
#if defined __GNUC__ && ! defined __ICC && 406 <= __GNUC__ * 100 + __GNUC_MINOR__
# if __GNUC__ * 100 + __GNUC_MINOR__ < 407
#  define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN                           \
    _Pragma ("GCC diagnostic push")                                     \
    _Pragma ("GCC diagnostic ignored \"-Wuninitialized\"")
# else
#  define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN                           \
    _Pragma ("GCC diagnostic push")                                     \
    _Pragma ("GCC diagnostic ignored \"-Wuninitialized\"")              \
    _Pragma ("GCC diagnostic ignored \"-Wmaybe-uninitialized\"")
# endif
# define YY_IGNORE_MAYBE_UNINITIALIZED_END      \
    _Pragma ("GCC diagnostic pop")
#else
# define YY_INITIAL_VALUE(Value) Value
#endif
#ifndef YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_END
#endif
#ifndef YY_INITIAL_VALUE
# define YY_INITIAL_VALUE(Value) /* Nothing. */
#endif

#if defined __cplusplus && defined __GNUC__ && ! defined __ICC && 6 <= __GNUC__
# define YY_IGNORE_USELESS_CAST_BEGIN                          \
    _Pragma ("GCC diagnostic push")                            \
    _Pragma ("GCC diagnostic ignored \"-Wuseless-cast\"")
# define YY_IGNORE_USELESS_CAST_END            \
    _Pragma ("GCC diagnostic pop")
#endif
#ifndef YY_IGNORE_USELESS_CAST_BEGIN
# define YY_IGNORE_USELESS_CAST_BEGIN
# define YY_IGNORE_USELESS_CAST_END
#endif


#if defined __GNUC__ && ! defined __ICC && 6 <= __GNUC__
# define YY_IGNORE_NULL_DEREFERENCE_BEGIN                               \
  _Pragma ("GCC diagnostic push")                                       \
  _Pragma ("GCC diagnostic ignored \"-Wnull-dereference\"")
# define YY_IGNORE_NULL_DEREFERENCE_END         \
  _Pragma ("GCC diagnostic pop")
#else
# define YY_IGNORE_NULL_DEREFERENCE_BEGIN
# define YY_IGNORE_NULL_DEREFERENCE_END
#endif

# ifndef YY_NULLPTR
#  if defined __cplusplus
#   if 201103L <= __cplusplus
#    define YY_NULLPTR nullptr
#   else
#    define YY_NULLPTR 0
#   endif
#  else
#   define YY_NULLPTR ((void*)0)
#  endif
# endif
# ifndef YY_CAST
#  ifdef __cplusplus
#   define YY_CAST(Type, Val) static_cast<Type> (Val)
#   define YY_REINTERPRET_CAST(Type, Val) reinterpret_cast<Type> (Val)
#  else
#   define YY_CAST(Type, Val) ((Type) (Val))
#   define YY_REINTERPRET_CAST(Type, Val) ((Type) (Val))
#  endif
# endif

// FIXME: Use the same conventions as lalr1.cc.

#ifndef YYASSERT
# define YYASSERT(Condition) ((void) ((Condition) || (abort (), 0)))
#endif

#ifdef YYDEBUG
# define YYDASSERT(Condition) YYASSERT(Condition)
#else
# define YYDASSERT(Condition)
#endif

/* YYFINAL -- State number of the termination state.  */
#define YYFINAL  54
/* YYLAST -- Last index in YYTABLE.  */
#define YYLAST   339

/* YYNTOKENS -- Number of terminals.  */
#define YYNTOKENS  60
/* YYNNTS -- Number of nonterminals.  */
#define YYNNTS  21
/* YYNRULES -- Number of rules.  */
#define YYNRULES  87
/* YYNSTATES -- Number of states.  */
#define YYNSTATES  174
/* YYMAXRHS -- Maximum number of symbols on right-hand side of rule.  */
#define YYMAXRHS 7
/* YYMAXLEFT -- Maximum number of symbols to the left of a handle
   accessed by $0, $-1, etc., in any rule.  */
#define YYMAXLEFT 0

namespace
{
#if YYDEBUG
  /* YYRLINE[YYN] -- source line where rule number YYN was defined.  */
  const short yyrline[] =
  {
       0,   361,   361,   363,   366,   368,   370,   375,   380,   382,
     384,   392,   396,   397,   401,   402,   403,   404,   405,   406,
     407,   408,   409,   410,   411,   412,   413,   414,   416,   417,
     418,   419,   420,   424,   431,   435,   437,   441,   443,   447,
     454,   455,   456,   457,   460,   461,   465,   472,   481,   484,
     486,   488,   490,   494,   495,   496,   500,   502,   503,   504,
     512,   519,   526,   527,   528,   532,   533,   542,   551,   555,
     556,   568,   572,   573,   582,   583,   595,   596,   600,   601,
     605,   606,   610,   612,   615,   616,   621,   622
  };
#endif

#define YYPACT_NINF -129
#define YYTABLE_NINF -69

  // YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
// STATE-NUM.
  const short yypact[] =
  {
     137,   -35,  -129,  -129,  -129,  -129,  -129,  -129,   137,   137,
     137,   -41,   -32,  -129,   162,    35,   162,     8,   137,  -129,
      29,  -129,  -129,  -129,   213,   214,  -129,   -19,   120,   137,
     -23,    59,    41,    66,  -129,    20,  -129,    91,  -129,   -41,
    -129,  -129,    73,  -129,    22,    28,    62,   247,    94,   137,
      80,    16,    82,     9,  -129,   162,   162,   162,   162,   162,
     162,   162,   162,   162,   162,   162,   162,   162,   162,   162,
      61,  -129,  -129,    61,    94,  -129,   174,  -129,     4,   137,
     137,   137,    24,  -129,   137,    84,  -129,   137,     8,    26,
    -129,  -129,    65,  -129,   137,  -129,   137,  -129,    81,     4,
     137,    88,  -129,  -129,   137,  -129,  -129,  -129,   261,   261,
     283,   239,   213,   295,   295,   295,   295,   157,   163,   163,
      37,    37,    37,    90,    67,  -129,   104,   146,  -129,  -129,
    -129,  -129,   137,    32,   105,   110,    61,   137,  -129,   116,
    -129,   137,   165,  -129,   118,  -129,   126,   214,   130,   137,
     125,  -129,  -129,  -129,  -129,  -129,  -129,  -129,   128,  -129,
    -129,   141,  -129,  -129,  -129,   137,  -129,  -129,  -129,   137,
    -129,   132,  -129,  -129
  };

  // YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
// Performed when YYTABLE does not specify something else to do.  Zero
// means the default is an error.
  const signed char yydefact[] =
  {
       0,    39,    40,    41,    60,    61,    46,    47,     0,     0,
       0,    68,     0,    64,     0,    68,     0,    55,     0,    81,
       0,     2,     3,    11,    13,    32,    34,    38,     0,     0,
       0,     0,     0,     0,    68,     0,    68,     0,    39,     0,
      68,    15,    86,    85,     0,     0,    83,    14,    53,     0,
       0,    54,     0,     0,     1,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,    33,    37,     0,     0,    44,     0,     4,    84,     0,
       0,     0,     0,    76,     0,    71,    77,     0,    55,     0,
      74,    75,     0,    62,     0,    43,     0,    51,     0,    84,
       0,     0,    42,    56,     0,    48,    52,    80,    16,    17,
      22,    23,    24,    18,    20,    19,    21,    25,    27,    28,
      29,    30,    31,    26,    35,    45,     0,     0,     8,     9,
      49,    10,     0,     0,     0,     0,     0,     0,    50,     0,
      87,     0,     0,    82,     0,    58,     0,     0,     0,     0,
       0,    66,    69,    70,    79,    78,    72,    73,     0,    63,
       5,     0,    59,    57,    36,     0,    12,    71,    65,     0,
       7,     0,     6,    67
  };

  // YYPGOTO[NTERM-NUM].
  const short yypgoto[] =
  {
    -129,  -129,    -8,   -26,  -129,    43,  -129,   -21,  -129,   101,
     167,  -129,  -129,   123,    44,    58,  -128,  -102,  -129,   -52,
    -129
  };

  // YYDEFGOTO[NTERM-NUM].
  const unsigned char yydefgoto[] =
  {
       0,    20,    21,    22,    23,    24,    25,    26,    27,    50,
      51,    28,    37,    44,   133,    89,    90,    91,    53,    45,
      46
  };

  // YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
// positive, shift that token.  If negative, reduce the rule whose
// number is the opposite.  If YYTABLE_NINF, syntax error.
  const short yytable[] =
  {
      31,    32,    33,    77,    71,   152,    34,    42,   156,    72,
      52,    29,    38,    48,    30,    36,     2,     3,     4,     5,
       6,   103,     7,    83,    78,    83,   126,    83,    39,    54,
      12,   153,   107,    73,   157,    83,    43,    49,    42,    13,
      84,   101,    85,   152,    85,   104,    85,   143,    86,    87,
      86,    87,    86,    87,   128,   129,    40,    41,   131,    47,
      86,    87,    17,    18,    83,    19,   106,    43,    83,   153,
      97,   127,   130,    88,    79,    88,    98,    88,   136,   134,
      69,    70,   151,   -68,   137,    88,   139,    85,   140,    86,
      87,    80,   144,    86,    87,   147,   146,    93,   108,   109,
     110,   111,   112,   113,   114,   115,   116,   117,   118,   119,
     120,   121,   122,   138,    88,   160,    81,    96,    88,   136,
      94,    99,    95,   100,   150,    74,   164,   141,   123,   158,
     142,   124,    75,   102,    35,    83,   145,   105,   132,   170,
       1,   166,   136,   172,     2,     3,     4,     5,     6,    49,
       7,     8,   148,   154,     9,    10,    11,    82,    12,    92,
      86,    87,   149,   155,   159,    38,   162,    13,   161,     2,
       3,     4,     5,     6,   163,     7,   165,    14,   168,   103,
     167,    39,   173,    12,    15,    88,   125,   169,    16,   135,
      17,    18,    13,    19,    64,    76,    65,    66,    67,    68,
      69,    70,    14,   104,    67,    68,    69,    70,     0,    40,
       0,   171,     0,    16,     0,    17,    18,    38,    19,     0,
       0,     2,     3,     4,     5,     6,     0,     7,     0,     0,
       0,     0,     0,    39,     0,    12,    55,    56,    57,    58,
      59,     0,     0,     0,    13,     0,    60,    61,    62,    63,
      64,     0,    65,    66,    67,    68,    69,    70,     0,     0,
       0,    40,    55,    56,    57,     0,     0,    17,    18,     0,
      19,     0,    60,    61,    62,    63,    64,     0,    65,    66,
      67,    68,    69,    70,   -69,   -69,    65,    66,    67,    68,
      69,    70,     0,     0,    60,    61,    62,    63,    64,     0,
      65,    66,    67,    68,    69,    70,    55,    56,     0,     0,
       0,     0,     0,     0,     0,     0,    60,    61,    62,    63,
      64,     0,    65,    66,    67,    68,    69,    70,   -69,   -69,
     -69,   -69,    64,     0,    65,    66,    67,    68,    69,    70
  };

  const short yycheck[] =
  {
       8,     9,    10,    29,    25,   133,    47,     3,   136,    28,
      18,    46,     3,     5,    49,    47,     7,     8,     9,    10,
      11,     5,    13,     3,    47,     3,    78,     3,    19,     0,
      21,   133,    53,    52,   136,     3,    32,    29,     3,    30,
      20,    49,    22,   171,    22,    29,    22,    99,    28,    29,
      28,    29,    28,    29,    80,    81,    47,    14,    84,    16,
      28,    29,    53,    54,     3,    56,    57,    32,     3,   171,
      48,    79,    48,    53,    15,    53,    48,    53,    52,    87,
      43,    44,    50,    48,    58,    53,    94,    22,    96,    28,
      29,    50,   100,    28,    29,    28,   104,     6,    55,    56,
      57,    58,    59,    60,    61,    62,    63,    64,    65,    66,
      67,    68,    69,    48,    53,   141,    50,    44,    53,    52,
      29,    59,    31,    29,   132,     5,   147,    46,    70,   137,
      49,    73,    12,    53,    11,     3,    48,    55,    54,   165,
       3,   149,    52,   169,     7,     8,     9,    10,    11,    29,
      13,    14,    48,    48,    17,    18,    19,    34,    21,    36,
      28,    29,    16,    53,    48,     3,    48,    30,     3,     7,
       8,     9,    10,    11,    48,    13,    46,    40,    50,     5,
      55,    19,    50,    21,    47,    53,    12,    46,    51,    88,
      53,    54,    30,    56,    37,    28,    39,    40,    41,    42,
      43,    44,    40,    29,    41,    42,    43,    44,    -1,    47,
      -1,   167,    -1,    51,    -1,    53,    54,     3,    56,    -1,
      -1,     7,     8,     9,    10,    11,    -1,    13,    -1,    -1,
      -1,    -1,    -1,    19,    -1,    21,    23,    24,    25,    26,
      27,    -1,    -1,    -1,    30,    -1,    33,    34,    35,    36,
      37,    -1,    39,    40,    41,    42,    43,    44,    -1,    -1,
      -1,    47,    23,    24,    25,    -1,    -1,    53,    54,    -1,
      56,    -1,    33,    34,    35,    36,    37,    -1,    39,    40,
      41,    42,    43,    44,    23,    24,    39,    40,    41,    42,
      43,    44,    -1,    -1,    33,    34,    35,    36,    37,    -1,
      39,    40,    41,    42,    43,    44,    23,    24,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    33,    34,    35,    36,
      37,    -1,    39,    40,    41,    42,    43,    44,    33,    34,
      35,    36,    37,    -1,    39,    40,    41,    42,    43,    44
  };

  // YYSTOS[STATE-NUM] -- The symbol kind of the accessing symbol of
// state STATE-NUM.
  const signed char yystos[] =
  {
       0,     3,     7,     8,     9,    10,    11,    13,    14,    17,
      18,    19,    21,    30,    40,    47,    51,    53,    54,    56,
      61,    62,    63,    64,    65,    66,    67,    68,    71,    46,
      49,    62,    62,    62,    47,    73,    47,    72,     3,    19,
      47,    65,     3,    32,    73,    79,    80,    65,     5,    29,
      69,    70,    62,    78,     0,    23,    24,    25,    26,    27,
      33,    34,    35,    36,    37,    39,    40,    41,    42,    43,
      44,    67,    28,    52,     5,    12,    70,    63,    47,    15,
      50,    50,    73,     3,    20,    22,    28,    29,    53,    75,
      76,    77,    73,     6,    29,    31,    44,    48,    48,    59,
      29,    62,    53,     5,    29,    55,    57,    67,    65,    65,
      65,    65,    65,    65,    65,    65,    65,    65,    65,    65,
      65,    65,    65,    75,    75,    12,    79,    62,    63,    63,
      48,    63,    54,    74,    62,    69,    52,    58,    48,    62,
      62,    46,    49,    79,    62,    48,    62,    28,    48,    16,
      62,    50,    76,    77,    48,    53,    76,    77,    62,    48,
      63,     3,    48,    48,    67,    46,    62,    55,    50,    46,
      63,    74,    63,    50
  };

  // YYR1[RULE-NUM] -- Symbol kind of the left-hand side of rule RULE-NUM.
  const signed char yyr1[] =
  {
       0,    60,    61,    62,    63,    63,    63,    63,    63,    63,
      63,    63,    64,    64,    65,    65,    65,    65,    65,    65,
      65,    65,    65,    65,    65,    65,    65,    65,    65,    65,
      65,    65,    65,    66,    66,    67,    67,    67,    67,    68,
      68,    68,    68,    68,    68,    68,    68,    68,    68,    68,
      68,    68,    68,    69,    69,    69,    70,    70,    70,    70,
      71,    71,    72,    72,    72,    73,    73,    73,    73,    74,
      74,    74,    75,    75,    75,    75,    76,    76,    77,    77,
      78,    78,    79,    79,    79,    79,    80,    80
  };

  // YYR2[RULE-NUM] -- Number of symbols on the right-hand side of rule RULE-NUM.
  const signed char yyr2[] =
  {
       0,     2,     1,     1,     3,     5,     7,     7,     4,     4,
       4,     1,     6,     1,     2,     2,     3,     3,     3,     3,
       3,     3,     3,     3,     3,     3,     3,     3,     3,     3,
       3,     3,     1,     2,     1,     3,     5,     2,     1,     1,
       1,     1,     3,     3,     2,     3,     1,     1,     3,     4,
       4,     3,     3,     1,     1,     0,     2,     4,     3,     4,
       1,     1,     2,     4,     0,     5,     4,     7,     0,     2,
       2,     0,     3,     3,     1,     1,     1,     1,     3,     3,
       2,     0,     3,     1,     0,     1,     1,     3
  };


  /* YYDPREC[RULE-NUM] -- Dynamic precedence of rule #RULE-NUM (0 if none).  */
  const signed char yydprec[] =
  {
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0
  };

  /* YYMERGER[RULE-NUM] -- Index of merging function for rule #RULE-NUM.  */
  const signed char yymerger[] =
  {
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0
  };

  /* YYIMMEDIATE[RULE-NUM] -- True iff rule #RULE-NUM is not to be deferred, as
     in the case of predicates.  */
  const bool yyimmediate[] =
  {
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0
  };

  /* YYCONFLP[YYPACT[STATE-NUM]] -- Pointer into YYCONFL of start of
     list of conflicting reductions corresponding to action entry for
     state STATE-NUM in yytable.  0 means no conflicts.  The list in
     yyconfl is terminated by a rule number of 0.  */
  const signed char yyconflp[] =
  {
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     1,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     3,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0
  };

  /* YYCONFL[I] -- lists of conflicting rule numbers, each terminated by
     0, pointed into by YYCONFLP.  */
        const short yyconfl[] =
  {
       0,    68,     0,    84,     0
  };
} // namespace


/* Error token number */
#define YYTERROR 1


/* YYLLOC_DEFAULT -- Set CURRENT to span from RHS[1] to RHS[N].
   If N is 0, then set CURRENT to the empty location which ends
   the previous symbol: RHS[0] (always defined).  */

# ifndef YYLLOC_DEFAULT
#  define YYLLOC_DEFAULT(Current, Rhs, N)                               \
    do                                                                  \
      if (N)                                                            \
        {                                                               \
          (Current).begin  = YYRHSLOC (Rhs, 1).begin;                   \
          (Current).end    = YYRHSLOC (Rhs, N).end;                     \
        }                                                               \
      else                                                              \
        {                                                               \
          (Current).begin = (Current).end = YYRHSLOC (Rhs, 0).end;      \
        }                                                               \
    while (false)
# endif

# define YYRHSLOC(Rhs, K) ((Rhs)[K].getState().yyloc)


enum YYRESULTTAG { yyok, yyaccept, yyabort, yyerr };

#define YYCHK(YYE)                              \
  do {                                          \
    YYRESULTTAG yychk_flag = YYE;               \
    if (yychk_flag != yyok)                     \
      return yychk_flag;                        \
  } while (false)

#if YYDEBUG

#define YYCDEBUG if (!yydebug) {} else std::cerr

# define YY_SYMBOL_PRINT(Title, Kind, Value, Location)                  \
  do {                                                                  \
    if (yydebug)                                                        \
      {                                                                 \
        std::cerr << Title << ' ';                                      \
        yyparser.yy_symbol_print_ (Kind, Value, Location); \
        std::cerr << '\n';                                              \
      }                                                                 \
  } while (false)

# define YY_REDUCE_PRINT(Args)                  \
  do {                                          \
    if (yydebug)                                \
      yystateStack.yy_reduce_print Args;        \
  } while (false)

/* Nonzero means print parse trace.  It is left uninitialized so that
   multiple parsers can coexist.  */
int yydebug;

namespace
{
  using glr_stack = yy::parser::glr_stack;
  using glr_state = yy::parser::glr_state;

  void yypstack (const glr_stack& yystack, size_t yyk)
    YY_ATTRIBUTE_UNUSED;
  void yypdumpstack (const glr_stack& yystack)
    YY_ATTRIBUTE_UNUSED;
}

#else /* !YYDEBUG */

# define YYCDEBUG if (true) {} else std::cerr
# define YY_SYMBOL_PRINT(Title, Kind, Value, Location) {}
# define YY_REDUCE_PRINT(Args) {}

#endif /* !YYDEBUG */

/* YYINITDEPTH -- initial size of the parser's stacks.  */
#ifndef YYINITDEPTH
# define YYINITDEPTH 200
#endif

/* YYMAXDEPTH -- maximum size the stacks can grow to (effective only
   if the built-in stack extension method is used).

   Do not make this value too large; the results are undefined if
   SIZE_MAX < YYMAXDEPTH * sizeof (GLRStackItem)
   evaluated with infinite-precision integer arithmetic.  */

#ifndef YYMAXDEPTH
# define YYMAXDEPTH 10000
#endif

/* Minimum number of free items on the stack allowed after an
   allocation.  This is to allow allocation and initialization
   to be completed by functions that call yyexpandGLRStack before the
   stack is expanded, thus insuring that all necessary pointers get
   properly redirected to new data.  */
#define YYHEADROOM 2

#ifndef YYSTACKEXPANDABLE
# define YYSTACKEXPANDABLE 1
#endif

namespace
{
  template <typename Parameter>
  class strong_index_alias
  {
  public:
    static strong_index_alias create (std::ptrdiff_t value)
    {
      strong_index_alias result;
      result.value_ = value;
      return result;
    }

    std::ptrdiff_t const& get () const { return value_; }

    size_t uget () const { return static_cast<size_t> (value_); }

    strong_index_alias operator+ (std::ptrdiff_t other) const
    {
      return strong_index_alias (get () + other);
    }

    void operator+= (std::ptrdiff_t other)
    {
      value_ += other;
    }

    strong_index_alias operator- (std::ptrdiff_t other)
    {
      return strong_index_alias (get () - other);
    }

    void operator-= (std::ptrdiff_t other)
    {
      value_ -= other;
    }

    size_t operator- (strong_index_alias other)
    {
      return strong_index_alias (get () - other.get ());
    }

    strong_index_alias& operator++ ()
    {
      ++value_;
      return *this;
    }

    bool isValid () const
    {
      return value_ != INVALID_INDEX;
    }

    void setInvalid()
    {
      value_ = INVALID_INDEX;
    }

    bool operator== (strong_index_alias other)
    {
      return get () == other.get ();
    }

    bool operator!= (strong_index_alias other)
    {
      return get () != other.get ();
    }

    bool operator< (strong_index_alias other)
    {
      return get () < other.get ();
    }

  private:
    static const std::ptrdiff_t INVALID_INDEX;

    // WARNING: 0-initialized.
    std::ptrdiff_t value_;
  }; // class strong_index_alias

  template<typename T>
  const std::ptrdiff_t strong_index_alias<T>::INVALID_INDEX =
    std::numeric_limits<std::ptrdiff_t>::max ();

  using state_set_index = strong_index_alias<struct glr_state_set_tag>;

  state_set_index create_state_set_index (std::ptrdiff_t value)
  {
    return state_set_index::create (value);
  }

  /** State numbers, as in LALR(1) machine */
  using state_num = int;

  /** Rule numbers, as in LALR(1) machine */
  using rule_num = int;

  using parser_type = yy::parser;
  using glr_state = parser_type::glr_state;
  using symbol_kind = parser_type::symbol_kind;
  using symbol_kind_type = parser_type::symbol_kind_type;
  using symbol_type = parser_type::symbol_type;
  using value_type = parser_type::value_type;
  using location_type = parser_type::location_type;

  // Forward declarations.
  class glr_stack_item;
  class semantic_option;
} // namespace

namespace
{
  /** Accessing symbol of state YYSTATE.  */
  inline symbol_kind_type
  yy_accessing_symbol (state_num yystate)
  {
    return YY_CAST (symbol_kind_type, yystos[yystate]);
  }

  /** Left-hand-side symbol for rule #YYRULE.  */
  inline symbol_kind_type
  yylhsNonterm (rule_num yyrule)
  {
    return static_cast<symbol_kind_type>(yyr1[yyrule]);
  }

  /** Number of symbols composing the right hand side of rule #RULE.  */
  inline int
  yyrhsLength (rule_num yyrule)
  {
    return yyr2[yyrule];
  }
}

namespace yy
{
  class parser::glr_state
  {
  public:
    glr_state ()
      : yyresolved (false)
      , yylrState (0)
      , yyposn (0)
      , yypred (0)
      , yyfirstVal (0)
      , yyloc ()
    {}

    /// Build with a semantic value.
    glr_state (state_num lrState, size_t posn, const value_type& val, const location_type& loc)
      : yyresolved (true)
      , yylrState (lrState)
      , yyposn (posn)
      , yypred (0)
      , yyval (val)
      , yyloc (loc)
    {}

    /// Build with a semantic option.
    glr_state (state_num lrState, size_t posn)
      : yyresolved (false)
      , yylrState (lrState)
      , yyposn (posn)
      , yypred (0)
      , yyfirstVal (0)
      , yyloc ()
    {}

    glr_state (const glr_state& other)
      : yyresolved (other.yyresolved)
      , yylrState (other.yylrState)
      , yyposn (other.yyposn)
      , yypred (0)
      , yyloc (other.yyloc)
    {
      setPred (other.pred ());
      if (other.yyresolved)
        new (&yyval) value_type (other.value ());
      else
        {
          yyfirstVal = 0;
          setFirstVal (other.firstVal ());
        }
    }

    ~glr_state ()
    {
      if (yyresolved)
        {
          yyval.~value_type ();
        }
    }

    glr_state& operator= (const glr_state& other)
    {
      if (!yyresolved && other.yyresolved)
        new (&yyval) value_type;
      yyresolved = other.yyresolved;
      yylrState = other.yylrState;
      yyposn = other.yyposn;
      setPred (other.pred ());
      if (other.yyresolved)
        value () = other.value ();
      else
        setFirstVal (other.firstVal ());
      yyloc = other.yyloc;
      return *this;
    }

    /** Type tag for the semantic value.  If true, yyval applies, otherwise
     *  yyfirstVal applies.  */
    bool yyresolved;
    /** Number of corresponding LALR(1) machine state.  */
    state_num yylrState;
    /** Source position of the last token produced by my symbol */
    size_t yyposn;

    /// Only call pred() and setPred() on objects in yyitems, not temporaries.
    glr_state* pred ();
    const glr_state* pred () const;
    void setPred (const glr_state* state);

    /// Only call firstVal() and setFirstVal() on objects in yyitems, not
    /// temporaries.
    semantic_option* firstVal ();
    const semantic_option* firstVal () const;
    void setFirstVal (const semantic_option* option);

    value_type& value ()
    {
      return yyval;
    }

    const value_type& value () const
    {
      return yyval;
    }

    void
    destroy (char const *yymsg, yy::parser& yyparser);

    /* DEBUGGING ONLY */
  #if YYDEBUG
    void yy_yypstack () const
    {
      if (pred () != YY_NULLPTR)
        {
          pred ()->yy_yypstack ();
          std::cerr << " -> ";
        }
      std::cerr << yylrState << "@" << yyposn;
    }
  #endif

    std::ptrdiff_t indexIn (const glr_stack_item* array) const YY_ATTRIBUTE_UNUSED;

    glr_stack_item* asItem ()
    {
      return asItem(this);
    }

    const glr_stack_item* asItem () const
    {
      return asItem (this);
    }

  private:
    template <typename T>
    static const glr_stack_item* asItem (const T* state)
    {
      return reinterpret_cast<const glr_stack_item*>(state);
    }
    template <typename T>
    static glr_stack_item* asItem (T* state)
    {
      return reinterpret_cast<glr_stack_item*> (state);
    }
    static const char *as_pointer_ (const glr_state *state)
    {
      return reinterpret_cast<const char *> (state);
    }
    static char *as_pointer_ (glr_state *state)
    {
      return reinterpret_cast<char *> (state);
    }
    /** Preceding state in this stack */
    std::ptrdiff_t yypred;
    union {
      /** First in a chain of alternative reductions producing the
       *  nonterminal corresponding to this state, threaded through
       *  yyfirstVal.  Value "0" means empty.  */
      std::ptrdiff_t yyfirstVal;
      /** Semantic value for this state.  */
      value_type yyval;
    };
   // FIXME: Why public?
   public:
    /** Source location for this state.  */
    location_type yyloc;


  }; // class parser::glr_state
} // namespace yy


namespace
{
  /** A stack of GLRState representing the different heads during
    * nondeterministic evaluation. */
  class glr_state_set
  {
  public:
    /** Initialize YYSET to a singleton set containing an empty stack.  */
    glr_state_set ()
      : yylastDeleted (YY_NULLPTR)
    {
      yystates.push_back (YY_NULLPTR);
      yylookaheadNeeds.push_back (false);
    }

    // Behave like a vector of states.
    glr_state*& operator[] (state_set_index index)
    {
      return yystates[index.uget()];
    }

    glr_state* operator[] (state_set_index index) const
    {
      return yystates[index.uget()];
    }

    size_t size () const
    {
      return yystates.size ();
    }

    std::vector<glr_state*>::iterator begin ()
    {
      return yystates.begin ();
    }

    std::vector<glr_state*>::iterator end ()
    {
      return yystates.end ();
    }

    bool lookaheadNeeds (state_set_index index) const
    {
      return yylookaheadNeeds[index.uget ()];
    }

    bool setLookaheadNeeds (state_set_index index, bool value)
    {
      return yylookaheadNeeds[index.uget ()] = value;
    }

    /** Invalidate stack #YYK.  */
    void
    yymarkStackDeleted (state_set_index yyk)
    {
      size_t k = yyk.uget ();
      if (yystates[k] != YY_NULLPTR)
        yylastDeleted = yystates[k];
      yystates[k] = YY_NULLPTR;
    }

    /** Undelete the last stack in *this that was marked as deleted.  Can
        only be done once after a deletion, and only when all other stacks have
        been deleted.  */
    void
    yyundeleteLastStack ()
    {
      if (yylastDeleted == YY_NULLPTR || !yystates.empty ())
        return;
      yystates.push_back (yylastDeleted);
      YYCDEBUG << "Restoring last deleted stack as stack #0.\n";
      clearLastDeleted ();
    }

    /** Remove the dead stacks (yystates[i] == YY_NULLPTR) and shift the later
     * ones.  */
    void
    yyremoveDeletes ()
    {
      size_t newsize = yystates.size ();
      /* j is the number of live stacks we have seen.  */
      for (size_t i = 0, j = 0; j < newsize; ++i)
        {
          if (yystates[i] == YY_NULLPTR)
            {
              if (i == j)
                {
                  YYCDEBUG << "Removing dead stacks.\n";
                }
              newsize -= 1;
            }
          else
            {
              yystates[j] = yystates[i];
              /* In the current implementation, it's unnecessary to copy
                 yylookaheadNeeds[i] since, after
                 yyremoveDeletes returns, the parser immediately either enters
                 deterministic operation or shifts a token.  However, it doesn't
                 hurt, and the code might evolve to need it.  */
              yylookaheadNeeds[j] = yylookaheadNeeds[i];
              if (j != i)
                {
                  YYCDEBUG << "Rename stack " << i << " -> " << j << ".\n";
                }
              j += 1;
            }
        }
      yystates.resize (newsize);
      yylookaheadNeeds.resize (newsize);
    }


    state_set_index
    yysplitStack (state_set_index yyk)
    {
      const size_t k = yyk.uget ();
      yystates.push_back (yystates[k]);
      yylookaheadNeeds.push_back (yylookaheadNeeds[k]);
      return create_state_set_index (static_cast<std::ptrdiff_t> (yystates.size () - 1));
    }

    void clearLastDeleted ()
    {
      yylastDeleted = YY_NULLPTR;
    }

  private:

    std::vector<glr_state*> yystates;
    /** During nondeterministic operation, yylookaheadNeeds tracks which
     *  stacks have actually needed the current lookahead.  During deterministic
     *  operation, yylookaheadNeeds[0] is not maintained since it would merely
     *  duplicate !yyla.empty ().  */
    std::vector<bool> yylookaheadNeeds;

    /** The last stack we invalidated.  */
    glr_state* yylastDeleted;
  }; // class glr_state_set
} // namespace

namespace
{
  class semantic_option
  {
  public:
    semantic_option ()
      : yyrule (0)
      , yystate (0)
      , yynext (0)
      , yyla ()
    {}

    semantic_option (rule_num rule)
      : yyrule (rule)
      , yystate (0)
      , yynext (0)
      , yyla ()
    {}

    semantic_option (const semantic_option& that)
      : yyrule (that.yyrule)
      , yystate (that.yystate)
      , yynext (that.yynext)
      , yyla (that.yyla)
    {
    }

    // Needed for the assignment in yynewSemanticOption.
    semantic_option& operator= (const semantic_option& that)
    {
      yyrule = that.yyrule;
      yystate = that.yystate;
      yynext = that.yynext;
      yyla = that.yyla;
      return *this;
    }

    /// Only call state() and setState() on objects in yyitems, not temporaries.
    glr_state* state();
    const glr_state* state() const;
    void setState(const glr_state* s);

    const semantic_option* next () const YY_ATTRIBUTE_UNUSED;
    semantic_option* next ();
    void setNext (const semantic_option* s);

    std::ptrdiff_t indexIn (const glr_stack_item* array) const YY_ATTRIBUTE_UNUSED;

    /** True iff YYY0 and YYY1 represent identical options at the top level.
     *  That is, they represent the same rule applied to RHS symbols
     *  that produce the same terminal symbols.  */
    bool
    isIdenticalTo (const semantic_option& yyy1) const
    {
      if (this->yyrule == yyy1.yyrule)
        {
          const glr_state *yys0, *yys1;
          int yyn;
          for (yys0 = this->state(),
               yys1 = yyy1.state(),
               yyn = yyrhsLength (this->yyrule);
               yyn > 0;
               yys0 = yys0->pred(),
               yys1 = yys1->pred(), yyn -= 1)
            if (yys0->yyposn != yys1->yyposn)
              return false;
          return true;
        }
      else
        return false;
    }

    /** Assuming identicalOptions (YYY0,YYY1), destructively merge the
     *  alternative semantic values for the RHS-symbols of YYY1 and YYY0.  */
    void
    mergeWith (semantic_option& yyy1)
    {
      glr_state *yys0 = this->state ();
      glr_state *yys1 = yyy1.state ();
      for (int yyn = yyrhsLength (this->yyrule);
           yyn > 0;
           yyn -= 1, yys0 = yys0->pred (), yys1 = yys1->pred ())
        {
          if (yys0 == yys1)
            break;
          else if (yys0->yyresolved)
            {
              yys1->yyresolved = true;
              yys1->value () = yys0->value ();
            }
          else if (yys1->yyresolved)
            {
              yys0->yyresolved = true;
              yys0->value () = yys1->value ();
            }
          else
            {
              semantic_option* yyz0prev = YY_NULLPTR;
              semantic_option* yyz0 = yys0->firstVal();
              semantic_option* yyz1 = yys1->firstVal();
              while (true)
                {
                  if (yyz1 == yyz0 || yyz1 == YY_NULLPTR)
                    break;
                  else if (yyz0 == YY_NULLPTR)
                    {
                      if (yyz0prev != YY_NULLPTR)
                        yyz0prev->setNext (yyz1);
                      else
                        yys0->setFirstVal (yyz1);
                      break;
                    }
                  else if (yyz0 < yyz1)
                    {
                      semantic_option* yyz = yyz0;
                      if (yyz0prev != YY_NULLPTR)
                        yyz0prev->setNext(yyz1);
                      else
                        yys0->setFirstVal(yyz1);
                      yyz1 = yyz1->next();
                      yyz0->setNext(yyz);
                    }
                  yyz0prev = yyz0;
                  yyz0 = yyz0->next();
                }
              yys1->setFirstVal(yys0->firstVal());
            }
        }
    }

#if YYDEBUG
    void yyreportTree (size_t yyindent = 2) const
    {
      int yynrhs = yyrhsLength (this->yyrule);
      const glr_state* yystates[1 + YYMAXRHS];
      glr_state yyleftmost_state;

      {
        const glr_state* yys = this->state();
        for (int yyi = yynrhs; 0 < yyi; yyi -= 1)
          {
            yystates[yyi] = yys;
            yys = yys->pred();
          }
        if (yys == YY_NULLPTR)
          {
            yyleftmost_state.yyposn = 0;
            yystates[0] = &yyleftmost_state;
          }
        else
          yystates[0] = yys;
      }

      std::string yylhs = yy::parser::symbol_name (yylhsNonterm (this->yyrule));
      YYASSERT(this->state());
      if (this->state()->yyposn < yystates[0]->yyposn + 1)
        std::cerr << std::string(yyindent, ' ') << yylhs << " -> <Rule "
                  << this->yyrule - 1 << ", empty>\n";
      else
        std::cerr << std::string(yyindent, ' ') << yylhs << " -> <Rule "
                  << this->yyrule - 1 << ", tokens "
                  << yystates[0]->yyposn + 1 << " .. "
                  << this->state()->yyposn << ">\n";
      for (int yyi = 1; yyi <= yynrhs; yyi += 1)
        {
          if (yystates[yyi]->yyresolved)
            {
              std::string yysym = yy::parser::symbol_name (yy_accessing_symbol (yystates[yyi]->yylrState));
              if (yystates[yyi-1]->yyposn+1 > yystates[yyi]->yyposn)
                std::cerr << std::string(yyindent + 2, ' ') << yysym
                          << " <empty>\n";
              else
                std::cerr << std::string(yyindent + 2, ' ') << yysym
                          << " <tokens " << yystates[yyi-1]->yyposn + 1
                          << " .. " << yystates[yyi]->yyposn << ">\n";
            }
          else
            yystates[yyi]->firstVal ()->yyreportTree (yyindent+2);
        }
    }
#endif

    /** Rule number for this reduction */
    rule_num yyrule;

  private:
    template <typename T>
    static const glr_stack_item* asItem(const T* state)
    {
      return reinterpret_cast<const glr_stack_item*>(state);
    }
    template <typename T>
    static glr_stack_item* asItem(T* state)
    {
      return reinterpret_cast<glr_stack_item*>(state);
    }
    /** The last RHS state in the list of states to be reduced.  */
    std::ptrdiff_t yystate;
    /** Next sibling in chain of options.  To facilitate merging,
     *  options are chained in decreasing order by address.  */
    std::ptrdiff_t yynext;

  public:
    /** The lookahead for this reduction.  */
    symbol_type yyla;


  }; // class semantic_option
} // namespace

namespace
{
  /** Type of the items in the GLR stack.
   *  It can be either a glr_state or a semantic_option. The is_state_ field
   *  indicates which item of the union is valid.  */
  class glr_stack_item
  {
  public:
    glr_stack_item (bool state = true)
      : is_state_ (state)
    {
      if (is_state_)
        new (&raw_) glr_state;
      else
        new (&raw_) semantic_option;
    }

    glr_stack_item (const glr_stack_item& other) YY_NOEXCEPT YY_NOTHROW
      : is_state_ (other.is_state_)
    {
      std::memcpy (raw_, other.raw_, union_size);
    }

    glr_stack_item& operator= (glr_stack_item other)
    {
      std::swap (is_state_, other.is_state_);
      std::swap (raw_, other.raw_);
      return *this;
    }

    ~glr_stack_item ()
    {
      if (is_state ())
        getState ().~glr_state ();
      else
        getOption ().~semantic_option ();
    }

    void setState (const glr_state &state)
    {
      if (this != state.asItem ())
        {
          if (is_state_)
            getState ().~glr_state ();
          else
            getOption ().~semantic_option ();
          new (&raw_) glr_state (state);
          is_state_ = true;
        }
    }

    glr_state& getState ()
    {
      YYDASSERT (is_state ());
      void *yyp = raw_;
      glr_state& res = *static_cast<glr_state*> (yyp);
      return res;
    }

    const glr_state& getState () const
    {
      YYDASSERT (is_state ());
      const void *yyp = raw_;
      const glr_state& res = *static_cast<const glr_state*> (yyp);
      return res;
    }

    semantic_option& getOption ()
    {
      YYDASSERT (!is_state ());
      void *yyp = raw_;
      return *static_cast<semantic_option*> (yyp);
    }
    const semantic_option& getOption () const
    {
      YYDASSERT (!is_state ());
      const void *yyp = raw_;
      return *static_cast<const semantic_option*> (yyp);
    }
    bool is_state () const
    {
      return is_state_;
    }

  private:
    /// The possible contents of raw_. Since they have constructors, they cannot
    /// be directly included in the union.
    union contents
    {
      char yystate[sizeof (glr_state)];
      char yyoption[sizeof (semantic_option)];
    };
    enum { union_size = sizeof (contents) };
    union {
      /// Strongest alignment constraints.
      long double yyalign_me;
      /// A buffer large enough to store the contents.
      char raw_[union_size];
    };
    /** Type tag for the union. */
    bool is_state_;

  }; // class glr_stack_item
} // namespace

glr_state* glr_state::pred ()
{
  YY_IGNORE_NULL_DEREFERENCE_BEGIN
  return yypred ? &asItem (as_pointer_ (this) - yypred)->getState () : YY_NULLPTR;
  YY_IGNORE_NULL_DEREFERENCE_END
}

const glr_state* glr_state::pred () const
{
  YY_IGNORE_NULL_DEREFERENCE_BEGIN
  return yypred ? &asItem (as_pointer_ (this) - yypred)->getState () : YY_NULLPTR;
  YY_IGNORE_NULL_DEREFERENCE_END
}

void glr_state::setPred (const glr_state* state)
{
  yypred = state ? as_pointer_ (this) - as_pointer_ (state) : 0;
}

semantic_option* glr_state::firstVal ()
{
  return yyfirstVal ? &(asItem(this) - yyfirstVal)->getOption() : YY_NULLPTR;
}

const semantic_option* glr_state::firstVal () const
{
  return yyfirstVal ? &(asItem(this) - yyfirstVal)->getOption() : YY_NULLPTR;
}

void glr_state::setFirstVal (const semantic_option* option)
{
  yyfirstVal = option ? asItem(this) - asItem(option) : 0;
}

std::ptrdiff_t glr_state::indexIn (const glr_stack_item* array) const
{
  return asItem(this) - array;
}

std::ptrdiff_t semantic_option::indexIn (const glr_stack_item* array) const
{
  return asItem(this) - array;
}

glr_state* semantic_option::state ()
{
  YY_IGNORE_NULL_DEREFERENCE_BEGIN
  return yystate ? &(asItem(this) - yystate)->getState() : YY_NULLPTR;
  YY_IGNORE_NULL_DEREFERENCE_END
}

const glr_state* semantic_option::state () const
{
  return yystate ? &(asItem(this) - yystate)->getState() : YY_NULLPTR;
}

void semantic_option::setState (const glr_state* s)
{
  yystate = s ? asItem(this) - asItem(s) : 0;
}

const semantic_option* semantic_option::next () const
{
  return yynext ? &(asItem(this) - yynext)->getOption() : YY_NULLPTR;
}

semantic_option* semantic_option::next ()
{
  return yynext ? &(asItem(this) - yynext)->getOption() : YY_NULLPTR;
}

void semantic_option::setNext (const semantic_option* s)
{
  yynext = s ? asItem(this) - asItem(s) : 0;
}

void glr_state::destroy (char const* yymsg, yy::parser& yyparser)
{
  if (yyresolved)
    yyparser.yy_destroy_ (yymsg, yy_accessing_symbol(yylrState),
                          value (), yyloc);
  else
    {
#if YYDEBUG
      YYCDEBUG << yymsg
               << (firstVal() ? " unresolved " : " incomplete ")
               << (yy_accessing_symbol (yylrState) < YYNTOKENS ? "token" : "nterm")
               << ' ' << yyparser.symbol_name (yy_accessing_symbol (yylrState))
               << " ("
               << yyloc << ": "
               << ")\n";
#endif
      if (firstVal() != YY_NULLPTR)
        {
          semantic_option& yyoption = *firstVal ();
          glr_state *yyrh = yyoption.state ();
          for (int yyn = yyrhsLength (yyoption.yyrule); yyn > 0; yyn -= 1)
            {
              yyrh->destroy (yymsg, yyparser);
              yyrh = yyrh->pred();
            }
        }
    }
}


#undef YYFILL
#define YYFILL(N) yyfill (yyvsp, yylow, (N), yynormal)

namespace
{
  class state_stack
  {
  public:
    using parser_type = yy::parser;
    using symbol_kind = parser_type::symbol_kind;
    using value_type = parser_type::value_type;
    using location_type = parser_type::location_type;

    /** Initialize to a single empty stack, with total maximum
     *  capacity for all stacks of YYSIZE.  */
    state_stack (size_t yysize)
      : yysplitPoint (YY_NULLPTR)
    {
      yyitems.reserve (yysize);
    }

#if YYSTACKEXPANDABLE
    /** Returns false if it tried to expand but could not. */
    bool
    yyexpandGLRStackIfNeeded ()
    {
      return YYHEADROOM <= spaceLeft () || yyexpandGLRStack ();
    }

  private:
    /** If *this is expandable, extend it.  WARNING: Pointers into the
        stack from outside should be considered invalid after this call.
        We always expand when there are 1 or fewer items left AFTER an
        allocation, so that we can avoid having external pointers exist
        across an allocation.  */
    bool
    yyexpandGLRStack ()
    {
      const size_t oldsize = yyitems.size();
      if (YYMAXDEPTH - YYHEADROOM < oldsize)
        return false;
      const size_t yynewSize = YYMAXDEPTH < 2 * oldsize ? YYMAXDEPTH : 2 * oldsize;
      const glr_stack_item *oldbase = &yyitems[0];

      yyitems.reserve (yynewSize);
      const glr_stack_item *newbase = &yyitems[0];

      // Adjust the pointers.  Perform raw pointer arithmetic, as there
      // is no reason for objects to be aligned on their size.
      const ptrdiff_t disp
        = reinterpret_cast<const char*> (newbase) - reinterpret_cast<const char*> (oldbase);
      if (yysplitPoint)
        const_cast<glr_state*&> (yysplitPoint)
          = reinterpret_cast<glr_state*> (reinterpret_cast<char*> (const_cast<glr_state*> (yysplitPoint)) + disp);

      for (std::vector<glr_state*>::iterator
             i = yytops.begin (),
             yyend = yytops.end ();
           i != yyend; ++i)
        if (glr_state_not_null (*i))
          *i = reinterpret_cast<glr_state*>(reinterpret_cast<char*>(*i) + disp);

      return true;
    }

  public:
#else
    bool yyexpandGLRStackIfNeeded ()
    {
      return YYHEADROOM <= spaceLeft ();
    }
#endif
#undef YYSTACKEXPANDABLE

    static bool glr_state_not_null (glr_state* s)
    {
      return s != YY_NULLPTR;
    }

    bool
    reduceToOneStack ()
    {
      using iterator = std::vector<glr_state*>::iterator;
      const iterator yybegin = yytops.begin();
      const iterator yyend = yytops.end();
      const iterator yyit = std::find_if(yybegin, yyend, glr_state_not_null);
      if (yyit == yyend)
        return false;
      for (state_set_index yyk = create_state_set_index(yyit + 1 - yybegin);
           yyk.uget() != numTops(); ++yyk)
        yytops.yymarkStackDeleted (yyk);
      yytops.yyremoveDeletes ();
      yycompressStack ();
      return true;
    }

    /** Called when returning to deterministic operation to clean up the extra
     * stacks. */
    void
    yycompressStack ()
    {
      if (yytops.size() != 1 || !isSplit())
        return;

      // yyr is the state after the split point.
      glr_state* yyr = YY_NULLPTR;
      for (glr_state *yyp = firstTop(), *yyq = yyp->pred();
           yyp != yysplitPoint;
           yyr = yyp, yyp = yyq, yyq = yyp->pred())
        yyp->setPred(yyr);

      // This const_cast is okay, since anyway we have access to the mutable
      // yyitems into which yysplitPoint points.
      glr_stack_item* nextFreeItem
        = const_cast<glr_state*> (yysplitPoint)->asItem () + 1;
      yysplitPoint = YY_NULLPTR;
      yytops.clearLastDeleted ();

      while (yyr != YY_NULLPTR)
        {
          nextFreeItem->setState (*yyr);
          glr_state& nextFreeState = nextFreeItem->getState();
          yyr = yyr->pred();
          nextFreeState.setPred(&(nextFreeItem - 1)->getState());
          setFirstTop (&nextFreeState);
          ++nextFreeItem;
        }
      yyitems.resize(static_cast<size_t>(nextFreeItem - yyitems.data()));
    }

    bool isSplit() const {
      return yysplitPoint != YY_NULLPTR;
    }

    // Present the interface of a vector of glr_stack_item.
    std::vector<glr_stack_item>::const_iterator begin () const
    {
      return yyitems.begin ();
    }

    std::vector<glr_stack_item>::const_iterator end () const
    {
      return yyitems.end ();
    }

    size_t size() const
    {
      return yyitems.size ();
    }

    glr_stack_item& operator[] (size_t i)
    {
      return yyitems[i];
    }

    glr_stack_item& stackItemAt (size_t index)
    {
      return yyitems[index];
    }

    size_t numTops () const
    {
      return yytops.size ();
    }

    glr_state* firstTop () const
    {
      return yytops[create_state_set_index (0)];
    }

    glr_state* topAt (state_set_index i) const
    {
      return yytops[i];
    }

    void setFirstTop (glr_state* value)
    {
      yytops[create_state_set_index (0)] = value;
    }

    void setTopAt (state_set_index i, glr_state* value)
    {
      yytops[i] = value;
    }

    void pop_back ()
    {
      yyitems.pop_back ();
    }

    void pop_back (size_t n)
    {
      yyitems.resize (yyitems.size () - n);
    }

    state_set_index
    yysplitStack (state_set_index yyk)
    {
      if (!isSplit ())
        {
          YYASSERT (yyk.get () == 0);
          yysplitPoint = topAt (yyk);
        }
      return yytops.yysplitStack (yyk);
    }

    /** Assuming that YYS is a GLRState somewhere on *this, update the
     *  splitpoint of *this, if needed, so that it is at least as deep as
     *  YYS.  */
    void
    yyupdateSplit (glr_state& yys)
    {
      if (isSplit() && &yys < yysplitPoint)
        yysplitPoint = &yys;
    }

    /** Return a fresh GLRState.
     * Callers should call yyreserveStack afterwards to make sure there is
     * sufficient headroom.  */
    glr_state& yynewGLRState (const glr_state& newState)
    {
      glr_state& state = yyitems[yynewGLRStackItem (true)].getState ();
#if false && 201103L <= YY_CPLUSPLUS
      state = std::move (newState);
#else
      state = newState;
#endif
      return state;
    }

    /** Return a fresh SemanticOption.
     * Callers should call yyreserveStack afterwards to make sure there is
     * sufficient headroom.  */
    semantic_option& yynewSemanticOption (semantic_option newOption)
    {
      semantic_option& option = yyitems[yynewGLRStackItem (false)].getOption ();
      option = std::move (newOption);
      return option;
    }

    /* Do nothing if YYNORMAL or if *YYLOW <= YYLOW1.  Otherwise, fill in
     * YYVSP[YYLOW1 .. *YYLOW-1] as in yyfillin and set *YYLOW = YYLOW1.
     * For convenience, always return YYLOW1.  */
    int
    yyfill (glr_stack_item *yyvsp, int &yylow, int yylow1, bool yynormal)
    {
      if (!yynormal && yylow1 < yylow)
        {
          yyfillin (yyvsp, yylow, yylow1);
          yylow = yylow1;
        }
      return yylow1;
    }

    /** Fill in YYVSP[YYLOW1 .. YYLOW0-1] from the chain of states starting
     *  at YYVSP[YYLOW0].getState().pred().  Leaves YYVSP[YYLOW1].getState().pred()
     *  containing the pointer to the next state in the chain.  */
    void
    yyfillin (glr_stack_item *yyvsp, int yylow0, int yylow1)
    {
      glr_state* s = yyvsp[yylow0].getState().pred();
      YYASSERT(s != YY_NULLPTR);
      for (int i = yylow0-1; i >= yylow1; i -= 1, s = s->pred())
        {
          glr_state& yys = yyvsp[i].getState();
#if YYDEBUG
          yys.yylrState = s->yylrState;
#endif
          yys.yyresolved = s->yyresolved;
          if (s->yyresolved)
            {
              new (&yys.value ()) value_type (s->value ());
            }
          else
            /* The effect of using yyval or yyloc (in an immediate
             * rule) is undefined.  */
            yys.setFirstVal (YY_NULLPTR);
          yys.yyloc = s->yyloc;
          yys.setPred(s->pred());
        }
    }

#if YYDEBUG

    /*----------------------------------------------------------------------.
    | Report that stack #YYK of *YYSTACKP is going to be reduced by YYRULE. |
    `----------------------------------------------------------------------*/

    void
    yy_reduce_print (bool yynormal, glr_stack_item* yyvsp, state_set_index yyk,
                     rule_num yyrule, parser_type& yyparser)
    {
      int yynrhs = yyrhsLength (yyrule);
      int yylow = 1;
      int yyi;
      std::cerr << "Reducing stack " << yyk.get() << " by rule " << yyrule - 1
                << " (line " << int (yyrline[yyrule]) << "):\n";
      if (! yynormal)
        yyfillin (yyvsp, 1, -yynrhs);
      /* The symbols being reduced.  */
      for (yyi = 0; yyi < yynrhs; yyi++)
        {
          std::cerr << "   $" << yyi + 1 << " = ";
          yyparser.yy_symbol_print_
            (yy_accessing_symbol (yyvsp[yyi - yynrhs + 1].getState().yylrState),
             yyvsp[yyi - yynrhs + 1].getState().value (),
             ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL ((yyi + 1) - (yynrhs))].getState().yyloc));
          if (!yyvsp[yyi - yynrhs + 1].getState().yyresolved)
            std::cerr << " (unresolved)";
          std::cerr << '\n';
        }
    }


#define YYINDEX(YYX)                                                         \
      ((YYX) == YY_NULLPTR ? -1 : (YYX)->indexIn (yyitems.data ()))

    void
    dumpStack () const
    {
      for (size_t yyi = 0; yyi < size(); ++yyi)
        {
          const glr_stack_item& item = yyitems[yyi];
          std::cerr << std::setw(3) << yyi << ". ";
          if (item.is_state())
            {
              std::cerr << "Res: " << item.getState().yyresolved
                        << ", LR State: " << item.getState().yylrState
                        << ", posn: " << item.getState().yyposn
                        << ", pred: " << YYINDEX(item.getState().pred());
              if (! item.getState().yyresolved)
                std::cerr << ", firstVal: "
                          << YYINDEX(item.getState().firstVal());
            }
          else
            {
              std::cerr << "Option. rule: " << item.getOption().yyrule - 1
                        << ", state: " << YYINDEX(item.getOption().state())
                        << ", next: " << YYINDEX(item.getOption().next());
            }
          std::cerr << '\n';
        }
      std::cerr << "Tops:";
      for (state_set_index yyi = create_state_set_index(0); yyi.uget() < numTops(); ++yyi) {
        std::cerr << yyi.get() << ": " << YYINDEX(topAt(yyi)) << "; ";
      }
      std::cerr << '\n';
    }

#undef YYINDEX
#endif

    YYRESULTTAG
    yyreportAmbiguity (const semantic_option& yyx0,
                       const semantic_option& yyx1, parser_type& yyparser, const location_type& yyloc)
    {
      YY_USE (yyx0);
      YY_USE (yyx1);

#if YYDEBUG
      std::cerr << "Ambiguity detected.\n"
        "Option 1,\n";
      yyx0.yyreportTree ();
      std::cerr << "\nOption 2,\n";
      yyx1.yyreportTree ();
      std::cerr << '\n';
#endif

      yyparser.error (yyloc, YY_("syntax is ambiguous"));
      return yyabort;
    }

#if YYDEBUG
    /* Print YYS (possibly NULL) and its predecessors. */
    void
    yypstates (const glr_state* yys) const
    {
      if (yys != YY_NULLPTR)
        yys->yy_yypstack();
      else
        std::cerr << "<null>";
      std::cerr << '\n';
    }
#endif

  private:
    size_t spaceLeft() const
    {
      return yyitems.capacity() - yyitems.size();
    }

    /** Return a fresh GLRStackItem in this.  The item is an LR state
     *  if YYIS_STATE, and otherwise a semantic option.  Callers should call
     *  yyreserveStack afterwards to make sure there is sufficient
     *  headroom.  */
    size_t
    yynewGLRStackItem (bool yyis_state)
    {
      YYDASSERT(yyitems.size() < yyitems.capacity());
      yyitems.push_back(glr_stack_item(yyis_state));
      return yyitems.size() - 1;
    }


  public:
    std::vector<glr_stack_item> yyitems;
    // Where the stack splits. Anything below this address is deterministic.
    const glr_state* yysplitPoint;
    glr_state_set yytops;
  }; // class state_stack
} // namespace

#undef YYFILL
#define YYFILL(N) yystateStack.yyfill (yyvsp, yylow, (N), yynormal)

namespace yy
{
  class parser::glr_stack
  {
  public:

    // Needs access to yypact_value_is_default, etc.
    friend context;


    glr_stack (size_t yysize, parser_type& yyparser_yyarg, void * scanner_yyarg, nix::ParseData * data_yyarg)
      : yyerrState (0)
      , yystateStack (yysize)
      , yyerrcnt (0)
      , yyla ()
      , yyparser (yyparser_yyarg),
      scanner (scanner_yyarg),
      data (data_yyarg)
    {}

    ~glr_stack ()
    {
      if (!this->yyla.empty ())
        yyparser.yy_destroy_ ("Cleanup: discarding lookahead",
                              this->yyla.kind (), this->yyla.value, this->yyla.location);
      popall_ ();
    }

    int yyerrState;
  /* To compute the location of the error token.  */
    glr_stack_item yyerror_range[3];
    state_stack yystateStack;
    int yyerrcnt;
    symbol_type yyla;
    YYJMP_BUF yyexception_buffer;
    parser_type& yyparser;

  #define YYCHK1(YYE)                                                          \
    do {                                                                       \
      switch (YYE) {                                                           \
      case yyok:                                                               \
        break;                                                                 \
      case yyabort:                                                            \
        goto yyabortlab;                                                       \
      case yyaccept:                                                           \
        goto yyacceptlab;                                                      \
      case yyerr:                                                              \
        goto yyuser_error;                                                     \
      default:                                                                 \
        goto yybuglab;                                                         \
      }                                                                        \
    } while (false)

    int
    parse ()
    {
      int yyresult;
      size_t yyposn;

      YYCDEBUG << "Starting parse\n";

      this->yyla.clear ();

      switch (YYSETJMP (this->yyexception_buffer))
        {
        case 0: break;
        case 1: goto yyabortlab;
        case 2: goto yyexhaustedlab;
        default: goto yybuglab;
        }
      this->yyglrShift (create_state_set_index(0), 0, 0, this->yyla.value, this->yyla.location);
      yyposn = 0;

      while (true)
        {
          /* For efficiency, we have two loops, the first of which is
             specialized to deterministic operation (single stack, no
             potential ambiguity).  */
          /* Standard mode */
          while (true)
            {
              const state_num yystate = this->firstTopState()->yylrState;
              YYCDEBUG << "Entering state " << yystate << '\n';
              if (yystate == YYFINAL)
                goto yyacceptlab;
              if (yy_is_defaulted_state (yystate))
                {
                  const rule_num yyrule = yy_default_action (yystate);
                  if (yyrule == 0)
                    {
                      this->yyerror_range[1].getState().yyloc = this->yyla.location;
                      this->yyreportSyntaxError ();
                      goto yyuser_error;
                    }
                  YYCHK1 (this->yyglrReduce (create_state_set_index(0), yyrule, true));
                }
              else
                {
                  yyget_token ();
                  const short* yyconflicts;
                  const int yyaction = yygetLRActions (yystate, this->yyla.kind (), yyconflicts);
                  if (*yyconflicts != 0)
                    break;
                  if (yy_is_shift_action (yyaction))
                    {
                      YY_SYMBOL_PRINT ("Shifting", this->yyla.kind (), this->yyla.value, this->yyla.location);
                      yyposn += 1;
                      // FIXME: we should move yylval.
                      this->yyglrShift (create_state_set_index(0), yyaction, yyposn, this->yyla.value, this->yyla.location);
                      yyla.clear ();
                      if (0 < this->yyerrState)
                        this->yyerrState -= 1;
                    }
                  else if (yy_is_error_action (yyaction))
                    {
                      this->yyerror_range[1].getState().yyloc = this->yyla.location;
                      /* Don't issue an error message again for exceptions
                         thrown from the scanner.  */
                      if (this->yyla.kind () != symbol_kind::S_YYerror)
                        this->yyreportSyntaxError ();
                      goto yyuser_error;
                    }
                  else
                    YYCHK1 (this->yyglrReduce (create_state_set_index(0), -yyaction, true));
                }
            }

          while (true)
            {
              for (state_set_index yys = create_state_set_index(0); yys.uget() < this->yystateStack.numTops(); ++yys)
                this->yystateStack.yytops.setLookaheadNeeds(yys, !this->yyla.empty ());

              /* yyprocessOneStack returns one of three things:

                  - An error flag.  If the caller is yyprocessOneStack, it
                    immediately returns as well.  When the caller is finally
                    yyparse, it jumps to an error label via YYCHK1.

                  - yyok, but yyprocessOneStack has invoked yymarkStackDeleted
                    (yys), which sets the top state of yys to NULL.  Thus,
                    yyparse's following invocation of yyremoveDeletes will remove
                    the stack.

                  - yyok, when ready to shift a token.

                 Except in the first case, yyparse will invoke yyremoveDeletes and
                 then shift the next token onto all remaining stacks.  This
                 synchronization of the shift (that is, after all preceding
                 reductions on all stacks) helps prevent double destructor calls
                 on yylval in the event of memory exhaustion.  */

              for (state_set_index yys = create_state_set_index (0); yys.uget () < this->yystateStack.numTops (); ++yys)
                YYCHK1 (this->yyprocessOneStack (yys, yyposn, &this->yyla.location));
              this->yystateStack.yytops.yyremoveDeletes ();
              if (this->yystateStack.yytops.size() == 0)
                {
                  this->yystateStack.yytops.yyundeleteLastStack ();
                  if (this->yystateStack.yytops.size() == 0)
                    this->yyFail (&this->yyla.location, YY_("syntax error"));
                  YYCHK1 (this->yyresolveStack ());
                  YYCDEBUG << "Returning to deterministic operation.\n";
                  this->yyerror_range[1].getState ().yyloc = this->yyla.location;
                  this->yyreportSyntaxError ();
                  goto yyuser_error;
                }

              /* If any yyglrShift call fails, it will fail after shifting.  Thus,
                 a copy of yylval will already be on stack 0 in the event of a
                 failure in the following loop.  Thus, yyla is emptied
                 before the loop to make sure the user destructor for yylval isn't
                 called twice.  */
              symbol_kind_type yytoken_to_shift = this->yyla.kind ();
              this->yyla.kind_ = symbol_kind::S_YYEMPTY;
              yyposn += 1;
              for (state_set_index yys = create_state_set_index (0); yys.uget () < this->yystateStack.numTops (); ++yys)
                {
                  const state_num yystate = this->topState (yys)->yylrState;
                  const short* yyconflicts;
                  const int yyaction = yygetLRActions (yystate, yytoken_to_shift, yyconflicts);
                  /* Note that yyconflicts were handled by yyprocessOneStack.  */
                  YYCDEBUG << "On stack " << yys.get() << ", ";
                  YY_SYMBOL_PRINT ("shifting", yytoken_to_shift, this->yyla.value, this->yyla.location);
                  this->yyglrShift (yys, yyaction, yyposn, this->yyla.value, this->yyla.location);
                  YYCDEBUG << "Stack " << yys.get() << " now in state "
                           << this->topState(yys)->yylrState << '\n';
                }


              if (this->yystateStack.yytops.size () == 1)
                {
                  YYCHK1 (this->yyresolveStack ());
                  YYCDEBUG << "Returning to deterministic operation.\n";
                  this->yystateStack.yycompressStack ();
                  break;
                }
            }
          continue;
        yyuser_error:
          this->yyrecoverSyntaxError (&this->yyla.location);
          yyposn = this->firstTopState()->yyposn;
        }

    yyacceptlab:
      yyresult = 0;
      goto yyreturn;

    yybuglab:
      YYASSERT (false);
      goto yyabortlab;

    yyabortlab:
      yyresult = 1;
      goto yyreturn;

    yyexhaustedlab:
      yyparser.error (this->yyla.location, YY_("memory exhausted"));
      yyresult = 2;
      goto yyreturn;

    yyreturn:
      return yyresult;
    }
  #undef YYCHK1

    void yyreserveGlrStack ()
    {
      if (!yystateStack.yyexpandGLRStackIfNeeded ())
        yyMemoryExhausted ();
    }

    _Noreturn void
    yyMemoryExhausted ()
    {
      YYLONGJMP (yyexception_buffer, 2);
    }

    _Noreturn void
    yyFail (location_type* yylocp, const char* yymsg)
    {
      if (yymsg != YY_NULLPTR)
        yyparser.error (*yylocp, yymsg);
      YYLONGJMP (yyexception_buffer, 1);
    }

                                  /* GLRStates */


    /** Add a new semantic action that will execute the action for rule
     *  YYRULE on the semantic values in YYRHS to the list of
     *  alternative actions for YYSTATE.  Assumes that YYRHS comes from
     *  stack #YYK of *this. */
    void
    yyaddDeferredAction (state_set_index yyk, glr_state* yystate,
                         glr_state* yyrhs, rule_num yyrule)
    {
      semantic_option& yyopt = yystateStack.yynewSemanticOption (semantic_option (yyrule));
      yyopt.setState (yyrhs);
      yyopt.setNext (yystate->firstVal ());
      if (yystateStack.yytops.lookaheadNeeds (yyk))
        yyopt.yyla = this->yyla;
      yystate->setFirstVal (&yyopt);

      yyreserveGlrStack ();
    }

  #if YYDEBUG
    void yypdumpstack () const
    {
      yystateStack.dumpStack();
    }
  #endif

    void
    yyreportSyntaxError ()
    {
      if (yyerrState != 0)
        return;

      context yyctx (*this, yyla);
      std::string msg = yyparser.yysyntax_error_ (yyctx);
      yyparser.error (yyla.location, YY_MOVE (msg));
      yyerrcnt += 1;
    }

    /* Recover from a syntax error on this, assuming that yytoken,
       yylval, and yylloc are the syntactic category, semantic value, and location
       of the lookahead.  */
    void
    yyrecoverSyntaxError (location_type* yylocp)
    {
      if (yyerrState == 3)
        /* We just shifted the error token and (perhaps) took some
           reductions.  Skip tokens until we can proceed.  */
        while (true)
          {
            if (this->yyla.kind () == symbol_kind::S_YYEOF)
              yyFail (yylocp, YY_NULLPTR);
            if (this->yyla.kind () != symbol_kind::S_YYEMPTY)
              {
                /* We throw away the lookahead, but the error range
                   of the shifted error token must take it into account.  */
                glr_state *yys = firstTopState();
                yyerror_range[1].getState().yyloc = yys->yyloc;
                yyerror_range[2].getState().yyloc = this->yyla.location;
                YYLLOC_DEFAULT ((yys->yyloc), yyerror_range, 2);
                yyparser.yy_destroy_ ("Error: discarding",
                                      this->yyla.kind (), this->yyla.value, this->yyla.location);
                this->yyla.kind_ = symbol_kind::S_YYEMPTY;
              }
            yyget_token ();
            int yyj = yypact[firstTopState()->yylrState];
            if (yypact_value_is_default (yyj))
              return;
            yyj += this->yyla.kind ();
            if (yyj < 0 || YYLAST < yyj || yycheck[yyj] != this->yyla.kind ())
              {
                if (yydefact[firstTopState()->yylrState] != 0)
                  return;
              }
            else if (! yytable_value_is_error (yytable[yyj]))
              return;
          }

      if (!yystateStack.reduceToOneStack())
        yyFail (yylocp, YY_NULLPTR);

      /* Now pop stack until we find a state that shifts the error token.  */
      yyerrState = 3;
      while (firstTopState () != YY_NULLPTR)
        {
          glr_state *yys = firstTopState ();
          int yyj = yypact[yys->yylrState];
          if (! yypact_value_is_default (yyj))
            {
              yyj += YYTERROR;
              if (0 <= yyj && yyj <= YYLAST && yycheck[yyj] == YYTERROR
                  && yy_is_shift_action (yytable[yyj]))
                {
                  /* Shift the error token.  */
                  /* First adjust its location.*/
                  location_type yyerrloc;
                  yyerror_range[2].getState().yyloc = this->yyla.location;
                  YYLLOC_DEFAULT (yyerrloc, (yyerror_range), 2);
                  YY_SYMBOL_PRINT ("Shifting", yy_accessing_symbol (yytable[yyj]),
                                   this->yyla.value, yyerrloc);
                  yyglrShift (create_state_set_index(0), yytable[yyj],
                              yys->yyposn, yyla.value, yyerrloc);
                  yys = firstTopState();
                  break;
                }
            }
          yyerror_range[1].getState().yyloc = yys->yyloc;
          if (yys->pred() != YY_NULLPTR)
            yys->destroy ("Error: popping", yyparser);
          yystateStack.setFirstTop(yys->pred());
          yystateStack.pop_back();
        }
      if (firstTopState() == YY_NULLPTR)
        yyFail (yylocp, YY_NULLPTR);
    }

    YYRESULTTAG
    yyprocessOneStack (state_set_index yyk,
                       size_t yyposn, location_type* yylocp)
    {
      while (yystateStack.topAt(yyk) != YY_NULLPTR)
        {
          const state_num yystate = topState(yyk)->yylrState;
          YYCDEBUG << "Stack " << yyk.get()
                   << " Entering state " << yystate << '\n';

          YYASSERT (yystate != YYFINAL);

          if (yy_is_defaulted_state (yystate))
            {
              const rule_num yyrule = yy_default_action (yystate);
              if (yyrule == 0)
                {
                  YYCDEBUG << "Stack " << yyk.get() << " dies.\n";
                  yystateStack.yytops.yymarkStackDeleted (yyk);
                  return yyok;
                }
              const YYRESULTTAG yyflag
                = yyglrReduce (yyk, yyrule, yyimmediate[yyrule]);
              if (yyflag == yyerr)
                {
                  YYCDEBUG << "Stack " << yyk.get() << " dies"
                    " (predicate failure or explicit user error).\n";
                  yystateStack.yytops.yymarkStackDeleted (yyk);
                  return yyok;
                }
              if (yyflag != yyok)
                return yyflag;
            }
          else
            {
              yystateStack.yytops.setLookaheadNeeds(yyk, true);
              yyget_token ();
              const short* yyconflicts;
              const int yyaction = yygetLRActions (yystate, this->yyla.kind (), yyconflicts);

              for (; *yyconflicts != 0; ++yyconflicts)
                {
                  state_set_index yynewStack = yystateStack.yysplitStack (yyk);
                  YYCDEBUG << "Splitting off stack " << yynewStack.get()
                           << " from " << yyk.get() << ".\n";
                  YYRESULTTAG yyflag =
                    yyglrReduce (yynewStack, *yyconflicts, yyimmediate[*yyconflicts]);
                  if (yyflag == yyok)
                    YYCHK (yyprocessOneStack (yynewStack,
                                              yyposn, yylocp));
                  else if (yyflag == yyerr)
                    {
                      YYCDEBUG << "Stack " << yynewStack.get() << " dies.\n";
                      yystateStack.yytops.yymarkStackDeleted (yynewStack);
                    }
                  else
                    return yyflag;
                }

              if (yy_is_shift_action (yyaction))
                break;
              else if (yy_is_error_action (yyaction))
                {
                  YYCDEBUG << "Stack " << yyk.get() << " dies.\n";
                  yystateStack.yytops.yymarkStackDeleted (yyk);
                  break;
                }
              else
                {
                  YYRESULTTAG yyflag
                    = yyglrReduce (yyk, -yyaction, yyimmediate[-yyaction]);
                  if (yyflag == yyerr)
                    {
                      YYCDEBUG << "Stack " << yyk.get() << " dies"
                        " (predicate failure or explicit user error).\n";
                      yystateStack.yytops.yymarkStackDeleted (yyk);
                      break;
                    }
                  else if (yyflag != yyok)
                    return yyflag;
                }
            }
        }
      return yyok;
    }

    /** Perform user action for rule number YYN, with RHS length YYRHSLEN,
     *  and top stack item YYVSP.  YYVALP points to place to put semantic
     *  value ($$), and yylocp points to place for location information
     *  (@$).  Returns yyok for normal return, yyaccept for YYACCEPT,
     *  yyerr for YYERROR, yyabort for YYABORT.  */
    YYRESULTTAG
    yyuserAction (rule_num yyrule, int yyrhslen, glr_stack_item* yyvsp, state_set_index yyk,
                  value_type* yyvalp, location_type* yylocp)
    {
      bool yynormal YY_ATTRIBUTE_UNUSED = !yystateStack.isSplit();
      int yylow = 1;
  YY_USE (yyvalp);
  YY_USE (yylocp);
  YY_USE (scanner);
  YY_USE (data);
      YY_USE (yyk);
      YY_USE (yyrhslen);
    # undef yyerrok
    # define yyerrok (yyerrState = 0)
    # undef YYACCEPT
    # define YYACCEPT return yyaccept
    # undef YYABORT
    # define YYABORT return yyabort
    # undef YYERROR
    # define YYERROR return yyerrok, yyerr
    # undef YYRECOVERING
    # define YYRECOVERING() (yyerrState != 0)
    # undef yytoken
    # define yytoken this->yyla.kind_
    # undef yyclearin
    # define yyclearin (yytoken = symbol_kind::S_YYEMPTY)
    # undef YYBACKUP
    # define YYBACKUP(Token, Value)                                              \
      return yyparser.error (*yylocp, YY_("syntax error: cannot back up")),     \
             yyerrok, yyerr


      if (yyrhslen == 0)
        *yyvalp = yyval_default;
      else
        *yyvalp = yyvsp[YYFILL (1-yyrhslen)].getState().value ();
      /* Default location. */
      YYLLOC_DEFAULT ((*yylocp), (yyvsp - yyrhslen), yyrhslen);
      yyerror_range[1].getState().yyloc = *yylocp;

    /* If yyk == -1, we are running a deferred action on a temporary
       stack.  In that case, YY_REDUCE_PRINT must not play with YYFILL,
       so pretend the stack is "normal". */
    YY_REDUCE_PRINT ((yynormal || yyk == create_state_set_index (-1), yyvsp, yyk, yyrule, yyparser));
    #if YY_EXCEPTIONS
      try
      {
    #endif // YY_EXCEPTIONS
      switch (yyrule)
        {
      case 2: // start: expr
#line 361 "src/parsers/nix/parser.y"
            { data->result = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e); }
#line 2719 "src/parsers/nix/parser-tab.cc"
    break;

  case 3: // expr: expr_function
#line 363 "src/parsers/nix/parser.y"
      { ((*yyvalp).e) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e); }
#line 2725 "src/parsers/nix/parser-tab.cc"
    break;

  case 4: // expr_function: ID ':' expr_function
#line 367 "src/parsers/nix/parser.y"
    { ((*yyvalp).e) = new ExprLambda(CUR_POS, data->symbols.create(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().id)), 0, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)); }
#line 2731 "src/parsers/nix/parser-tab.cc"
    break;

  case 5: // expr_function: '{' formals '}' ':' expr_function
#line 369 "src/parsers/nix/parser.y"
    { ((*yyvalp).e) = new ExprLambda(CUR_POS, data->symbols.create(""), toFormals(*data, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-3)].getState().value ().formals)), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)); }
#line 2737 "src/parsers/nix/parser-tab.cc"
    break;

  case 6: // expr_function: '{' formals '}' '@' ID ':' expr_function
#line 371 "src/parsers/nix/parser.y"
    {
      Symbol arg = data->symbols.create(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().id));
      ((*yyvalp).e) = new ExprLambda(CUR_POS, arg, toFormals(*data, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-5)].getState().value ().formals), CUR_POS, arg), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e));
    }
#line 2746 "src/parsers/nix/parser-tab.cc"
    break;

  case 7: // expr_function: ID '@' '{' formals '}' ':' expr_function
#line 376 "src/parsers/nix/parser.y"
    {
      Symbol arg = data->symbols.create(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-6)].getState().value ().id));
      ((*yyvalp).e) = new ExprLambda(CUR_POS, arg, toFormals(*data, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-3)].getState().value ().formals), CUR_POS, arg), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e));
    }
#line 2755 "src/parsers/nix/parser-tab.cc"
    break;

  case 8: // expr_function: ASSERT expr ';' expr_function
#line 381 "src/parsers/nix/parser.y"
    { ((*yyvalp).e) = new ExprAssert(CUR_POS, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)); }
#line 2761 "src/parsers/nix/parser-tab.cc"
    break;

  case 9: // expr_function: WITH expr ';' expr_function
#line 383 "src/parsers/nix/parser.y"
    { ((*yyvalp).e) = new ExprWith(CUR_POS, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)); }
#line 2767 "src/parsers/nix/parser-tab.cc"
    break;

  case 10: // expr_function: LET binds IN expr_function
#line 385 "src/parsers/nix/parser.y"
    { if (!((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().attrs)->dynamicAttrs.empty())
        throw ParseError({
            .msg = hintfmt("dynamic attributes not allowed in let"),
            .errPos = CUR_POS
        });
      ((*yyvalp).e) = new ExprLet(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().attrs), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e));
    }
#line 2779 "src/parsers/nix/parser-tab.cc"
    break;

  case 11: // expr_function: expr_if
#line 392 "src/parsers/nix/parser.y"
    { ((*yyvalp).e) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e); }
#line 2785 "src/parsers/nix/parser-tab.cc"
    break;

  case 12: // expr_if: IF expr THEN expr ELSE expr
#line 396 "src/parsers/nix/parser.y"
                                { ((*yyvalp).e) = new ExprIf(CUR_POS, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-4)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)); }
#line 2791 "src/parsers/nix/parser-tab.cc"
    break;

  case 13: // expr_if: expr_op
#line 397 "src/parsers/nix/parser.y"
    { ((*yyvalp).e) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e); }
#line 2797 "src/parsers/nix/parser-tab.cc"
    break;

  case 14: // expr_op: '!' expr_op
#line 401 "src/parsers/nix/parser.y"
                          { ((*yyvalp).e) = new ExprOpNot(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)); }
#line 2803 "src/parsers/nix/parser-tab.cc"
    break;

  case 15: // expr_op: '-' expr_op
#line 402 "src/parsers/nix/parser.y"
                             { ((*yyvalp).e) = new ExprCall(CUR_POS, new ExprVar(data->symbols.create("__sub")), {new ExprInt(0), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)}); }
#line 2809 "src/parsers/nix/parser-tab.cc"
    break;

  case 16: // expr_op: expr_op EQ expr_op
#line 403 "src/parsers/nix/parser.y"
                       { ((*yyvalp).e) = new ExprOpEq(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)); }
#line 2815 "src/parsers/nix/parser-tab.cc"
    break;

  case 17: // expr_op: expr_op NEQ expr_op
#line 404 "src/parsers/nix/parser.y"
                        { ((*yyvalp).e) = new ExprOpNEq(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)); }
#line 2821 "src/parsers/nix/parser-tab.cc"
    break;

  case 18: // expr_op: expr_op '<' expr_op
#line 405 "src/parsers/nix/parser.y"
                        { ((*yyvalp).e) = new ExprCall(CUR_POS, new ExprVar(data->symbols.create("__lessThan")), {((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)}); }
#line 2827 "src/parsers/nix/parser-tab.cc"
    break;

  case 19: // expr_op: expr_op LEQ expr_op
#line 406 "src/parsers/nix/parser.y"
                        { ((*yyvalp).e) = new ExprOpNot(new ExprCall(CUR_POS, new ExprVar(data->symbols.create("__lessThan")), {((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e)})); }
#line 2833 "src/parsers/nix/parser-tab.cc"
    break;

  case 20: // expr_op: expr_op '>' expr_op
#line 407 "src/parsers/nix/parser.y"
                        { ((*yyvalp).e) = new ExprCall(CUR_POS, new ExprVar(data->symbols.create("__lessThan")), {((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e)}); }
#line 2839 "src/parsers/nix/parser-tab.cc"
    break;

  case 21: // expr_op: expr_op GEQ expr_op
#line 408 "src/parsers/nix/parser.y"
                        { ((*yyvalp).e) = new ExprOpNot(new ExprCall(CUR_POS, new ExprVar(data->symbols.create("__lessThan")), {((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)})); }
#line 2845 "src/parsers/nix/parser-tab.cc"
    break;

  case 22: // expr_op: expr_op AND expr_op
#line 409 "src/parsers/nix/parser.y"
                        { ((*yyvalp).e) = new ExprOpAnd(CUR_POS, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)); }
#line 2851 "src/parsers/nix/parser-tab.cc"
    break;

  case 23: // expr_op: expr_op OR expr_op
#line 410 "src/parsers/nix/parser.y"
                       { ((*yyvalp).e) = new ExprOpOr(CUR_POS, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)); }
#line 2857 "src/parsers/nix/parser-tab.cc"
    break;

  case 24: // expr_op: expr_op IMPL expr_op
#line 411 "src/parsers/nix/parser.y"
                         { ((*yyvalp).e) = new ExprOpImpl(CUR_POS, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)); }
#line 2863 "src/parsers/nix/parser-tab.cc"
    break;

  case 25: // expr_op: expr_op UPDATE expr_op
#line 412 "src/parsers/nix/parser.y"
                           { ((*yyvalp).e) = new ExprOpUpdate(CUR_POS, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)); }
#line 2869 "src/parsers/nix/parser-tab.cc"
    break;

  case 26: // expr_op: expr_op '?' attrpath
#line 413 "src/parsers/nix/parser.y"
                         { ((*yyvalp).e) = new ExprOpHasAttr(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), *((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().attrNames)); }
#line 2875 "src/parsers/nix/parser-tab.cc"
    break;

  case 27: // expr_op: expr_op '+' expr_op
#line 415 "src/parsers/nix/parser.y"
    { ((*yyvalp).e) = new ExprConcatStrings(CUR_POS, false, new vector<std::pair<Pos, Expr *> >({{makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().yyloc), data), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e)}, {makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().yyloc), data), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)}})); }
#line 2881 "src/parsers/nix/parser-tab.cc"
    break;

  case 28: // expr_op: expr_op '-' expr_op
#line 416 "src/parsers/nix/parser.y"
                        { ((*yyvalp).e) = new ExprCall(CUR_POS, new ExprVar(data->symbols.create("__sub")), {((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)}); }
#line 2887 "src/parsers/nix/parser-tab.cc"
    break;

  case 29: // expr_op: expr_op '*' expr_op
#line 417 "src/parsers/nix/parser.y"
                        { ((*yyvalp).e) = new ExprCall(CUR_POS, new ExprVar(data->symbols.create("__mul")), {((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)}); }
#line 2893 "src/parsers/nix/parser-tab.cc"
    break;

  case 30: // expr_op: expr_op '/' expr_op
#line 418 "src/parsers/nix/parser.y"
                        { ((*yyvalp).e) = new ExprCall(CUR_POS, new ExprVar(data->symbols.create("__div")), {((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)}); }
#line 2899 "src/parsers/nix/parser-tab.cc"
    break;

  case 31: // expr_op: expr_op CONCAT expr_op
#line 419 "src/parsers/nix/parser.y"
                           { ((*yyvalp).e) = new ExprOpConcatLists(CUR_POS, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)); }
#line 2905 "src/parsers/nix/parser-tab.cc"
    break;

  case 32: // expr_op: expr_app
#line 420 "src/parsers/nix/parser.y"
    { ((*yyvalp).e) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e); }
#line 2911 "src/parsers/nix/parser-tab.cc"
    break;

  case 33: // expr_app: expr_app expr_select
#line 424 "src/parsers/nix/parser.y"
                         {
      if (auto e2 = dynamic_cast<ExprCall *>(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().e))) {
          e2->args.push_back(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e));
          ((*yyvalp).e) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().e);
      } else
          ((*yyvalp).e) = new ExprCall(CUR_POS, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().e), {((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)});
  }
#line 2923 "src/parsers/nix/parser-tab.cc"
    break;

  case 34: // expr_app: expr_select
#line 431 "src/parsers/nix/parser.y"
    { ((*yyvalp).e) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e); }
#line 2929 "src/parsers/nix/parser-tab.cc"
    break;

  case 35: // expr_select: expr_simple '.' attrpath
#line 436 "src/parsers/nix/parser.y"
    { ((*yyvalp).e) = new ExprSelect(CUR_POS, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e), *((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().attrNames), 0); }
#line 2935 "src/parsers/nix/parser-tab.cc"
    break;

  case 36: // expr_select: expr_simple '.' attrpath OR_KW expr_select
#line 438 "src/parsers/nix/parser.y"
    { ((*yyvalp).e) = new ExprSelect(CUR_POS, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-4)].getState().value ().e), *((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().attrNames), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)); }
#line 2941 "src/parsers/nix/parser-tab.cc"
    break;

  case 37: // expr_select: expr_simple OR_KW
#line 442 "src/parsers/nix/parser.y"
    { ((*yyvalp).e) = new ExprCall(CUR_POS, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().e), {new ExprVar(CUR_POS, data->symbols.create("or"))}); }
#line 2947 "src/parsers/nix/parser-tab.cc"
    break;

  case 38: // expr_select: expr_simple
#line 443 "src/parsers/nix/parser.y"
                { ((*yyvalp).e) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e); }
#line 2953 "src/parsers/nix/parser-tab.cc"
    break;

  case 39: // expr_simple: ID
#line 447 "src/parsers/nix/parser.y"
       {
      std::string_view s = "__curPos";
      if (((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().id).l == s.size() && strncmp(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().id).p, s.data(), s.size()) == 0)
          ((*yyvalp).e) = new ExprPos(CUR_POS);
      else
          ((*yyvalp).e) = new ExprVar(CUR_POS, data->symbols.create(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().id)));
  }
#line 2965 "src/parsers/nix/parser-tab.cc"
    break;

  case 40: // expr_simple: INT
#line 454 "src/parsers/nix/parser.y"
        { ((*yyvalp).e) = new ExprInt(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().n)); }
#line 2971 "src/parsers/nix/parser-tab.cc"
    break;

  case 41: // expr_simple: FLOAT
#line 455 "src/parsers/nix/parser.y"
          { ((*yyvalp).e) = new ExprFloat(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().nf)); }
#line 2977 "src/parsers/nix/parser-tab.cc"
    break;

  case 42: // expr_simple: '"' string_parts '"'
#line 456 "src/parsers/nix/parser.y"
                         { ((*yyvalp).e) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().e); }
#line 2983 "src/parsers/nix/parser-tab.cc"
    break;

  case 43: // expr_simple: IND_STRING_OPEN ind_string_parts IND_STRING_CLOSE
#line 457 "src/parsers/nix/parser.y"
                                                      {
      ((*yyvalp).e) = stripIndentation(CUR_POS, data->symbols, *((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().ind_string_parts));
  }
#line 2991 "src/parsers/nix/parser-tab.cc"
    break;

  case 44: // expr_simple: path_start PATH_END
#line 460 "src/parsers/nix/parser.y"
                        { ((*yyvalp).e) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().e); }
#line 2997 "src/parsers/nix/parser-tab.cc"
    break;

  case 45: // expr_simple: path_start string_parts_interpolated PATH_END
#line 461 "src/parsers/nix/parser.y"
                                                  {
      ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().string_parts)->insert(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().string_parts)->begin(), {makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().yyloc), data), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().e)});
      ((*yyvalp).e) = new ExprConcatStrings(CUR_POS, false, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().string_parts));
  }
#line 3006 "src/parsers/nix/parser-tab.cc"
    break;

  case 46: // expr_simple: SPATH
#line 465 "src/parsers/nix/parser.y"
          {
      string path(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().path).p + 1, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().path).l - 2);
      ((*yyvalp).e) = new ExprCall(CUR_POS,
          new ExprVar(data->symbols.create("__findFile")),
          {new ExprVar(data->symbols.create("__nixPath")),
           new ExprString(path)});
  }
#line 3018 "src/parsers/nix/parser-tab.cc"
    break;

  case 47: // expr_simple: URI
#line 472 "src/parsers/nix/parser.y"
        {
      static bool noURLLiterals = settings.isExperimentalFeatureEnabled(Xp::NoUrlLiterals);
      if (noURLLiterals)
          throw ParseError({
              .msg = hintfmt("URL literals are disabled"),
              .errPos = CUR_POS
          });
      ((*yyvalp).e) = new ExprString(string(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().uri)));
  }
#line 3032 "src/parsers/nix/parser-tab.cc"
    break;

  case 48: // expr_simple: '(' expr ')'
#line 481 "src/parsers/nix/parser.y"
                 { ((*yyvalp).e) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().e); }
#line 3038 "src/parsers/nix/parser-tab.cc"
    break;

  case 49: // expr_simple: LET '{' binds '}'
#line 485 "src/parsers/nix/parser.y"
    { ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().attrs)->recursive = true; ((*yyvalp).e) = new ExprSelect(noPos, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().attrs), data->symbols.create("body")); }
#line 3044 "src/parsers/nix/parser-tab.cc"
    break;

  case 50: // expr_simple: REC '{' binds '}'
#line 487 "src/parsers/nix/parser.y"
    { ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().attrs)->recursive = true; ((*yyvalp).e) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().attrs); }
#line 3050 "src/parsers/nix/parser-tab.cc"
    break;

  case 51: // expr_simple: '{' binds '}'
#line 489 "src/parsers/nix/parser.y"
    { ((*yyvalp).e) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().attrs); }
#line 3056 "src/parsers/nix/parser-tab.cc"
    break;

  case 52: // expr_simple: '[' expr_list ']'
#line 490 "src/parsers/nix/parser.y"
                      { ((*yyvalp).e) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().list); }
#line 3062 "src/parsers/nix/parser-tab.cc"
    break;

  case 53: // string_parts: STR
#line 494 "src/parsers/nix/parser.y"
        { ((*yyvalp).e) = new ExprString(string(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().str))); }
#line 3068 "src/parsers/nix/parser-tab.cc"
    break;

  case 54: // string_parts: string_parts_interpolated
#line 495 "src/parsers/nix/parser.y"
                              { ((*yyvalp).e) = new ExprConcatStrings(CUR_POS, true, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().string_parts)); }
#line 3074 "src/parsers/nix/parser-tab.cc"
    break;

  case 55: // string_parts: %empty
#line 496 "src/parsers/nix/parser.y"
    { ((*yyvalp).e) = new ExprString(""); }
#line 3080 "src/parsers/nix/parser-tab.cc"
    break;

  case 56: // string_parts_interpolated: string_parts_interpolated STR
#line 501 "src/parsers/nix/parser.y"
  { ((*yyvalp).string_parts) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().string_parts); ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().string_parts)->emplace_back(makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().yyloc), data), new ExprString(string(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().str)))); }
#line 3086 "src/parsers/nix/parser-tab.cc"
    break;

  case 57: // string_parts_interpolated: string_parts_interpolated DOLLAR_CURLY expr '}'
#line 502 "src/parsers/nix/parser.y"
                                                    { ((*yyvalp).string_parts) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-3)].getState().value ().string_parts); ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-3)].getState().value ().string_parts)->emplace_back(makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().yyloc), data), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().e)); }
#line 3092 "src/parsers/nix/parser-tab.cc"
    break;

  case 58: // string_parts_interpolated: DOLLAR_CURLY expr '}'
#line 503 "src/parsers/nix/parser.y"
                          { ((*yyvalp).string_parts) = new vector<std::pair<Pos, Expr *> >; ((*yyvalp).string_parts)->emplace_back(makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().yyloc), data), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().e)); }
#line 3098 "src/parsers/nix/parser-tab.cc"
    break;

  case 59: // string_parts_interpolated: STR DOLLAR_CURLY expr '}'
#line 504 "src/parsers/nix/parser.y"
                              {
      ((*yyvalp).string_parts) = new vector<std::pair<Pos, Expr *> >;
      ((*yyvalp).string_parts)->emplace_back(makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-3)].getState().yyloc), data), new ExprString(string(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-3)].getState().value ().str))));
      ((*yyvalp).string_parts)->emplace_back(makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().yyloc), data), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().e));
    }
#line 3108 "src/parsers/nix/parser-tab.cc"
    break;

  case 60: // path_start: PATH
#line 512 "src/parsers/nix/parser.y"
         {
    Path path(absPath({((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().path).p, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().path).l}, data->basePath));
    /* add back in the trailing '/' to the first segment */
    if (((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().path).p[((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().path).l-1] == '/' && ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().path).l > 1)
      path += "/";
    ((*yyvalp).e) = new ExprPath(path);
  }
#line 3120 "src/parsers/nix/parser-tab.cc"
    break;

  case 61: // path_start: HPATH
#line 519 "src/parsers/nix/parser.y"
          {
    Path path(getHome() + string(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().path).p + 1, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().path).l - 1));
    ((*yyvalp).e) = new ExprPath(path);
  }
#line 3129 "src/parsers/nix/parser-tab.cc"
    break;

  case 62: // ind_string_parts: ind_string_parts IND_STR
#line 526 "src/parsers/nix/parser.y"
                             { ((*yyvalp).ind_string_parts) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().ind_string_parts); ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().ind_string_parts)->emplace_back(makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().yyloc), data), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().str)); }
#line 3135 "src/parsers/nix/parser-tab.cc"
    break;

  case 63: // ind_string_parts: ind_string_parts DOLLAR_CURLY expr '}'
#line 527 "src/parsers/nix/parser.y"
                                           { ((*yyvalp).ind_string_parts) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-3)].getState().value ().ind_string_parts); ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-3)].getState().value ().ind_string_parts)->emplace_back(makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().yyloc), data), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().e)); }
#line 3141 "src/parsers/nix/parser-tab.cc"
    break;

  case 64: // ind_string_parts: %empty
#line 528 "src/parsers/nix/parser.y"
    { ((*yyvalp).ind_string_parts) = new vector<std::pair<Pos, std::variant<Expr *, StringToken> > >; }
#line 3147 "src/parsers/nix/parser-tab.cc"
    break;

  case 65: // binds: binds attrpath '=' expr ';'
#line 532 "src/parsers/nix/parser.y"
                                { ((*yyvalp).attrs) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-4)].getState().value ().attrs); addAttr(((*yyvalp).attrs), *((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-3)].getState().value ().attrNames), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().e), makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-3)].getState().yyloc), data)); }
#line 3153 "src/parsers/nix/parser-tab.cc"
    break;

  case 66: // binds: binds INHERIT attrs ';'
#line 534 "src/parsers/nix/parser.y"
    { ((*yyvalp).attrs) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-3)].getState().value ().attrs);
      for (auto & i : *((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().attrNames)) {
          if (((*yyvalp).attrs)->attrs.find(i.symbol) != ((*yyvalp).attrs)->attrs.end())
              dupAttr(i.symbol, makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().yyloc), data), ((*yyvalp).attrs)->attrs[i.symbol].pos);
          Pos pos = makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().yyloc), data);
          ((*yyvalp).attrs)->attrs.emplace(i.symbol, ExprAttrs::AttrDef(new ExprVar(CUR_POS, i.symbol), pos, true));
      }
    }
#line 3166 "src/parsers/nix/parser-tab.cc"
    break;

  case 67: // binds: binds INHERIT '(' expr ')' attrs ';'
#line 543 "src/parsers/nix/parser.y"
    { ((*yyvalp).attrs) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-6)].getState().value ().attrs);
      /* !!! Should ensure sharing of the expression in $4. */
      for (auto & i : *((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().attrNames)) {
          if (((*yyvalp).attrs)->attrs.find(i.symbol) != ((*yyvalp).attrs)->attrs.end())
              dupAttr(i.symbol, makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().yyloc), data), ((*yyvalp).attrs)->attrs[i.symbol].pos);
          ((*yyvalp).attrs)->attrs.emplace(i.symbol, ExprAttrs::AttrDef(new ExprSelect(CUR_POS, ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-3)].getState().value ().e), i.symbol), makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().yyloc), data)));
      }
    }
#line 3179 "src/parsers/nix/parser-tab.cc"
    break;

  case 68: // binds: %empty
#line 551 "src/parsers/nix/parser.y"
    { ((*yyvalp).attrs) = new ExprAttrs(makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().yyloc), data)); }
#line 3185 "src/parsers/nix/parser-tab.cc"
    break;

  case 69: // attrs: attrs attr
#line 555 "src/parsers/nix/parser.y"
               { ((*yyvalp).attrNames) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().attrNames); ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().attrNames)->push_back(AttrName(data->symbols.create(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().id)))); }
#line 3191 "src/parsers/nix/parser-tab.cc"
    break;

  case 70: // attrs: attrs string_attr
#line 557 "src/parsers/nix/parser.y"
    { ((*yyvalp).attrNames) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().attrNames);
      ExprString * str = dynamic_cast<ExprString *>(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e));
      if (str) {
          ((*yyvalp).attrNames)->push_back(AttrName(data->symbols.create(str->s)));
          delete str;
      } else
          throw ParseError({
              .msg = hintfmt("dynamic attributes not allowed in inherit"),
              .errPos = makeCurPos(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().yyloc), data)
          });
    }
#line 3207 "src/parsers/nix/parser-tab.cc"
    break;

  case 71: // attrs: %empty
#line 568 "src/parsers/nix/parser.y"
    { ((*yyvalp).attrNames) = new AttrPath; }
#line 3213 "src/parsers/nix/parser-tab.cc"
    break;

  case 72: // attrpath: attrpath '.' attr
#line 572 "src/parsers/nix/parser.y"
                      { ((*yyvalp).attrNames) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().attrNames); ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().attrNames)->push_back(AttrName(data->symbols.create(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().id)))); }
#line 3219 "src/parsers/nix/parser-tab.cc"
    break;

  case 73: // attrpath: attrpath '.' string_attr
#line 574 "src/parsers/nix/parser.y"
    { ((*yyvalp).attrNames) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().attrNames);
      ExprString * str = dynamic_cast<ExprString *>(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e));
      if (str) {
          ((*yyvalp).attrNames)->push_back(AttrName(data->symbols.create(str->s)));
          delete str;
      } else
          ((*yyvalp).attrNames)->push_back(AttrName(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)));
    }
#line 3232 "src/parsers/nix/parser-tab.cc"
    break;

  case 74: // attrpath: attr
#line 582 "src/parsers/nix/parser.y"
         { ((*yyvalp).attrNames) = new vector<AttrName>; ((*yyvalp).attrNames)->push_back(AttrName(data->symbols.create(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().id)))); }
#line 3238 "src/parsers/nix/parser-tab.cc"
    break;

  case 75: // attrpath: string_attr
#line 584 "src/parsers/nix/parser.y"
    { ((*yyvalp).attrNames) = new vector<AttrName>;
      ExprString *str = dynamic_cast<ExprString *>(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e));
      if (str) {
          ((*yyvalp).attrNames)->push_back(AttrName(data->symbols.create(str->s)));
          delete str;
      } else
          ((*yyvalp).attrNames)->push_back(AttrName(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)));
    }
#line 3251 "src/parsers/nix/parser-tab.cc"
    break;

  case 76: // attr: ID
#line 595 "src/parsers/nix/parser.y"
       { ((*yyvalp).id) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().id); }
#line 3257 "src/parsers/nix/parser-tab.cc"
    break;

  case 77: // attr: OR_KW
#line 596 "src/parsers/nix/parser.y"
          { ((*yyvalp).id) = {"or", 2}; }
#line 3263 "src/parsers/nix/parser-tab.cc"
    break;

  case 78: // string_attr: '"' string_parts '"'
#line 600 "src/parsers/nix/parser.y"
                         { ((*yyvalp).e) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().e); }
#line 3269 "src/parsers/nix/parser-tab.cc"
    break;

  case 79: // string_attr: DOLLAR_CURLY expr '}'
#line 601 "src/parsers/nix/parser.y"
                          { ((*yyvalp).e) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().e); }
#line 3275 "src/parsers/nix/parser-tab.cc"
    break;

  case 80: // expr_list: expr_list expr_select
#line 605 "src/parsers/nix/parser.y"
                          { ((*yyvalp).list) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().list); ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-1)].getState().value ().list)->elems.push_back(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)); /* !!! dangerous */ }
#line 3281 "src/parsers/nix/parser-tab.cc"
    break;

  case 81: // expr_list: %empty
#line 606 "src/parsers/nix/parser.y"
    { ((*yyvalp).list) = new ExprList; }
#line 3287 "src/parsers/nix/parser-tab.cc"
    break;

  case 82: // formals: formal ',' formals
#line 611 "src/parsers/nix/parser.y"
    { ((*yyvalp).formals) = ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().formals); ((*yyvalp).formals)->formals.push_back(*((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().formal)); }
#line 3293 "src/parsers/nix/parser-tab.cc"
    break;

  case 83: // formals: formal
#line 613 "src/parsers/nix/parser.y"
    { ((*yyvalp).formals) = new ParserFormals; ((*yyvalp).formals)->formals.push_back(*((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().formal)); ((*yyvalp).formals)->ellipsis = false; }
#line 3299 "src/parsers/nix/parser-tab.cc"
    break;

  case 84: // formals: %empty
#line 615 "src/parsers/nix/parser.y"
    { ((*yyvalp).formals) = new ParserFormals; ((*yyvalp).formals)->ellipsis = false; }
#line 3305 "src/parsers/nix/parser-tab.cc"
    break;

  case 85: // formals: ELLIPSIS
#line 617 "src/parsers/nix/parser.y"
    { ((*yyvalp).formals) = new ParserFormals; ((*yyvalp).formals)->ellipsis = true; }
#line 3311 "src/parsers/nix/parser-tab.cc"
    break;

  case 86: // formal: ID
#line 621 "src/parsers/nix/parser.y"
       { ((*yyvalp).formal) = new Formal(CUR_POS, data->symbols.create(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().id)), 0); }
#line 3317 "src/parsers/nix/parser-tab.cc"
    break;

  case 87: // formal: ID '?' expr
#line 622 "src/parsers/nix/parser.y"
                { ((*yyvalp).formal) = new Formal(CUR_POS, data->symbols.create(((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (-2)].getState().value ().id)), ((static_cast<glr_stack_item const *>(yyvsp))[YYFILL (0)].getState().value ().e)); }
#line 3323 "src/parsers/nix/parser-tab.cc"
    break;


#line 3327 "src/parsers/nix/parser-tab.cc"

          default: break;
        }
    #if YY_EXCEPTIONS
      }
      catch (const syntax_error& yyexc)
        {
          YYCDEBUG << "Caught exception: " << yyexc.what() << '\n';
          *yylocp = yyexc.location;
          yyparser.error (*yylocp, yyexc.what ());
          YYERROR;
        }
    #endif // YY_EXCEPTIONS
    YY_SYMBOL_PRINT ("-> $$ =", yylhsNonterm (yyrule), *yyvalp, *yylocp);

      return yyok;
    # undef yyerrok
    # undef YYABORT
    # undef YYACCEPT
    # undef YYERROR
    # undef YYBACKUP
    # undef yytoken
    # undef yyclearin
    # undef YYRECOVERING
    }

    YYRESULTTAG
    yyresolveStack ()
    {
      if (yystateStack.isSplit ())
        {
          int yyn = 0;
          for (glr_state* yys = firstTopState ();
               yys != yystateStack.yysplitPoint;
               yys = yys->pred ())
            yyn += 1;
          YYCHK (yyresolveStates (*firstTopState (), yyn));
        }
      return yyok;
    }

    /** Pop the symbols consumed by reduction #YYRULE from the top of stack
     *  #YYK of *YYSTACKP, and perform the appropriate semantic action on their
     *  semantic values.  Assumes that all ambiguities in semantic values
     *  have been previously resolved.  Set *YYVALP to the resulting value,
     *  and *YYLOCP to the computed location (if any).  Return value is as
     *  for userAction.  */
    YYRESULTTAG
    yydoAction (state_set_index yyk, rule_num yyrule,
                value_type* yyvalp, location_type* yylocp)
    {
      const int yynrhs = yyrhsLength (yyrule);

      if (!yystateStack.isSplit())
        {
          /* Standard special case: single stack.  */
          YYASSERT (yyk.get() == 0);
          glr_stack_item* yyrhs = yystateStack.firstTop()->asItem();
          const YYRESULTTAG res
            = yyuserAction (yyrule, yynrhs, yyrhs, yyk, yyvalp, yylocp);
          yystateStack.pop_back(static_cast<size_t>(yynrhs));
          yystateStack.setFirstTop(&yystateStack[yystateStack.size() - 1].getState());
          return res;
        }
      else
        {
          glr_stack_item yyrhsVals[YYMAXRHS + YYMAXLEFT + 1];
          glr_state* yys = yystateStack.topAt(yyk);
          yyrhsVals[YYMAXRHS + YYMAXLEFT].getState().setPred(yys);
          if (yynrhs == 0)
            /* Set default location.  */
            yyrhsVals[YYMAXRHS + YYMAXLEFT - 1].getState().yyloc = yys->yyloc;
          for (int yyi = 0; yyi < yynrhs; yyi += 1)
            {
              yys = yys->pred();
              YYASSERT (yys != YY_NULLPTR);
            }
          yystateStack.yyupdateSplit (*yys);
          yystateStack.setTopAt(yyk, yys);
          return yyuserAction (yyrule, yynrhs, yyrhsVals + YYMAXRHS + YYMAXLEFT - 1,
                               yyk,
                               yyvalp, yylocp);
        }
    }

    /** Pop items off stack #YYK of *YYSTACKP according to grammar rule YYRULE,
     *  and push back on the resulting nonterminal symbol.  Perform the
     *  semantic action associated with YYRULE and store its value with the
     *  newly pushed state, if YYFORCEEVAL or if *YYSTACKP is currently
     *  unambiguous.  Otherwise, store the deferred semantic action with
     *  the new state.  If the new state would have an identical input
     *  position, LR state, and predecessor to an existing state on the stack,
     *  it is identified with that existing state, eliminating stack #YYK from
     *  *YYSTACKP.  In this case, the semantic value is
     *  added to the options for the existing state's semantic value.
     */
    YYRESULTTAG
    yyglrReduce (state_set_index yyk, rule_num yyrule, bool yyforceEval)
    {
      size_t yyposn = topState(yyk)->yyposn;

      if (yyforceEval || !yystateStack.isSplit())
        {
          value_type val;
          location_type loc;

          YYRESULTTAG yyflag = yydoAction (yyk, yyrule, &val, &loc);
          if (yyflag == yyerr && yystateStack.isSplit())
            {}
          if (yyflag != yyok)
            return yyflag;
          yyglrShift (yyk,
                      yyLRgotoState (topState(yyk)->yylrState,
                                     yylhsNonterm (yyrule)),
                      yyposn, val, loc);
        }
      else
        {
          glr_state *yys = yystateStack.topAt(yyk);
          glr_state *yys0 = yys;
          for (int yyn = yyrhsLength (yyrule); 0 < yyn; yyn -= 1)
            {
              yys = yys->pred();
              YYASSERT (yys != YY_NULLPTR);
            }
          yystateStack.yyupdateSplit (*yys);
          state_num yynewLRState = yyLRgotoState (yys->yylrState, yylhsNonterm (yyrule));
          for (state_set_index yyi = create_state_set_index(0); yyi.uget() < yystateStack.numTops(); ++yyi)
            if (yyi != yyk && yystateStack.topAt(yyi) != YY_NULLPTR)
              {
                const glr_state* yysplit = yystateStack.yysplitPoint;
                glr_state* yyp = yystateStack.topAt(yyi);
                while (yyp != yys && yyp != yysplit
                       && yyp->yyposn >= yyposn)
                  {
                    if (yyp->yylrState == yynewLRState
                        && yyp->pred() == yys)
                      {
                        yyaddDeferredAction (yyk, yyp, yys0, yyrule);
                        yystateStack.yytops.yymarkStackDeleted (yyk);
                        YYCDEBUG << "Merging stack " << yyk.get ()
                                 << " into stack " << yyi.get () << ".\n";
                        return yyok;
                      }
                    yyp = yyp->pred();
                  }
              }
          yystateStack.setTopAt(yyk, yys);
          yyglrShiftDefer (yyk, yynewLRState, yyposn, yys0, yyrule);
        }
      return yyok;
    }

    /** Shift stack #YYK of *YYSTACKP, to a new state corresponding to LR
     *  state YYLRSTATE, at input position YYPOSN, with the (unresolved)
     *  semantic value of YYRHS under the action for YYRULE.  */
    void
    yyglrShiftDefer (state_set_index yyk, state_num yylrState,
                     size_t yyposn, glr_state* yyrhs, rule_num yyrule)
    {
      glr_state& yynewState = yystateStack.yynewGLRState (
        glr_state (yylrState, yyposn));
      yynewState.setPred (yystateStack.topAt (yyk));
      yystateStack.setTopAt (yyk, &yynewState);

      /* Invokes yyreserveStack.  */
      yyaddDeferredAction (yyk, &yynewState, yyrhs, yyrule);
    }

    /** Shift to a new state on stack #YYK of *YYSTACKP, corresponding to LR
     * state YYLRSTATE, at input position YYPOSN, with (resolved) semantic
     * value YYVAL_ARG and source location YYLOC_ARG.  */
    void
    yyglrShift (state_set_index yyk, state_num yylrState,
                size_t yyposn,
                const value_type& yyval_arg, const location_type& yyloc_arg)
    {
      glr_state& yynewState = yystateStack.yynewGLRState (
        glr_state (yylrState, yyposn, yyval_arg, yyloc_arg));
      yynewState.setPred (yystateStack.topAt(yyk));
      yystateStack.setTopAt (yyk, &yynewState);
      yyreserveGlrStack ();
    }

#if YYDEBUG
    void
    yypstack (state_set_index yyk) const
    {
      yystateStack.yypstates (yystateStack.topAt (yyk));
    }
#endif

    glr_state* topState(state_set_index i) {
      return yystateStack.topAt(i);
    }

    glr_state* firstTopState() {
      return yystateStack.firstTop();
    }

  private:

    void popall_ ()
    {
      /* If the stack is well-formed, pop the stack until it is empty,
         destroying its entries as we go.  But free the stack regardless
         of whether it is well-formed.  */
      for (state_set_index k = create_state_set_index(0); k.uget() < yystateStack.numTops(); k += 1)
        if (yystateStack.topAt(k) != YY_NULLPTR)
          {
            while (yystateStack.topAt(k) != YY_NULLPTR)
              {
                glr_state* state = topState(k);
                yyerror_range[1].getState().yyloc = state->yyloc;
                if (state->pred() != YY_NULLPTR)
                  state->destroy ("Cleanup: popping", yyparser);
                yystateStack.setTopAt(k, state->pred());
                yystateStack.pop_back();
              }
              break;
          }
    }

    /** Resolve the previous YYN states starting at and including state YYS
     *  on *YYSTACKP. If result != yyok, some states may have been left
     *  unresolved possibly with empty semantic option chains.  Regardless
     *  of whether result = yyok, each state has been left with consistent
     *  data so that destroy can be invoked if necessary.  */
    YYRESULTTAG
    yyresolveStates (glr_state& yys, int yyn)
    {
      if (0 < yyn)
        {
          YYASSERT (yys.pred() != YY_NULLPTR);
          YYCHK (yyresolveStates (*yys.pred(), yyn-1));
          if (! yys.yyresolved)
            YYCHK (yyresolveValue (yys));
        }
      return yyok;
    }

    static void
    yyuserMerge (int yyn, value_type& yy0, value_type& yy1)
    {
      YY_USE (yy0);
      YY_USE (yy1);

      switch (yyn)
        {

          default: break;
        }
    }

    /** Resolve the ambiguity represented in state YYS in *YYSTACKP,
     *  perform the indicated actions, and set the semantic value of YYS.
     *  If result != yyok, the chain of semantic options in YYS has been
     *  cleared instead or it has been left unmodified except that
     *  redundant options may have been removed.  Regardless of whether
     *  result = yyok, YYS has been left with consistent data so that
     *  destroy can be invoked if necessary.  */
    YYRESULTTAG
    yyresolveValue (glr_state& yys)
    {
      semantic_option* yybest = yys.firstVal();
      YYASSERT(yybest != YY_NULLPTR);
      bool yymerge = false;
      YYRESULTTAG yyflag;
      location_type *yylocp = &yys.yyloc;

      semantic_option* yypPrev = yybest;
      for (semantic_option* yyp = yybest->next();
           yyp != YY_NULLPTR; )
        {
          if (yybest->isIdenticalTo (*yyp))
            {
              yybest->mergeWith (*yyp);
              yypPrev->setNext(yyp->next());
              yyp = yypPrev->next();
            }
          else
            {
              switch (yypreference (*yybest, *yyp))
                {
                case 0:
                  yyresolveLocations (yys, 1);
                  return yystateStack.yyreportAmbiguity (*yybest, *yyp, yyparser, *yylocp);
                  break;
                case 1:
                  yymerge = true;
                  break;
                case 2:
                  break;
                case 3:
                  yybest = yyp;
                  yymerge = false;
                  break;
                default:
                  /* This cannot happen so it is not worth a YYASSERT (false),
                     but some compilers complain if the default case is
                     omitted.  */
                  break;
                }
              yypPrev = yyp;
              yyp = yyp->next();
            }
        }

      value_type val;
      if (yymerge)
        {
          int yyprec = yydprec[yybest->yyrule];
          yyflag = yyresolveAction (*yybest, &val, yylocp);
          if (yyflag == yyok)
            for (semantic_option* yyp = yybest->next();
                 yyp != YY_NULLPTR;
                 yyp = yyp->next())
              {
                if (yyprec == yydprec[yyp->yyrule])
                  {
                    value_type yyval_other;
                    location_type yydummy;
                    yyflag = yyresolveAction (*yyp, &yyval_other, &yydummy);
                    if (yyflag != yyok)
                      {
                        yyparser.yy_destroy_ ("Cleanup: discarding incompletely merged value for",
                                              yy_accessing_symbol (yys.yylrState),
                                              this->yyla.value, *yylocp);
                        break;
                      }
                    yyuserMerge (yymerger[yyp->yyrule], val, yyval_other);
                  }
              }
        }
      else
        yyflag = yyresolveAction (*yybest, &val, yylocp);

      if (yyflag == yyok)
        {
          yys.yyresolved = true;
          YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
          new (&yys.value ()) value_type (val);

          YY_IGNORE_MAYBE_UNINITIALIZED_END
        }
      else
        yys.setFirstVal(YY_NULLPTR);

      return yyflag;
    }

    /** Resolve the states for the RHS of YYOPT on *YYSTACKP, perform its
     *  user action, and return the semantic value and location in *YYVALP
     *  and *YYLOCP.  Regardless of whether result = yyok, all RHS states
     *  have been destroyed (assuming the user action destroys all RHS
     *  semantic values if invoked).  */
    YYRESULTTAG
    yyresolveAction (semantic_option& yyopt, value_type* yyvalp, location_type* yylocp)
    {
      glr_state* yyoptState = yyopt.state();
      YYASSERT(yyoptState != YY_NULLPTR);
      int yynrhs = yyrhsLength (yyopt.yyrule);
      YYRESULTTAG yyflag = yyresolveStates (*yyoptState, yynrhs);
      if (yyflag != yyok)
        {
          for (glr_state *yys = yyoptState; yynrhs > 0; yys = yys->pred(), yynrhs -= 1)
            yys->destroy ("Cleanup: popping", yyparser);
          return yyflag;
        }

      glr_stack_item yyrhsVals[YYMAXRHS + YYMAXLEFT + 1];
      yyrhsVals[YYMAXRHS + YYMAXLEFT].getState().setPred(yyopt.state());
      if (yynrhs == 0)
        /* Set default location.  */
        yyrhsVals[YYMAXRHS + YYMAXLEFT - 1].getState().yyloc = yyoptState->yyloc;
      {
        symbol_type yyla_current = std::move (this->yyla);
        this->yyla = std::move (yyopt.yyla);
        yyflag = yyuserAction (yyopt.yyrule, yynrhs,
                               yyrhsVals + YYMAXRHS + YYMAXLEFT - 1,
                               create_state_set_index (-1),
                               yyvalp, yylocp);
        this->yyla = std::move (yyla_current);
      }
      return yyflag;
    }

    /** Resolve the locations for each of the YYN1 states in *YYSTACKP,
     *  ending at YYS1.  Has no effect on previously resolved states.
     *  The first semantic option of a state is always chosen.  */
    void
    yyresolveLocations (glr_state &yys1, int yyn1)
    {
      if (0 < yyn1)
        {
          yyresolveLocations (*yys1.pred(), yyn1 - 1);
          if (!yys1.yyresolved)
            {
              glr_stack_item yyrhsloc[1 + YYMAXRHS];
              YYASSERT (yys1.firstVal() != YY_NULLPTR);
              semantic_option& yyoption = *yys1.firstVal();
              const int yynrhs = yyrhsLength (yyoption.yyrule);
              if (0 < yynrhs)
                {
                  yyresolveLocations (*yyoption.state(), yynrhs);
                  const glr_state *yys = yyoption.state();
                  for (int yyn = yynrhs; yyn > 0; yyn -= 1)
                  {
                    yyrhsloc[yyn].getState().yyloc = yys->yyloc;
                    yys = yys->pred();
                  }
                }
              else
                {
                  /* Both yyresolveAction and yyresolveLocations traverse the GSS
                     in reverse rightmost order.  It is only necessary to invoke
                     yyresolveLocations on a subforest for which yyresolveAction
                     would have been invoked next had an ambiguity not been
                     detected.  Thus the location of the previous state (but not
                     necessarily the previous state itself) is guaranteed to be
                     resolved already.  */
                  YY_IGNORE_NULL_DEREFERENCE_BEGIN
                  yyrhsloc[0].getState().yyloc = yyoption.state()->yyloc;
                  YY_IGNORE_NULL_DEREFERENCE_END
                }
              YYLLOC_DEFAULT ((yys1.yyloc), yyrhsloc, yynrhs);
            }
        }
    }

    /** If yytoken is empty, fetch the next token.  */
    void
    yyget_token ()
    {
  YY_USE (scanner);
  YY_USE (data);
      if (this->yyla.empty ())
        {
          YYCDEBUG << "Reading a token\n";
#if YY_EXCEPTIONS
          try
#endif // YY_EXCEPTIONS
            {
              yyla.kind_ = yyparser.yytranslate_ (yylex (&this->yyla.value, &this->yyla.location, scanner, data));
            }
#if YY_EXCEPTIONS
          catch (const parser_type::syntax_error& yyexc)
            {
              YYCDEBUG << "Caught exception: " << yyexc.what () << '\n';
              this->yyla.location = yyexc.location;
              yyparser.error (this->yyla.location, yyexc.what ());
              // Map errors caught in the scanner to the error token, so that error
              // handling is started.
              this->yyla.kind_ = symbol_kind::S_YYerror;
            }
        }
#endif // YY_EXCEPTIONS
      if (this->yyla.kind () == symbol_kind::S_YYEOF)
        YYCDEBUG << "Now at end of input.\n";
      else
        YY_SYMBOL_PRINT ("Next token is", this->yyla.kind (), this->yyla.value, this->yyla.location);
    }


                                /* Bison grammar-table manipulation.  */

    /** The action to take in YYSTATE on seeing YYTOKEN.
     *  Result R means
     *    R < 0:  Reduce on rule -R.
     *    R = 0:  Error.
     *    R > 0:  Shift to state R.
     *  Set *YYCONFLICTS to a pointer into yyconfl to a 0-terminated list
     *  of conflicting reductions.
     */
    static int
    yygetLRActions (state_num yystate, symbol_kind_type yytoken, const short*& yyconflicts)
    {
      int yyindex = yypact[yystate] + yytoken;
      if (yytoken == symbol_kind::S_YYerror)
        {
          // This is the error token.
          yyconflicts = yyconfl;
          return 0;
        }
      else if (yy_is_defaulted_state (yystate)
               || yyindex < 0 || YYLAST < yyindex || yycheck[yyindex] != yytoken)
        {
          yyconflicts = yyconfl;
          return -yydefact[yystate];
        }
      else if (! yytable_value_is_error (yytable[yyindex]))
        {
          yyconflicts = yyconfl + yyconflp[yyindex];
          return yytable[yyindex];
        }
      else
        {
          yyconflicts = yyconfl + yyconflp[yyindex];
          return 0;
        }
    }

    /** Compute post-reduction state.
     * \param yystate   the current state
     * \param yysym     the nonterminal to push on the stack
     */
    static state_num
    yyLRgotoState (state_num yystate, symbol_kind_type yysym)
    {
      const int yyr = yypgoto[yysym - YYNTOKENS] + yystate;
      if (0 <= yyr && yyr <= YYLAST && yycheck[yyr] == yystate)
        return yytable[yyr];
      else
        return yydefgoto[yysym - YYNTOKENS];
    }

    static bool
    yypact_value_is_default (state_num yystate)
    {
      return ((yystate) == YYPACT_NINF);
    }

    static bool
    yytable_value_is_error (int yytable_value YY_ATTRIBUTE_UNUSED)
    {
      return ((yytable_value) == YYTABLE_NINF);
    }

    static bool
    yy_is_shift_action (int yyaction) YY_NOEXCEPT
    {
      return 0 < yyaction;
    }

    static bool
    yy_is_error_action (int yyaction) YY_NOEXCEPT
    {
      return yyaction == 0;
    }

    /** Whether LR state YYSTATE has only a default reduction
     *  (regardless of token).  */
    static bool
    yy_is_defaulted_state (state_num yystate)
    {
      return yypact_value_is_default (yypact[yystate]);
    }

    /** The default reduction for YYSTATE, assuming it has one.  */
    static rule_num
    yy_default_action (state_num yystate)
    {
      return yydefact[yystate];
    }

                                    /* GLRStacks */

    /** Y0 and Y1 represent two possible actions to take in a given
     *  parsing state; return 0 if no combination is possible,
     *  1 if user-mergeable, 2 if Y0 is preferred, 3 if Y1 is preferred.  */
    static int
    yypreference (const semantic_option& y0, const semantic_option& y1)
    {
      const rule_num r0 = y0.yyrule, r1 = y1.yyrule;
      const int p0 = yydprec[r0], p1 = yydprec[r1];

      if (p0 == p1)
        {
          if (yymerger[r0] == 0 || yymerger[r0] != yymerger[r1])
            return 0;
          else
            return 1;
        }
      else if (p0 == 0 || p1 == 0)
        return 0;
      else if (p0 < p1)
        return 3;
      else if (p1 < p0)
        return 2;
      else
        return 0;
    }


    // User arguments.
    void * scanner;
    nix::ParseData * data;
  }; // class parser::glr_stack
} // namespace yy


#if YYDEBUG
namespace
{
  void
  yypstack (const glr_stack& yystack, size_t yyk)
  {
    yystack.yypstack (create_state_set_index (static_cast<std::ptrdiff_t> (yyk)));
  }

  void
  yypdumpstack (const glr_stack& yystack)
  {
    yystack.yypdumpstack ();
  }
}
#endif

namespace yy {
#line 3937 "src/parsers/nix/parser-tab.cc"

  /// Build a parser object.
  parser::parser (void * scanner_yyarg, nix::ParseData * data_yyarg)
    :
#if YYDEBUG
      yycdebug_ (&std::cerr),
#endif
      scanner (scanner_yyarg),
      data (data_yyarg)
  {}

  parser::~parser ()
  {}

  parser::syntax_error::~syntax_error () YY_NOEXCEPT YY_NOTHROW
  {}

  int
  parser::operator() ()
  {
    return parse ();
  }

  int
  parser::parse ()
  {
    glr_stack yystack(YYINITDEPTH, *this, scanner, data);
    return yystack.parse ();
  }

  /* Return YYSTR after stripping away unnecessary quotes and
     backslashes, so that it's suitable for yyerror.  The heuristic is
     that double-quoting is unnecessary unless the string contains an
     apostrophe, a comma, or backslash (other than backslash-backslash).
     YYSTR is taken from yytname.  */
  std::string
  parser::yytnamerr_ (const char *yystr)
  {
    if (*yystr == '"')
      {
        std::string yyr;
        char const *yyp = yystr;

        for (;;)
          switch (*++yyp)
            {
            case '\'':
            case ',':
              goto do_not_strip_quotes;

            case '\\':
              if (*++yyp != '\\')
                goto do_not_strip_quotes;
              else
                goto append;

            append:
            default:
              yyr += *yyp;
              break;

            case '"':
              return yyr;
            }
      do_not_strip_quotes: ;
      }

    return yystr;
  }

  std::string
  parser::symbol_name (symbol_kind_type yysymbol)
  {
    return yytnamerr_ (yytname_[yysymbol]);
  }


#if YYDEBUG || 1
  // YYTNAME[SYMBOL-NUM] -- String name of the symbol SYMBOL-NUM.
  // First, the terminals, then, starting at \a YYNTOKENS, nonterminals.
  const char*
  const parser::yytname_[] =
  {
  "\"end of file\"", "error", "\"invalid token\"", "ID", "ATTRPATH",
  "STR", "IND_STR", "INT", "FLOAT", "PATH", "HPATH", "SPATH", "PATH_END",
  "URI", "IF", "THEN", "ELSE", "ASSERT", "WITH", "LET", "IN", "REC",
  "INHERIT", "EQ", "NEQ", "AND", "OR", "IMPL", "OR_KW", "DOLLAR_CURLY",
  "IND_STRING_OPEN", "IND_STRING_CLOSE", "ELLIPSIS", "'<'", "'>'", "LEQ",
  "GEQ", "UPDATE", "NOT", "'+'", "'-'", "'*'", "'/'", "CONCAT", "'?'",
  "NEGATE", "':'", "'{'", "'}'", "'@'", "';'", "'!'", "'.'", "'\"'", "'('",
  "')'", "'['", "']'", "'='", "','", "$accept", "start", "expr",
  "expr_function", "expr_if", "expr_op", "expr_app", "expr_select",
  "expr_simple", "string_parts", "string_parts_interpolated", "path_start",
  "ind_string_parts", "binds", "attrs", "attrpath", "attr", "string_attr",
  "expr_list", "formals", "formal", YY_NULLPTR
  };
#endif



  // parser::context.
  parser::context::context (glr_stack& yystack, const symbol_type& yyla)
    : yystack_ (yystack)
    , yyla_ (yyla)
  {}

  int
  parser::context::expected_tokens (symbol_kind_type yyarg[], int yyargn) const
  {
    // Actual number of expected tokens
    int yycount = 0;
    const int yyn = yypact[yystack_.firstTopState()->yylrState];
    if (!yystack_.yypact_value_is_default (yyn))
      {
        /* Start YYX at -YYN if negative to avoid negative indexes in
           YYCHECK.  In other words, skip the first -YYN actions for this
           state because they are default actions.  */
        const int yyxbegin = yyn < 0 ? -yyn : 0;
        /* Stay within bounds of both yycheck and yytname.  */
        const int yychecklim = YYLAST - yyn + 1;
        const int yyxend = yychecklim < YYNTOKENS ? yychecklim : YYNTOKENS;
        for (int yyx = yyxbegin; yyx < yyxend; ++yyx)
          if (yycheck[yyx + yyn] == yyx && yyx != symbol_kind::S_YYerror
              && !yystack_.yytable_value_is_error (yytable[yyx + yyn]))
            {
              if (!yyarg)
                ++yycount;
              else if (yycount == yyargn)
                return 0;
              else
                yyarg[yycount++] = YY_CAST (symbol_kind_type, yyx);
            }
      }
    if (yyarg && yycount == 0 && 0 < yyargn)
      yyarg[0] = symbol_kind::S_YYEMPTY;
    return yycount;
  }




  int
  parser::yy_syntax_error_arguments_ (const context& yyctx,
                                                 symbol_kind_type yyarg[], int yyargn) const
  {
    /* There are many possibilities here to consider:
       - If this state is a consistent state with a default action, then
         the only way this function was invoked is if the default action
         is an error action.  In that case, don't check for expected
         tokens because there are none.
       - The only way there can be no lookahead present (in yyla) is
         if this state is a consistent state with a default action.
         Thus, detecting the absence of a lookahead is sufficient to
         determine that there is no unexpected or expected token to
         report.  In that case, just report a simple "syntax error".
       - Don't assume there isn't a lookahead just because this state is
         a consistent state with a default action.  There might have
         been a previous inconsistent state, consistent state with a
         non-default action, or user semantic action that manipulated
         yyla.  (However, yyla is currently not documented for users.)
    */

    if (!yyctx.lookahead ().empty ())
      {
        if (yyarg)
          yyarg[0] = yyctx.token ();
        int yyn = yyctx.expected_tokens (yyarg ? yyarg + 1 : yyarg, yyargn - 1);
        return yyn + 1;
      }
    return 0;
  }

  // Generate an error message.
  std::string
  parser::yysyntax_error_ (const context& yyctx) const
  {
    // Its maximum.
    enum { YYARGS_MAX = 5 };
    // Arguments of yyformat.
    symbol_kind_type yyarg[YYARGS_MAX];
    int yycount = yy_syntax_error_arguments_ (yyctx, yyarg, YYARGS_MAX);

    char const* yyformat = YY_NULLPTR;
    switch (yycount)
      {
#define YYCASE_(N, S)                         \
        case N:                               \
          yyformat = S;                       \
        break
      default: // Avoid compiler warnings.
        YYCASE_ (0, YY_("syntax error"));
        YYCASE_ (1, YY_("syntax error, unexpected %s"));
        YYCASE_ (2, YY_("syntax error, unexpected %s, expecting %s"));
        YYCASE_ (3, YY_("syntax error, unexpected %s, expecting %s or %s"));
        YYCASE_ (4, YY_("syntax error, unexpected %s, expecting %s or %s or %s"));
        YYCASE_ (5, YY_("syntax error, unexpected %s, expecting %s or %s or %s or %s"));
#undef YYCASE_
      }

    std::string yyres;
    // Argument number.
    std::ptrdiff_t yyi = 0;
    for (char const* yyp = yyformat; *yyp; ++yyp)
      if (yyp[0] == '%' && yyp[1] == 's' && yyi < yycount)
        {
          yyres += symbol_name (yyarg[yyi++]);
          ++yyp;
        }
      else
        yyres += *yyp;
    return yyres;
  }

  void
  parser::yy_destroy_ (const char* yymsg, symbol_kind_type yykind,
                           value_type& yyval,
                           location_type& yyloc)
  {
    YY_USE (yyval);
    YY_USE (yyloc);
    if (!yymsg)
      yymsg = "Deleting";
    parser& yyparser = *this;
    YY_USE (yyparser);
    YY_SYMBOL_PRINT (yymsg, yykind, yyval, yyloc);

    YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
    YY_USE (yykind);
    YY_IGNORE_MAYBE_UNINITIALIZED_END
  }

#if YYDEBUG
  /*--------------------.
  | Print this symbol.  |
  `--------------------*/

  void
  parser::yy_symbol_value_print_ (symbol_kind_type yykind,
                           const value_type& yyval,
                           const location_type& yyloc) const
  {
    YY_USE (yyloc);
    YY_USE (yyval);
    std::ostream& yyo = debug_stream ();
    YY_USE (yyo);
    YY_USE (yykind);
  }

  void
  parser::yy_symbol_print_ (symbol_kind_type yykind,
                           const value_type& yyval,
                           const location_type& yyloc) const
  {
    *yycdebug_ << (yykind < YYNTOKENS ? "token" : "nterm")
               << ' ' << symbol_name (yykind) << " ("
               << yyloc << ": ";
    yy_symbol_value_print_ (yykind, yyval, yyloc);
    *yycdebug_ << ')';
  }

  std::ostream&
  parser::debug_stream () const
  {
    return *yycdebug_;
  }

  void
  parser::set_debug_stream (std::ostream& o)
  {
    yycdebug_ = &o;
  }


  parser::debug_level_type
  parser::debug_level () const
  {
    return yydebug;
  }

  void
  parser::set_debug_level (debug_level_type l)
  {
    // Actually, it is yydebug which is really used.
    yydebug = l;
  }
#endif // YYDEBUG

  parser::symbol_kind_type
  parser::yytranslate_ (int t) YY_NOEXCEPT
  {
    // YYTRANSLATE[TOKEN-NUM] -- Symbol number corresponding to
    // TOKEN-NUM as returned by yylex.
    static
    const signed char
    translate_table[] =
    {
       0,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,    51,    53,     2,     2,     2,     2,     2,
      54,    55,    41,    39,    59,    40,    52,    42,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,    46,    50,
      33,    58,    34,    44,    49,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,    56,     2,    57,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,    47,     2,    48,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     1,     2,     3,     4,
       5,     6,     7,     8,     9,    10,    11,    12,    13,    14,
      15,    16,    17,    18,    19,    20,    21,    22,    23,    24,
      25,    26,    27,    28,    29,    30,    31,    32,    35,    36,
      37,    38,    43,    45
    };
    // Last valid token kind.
    const int code_max = 293;

    if (t <= 0)
      return symbol_kind::S_YYEOF;
    else if (t <= code_max)
      return static_cast <symbol_kind_type> (translate_table[t]);
    else
      return symbol_kind::S_YYUNDEF;
  }



  /*---------.
  | symbol.  |
  `---------*/
  // basic_symbol.
  template <typename Base>
  parser::basic_symbol<Base>::basic_symbol (const basic_symbol& that)
    : Base (that)
    , value (that.value)
    , location (that.location)
  {}


  /// Constructor for valueless symbols.
  template <typename Base>
  parser::basic_symbol<Base>::basic_symbol (typename Base::kind_type t, YY_MOVE_REF (location_type) l)
    : Base (t)
    , value ()
    , location (l)
  {}

  template <typename Base>
  parser::basic_symbol<Base>::basic_symbol (typename Base::kind_type t, YY_RVREF (value_type) v, YY_RVREF (location_type) l)
    : Base (t)
    , value (YY_MOVE (v))
    , location (YY_MOVE (l))
  {}



  template <typename Base>
  bool
  parser::basic_symbol<Base>::empty () const YY_NOEXCEPT
  {
    return this->kind () == symbol_kind::S_YYEMPTY;
  }

  template <typename Base>
  void
  parser::basic_symbol<Base>::move (basic_symbol& s)
  {
    super_type::move (s);
    value = YY_MOVE (s.value);
    location = YY_MOVE (s.location);
  }

  // by_kind.
  parser::by_kind::by_kind () YY_NOEXCEPT
    : kind_ (symbol_kind::S_YYEMPTY)
  {}

#if 201103L <= YY_CPLUSPLUS
  parser::by_kind::by_kind (by_kind&& that) YY_NOEXCEPT
    : kind_ (that.kind_)
  {
    that.clear ();
  }
#endif

  parser::by_kind::by_kind (const by_kind& that) YY_NOEXCEPT
    : kind_ (that.kind_)
  {}

  parser::by_kind::by_kind (token_kind_type t) YY_NOEXCEPT
    : kind_ (yytranslate_ (t))
  {}


  parser::by_kind&
  parser::by_kind::by_kind::operator= (const by_kind& that)
  {
    kind_ = that.kind_;
    return *this;
  }

  parser::by_kind&
  parser::by_kind::by_kind::operator= (by_kind&& that)
  {
    kind_ = that.kind_;
    that.clear ();
    return *this;
  }


  void
  parser::by_kind::clear () YY_NOEXCEPT
  {
    kind_ = symbol_kind::S_YYEMPTY;
  }

  void
  parser::by_kind::move (by_kind& that)
  {
    kind_ = that.kind_;
    that.clear ();
  }

  parser::symbol_kind_type
  parser::by_kind::kind () const YY_NOEXCEPT
  {
    return kind_;
  }



} // yy
#line 4383 "src/parsers/nix/parser-tab.cc"
#line 625 "src/parsers/nix/parser.y"



#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>

#include "eval.hh"
#include "filetransfer.hh"
#include "fetchers.hh"
#include "store-api.hh"


namespace nix {


Expr * EvalState::parse(char * text, size_t length, FileOrigin origin,
    const PathView path, const PathView basePath, StaticEnv & staticEnv)
{
    yyscan_t scanner;
    ParseData data(*this);
    data.origin = origin;
    switch (origin) {
        case foFile:
            data.file = data.symbols.create(path);
            break;
        case foStdin:
        case foString:
            data.file = data.symbols.create(text);
            break;
        default:
            assert(false);
    }
    data.basePath = basePath;

    yylex_init(&scanner);
    yy_scan_buffer(text, length, scanner);
    int res = yyparse(scanner, &data);
    yylex_destroy(scanner);

    if (res) throw ParseError(data.error.value());

    data.result->bindVars(staticEnv);

    return data.result;
}


Path resolveExprPath(Path path)
{
    assert(path[0] == '/');

    unsigned int followCount = 0, maxFollow = 1024;

    /* If `path' is a symlink, follow it.  This is so that relative
       path references work. */
    struct stat st;
    while (true) {
        // Basic cycle/depth limit to avoid infinite loops.
        if (++followCount >= maxFollow)
            throw Error("too many symbolic links encountered while traversing the path '%s'", path);
        st = lstat(path);
        if (!S_ISLNK(st.st_mode)) break;
        path = absPath(readLink(path), dirOf(path));
    }

    /* If `path' refers to a directory, append `/default.nix'. */
    if (S_ISDIR(st.st_mode))
        path = canonPath(path + "/default.nix");

    return path;
}


Expr * EvalState::parseExprFromFile(const Path & path)
{
    return parseExprFromFile(path, staticBaseEnv);
}


Expr * EvalState::parseExprFromFile(const Path & path, StaticEnv & staticEnv)
{
    auto buffer = readFile(path);
    // readFile should have left some extra space for terminators
    buffer.append("\0\0", 2);
    return parse(buffer.data(), buffer.size(), foFile, path, dirOf(path), staticEnv);
}


Expr * EvalState::parseExprFromString(std::string s, const Path & basePath, StaticEnv & staticEnv)
{
    s.append("\0\0", 2);
    return parse(s.data(), s.size(), foString, "", basePath, staticEnv);
}


Expr * EvalState::parseExprFromString(std::string s, const Path & basePath)
{
    return parseExprFromString(std::move(s), basePath, staticBaseEnv);
}


Expr * EvalState::parseStdin()
{
    //Activity act(*logger, lvlTalkative, format("parsing standard input"));
    auto buffer = drainFD(0);
    // drainFD should have left some extra space for terminators
    buffer.append("\0\0", 2);
    return parse(buffer.data(), buffer.size(), foStdin, "", absPath("."), staticBaseEnv);
}


void EvalState::addToSearchPath(const string & s)
{
    size_t pos = s.find('=');
    string prefix;
    Path path;
    if (pos == string::npos) {
        path = s;
    } else {
        prefix = string(s, 0, pos);
        path = string(s, pos + 1);
    }

    searchPath.emplace_back(prefix, path);
}


Path EvalState::findFile(const std::string_view path)
{
    return findFile(searchPath, path);
}


Path EvalState::findFile(SearchPath & searchPath, const std::string_view path, const Pos & pos)
{
    for (auto & i : searchPath) {
        std::string suffix;
        if (i.first.empty())
            suffix = concatStrings("/", path);
        else {
            auto s = i.first.size();
            if (path.compare(0, s, i.first) != 0 ||
                (path.size() > s && path[s] != '/'))
                continue;
            suffix = path.size() == s ? "" : concatStrings("/", path.substr(s));
        }
        auto r = resolveSearchPathElem(i);
        if (!r.first) continue;
        Path res = r.second + suffix;
        if (pathExists(res)) return canonPath(res);
    }

    if (hasPrefix(path, "nix/"))
        return concatStrings(corepkgsPrefix, path.substr(4));

    throw ThrownError({
        .msg = hintfmt(evalSettings.pureEval
            ? "cannot look up '<%s>' in pure evaluation mode (use '--impure' to override)"
            : "file '%s' was not found in the Nix search path (add it using $NIX_PATH or -I)",
            path),
        .errPos = pos
    });
}


std::pair<bool, std::string> EvalState::resolveSearchPathElem(const SearchPathElem & elem)
{
    auto i = searchPathResolved.find(elem.second);
    if (i != searchPathResolved.end()) return i->second;

    std::pair<bool, std::string> res;

    if (isUri(elem.second)) {
        try {
            res = { true, store->toRealPath(fetchers::downloadTarball(
                        store, resolveUri(elem.second), "source", false).first.storePath) };
        } catch (FileTransferError & e) {
            logWarning({
                .msg = hintfmt("Nix search path entry '%1%' cannot be downloaded, ignoring", elem.second)
            });
            res = { false, "" };
        }
    } else {
        auto path = absPath(elem.second);
        if (pathExists(path))
            res = { true, path };
        else {
            logWarning({
                .msg = hintfmt("Nix search path entry '%1%' does not exist, ignoring", elem.second)
            });
            res = { false, "" };
        }
    }

    debug(format("resolved search path element '%s' to '%s'") % elem.second % res.second);

    searchPathResolved[elem.second] = res;
    return res;
}


}
