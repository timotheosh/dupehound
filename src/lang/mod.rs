use std::sync::OnceLock;
use tree_sitter::{Language, Node, Query};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    Typescript,
    Tsx,
    Javascript,
    Python,
    Rust,
    Go,
    Java,
    Ruby,
    Swift,
    C,
    Cpp,
    Php,
    Csharp,
    Kotlin,
    Scala,
    CommonLisp,
    Elisp,
    Clojure,
}

#[cfg(test)]
pub const ALL: [Lang; 18] = [
    Lang::Typescript,
    Lang::Tsx,
    Lang::Javascript,
    Lang::Python,
    Lang::Rust,
    Lang::Go,
    Lang::Java,
    Lang::Ruby,
    Lang::Swift,
    Lang::C,
    Lang::Cpp,
    Lang::Php,
    Lang::Csharp,
    Lang::Kotlin,
    Lang::Scala,
    Lang::CommonLisp,
    Lang::Elisp,
    Lang::Clojure,
];

impl Lang {
    pub fn from_path(path: &str) -> Option<Lang> {
        let ext = path.rsplit('.').next()?;
        match ext {
            "ts" | "mts" | "cts" => Some(Lang::Typescript),
            "tsx" => Some(Lang::Tsx),
            "js" | "mjs" | "cjs" | "jsx" => Some(Lang::Javascript),
            "py" | "pyi" => Some(Lang::Python),
            "rs" => Some(Lang::Rust),
            "go" => Some(Lang::Go),
            "java" => Some(Lang::Java),
            "rb" => Some(Lang::Ruby),
            "swift" => Some(Lang::Swift),
            "c" | "h" => Some(Lang::C),
            "cc" | "cpp" | "cxx" | "c++" | "hpp" | "hh" | "hxx" => Some(Lang::Cpp),
            "php" => Some(Lang::Php),
            "cs" => Some(Lang::Csharp),
            "kt" | "kts" => Some(Lang::Kotlin),
            "scala" | "sc" => Some(Lang::Scala),
            "lisp" | "lsp" | "asd" => Some(Lang::CommonLisp),
            "el" => Some(Lang::Elisp),
            "clj" | "cljc" | "cljs" => Some(Lang::Clojure),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Lang::Typescript => "TypeScript",
            Lang::Tsx => "TSX",
            Lang::Javascript => "JavaScript",
            Lang::Python => "Python",
            Lang::Rust => "Rust",
            Lang::Go => "Go",
            Lang::Java => "Java",
            Lang::Ruby => "Ruby",
            Lang::Swift => "Swift",
            Lang::C => "C",
            Lang::Cpp => "C++",
            Lang::Php => "Php",
            Lang::Csharp => "C#",
            Lang::Kotlin => "Kotlin",
            Lang::Scala => "Scala",
            Lang::CommonLisp => "Common Lisp",
            Lang::Elisp => "Emacs Lisp",
            Lang::Clojure => "Clojure",
        }
    }

    pub fn language(self) -> Language {
        match self {
            Lang::Php => tree_sitter_php::language_php(),
            Lang::Typescript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            Lang::Tsx => tree_sitter_typescript::LANGUAGE_TSX.into(),
            Lang::Javascript => tree_sitter_javascript::LANGUAGE.into(),
            Lang::Python => tree_sitter_python::LANGUAGE.into(),
            Lang::Rust => tree_sitter_rust::LANGUAGE.into(),
            Lang::Go => tree_sitter_go::LANGUAGE.into(),
            Lang::Java => tree_sitter_java::LANGUAGE.into(),
            Lang::Ruby => tree_sitter_ruby::LANGUAGE.into(),
            Lang::Swift => tree_sitter_swift::LANGUAGE.into(),
            Lang::C => tree_sitter_c::LANGUAGE.into(),
            Lang::Cpp => tree_sitter_cpp::LANGUAGE.into(),
            Lang::Csharp => tree_sitter_c_sharp::LANGUAGE.into(),
            Lang::Kotlin => tree_sitter_kotlin_ng::LANGUAGE.into(),
            Lang::Scala => tree_sitter_scala::LANGUAGE.into(),
            Lang::CommonLisp => tree_sitter_commonlisp::LANGUAGE_COMMONLISP.into(),
            Lang::Elisp => tree_sitter_elisp::LANGUAGE.into(),
            Lang::Clojure => tree_sitter_clojure_orchard::LANGUAGE.into(),
        }
    }

