; ============================================================
; languages/razor/injections.scm
;
; Injects HTML and C# grammars into appropriate Razor nodes
; so they get proper syntax highlighting from their own grammars.
; ============================================================

; Inject C# into code block bodies  @{ <here> }
((csharp_code) @injection.content
  (#set! injection.language "c_sharp")
  (#set! injection.combined))

; Inject C# into explicit expressions  @( <here> )
((csharp_expression) @injection.content
  (#set! injection.language "c_sharp"))

; Inject C# into directive values for type-bearing directives
; e.g. @model MyApp.Models.Foo  ->  highlight "MyApp.Models.Foo" as C#
((razor_directive
  (directive_name) @_dname
  (directive_value) @injection.content)
  (#match? @_dname "^(model|inject|inherits|implements|typeparam)$")
  (#set! injection.language "c_sharp"))

; Inject HTML into raw text/markup chunks
((_text_chunk) @injection.content
  (#set! injection.language "html")
  (#set! injection.include-children))
