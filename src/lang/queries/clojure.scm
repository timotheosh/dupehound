(list_lit
  .
  (sym_lit name: (sym_name) @_kw)
  .
  (sym_lit name: (sym_name) @name)
  .
  (#any-of? @_kw "defn" "defn-" "defmacro")) @func @body