    fn query_source(self) -> &'static str {
        match self {
            Lang::Typescript | Lang::Tsx => include_str!("queries/typescript.scm"),
            Lang::Javascript => include_str!("queries/javascript.scm"),
            Lang::Python => include_str!("queries/python.scm"),
            Lang::Rust => include_str!("queries/rust.scm"),
            Lang::Go => include_str!("queries/go.scm"),
            Lang::Java => include_str!("queries/java.scm"),
            Lang::Ruby => include_str!("queries/ruby.scm"),
            Lang::Swift => include_str!("queries/swift.scm"),
            Lang::C => include_str!("queries/c.scm"),
            Lang::Cpp => include_str!("queries/cpp.scm"),
            Lang::Php => include_str!("queries/php.scm"),
            Lang::Csharp => include_str!("queries/csharp.scm"),
            Lang::Kotlin => include_str!("queries/kotlin.scm"),
            Lang::Scala => include_str!("queries/scala.scm"),
            Lang::CommonLisp => include_str!("queries/commonlisp.scm"),
            Lang::Elisp => include_str!("queries/elisp.scm"),
            Lang::Clojure => include_str!("queries/clojure.scm"),
        }
    }

    pub fn query(self) -> &'static Query {
        static QUERIES: [OnceLock<Query>; 18] = [const { OnceLock::new() }; 18];
        QUERIES[self as usize].get_or_init(|| {
            Query::new(&self.language(), self.query_source())
                .unwrap_or_else(|e| panic!("bad {} query: {e}", self.name()))
        })
    }

    /// Experimental: query capturing type declarations for the opt-in
    /// `--include-classes` "class shape" mode. Only C# is wired up so far.
    fn shape_query_source(self) -> Option<&'static str> {
        match self {
            Lang::Csharp => Some(include_str!("queries/csharp_shape.scm")),
            _ => None,
        }
    }

    pub fn shape_query(self) -> Option<&'static Query> {
        let src = self.shape_query_source()?;
        static SHAPE_QUERIES: [OnceLock<Query>; 18] = [const { OnceLock::new() }; 18];
        Some(SHAPE_QUERIES[self as usize].get_or_init(|| {
            Query::new(&self.language(), src)
                .unwrap_or_else(|e| panic!("bad {} shape query: {e}", self.name()))
        }))
    }
}

/// What a leaf token contributes to the normalized stream.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TokenClass {
    /// Identifier-like: folded to one sentinel so renames don't matter.
    Ident,
    /// String/char/template literal parts: folded and run-collapsed.
    Str,
    /// Numeric literal: folded.
    Num,
    /// Comments: dropped entirely.
    Comment,
    /// Structure (keywords, operators, punctuation): kept verbatim by kind id.
    Other,
    /// Operator/special-form/function-call symbol in a grammar too coarse
    /// to give it its own node kind (the Lisp family: a bare symbol names
    /// `+`, `if`, `reduce`, and an ordinary local variable alike, all under
    /// the same node kind): kept verbatim by hashed text, since kind id
    /// alone can't distinguish it from an identifier.
    Op,
}

/// Classify a leaf node. Kind-name conventions are consistent enough across
/// the bundled grammars that substring rules on `leaf.kind()` beat
/// per-grammar tables for almost everything — and they survive grammar
/// upgrades better.
///
/// The Lisp family is the exception: their `sym_lit`/`symbol` node covers
/// call heads, special forms, macros, functions, and local variables alike,
/// so kind name alone can't separate "operator" from "identifier" the way
/// every other supported grammar's node kinds do. `is_*_call_head` breaks
/// the tie using the leaf's position instead — hence this function takes
/// the whole `Node`, not just its kind string. `src` is the leaf's file's
/// bytes; the Lisp helpers need it to read a candidate binding form's head
/// text (`let` vs. a real call).
pub fn classify(leaf: Node, src: &[u8]) -> TokenClass {
    let kind = leaf.kind();
    if kind.contains("comment") {
        TokenClass::Comment
    } else if kind == "sym_lit" {
        if is_commonlisp_call_head(leaf, src) {
            TokenClass::Op
        } else {
            TokenClass::Ident
        }
    } else if kind == "symbol" {
        if is_elisp_call_head(leaf) {
            TokenClass::Op
        } else {
            TokenClass::Ident
        }
    } else if kind == "sym_name" {
        if is_clojure_call_head(leaf) {
            TokenClass::Op
        } else {
            TokenClass::Ident
        }
    } else if kind.contains("identifier") || kind == "shorthand_property_identifier_pattern" {
        TokenClass::Ident
    } else if kind.contains("string")
        || kind.contains("char")
        || kind.contains("rune")
        || kind == "escape_sequence"
        || kind == "template_chars"
        || kind == "str_lit"
        || kind == "kwd_symbol"
        || kind == "kwd_name"
        || kind == "\""
        || kind == "'"
        || kind == "`"
    {
        TokenClass::Str
    } else if kind.contains("number")
        || kind.contains("integer")
        || kind.contains("float")
        || kind.contains("decimal")
        || kind.contains("imaginary")
        || kind == "int_literal"
        || kind == "num_lit"
    {
        TokenClass::Num
    } else {
        TokenClass::Other
    }
}

