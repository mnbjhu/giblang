#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 8
#define LARGE_STATE_COUNT 7
#define SYMBOL_COUNT 46
#define ALIAS_COUNT 0
#define TOKEN_COUNT 40
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 2
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  sym_identifier = 1,
  sym_number = 2,
  sym_string = 3,
  sym_comment = 4,
  sym_whitespace = 5,
  anon_sym_fn = 6,
  anon_sym_struct = 7,
  anon_sym_enum = 8,
  anon_sym_impl = 9,
  anon_sym_trait = 10,
  anon_sym_for = 11,
  anon_sym_let = 12,
  anon_sym_if = 13,
  anon_sym_else = 14,
  anon_sym_return = 15,
  anon_sym_true = 16,
  anon_sym_false = 17,
  anon_sym_match = 18,
  anon_sym_use = 19,
  anon_sym_while = 20,
  anon_sym_in = 21,
  anon_sym_break = 22,
  anon_sym_continue = 23,
  anon_sym_LPAREN = 24,
  anon_sym_RPAREN = 25,
  anon_sym_LBRACE = 26,
  anon_sym_RBRACE = 27,
  anon_sym_LBRACK = 28,
  anon_sym_RBRACK = 29,
  anon_sym_COMMA = 30,
  anon_sym_DOT = 31,
  anon_sym_COLON = 32,
  anon_sym_SEMI = 33,
  anon_sym_PLUS = 34,
  anon_sym_DASH = 35,
  anon_sym_STAR = 36,
  anon_sym_SLASH = 37,
  anon_sym_PERCENT = 38,
  anon_sym_EQ = 39,
  sym_source_file = 40,
  sym_keyword = 41,
  sym_punctuation = 42,
  sym_op = 43,
  sym__token = 44,
  aux_sym_source_file_repeat1 = 45,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [sym_number] = "number",
  [sym_string] = "string",
  [sym_comment] = "comment",
  [sym_whitespace] = "whitespace",
  [anon_sym_fn] = "fn",
  [anon_sym_struct] = "struct",
  [anon_sym_enum] = "enum",
  [anon_sym_impl] = "impl",
  [anon_sym_trait] = "trait",
  [anon_sym_for] = "for",
  [anon_sym_let] = "let",
  [anon_sym_if] = "if",
  [anon_sym_else] = "else",
  [anon_sym_return] = "return",
  [anon_sym_true] = "true",
  [anon_sym_false] = "false",
  [anon_sym_match] = "match",
  [anon_sym_use] = "use",
  [anon_sym_while] = "while",
  [anon_sym_in] = "in",
  [anon_sym_break] = "break",
  [anon_sym_continue] = "continue",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [anon_sym_COMMA] = ",",
  [anon_sym_DOT] = ".",
  [anon_sym_COLON] = ":",
  [anon_sym_SEMI] = ";",
  [anon_sym_PLUS] = "+",
  [anon_sym_DASH] = "-",
  [anon_sym_STAR] = "*",
  [anon_sym_SLASH] = "/",
  [anon_sym_PERCENT] = "%",
  [anon_sym_EQ] = "=",
  [sym_source_file] = "source_file",
  [sym_keyword] = "keyword",
  [sym_punctuation] = "punctuation",
  [sym_op] = "op",
  [sym__token] = "_token",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_identifier] = sym_identifier,
  [sym_number] = sym_number,
  [sym_string] = sym_string,
  [sym_comment] = sym_comment,
  [sym_whitespace] = sym_whitespace,
  [anon_sym_fn] = anon_sym_fn,
  [anon_sym_struct] = anon_sym_struct,
  [anon_sym_enum] = anon_sym_enum,
  [anon_sym_impl] = anon_sym_impl,
  [anon_sym_trait] = anon_sym_trait,
  [anon_sym_for] = anon_sym_for,
  [anon_sym_let] = anon_sym_let,
  [anon_sym_if] = anon_sym_if,
  [anon_sym_else] = anon_sym_else,
  [anon_sym_return] = anon_sym_return,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_false] = anon_sym_false,
  [anon_sym_match] = anon_sym_match,
  [anon_sym_use] = anon_sym_use,
  [anon_sym_while] = anon_sym_while,
  [anon_sym_in] = anon_sym_in,
  [anon_sym_break] = anon_sym_break,
  [anon_sym_continue] = anon_sym_continue,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [anon_sym_PERCENT] = anon_sym_PERCENT,
  [anon_sym_EQ] = anon_sym_EQ,
  [sym_source_file] = sym_source_file,
  [sym_keyword] = sym_keyword,
  [sym_punctuation] = sym_punctuation,
  [sym_op] = sym_op,
  [sym__token] = sym__token,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_whitespace] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_fn] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_struct] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_enum] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_impl] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_trait] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_for] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_let] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_if] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_else] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_return] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_true] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_false] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_match] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_use] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_while] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_in] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_break] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_continue] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PERCENT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_keyword] = {
    .visible = true,
    .named = true,
  },
  [sym_punctuation] = {
    .visible = true,
    .named = true,
  },
  [sym_op] = {
    .visible = true,
    .named = true,
  },
  [sym__token] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(2);
      if (lookahead == '"') ADVANCE(1);
      if (lookahead == '%') ADVANCE(91);
      if (lookahead == '(') ADVANCE(77);
      if (lookahead == ')') ADVANCE(78);
      if (lookahead == '*') ADVANCE(89);
      if (lookahead == '+') ADVANCE(87);
      if (lookahead == ',') ADVANCE(83);
      if (lookahead == '-') ADVANCE(88);
      if (lookahead == '.') ADVANCE(84);
      if (lookahead == '/') ADVANCE(90);
      if (lookahead == ':') ADVANCE(85);
      if (lookahead == ';') ADVANCE(86);
      if (lookahead == '=') ADVANCE(92);
      if (lookahead == '[') ADVANCE(81);
      if (lookahead == ']') ADVANCE(82);
      if (lookahead == 'b') ADVANCE(37);
      if (lookahead == 'c') ADVANCE(33);
      if (lookahead == 'e') ADVANCE(27);
      if (lookahead == 'f') ADVANCE(3);
      if (lookahead == 'i') ADVANCE(18);
      if (lookahead == 'l') ADVANCE(16);
      if (lookahead == 'm') ADVANCE(6);
      if (lookahead == 'r') ADVANCE(17);
      if (lookahead == 's') ADVANCE(49);
      if (lookahead == 't') ADVANCE(35);
      if (lookahead == 'u') ADVANCE(40);
      if (lookahead == 'w') ADVANCE(19);
      if (lookahead == '{') ADVANCE(79);
      if (lookahead == '}') ADVANCE(80);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(58);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(55);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(56);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(28);
      if (lookahead == 'n') ADVANCE(59);
      if (lookahead == 'o') ADVANCE(36);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(24);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(23);
      if (lookahead == 'u') ADVANCE(11);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(44);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'c') ADVANCE(20);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'c') ADVANCE(46);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(72);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(67);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(69);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(70);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(73);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(76);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(4);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(43);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(47);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'f') ADVANCE(66);
      if (lookahead == 'm') ADVANCE(34);
      if (lookahead == 'n') ADVANCE(74);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'h') ADVANCE(22);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'h') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(31);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(26);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(45);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'k') ADVANCE(75);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(62);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(13);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(41);
      if (lookahead == 'n') ADVANCE(50);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(42);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'm') ADVANCE(61);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(68);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(53);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(48);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(32);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'p') ADVANCE(25);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(5);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(64);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(15);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(30);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(51);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(9);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(10);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(12);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(65);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(7);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(63);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(60);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(52);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(21);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(39);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(29);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(8);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(38);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(14);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(55);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(sym_string);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(57);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(sym_whitespace);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(58);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(anon_sym_fn);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(anon_sym_struct);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(anon_sym_enum);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(anon_sym_impl);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(anon_sym_trait);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(anon_sym_for);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(anon_sym_let);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(anon_sym_if);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(anon_sym_else);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(anon_sym_return);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(anon_sym_true);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(anon_sym_false);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(anon_sym_match);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(anon_sym_use);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(anon_sym_while);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(anon_sym_in);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(anon_sym_break);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(anon_sym_continue);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(54);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '/') ADVANCE(57);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
    [sym_string] = ACTIONS(1),
    [sym_comment] = ACTIONS(1),
    [sym_whitespace] = ACTIONS(1),
    [anon_sym_fn] = ACTIONS(1),
    [anon_sym_struct] = ACTIONS(1),
    [anon_sym_enum] = ACTIONS(1),
    [anon_sym_impl] = ACTIONS(1),
    [anon_sym_trait] = ACTIONS(1),
    [anon_sym_for] = ACTIONS(1),
    [anon_sym_let] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [anon_sym_return] = ACTIONS(1),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
    [anon_sym_match] = ACTIONS(1),
    [anon_sym_use] = ACTIONS(1),
    [anon_sym_while] = ACTIONS(1),
    [anon_sym_in] = ACTIONS(1),
    [anon_sym_break] = ACTIONS(1),
    [anon_sym_continue] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_PERCENT] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(7),
    [sym_keyword] = STATE(2),
    [sym_punctuation] = STATE(2),
    [sym_op] = STATE(2),
    [sym__token] = STATE(2),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym_identifier] = ACTIONS(5),
    [sym_number] = ACTIONS(5),
    [sym_string] = ACTIONS(5),
    [sym_comment] = ACTIONS(5),
    [sym_whitespace] = ACTIONS(7),
    [anon_sym_fn] = ACTIONS(9),
    [anon_sym_struct] = ACTIONS(9),
    [anon_sym_enum] = ACTIONS(9),
    [anon_sym_impl] = ACTIONS(9),
    [anon_sym_trait] = ACTIONS(9),
    [anon_sym_for] = ACTIONS(9),
    [anon_sym_let] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_else] = ACTIONS(9),
    [anon_sym_return] = ACTIONS(9),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_match] = ACTIONS(9),
    [anon_sym_use] = ACTIONS(9),
    [anon_sym_while] = ACTIONS(9),
    [anon_sym_in] = ACTIONS(9),
    [anon_sym_break] = ACTIONS(9),
    [anon_sym_continue] = ACTIONS(9),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_RPAREN] = ACTIONS(11),
    [anon_sym_LBRACE] = ACTIONS(11),
    [anon_sym_RBRACE] = ACTIONS(11),
    [anon_sym_LBRACK] = ACTIONS(11),
    [anon_sym_RBRACK] = ACTIONS(11),
    [anon_sym_COMMA] = ACTIONS(11),
    [anon_sym_DOT] = ACTIONS(11),
    [anon_sym_COLON] = ACTIONS(11),
    [anon_sym_SEMI] = ACTIONS(11),
    [anon_sym_PLUS] = ACTIONS(13),
    [anon_sym_DASH] = ACTIONS(13),
    [anon_sym_STAR] = ACTIONS(13),
    [anon_sym_SLASH] = ACTIONS(13),
    [anon_sym_PERCENT] = ACTIONS(13),
    [anon_sym_EQ] = ACTIONS(13),
  },
  [2] = {
    [sym_keyword] = STATE(3),
    [sym_punctuation] = STATE(3),
    [sym_op] = STATE(3),
    [sym__token] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(15),
    [sym_identifier] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
    [sym_string] = ACTIONS(17),
    [sym_comment] = ACTIONS(17),
    [sym_whitespace] = ACTIONS(19),
    [anon_sym_fn] = ACTIONS(9),
    [anon_sym_struct] = ACTIONS(9),
    [anon_sym_enum] = ACTIONS(9),
    [anon_sym_impl] = ACTIONS(9),
    [anon_sym_trait] = ACTIONS(9),
    [anon_sym_for] = ACTIONS(9),
    [anon_sym_let] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_else] = ACTIONS(9),
    [anon_sym_return] = ACTIONS(9),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_match] = ACTIONS(9),
    [anon_sym_use] = ACTIONS(9),
    [anon_sym_while] = ACTIONS(9),
    [anon_sym_in] = ACTIONS(9),
    [anon_sym_break] = ACTIONS(9),
    [anon_sym_continue] = ACTIONS(9),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_RPAREN] = ACTIONS(11),
    [anon_sym_LBRACE] = ACTIONS(11),
    [anon_sym_RBRACE] = ACTIONS(11),
    [anon_sym_LBRACK] = ACTIONS(11),
    [anon_sym_RBRACK] = ACTIONS(11),
    [anon_sym_COMMA] = ACTIONS(11),
    [anon_sym_DOT] = ACTIONS(11),
    [anon_sym_COLON] = ACTIONS(11),
    [anon_sym_SEMI] = ACTIONS(11),
    [anon_sym_PLUS] = ACTIONS(13),
    [anon_sym_DASH] = ACTIONS(13),
    [anon_sym_STAR] = ACTIONS(13),
    [anon_sym_SLASH] = ACTIONS(13),
    [anon_sym_PERCENT] = ACTIONS(13),
    [anon_sym_EQ] = ACTIONS(13),
  },
  [3] = {
    [sym_keyword] = STATE(3),
    [sym_punctuation] = STATE(3),
    [sym_op] = STATE(3),
    [sym__token] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(21),
    [sym_identifier] = ACTIONS(23),
    [sym_number] = ACTIONS(23),
    [sym_string] = ACTIONS(23),
    [sym_comment] = ACTIONS(23),
    [sym_whitespace] = ACTIONS(26),
    [anon_sym_fn] = ACTIONS(29),
    [anon_sym_struct] = ACTIONS(29),
    [anon_sym_enum] = ACTIONS(29),
    [anon_sym_impl] = ACTIONS(29),
    [anon_sym_trait] = ACTIONS(29),
    [anon_sym_for] = ACTIONS(29),
    [anon_sym_let] = ACTIONS(29),
    [anon_sym_if] = ACTIONS(29),
    [anon_sym_else] = ACTIONS(29),
    [anon_sym_return] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(29),
    [anon_sym_false] = ACTIONS(29),
    [anon_sym_match] = ACTIONS(29),
    [anon_sym_use] = ACTIONS(29),
    [anon_sym_while] = ACTIONS(29),
    [anon_sym_in] = ACTIONS(29),
    [anon_sym_break] = ACTIONS(29),
    [anon_sym_continue] = ACTIONS(29),
    [anon_sym_LPAREN] = ACTIONS(32),
    [anon_sym_RPAREN] = ACTIONS(32),
    [anon_sym_LBRACE] = ACTIONS(32),
    [anon_sym_RBRACE] = ACTIONS(32),
    [anon_sym_LBRACK] = ACTIONS(32),
    [anon_sym_RBRACK] = ACTIONS(32),
    [anon_sym_COMMA] = ACTIONS(32),
    [anon_sym_DOT] = ACTIONS(32),
    [anon_sym_COLON] = ACTIONS(32),
    [anon_sym_SEMI] = ACTIONS(32),
    [anon_sym_PLUS] = ACTIONS(35),
    [anon_sym_DASH] = ACTIONS(35),
    [anon_sym_STAR] = ACTIONS(35),
    [anon_sym_SLASH] = ACTIONS(35),
    [anon_sym_PERCENT] = ACTIONS(35),
    [anon_sym_EQ] = ACTIONS(35),
  },
  [4] = {
    [ts_builtin_sym_end] = ACTIONS(38),
    [sym_identifier] = ACTIONS(40),
    [sym_number] = ACTIONS(40),
    [sym_string] = ACTIONS(40),
    [sym_comment] = ACTIONS(40),
    [sym_whitespace] = ACTIONS(38),
    [anon_sym_fn] = ACTIONS(40),
    [anon_sym_struct] = ACTIONS(40),
    [anon_sym_enum] = ACTIONS(40),
    [anon_sym_impl] = ACTIONS(40),
    [anon_sym_trait] = ACTIONS(40),
    [anon_sym_for] = ACTIONS(40),
    [anon_sym_let] = ACTIONS(40),
    [anon_sym_if] = ACTIONS(40),
    [anon_sym_else] = ACTIONS(40),
    [anon_sym_return] = ACTIONS(40),
    [anon_sym_true] = ACTIONS(40),
    [anon_sym_false] = ACTIONS(40),
    [anon_sym_match] = ACTIONS(40),
    [anon_sym_use] = ACTIONS(40),
    [anon_sym_while] = ACTIONS(40),
    [anon_sym_in] = ACTIONS(40),
    [anon_sym_break] = ACTIONS(40),
    [anon_sym_continue] = ACTIONS(40),
    [anon_sym_LPAREN] = ACTIONS(40),
    [anon_sym_RPAREN] = ACTIONS(40),
    [anon_sym_LBRACE] = ACTIONS(40),
    [anon_sym_RBRACE] = ACTIONS(40),
    [anon_sym_LBRACK] = ACTIONS(40),
    [anon_sym_RBRACK] = ACTIONS(40),
    [anon_sym_COMMA] = ACTIONS(40),
    [anon_sym_DOT] = ACTIONS(40),
    [anon_sym_COLON] = ACTIONS(40),
    [anon_sym_SEMI] = ACTIONS(40),
    [anon_sym_PLUS] = ACTIONS(40),
    [anon_sym_DASH] = ACTIONS(40),
    [anon_sym_STAR] = ACTIONS(40),
    [anon_sym_SLASH] = ACTIONS(40),
    [anon_sym_PERCENT] = ACTIONS(40),
    [anon_sym_EQ] = ACTIONS(40),
  },
  [5] = {
    [ts_builtin_sym_end] = ACTIONS(42),
    [sym_identifier] = ACTIONS(44),
    [sym_number] = ACTIONS(44),
    [sym_string] = ACTIONS(44),
    [sym_comment] = ACTIONS(44),
    [sym_whitespace] = ACTIONS(42),
    [anon_sym_fn] = ACTIONS(44),
    [anon_sym_struct] = ACTIONS(44),
    [anon_sym_enum] = ACTIONS(44),
    [anon_sym_impl] = ACTIONS(44),
    [anon_sym_trait] = ACTIONS(44),
    [anon_sym_for] = ACTIONS(44),
    [anon_sym_let] = ACTIONS(44),
    [anon_sym_if] = ACTIONS(44),
    [anon_sym_else] = ACTIONS(44),
    [anon_sym_return] = ACTIONS(44),
    [anon_sym_true] = ACTIONS(44),
    [anon_sym_false] = ACTIONS(44),
    [anon_sym_match] = ACTIONS(44),
    [anon_sym_use] = ACTIONS(44),
    [anon_sym_while] = ACTIONS(44),
    [anon_sym_in] = ACTIONS(44),
    [anon_sym_break] = ACTIONS(44),
    [anon_sym_continue] = ACTIONS(44),
    [anon_sym_LPAREN] = ACTIONS(44),
    [anon_sym_RPAREN] = ACTIONS(44),
    [anon_sym_LBRACE] = ACTIONS(44),
    [anon_sym_RBRACE] = ACTIONS(44),
    [anon_sym_LBRACK] = ACTIONS(44),
    [anon_sym_RBRACK] = ACTIONS(44),
    [anon_sym_COMMA] = ACTIONS(44),
    [anon_sym_DOT] = ACTIONS(44),
    [anon_sym_COLON] = ACTIONS(44),
    [anon_sym_SEMI] = ACTIONS(44),
    [anon_sym_PLUS] = ACTIONS(44),
    [anon_sym_DASH] = ACTIONS(44),
    [anon_sym_STAR] = ACTIONS(44),
    [anon_sym_SLASH] = ACTIONS(44),
    [anon_sym_PERCENT] = ACTIONS(44),
    [anon_sym_EQ] = ACTIONS(44),
  },
  [6] = {
    [ts_builtin_sym_end] = ACTIONS(46),
    [sym_identifier] = ACTIONS(48),
    [sym_number] = ACTIONS(48),
    [sym_string] = ACTIONS(48),
    [sym_comment] = ACTIONS(48),
    [sym_whitespace] = ACTIONS(46),
    [anon_sym_fn] = ACTIONS(48),
    [anon_sym_struct] = ACTIONS(48),
    [anon_sym_enum] = ACTIONS(48),
    [anon_sym_impl] = ACTIONS(48),
    [anon_sym_trait] = ACTIONS(48),
    [anon_sym_for] = ACTIONS(48),
    [anon_sym_let] = ACTIONS(48),
    [anon_sym_if] = ACTIONS(48),
    [anon_sym_else] = ACTIONS(48),
    [anon_sym_return] = ACTIONS(48),
    [anon_sym_true] = ACTIONS(48),
    [anon_sym_false] = ACTIONS(48),
    [anon_sym_match] = ACTIONS(48),
    [anon_sym_use] = ACTIONS(48),
    [anon_sym_while] = ACTIONS(48),
    [anon_sym_in] = ACTIONS(48),
    [anon_sym_break] = ACTIONS(48),
    [anon_sym_continue] = ACTIONS(48),
    [anon_sym_LPAREN] = ACTIONS(48),
    [anon_sym_RPAREN] = ACTIONS(48),
    [anon_sym_LBRACE] = ACTIONS(48),
    [anon_sym_RBRACE] = ACTIONS(48),
    [anon_sym_LBRACK] = ACTIONS(48),
    [anon_sym_RBRACK] = ACTIONS(48),
    [anon_sym_COMMA] = ACTIONS(48),
    [anon_sym_DOT] = ACTIONS(48),
    [anon_sym_COLON] = ACTIONS(48),
    [anon_sym_SEMI] = ACTIONS(48),
    [anon_sym_PLUS] = ACTIONS(48),
    [anon_sym_DASH] = ACTIONS(48),
    [anon_sym_STAR] = ACTIONS(48),
    [anon_sym_SLASH] = ACTIONS(48),
    [anon_sym_PERCENT] = ACTIONS(48),
    [anon_sym_EQ] = ACTIONS(48),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 1,
    ACTIONS(50), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(7)] = 0,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(2),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(4),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(5),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [23] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(3),
  [26] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(3),
  [29] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(4),
  [32] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(5),
  [35] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(6),
  [38] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_keyword, 1),
  [40] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_keyword, 1),
  [42] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_punctuation, 1),
  [44] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_punctuation, 1),
  [46] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_op, 1),
  [48] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_op, 1),
  [50] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_giblang() {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
