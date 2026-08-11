; tree-sitter-commonlisp folds defun/defmacro/defmethod/defgeneric into one
; `defun` node kind, distinguished only by the `defun_header`'s `keyword`
; field, so a single pattern captures all four forms. There is no distinct
; body node (the body is `defun`'s own repeated, unnamed `value` children
; after the header), so @func and @body both capture the whole form.
(defun
  (defun_header function_name: (sym_lit) @name)) @func @body