/// True if `sym_lit` occupies the head (first) position of a Common Lisp
/// `list_lit` form — `(reduce ...)`, `(when ...)`, `(+ ...)` — meaning the
/// symbol names the operator/function being invoked rather than referencing
/// a bound value. Deliberately grammar-specific (it names `list_lit`/`value`
/// directly, tree-sitter-commonlisp's field for a list's elements).
///
/// Common Lisp reuses this exact same `list_lit` shape for parameter and
/// binding lists — `(defun f (x) ...)`, `(let ((x 1)) ...)` — so the raw
/// "first value child of a list_lit" rule alone would also flag a
/// function's own first parameter, or a `let` binding's own name, as an
/// Op (breaking a renamed clone's match on the very name that got
/// renamed). The two exclusion checks below cover the common cases: a
/// `defun`/`defmacro`/`defmethod`/`defgeneric` lambda-list, and a
/// `let`/`let*`/`do`/`do*`/`flet`/`labels`-style binding name. Other,
/// rarer binding forms (e.g. `destructuring-bind` patterns) are not
/// special-cased and will still be treated as Op.
fn is_commonlisp_call_head(sym_lit: Node, src: &[u8]) -> bool {
    let Some(list) = sym_lit.parent() else {
        return false;
    };
    if list.kind() != "list_lit" {
        return false;
    }
    let is_head = list
        .child_by_field_name("value")
        .is_some_and(|first| first.id() == sym_lit.id());
    if !is_head {
        return false;
    }
    if list.parent().is_some_and(|p| p.kind() == "defun_header") {
        return false;
    }
    if is_commonlisp_binding_name(list, src) {
        return false;
    }
    true
}

/// The `list_lit`'s first `sym_lit`/`value` child, if any — used to read a
/// candidate special form's head keyword.
fn commonlisp_list_head_text<'a>(list: Node, src: &'a [u8]) -> Option<&'a str> {
    let head = list.child_by_field_name("value")?;
    if head.kind() != "sym_lit" {
        return None;
    }
    head.utf8_text(src).ok()
}

/// The `list_lit`'s Nth `value`-field child (0-based).
fn commonlisp_nth_value_child(list: Node, n: usize) -> Option<Node> {
    let mut cursor = list.walk();
    list.children_by_field_name("value", &mut cursor).nth(n)
}

/// True if `pair` (e.g. `(total (reduce ...))`) is a binding pair whose
/// first element names a local variable rather than calling it — either a
/// `let`/`let*`-style binding (`pair` inside a bindings list that is the
/// form's second `value` child) or a `dolist`/`dotimes`-style binding
/// (`pair` itself is the form's second `value` child directly).
fn is_commonlisp_binding_name(pair: Node, src: &[u8]) -> bool {
    let Some(parent) = pair.parent() else {
        return false;
    };
    if parent.kind() != "list_lit" {
        return false;
    }
    // dolist/dotimes: `(dolist (item items) ...)` — pair is the form's own
    // second value child.
    if commonlisp_nth_value_child(parent, 1).map(|n| n.id()) == Some(pair.id())
        && matches!(
            commonlisp_list_head_text(parent, src),
            Some("dolist" | "dotimes")
        )
    {
        return true;
    }
    // let/let*/do/do*/flet/labels: `(let ((total ...)) ...)` — pair is a
    // value child of a bindings list that is itself the form's second
    // value child.
    let Some(form) = parent.parent() else {
        return false;
    };
    if form.kind() != "list_lit" {
        return false;
    }
    if commonlisp_nth_value_child(form, 1).map(|n| n.id()) != Some(parent.id()) {
        return false;
    }
    matches!(
        commonlisp_list_head_text(form, src),
        Some("let" | "let*" | "do" | "do*" | "flet" | "labels")
    )
}

