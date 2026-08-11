; defun/defsubst parse as function_definition, defmacro as macro_definition.
; Neither has a distinct body node (the body is trailing repeated,
; unnamed children after the name/parameters/docstring fields), so @func
; and @body both capture the whole form.
(function_definition name: (symbol) @name) @func @body

(macro_definition name: (symbol) @name) @func @body
