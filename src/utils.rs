use proc_macro2::Span;
use syn::{Expr, Ident, ImplItemFn, Item, ItemImpl, Lit, Pat, Path, TraitItemFn, Type};

pub(crate) fn match_expr(expr: &Expr) -> Ident {
    match expr {
        syn::Expr::Call(call) => match call.func.as_ref() {
            syn::Expr::Path(p) => match_path(&p.path).first().unwrap().to_owned(),
            syn::Expr::Let(let_expr) => match_pat(&let_expr.pat).first().unwrap().to_owned(),
            syn::Expr::Lit(lit_expr) => match_lit_expr(&lit_expr.lit),
            &_ => Ident::new("Call", Span::call_site()), // TODO
        },
        &_ => Ident::new("Expr", Span::call_site()), // TODO
    }
}

pub(crate) fn match_item(item: &Item) -> String {
    match item {
        syn::Item::Fn(func) => format!("ItemFn: {}", func.sig.ident.to_string(),),
        syn::Item::Struct(strct) => format!("ItemStruct: {}", strct.ident.to_string()),
        syn::Item::Impl(imp) => format!("Impl: {:?}", match_item_impl(imp)),
        syn::Item::Trait(trt) => format!("Trait: {}", trt.ident),
        syn::Item::Enum(en) => format!("Enum: {}", en.ident),
        syn::Item::Const(cnst) => format!("Const: {}", cnst.ident),
        &_ => "syn::Item".to_string(), // TODO
    }
}

pub(crate) fn match_pat(pat: &Pat) -> Vec<Ident> {
    match pat {
        syn::Pat::Path(p) => p
            .path
            .segments
            .iter()
            .map(|p| p.ident.to_owned())
            .collect::<Vec<Ident>>(),
        syn::Pat::Ident(id) => vec![id.ident.to_owned()],
        &_ => vec![Ident::new("Pat", Span::call_site())], // TODO
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

pub(crate) fn match_path(path: &Path) -> Vec<Ident> {
    path.segments
        .iter()
        .map(|p| p.ident.to_owned())
        .collect::<Vec<Ident>>()
}

pub(crate) fn match_item_impl(it: &ItemImpl) -> Vec<Ident> {
    if let Some(item) = it.trait_.to_owned() {
        match_path(&item.1)
    } else {
        vec![Ident::new("ItemImpl", Span::call_site())]
    }
}

pub(crate) fn match_lit_expr(lit: &Lit) -> Ident {
    match lit {
        syn::Lit::Char(ch) => Ident::new(format!("{}", ch.value()).as_str(), Span::call_site()),
        syn::Lit::Str(st) => Ident::new(format!("{}", st.value()).as_str(), Span::call_site()),
        syn::Lit::Verbatim(verb) => Ident::new(format!("{}", verb).as_str(), Span::call_site()),
        syn::Lit::Bool(b) => Ident::new(format!("{}", b.value()).as_str(), Span::call_site()),
        syn::Lit::Byte(by) => Ident::new(format!("{}", by.value()).as_str(), Span::call_site()),
        syn::Lit::ByteStr(bs) => {
            let string = String::from_utf8(bs.value()).unwrap();
            Ident::new(string.as_str(), Span::call_site())
        }
        syn::Lit::Float(fl) => Ident::new(fl.to_string().as_str(), Span::call_site()),
        _ => Ident::new("LitExpr", Span::call_site()),
    }
}

pub(crate) fn match_trait_item_fn(trait_item: &TraitItemFn) -> Ident {
    trait_item.sig.ident.to_owned()
}

pub(crate) fn match_impl_item_fn(impl_item_fn: &ImplItemFn) -> Ident {
    impl_item_fn.sig.ident.to_owned()
}