/// True if `symbol` occupies the head (first named child) position of an
/// Emacs Lisp `list` form — `(reduce ...)`, `(my-helper ...)`. Emacs Lisp's
/// built-in special forms (`if`, `let`, `while`, ...) parse as their own
/// literal keyword tokens under `special_form`, not as a `symbol`, so this
/// only fires for ordinary function calls that a generic `list` produces.
/// tree-sitter-elisp's `list` rule has no field names (`seq("(", repeat($._sexp), ")")`),
/// so the head is found positionally rather than by field, unlike the
/// Common Lisp version of this check.
///
/// As with Common Lisp, parameter and binding lists share this same `list`
/// shape, so a function's own parameters and a `let`/`let*`/`lambda`
/// binding's own name are excluded below (see
/// `is_commonlisp_call_head`'s doc comment for why). Other binding forms
/// (e.g. `dolist`, `cl-destructuring-bind`) are not special-cased.
fn is_elisp_call_head(symbol: Node) -> bool {
    let Some(list) = symbol.parent() else {
        return false;
    };
    if list.kind() != "list" {
        return false;
    }
    let is_head = list
        .named_child(0)
        .is_some_and(|first| first.id() == symbol.id());
    if !is_head {
        return false;
    }
    if is_elisp_definition_parameters(list) {
        return false;
    }
    if is_elisp_binding_name(list) {
        return false;
    }
    true
}

/// True if `list` is the `parameters` field of a `function_definition` or
/// `macro_definition` — `(defun f (x) ...)`.
fn is_elisp_definition_parameters(list: Node) -> bool {
    list.parent().is_some_and(|p| {
        matches!(p.kind(), "function_definition" | "macro_definition")
            && p.child_by_field_name("parameters")
                .is_some_and(|params| params.id() == list.id())
    })
}

/// True if `pair` is a `let`/`let*` binding pair (`(total ...)` inside
/// `(let ((total ...)) ...)`) or a `lambda`'s own parameter list. Both
/// `let`/`let*` and `lambda` parse as literal keyword tokens under
/// `special_form`, so the check is structural, no text lookup needed.
///
/// The two forms nest at different depths — a `lambda` parameter list sits
/// directly under the `special_form`, while a `let` binding pair sits one
/// level deeper (inside the bindings list) — hence the match on `pair`'s
/// immediate parent's kind rather than a single shared shape.
fn is_elisp_binding_name(pair: Node) -> bool {
    let Some(parent) = pair.parent() else {
        return false;
    };
    match parent.kind() {
        "special_form" => {
            parent.named_child(0).is_some_and(|n| n.id() == pair.id())
                && matches!(parent.child(1).map(|k| k.kind()), Some("lambda"))
        }
        "list" => {
            let Some(form) = parent.parent() else {
                return false;
            };
            form.kind() == "special_form"
                && form.named_child(0).is_some_and(|n| n.id() == parent.id())
                && matches!(form.child(1).map(|k| k.kind()), Some("let" | "let*"))
        }
        _ => false,
    }
}

/// True if `sym_name` occupies the head (first) position of a Clojure
/// `list_lit` form — `(reduce ...)`, `(if ...)`, `(+ ...)` — meaning the
/// symbol names the operator/special-form/macro being invoked rather than
/// referencing a bound value. Deliberately grammar-specific (it names
/// `list_lit`/`value` directly, and `sym_name`'s immediate parent is the
/// `sym_lit` wrapper, not the containing list — unlike Common Lisp's
/// `sym_lit`, which *is* the leaf).
///
/// Unlike Common Lisp and Emacs Lisp, Clojure needs no parameter/binding-list
/// exclusion here: `defn` parameters and `let`/`fn` bindings are written
/// with `[...]` (`vec_lit`), a distinct node kind from the `(...)` call
/// list (`list_lit`), so the ambiguity that motivates
/// `is_commonlisp_call_head`'s and `is_elisp_call_head`'s extra checks
/// doesn't arise here.
fn is_clojure_call_head(sym_name: Node) -> bool {
    let Some(sym_lit) = sym_name.parent() else {
        return false;
    };
    let Some(list) = sym_lit.parent() else {
        return false;
    };
    if list.kind() != "list_lit" {
        return false;
    }
    list.child_by_field_name("value")
        .is_some_and(|first| first.id() == sym_lit.id())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queries_compile_for_every_language() {
        for lang in ALL {
            let _ = lang.query();
        }
    }

    #[test]
    fn abi_in_supported_range() {
        for lang in ALL {
            let v = lang.language().abi_version();
            assert!(
                (tree_sitter::MIN_COMPATIBLE_LANGUAGE_VERSION..=tree_sitter::LANGUAGE_VERSION)
                    .contains(&v),
                "{} grammar ABI {v} outside supported range",
                lang.name()
            );
        }
    }
}
