use darling::util::IdentString;
use darling::{ast, util};
use darling::{FromDeriveInput, FromField, FromMeta};

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(table),
    supports(struct_named),
    forward_attrs(allow, doc, cfg)
)]
pub(crate) struct TableComponentDeriveInput {
    pub(crate) ident: syn::Ident,
    pub(crate) data: ast::Data<util::Ignored, TableDataField>,

    #[darling(default)]
    pub(crate) component_name: Option<String>,

    #[darling(default)]
    pub(crate) classes_provider: Option<IdentString>,

    #[darling(default)]
    pub(crate) tag: Option<IdentString>,

    #[darling(default)]
    pub(crate) row_renderer: Option<IdentString>,

    #[darling(default)]
    pub(crate) row_class: Option<String>,

    #[darling(default)]
    pub(crate) head_row_class: Option<String>,

    #[darling(default)]
    pub(crate) head_cell_renderer: Option<IdentString>,

    #[darling(default)]
    pub(crate) selection_mode: SelectionMode,

    #[darling(default)]
    pub(crate) sortable: bool,
}

#[derive(Debug, FromMeta, Default, Copy, Clone)]
pub(crate) enum SelectionMode {
    #[default]
    None,
    Single,
    Multiple,
}

#[derive(Debug, FromField)]
#[darling(attributes(table))]
pub(crate) struct TableDataField {
    pub(crate) ident: Option<syn::Ident>,
    pub(crate) ty: syn::Type,

    #[darling(default)]
    pub(crate) renderer: Option<IdentString>,

    #[darling(default)]
    pub(crate) format: Format,

    #[darling(default)]
    pub(crate) class: Option<String>,

    #[darling(default)]
    pub(crate) cell_class: Option<String>,

    #[darling(default)]
    pub(crate) head_class: Option<String>,

    #[darling(default)]
    pub(crate) title: Option<String>,

    #[darling(default)]
    pub(crate) skip: bool,

    #[darling(default)]
    pub(crate) key: bool,

    #[darling(default)]
    pub(crate) skip_sort: bool,
}

impl TableDataField {
    pub(crate) fn cell_class(&self) -> String {
        let mut class = "".to_owned();

        if let Some(ref c) = self.class {
            class.push_str(c);
        }
        if let Some(ref c) = self.cell_class {
            class.push(' ');
            class.push_str(c);
        }

        class
    }

    pub(crate) fn head_class(&self) -> String {
        let mut class = "".to_owned();

        if let Some(ref c) = self.class {
            class.push_str(c);
        }
        if let Some(ref c) = self.head_class {
            class.push(' ');
            class.push_str(c);
        }

        class
    }
}

#[derive(Default, FromMeta, Debug)]
#[darling(default)]
pub(crate) struct Format {
    pub(crate) string: Option<String>,
    pub(crate) precision: Option<i64>,
}
