use syn::{Expr, Ident, Item, ItemImpl, Lit, Pat, Path, Type};

pub(crate) fn match_expr(expr: &Expr) -> String {
    match expr {
        syn::Expr::Call(call) => match call.func.as_ref() {
            syn::Expr::Path(p) => format!("Path: {:?}", match_path(&p.path)),
            syn::Expr::Let(let_expr) => match_pat(&let_expr.pat),
            syn::Expr::Lit(lit_expr) => match_lit_expr(&lit_expr.lit),
            &_ => "Call: ".to_string(), // TODO
        },
        &_ => "Expr: ".to_string(), // TODO
    }
}

pub(crate) fn match_item(item: &Item) -> String {
    match item {
        syn::Item::Fn(func) => format!("ItemFn: {}", func.sig.ident.to_string(),),
        syn::Item::Struct(strct) => format!("ItemStruct: {}", strct.ident.to_string()),
        syn::Item::Impl(imp) => format!("Impl: {:?}", match_item_impl(imp.to_owned())),
        syn::Item::Trait(trt) => format!("Trait: {}", trt.ident),
        syn::Item::Enum(en) => format!("Enum: {}", en.ident),
        syn::Item::Const(cnst) => format!("Const: {}", cnst.ident),
        &_ => "syn::Item".to_string(), // TODO
    }
}

pub(crate) fn match_pat(pat: &Pat) -> String {
    match pat {
        syn::Pat::Path(p) => {
            let segs = p
                .path
                .segments
                .iter()
                .map(|p| p.ident.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            segs
        }
        syn::Pat::Ident(id) => id.ident.to_string(),
        &_ => "syn::Pat".to_string(), // TODO
    }
}

pub(crate) fn match_type(typ: &Type) -> String {
    match typ {
        Type::Path(pat) => {
            let p = pat
                .path
                .segments
                .iter()
                .map(|p| p.ident.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            format!("{p}")
        }
        &_ => "syn::Type".to_string(), // TODO
    }
}

pub(crate) fn match_path(pat: &Path) -> Vec<Ident> {
    pat.segments
        .iter()
        .map(|p| p.ident.to_owned())
        .collect::<Vec<Ident>>()
}

pub(crate) fn match_item_impl(it: ItemImpl) -> String {
    if let Some(item) = it.trait_ {
        format!("{:?}", match_path(&item.1))
    } else {
        return "ItemImpl".to_string();
    }
}

pub(crate) fn match_lit_expr(lit: &Lit) -> String {
    match lit {
        syn::Lit::Char(ch) => format!("{:?}", ch.value()),
        syn::Lit::Str(st) => format!("{}", st.value()),
        syn::Lit::Verbatim(verb) => format!("{}", verb),
        _ => "".to_string(),
    }
}
