use quote::{ToTokens, quote};
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{braced, punctuated::Punctuated};
use syn::{Ident, Result, Token, Type, parse_macro_input};
use proc_macro2::{TokenStream as TokenStream2, TokenTree, Group};

struct ImplBlock {
    pub target: Type,
    pub types: Vec<Type>,
    generics: syn::Generics,
}

struct OverTraitMethod {
    signature: syn::Signature,
    body: proc_macro2::TokenStream,
    pub attributes: Vec<syn::Attribute>,
}

struct OverTraitInput {
    trait_name: Ident,
    vis: syn::Visibility,
    impls: Vec<ImplBlock>,
    generics: syn::Generics,
    methods: Vec<OverTraitMethod>,
    attributes: Vec<syn::Attribute>,
}

impl Parse for ImplBlock {
    // impl<generics> for Target => { Type1, Type2, .. }
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![impl]>()?;
        let mut generics: syn::Generics = input.parse()?;
        generics.where_clause = input.parse()?;

        input.parse::<Token![for]>()?;
        let target: Type = input.parse()?;
        input.parse::<Token![=>]>()?;

        let types_content;
        braced!(types_content in input);
        let types = Punctuated::<Type, Token![,]>::parse_terminated(&types_content)?
            .into_iter()
        .collect();

        return Ok(Self {
            types: types,
            target: target,
            generics: generics,
        })
    }
}


impl Parse for OverTraitMethod {
    // fn method_name(...) -> ReturnType { #placeholder }
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let signature: syn::Signature = input.parse()?;

        let content;
        braced!(content in input);
        // Forgive me, syn doesnt like custom placeholders
        let body = content.parse::<proc_macro2::TokenStream>()?;

        return Ok(Self {
            body: body,
            attributes: attrs,
            signature: signature,
        })
    }
}

impl Parse for OverTraitInput {
    // vis trait Name<generics> { method1 { ... } } impl ...
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;

        let vis: syn::Visibility = input.parse()?;
        input.parse::<Token![trait]>()?;
        let trait_name: Ident = input.parse()?;

        let mut generics: syn::Generics = input.parse()?;
        generics.where_clause = input.parse()?;

        let methods_content;
        let mut methods = Vec::new();
        braced!(methods_content in input);
        while !methods_content.is_empty() {
            methods.push(methods_content.parse()?);
        }

        let mut impls = Vec::new();
        while !input.is_empty() {
            impls.push(input.parse()?);
        }

        return Ok(Self {
            vis: vis,
            impls: impls,
            methods: methods,
            attributes: attrs,
            generics: generics,
            trait_name: trait_name,
        })
    }
}

fn array_len_ident(ty: &Type) -> Option<Ident> {
    match ty {
        Type::Reference(r) => array_len_ident(&r.elem),
        Type::Array(a) => {
            if let syn::Expr::Path(p) = &a.len {
                p.path.get_ident().cloned()
            } else { None }
        }
        _ => None,
    }
}

struct Placeholders {
    param_ident: Ident,
    len_expr: TokenStream2,
    bytes_expr: TokenStream2,
    capacity_expr: TokenStream2,
}

impl Placeholders {
    fn resolve(&self, name: &Ident) -> TokenStream2 {
        match name.to_string().as_str() {
            "len_expr" => self.len_expr.clone(),
            "bytes_expr" => self.bytes_expr.clone(),
            "capacity_expr" => self.capacity_expr.clone(),
            "param" => self.param_ident.clone().into_token_stream(),
            unknown => panic!("Unknown placeholder used: #{}", unknown),
        }
    }

    fn substitute(&self, tokens: TokenStream2) -> TokenStream2 {
        let mut generated = TokenStream2::new();
        let mut iter = tokens.into_iter().peekable();

        while let Some(tt) = iter.next() {
            if matches!(&tt, TokenTree::Punct(p) if p.as_char() == '#') {
                if let Some(TokenTree::Ident(name)) = iter.peek() {
                    generated.extend(self.resolve(name));
                    iter.next(); // consume the ident
                    continue;
                }
            }

            match tt {
                TokenTree::Group(group) => {
                    let mut expanded = Group::new(
                        group.delimiter(),
                        self.substitute(group.stream()),
                    );
                    expanded.set_span(group.span());
                    generated.extend(std::iter::once(TokenTree::Group(expanded)));
                }
                other => generated.extend(std::iter::once(other)),
            }
        }

        return generated;
    }
}

pub fn overtrait_impl(input: TokenStream) -> TokenStream {
    let mut methods = proc_macro2::TokenStream::new();
    let mut generated = proc_macro2::TokenStream::new();
    let input = parse_macro_input!(input as OverTraitInput);

    let vis = &input.vis;
    let trait_name = &input.trait_name;
    let trait_attrs = &input.attributes;
    let trait_generics = &input.generics;
    for method in &input.methods {
        let sig = &method.signature;
        let attrs = &method.attributes;
        methods.extend(quote! {
            #(#attrs)*
            #sig;
        });
    }

    generated.extend(quote! {
        #(#trait_attrs)*
        #vis trait #trait_name #trait_generics {
            #methods
        }
    });

    for impl_block in &input.impls {
        let trait_name = &input.trait_name;
        let target = &impl_block.target.clone();
        let mut generics = impl_block.generics.clone();
        // Target specific length identifiers `impl for [u8; N]`
        let target_len_ident = array_len_ident(&impl_block.target);
        let param = impl_block.generics.type_params().next().map(|p| p.ident.clone())
            .unwrap_or_else(|| Ident::new("T", proc_macro2::Span::call_site())).clone();

        for ty in &impl_block.types {
            let is_reference = matches!(ty, Type::Reference(_));
            let len_ident = array_len_ident(ty).or_else(|| target_len_ident.clone());

            let len_expr = match &len_ident {
                Some(len) => quote! { #len },
                None => quote! { self.len() },
            };

            let capacity_expr = match &len_ident {
                Some(len) => quote! { #len },
                None => quote! { self.capacity() },
            };

            let bytes_expr = match &len_ident {
                None => match is_reference {
                    true => quote! {bytemuck::cast_slice(*self)},
                    false => quote! { bytemuck::cast_slice(self) },
                }
                Some(n) => quote! { bytemuck::cast_slice(&self[..#n]) },
            };

            let placeholders = Placeholders {
                len_expr: len_expr,
                bytes_expr: bytes_expr,
                param_ident: param.clone(),
                capacity_expr: capacity_expr,
            };

            if let Some(n) = &len_ident {
                // Inject missing const generic parameters
                if !generics.const_params().any(|cp| cp.ident == *n) {
                    generics.params.push(syn::GenericParam::Const(
                        syn::parse_quote!(const #n: usize)
                    ));
                }
            }

            let bodies = input.methods.iter().map(|m| {
                let (attrs, sig) = (&m.attributes, &m.signature);
                let body = placeholders.substitute(m.body.clone());
                return quote! {
                    #(#attrs)*
                    #sig { #body }
                }
            });

            generated.extend(quote! {
                impl #generics #trait_name<#target> for #ty {
                    #(#bodies)*
                }
            });
        }
    }

    return TokenStream::from(generated);
}
