/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "giblang",

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => repeat($._token),

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,
    number: $ => /\d+/,
    string: $ => /"[^"]*"/,
    comment: $ => /\/\/.*/,
    whitespace: $ => /\s+/,
    keyword: $ => choice(
      "fn",
      "struct",
      "enum",
      "impl",
      "trait",
      "for",
      "let",
      "if",
      "else",
      "return",
      "true",
      "false",
      "match",
    ),

    punctuation: $ => choice(
      "(",
      ")",
      "{",
      "}",
      "[",
      "]",
      ",",
      ".",
      ":",
      ";",
    ),

    op: $ => choice(
      "+",
      "-",
      "*",
      "/",
      "%",
      "=",
    ),

    _token: $ => choice(
      $.identifier,
      $.number,
      $.string,
      $.comment,
      $.whitespace,
      $.keyword,
      $.punctuation,
      $.op,
    ),
  }
});
