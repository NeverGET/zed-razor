; ============================================================
; languages/razor/brackets.scm
;
; Teaches Zed which nodes are bracket pairs so it can
; highlight matching brackets and enable bracket-aware editing.
; ============================================================

; Code block braces
(razor_code_block "{" @open "}" @close)

; Section block braces
(razor_section_block "{" @open "}" @close)

; Functions/code directive braces
(razor_functions_directive "{" @open "}" @close)

; Control flow braces
(razor_control_flow "{" @open "}" @close)
(razor_else_clause "{" @open "}" @close)

; Explicit expression parens
(razor_explicit_expression "@(" @open ")" @close)

; Call arguments inside implicit expressions
(call_arguments "(" @open ")" @close)
