use crate::{
    check::{scoped_state::Scoped, state::CheckState},
    db::{
        decl::{func::Function, struct_::StructDecl, Decl, DeclKind},
        input::Db,
    },
    ir::{AstKind, IrState},
};
use async_lsp::lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat};

impl<'db> Decl<'db> {
    #[allow(unused)]
    pub fn hover(self, state: &mut CheckState<'db>) -> String {
        let path_name = self.path(state.db).name(state.db).join("::");
        let kind = self.get_kind_name(state.db);
        format!("{kind} {path_name}")
    }

    pub fn get_kind_name(self, db: &'db dyn Db) -> &'static str {
        match self.kind(db) {
            DeclKind::Struct { .. } => "struct",
            DeclKind::Trait { .. } => "trait",
            DeclKind::Enum { .. } => "enum",
            DeclKind::Member { .. } => "member",
            DeclKind::Function(Function { .. }) => "function",
            DeclKind::Module(_) => "module",
        }
    }

    pub fn complention_kind(self, db: &'db dyn Db) -> CompletionItemKind {
        match self.kind(db) {
            DeclKind::Struct { .. } => CompletionItemKind::STRUCT,
            DeclKind::Enum { .. } => CompletionItemKind::ENUM,
            DeclKind::Trait { .. } => CompletionItemKind::INTERFACE,
            DeclKind::Function { .. } => CompletionItemKind::FUNCTION,
            DeclKind::Member { .. } => CompletionItemKind::ENUM_MEMBER,
            DeclKind::Module(_) => CompletionItemKind::MODULE,
        }
    }

    #[must_use]
    pub fn completions(self, state: &IrState<'db>) -> Vec<CompletionItem> {
        let mut completions = vec![];
        if state.kind != AstKind::Type
            || !matches!(
                self.kind(state.db),
                DeclKind::Function(_) | DeclKind::Member { .. }
            )
        {
            completions.push(CompletionItem {
                label: self.name(state.db()),
                kind: Some(self.complention_kind(state.db())),
                detail: Some(self.path(state.db()).name(state.db()).join("::")),
                ..Default::default()
            });
        }
        match &state.kind {
            AstKind::Expr => {
                completions.extend(self.get_expr_completions(state));
            }
            AstKind::Pattern => {
                completions.extend(self.get_pattern_completions(state));
            }
            AstKind::Type => {
                completions.extend(self.get_type_completion(state));
            }
            _ => {}
        }
        completions
    }

    fn get_pattern_completions(self, state: &IrState) -> Vec<CompletionItem> {
        match self.kind(state.db()) {
            DeclKind::Struct { body, .. } | DeclKind::Member { body } => {
                body.pattern_completions(self.name(state.db), self.complention_kind(state.db))
            }
            _ => vec![],
        }
    }

    fn get_expr_completions(self, state: &IrState) -> Vec<CompletionItem> {
        let mut completions = vec![];
        match self.kind(state.db()) {
            DeclKind::Struct { body, .. } | DeclKind::Member { body } => {
                let comp = body
                    .contructor_completion(self.name(state.db), self.complention_kind(state.db));
                if let Some(c) = comp {
                    completions.push(c);
                }
                completions
            }
            DeclKind::Function(func) => {
                completions.push(func.call_completion(self.name(state.db)));
                completions
            }
            _ => vec![],
        }
    }

    pub fn get_type_completion(self, state: &IrState<'db>) -> Option<CompletionItem> {
        match self.kind(state.db) {
            DeclKind::Struct { generics, .. }
            | DeclKind::Trait { generics, .. }
            | DeclKind::Enum { generics, .. }
                if !generics.is_empty() =>
            {
                let name = self.name(state.db);
                let kind = self.complention_kind(state.db);
                let args = generics
                    .iter()
                    .enumerate()
                    .map(|(i, g)| format!("${{{}:{}}}", i + 1, g.name.0))
                    .collect::<Vec<_>>()
                    .join(", ");
                let insert_text = format!("{name}[{args}]");

                Some(CompletionItem {
                    label: self.name(state.db),
                    kind: Some(kind),
                    insert_text: Some(insert_text),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                })
            }
            _ => None,
        }
    }

    pub fn get_static_access_completions(self, state: &IrState) -> Vec<CompletionItem> {
        match self.kind(state.db()) {
            DeclKind::Enum { variants, .. } => variants
                .iter()
                .flat_map(|variant| variant.completions(state))
                .collect(),
            DeclKind::Trait { body, .. } => body
                .iter()
                .flat_map(|item| item.completions(state))
                .collect(),
            DeclKind::Module(decls) => decls.iter().flat_map(|d| d.completions(state)).collect(),
            _ => vec![],
        }
    }
}

impl<'db> StructDecl<'db> {
    pub fn contructor_completion(
        &self,
        label: String,
        kind: CompletionItemKind,
    ) -> Option<CompletionItem> {
        match self {
            StructDecl::Fields(fields) => {
                let fields_text = fields
                    .iter()
                    .enumerate()
                    .map(|(i, (name, _))| format!("${{{}:{name}}}", i + 1))
                    .collect::<Vec<_>>()
                    .join(", ");
                let insert_text = Some(format!("{label}({fields_text})"));
                Some(CompletionItem {
                    label,
                    kind: Some(kind),
                    insert_text,
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                })
            }
            StructDecl::Tuple(tys) => {
                let fields_text = (1..=tys.len())
                    .map(|i| format!("${{{i}:arg{i}}}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                let insert_text = Some(format!("{label}({fields_text})"));
                Some(CompletionItem {
                    label,
                    kind: Some(kind),
                    insert_text,
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                })
            }
            StructDecl::None => None,
        }
    }

    pub fn pattern_completions(
        &self,
        label: String,
        kind: CompletionItemKind,
    ) -> Vec<CompletionItem> {
        match self {
            StructDecl::Fields(fields) => {
                let fields_text = fields
                    .iter()
                    .map(|(name, _)| name.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                let insert_text = Some(format!("{label}{{{fields_text}}}"));
                vec![CompletionItem {
                    label,
                    kind: Some(kind),
                    insert_text,
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                }]
            }
            StructDecl::Tuple(tys) => {
                let fields_text = (1..=tys.len())
                    .map(|i| format!("${{{i}:arg{i}}}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                let insert_text = Some(format!("{label}({fields_text})"));
                vec![CompletionItem {
                    label,
                    kind: Some(kind),
                    insert_text,
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                }]
            }
            StructDecl::None => vec![],
        }
    }
}

impl<'db> Function<'db> {
    pub fn call_completion(&self, label: String) -> CompletionItem {
        let fields_text = self
            .args
            .iter()
            .enumerate()
            .map(|(i, (name, _))| format!("${{{}:{name}}}", i + 1))
            .collect::<Vec<_>>()
            .join(", ");
        let insert_text = Some(format!("{label}({fields_text})"));
        CompletionItem {
            label,
            kind: Some(CompletionItemKind::FUNCTION),
            insert_text,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        }
    }
}
