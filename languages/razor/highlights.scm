; ============================================================
; languages/razor/highlights.scm
;
; Maps tree-sitter-razor nodes -> Zed highlight scopes.
; C# and HTML nodes are covered by their injected grammars;
; this file handles only Razor-level constructs.
; ============================================================

; ---- Razor Comments  @* ... *@ ----------------------------
(razor_comment) @comment

; ---- Escaped @ --------------------------------------------
(razor_escaped_at) @string.escape

; ---- @ sigil ----------------------------------------------
(razor_code_block "@" @keyword.operator)
(razor_implicit_expression "@" @keyword.operator)
(razor_explicit_expression "@(" @keyword.operator)
(razor_control_flow "@" @keyword.operator)
(razor_section_block "@" @keyword.operator)
(razor_functions_directive "@" @keyword.operator)
(razor_directive "@" @keyword.operator)

; ---- Directive keywords -----------------------------------
(razor_directive (directive_name) @keyword)
(razor_directive (directive_value) @type)

(razor_section_block (directive_name) @keyword)
(razor_section_block (section_name) @function)
(razor_functions_directive (directive_name) @keyword)

; ---- Control flow keywords --------------------------------
(razor_control_flow (control_keyword) @keyword.control)
(razor_control_flow (control_condition) @embedded)
(razor_else_clause (control_keyword) @keyword.control)
(razor_else_clause (control_condition) @embedded)

; ---- Code blocks ------------------------------------------
(razor_code_block "{" @punctuation.bracket)
(razor_code_block "}" @punctuation.bracket)
(razor_section_block "{" @punctuation.bracket)
(razor_section_block "}" @punctuation.bracket)
(razor_functions_directive "{" @punctuation.bracket)
(razor_functions_directive "}" @punctuation.bracket)
(razor_control_flow "{" @punctuation.bracket)
(razor_control_flow "}" @punctuation.bracket)
(razor_else_clause "{" @punctuation.bracket)
(razor_else_clause "}" @punctuation.bracket)

; ---- Explicit expression parens ---------------------------
(razor_explicit_expression ")" @keyword.operator)

; ---- C# code and expressions ------------------------------
(csharp_code) @embedded
(csharp_expression) @embedded

; ---- Identifiers in implicit expressions ------------------
(identifier) @variable
(index_argument) @variable
(directive_value) @type
(section_name) @function
