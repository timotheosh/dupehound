(list_lit
  .
  (sym_lit name: (sym_name) @_kw)
  .
  (sym_lit name: (sym_name) @name)
  .
  (#any-of? @_kw "defn" "defn-" "defmacro" "deftest")) @func @body

; (def name (fn [args] ...)) -- a function value bound via def instead of
; defn. Mirrors javascript.scm's variable_declarator pattern: only fn-valued
; defs match, so a bare (def x 5) or (def config {...}) is left alone. @body
; is the inner (fn ...) form, not the whole def -- unlike defn, def actually
; has a distinct value node to point at, so there's no need to fall back to
; "the whole form is the body" here.
(list_lit
  .
  (sym_lit name: (sym_name) @_kw)
  .
  (sym_lit name: (sym_name) @name)
  .
  (list_lit
    .
    (sym_lit name: (sym_name) @_fnkw)
    .
    (#eq? @_fnkw "fn")) @body
  (#eq? @_kw "def")) @func
