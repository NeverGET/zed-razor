; ============================================================
; languages/razor/highlights.scm
;
; Maps tree-sitter-razor nodes -> Zed highlight scopes.
; C# and HTML nodes are covered by their injected grammars;
; this file handles only Razor-level constructs.
; ============================================================

; ---- Razor Comments  @* ... *@ ----------------------------
(razor_comment
  (comment_start) @comment
  (comment_content) @comment
  (comment_end) @comment) @comment

; ---- Escaped @ --------------------------------------------
(razor_escaped_at) @string.escape

; ---- @ sigil ----------------------------------------------
(razor_code_block "@" @keyword.operator)
(razor_implicit_expression "@" @keyword.operator)
(razor_explicit_expression "@(" @keyword.operator)

; ---- Directive keywords -----------------------------------
(razor_directive (directive_name) @keyword)
(razor_directive (directive_value) @type)

(razor_section_directive (directive_name) @keyword)
(razor_section_directive (section_name) @function)

; ---- Control flow keywords --------------------------------
(razor_control_flow (control_keyword) @keyword.control)
(razor_else_clause  (control_keyword) @keyword.control)

; ---- Braces -----------------------------------------------
; Code block
(razor_code_block "{" @punctuation.bracket)
(razor_code_block "}" @punctuation.bracket)

; Explicit expression parens
(razor_explicit_expression ")" @punctuation.special)

; ---- Identifiers in implicit expressions ------------------
(implicit_expression_body
  (identifier) @variable.member)

; Member access dot
(implicit_expression_body "." @operator)

; Null-conditional ?. operator
(implicit_expression_body "?" @operator)

; Null-forgiving !
(implicit_expression_body "!" @operator)
