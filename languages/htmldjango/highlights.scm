; Delimiters
[
  "{{"
  "}}"
  "{%"
  "%}"
] @punctuation.special

; Tag names (if, for, block, extends, load, etc.)
(tag_name) @function.builtin

(end_paired_statement) @punctuation.special

; Variables
(variable_name) @variable

; Filter names (e.g., |default, |length, |safe)
(filter_name) @function

; Filter arguments
(filter_argument) @string.special

; Comments
(unpaired_comment) @comment
(paired_comment) @comment

; Strings
(string) @string

; Numbers
(number) @number

; Booleans (True, False)
(boolean) @boolean

; Keywords (as, with, only, from, by, etc.)
(keyword) @keyword

; Keyword operators (and, or, not, in, is, is not, not in)
(keyword_operator) @keyword

; Operators (==, !=, <, >, <=, >=)
(operator) @operator

; Punctuation
[
  ","
  "."
  ":"
  "|"
  "="
] @punctuation.delimiter

; Comparison operators
[
  "=="
  "!="
  "<"
  "<="
  ">"
  ">="
] @operator
