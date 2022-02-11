#![allow(warnings)]
#![feature(extend_one)]

extern crate proc_macro;

use colored::Colorize;
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree, Punct, Spacing, Group, };
use proc_macro2::{Ident, Span};
use proc_macro2::Delimiter;
use proc_macro_error::{abort, proc_macro_error, ResultExt};
use quote::{format_ident, quote, quote_spanned, ToTokens, TokenStreamExt};
use rust_internal::{PluginAttributes, Attribute, NumberAttribute, BoolAttribute, TextAttribute};
use syn::parse::Parser;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, Type, Lit};


extern crate syn;
extern crate quote;
extern crate proc_macro2;

use syn::{DataStruct, Meta};
use yew::{Html, html};
use yew::virtual_dom::VNode;

mod generate;


fn attribute_type(ty: &Type) -> String {
    match ty {
        Type::Path(path) => {
            let ty = &path.path.segments.first().unwrap().ident;
            
            ty.to_string()
        },
        _ => todo!()
    }
}



#[proc_macro_derive(Plugin, attributes(get, with_prefix, option))]
#[proc_macro_error]
pub fn plugin(input: TokenStream) -> TokenStream {
    
    // Parse the string representation
    let ast: DeriveInput = syn::parse(input).expect_or_abort("Couldn't parse for plugin");

    let number_types: Vec<&str>  = vec!["i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64"];

    let mut attrs: Vec<PluginAttributes> = Vec::new();
    match &ast.data {
        Data::Struct(s) => {
            match &s.fields {
                Fields::Named(n) => {
                    
                    for named in &n.named { // Field
                        let name = named.ident.as_ref().unwrap();
                        let ty = attribute_type(&named.ty);
                        
                        let is_number = number_types.contains(&&ty[..]);
                        
                        for attribute in &named.attrs {                            
                            if !attribute.path.is_ident("option") {
                                panic!("attribute {} has no option annotation.", name.to_string());
                            }

                            let metas = parse_attr(attribute);
                            let label = get_mandatory_meta_value_as_string(&metas, "label").unwrap_or_else(|| panic!("the attribute {} is missing for {}", "label".red(), name));
                            let description = get_mandatory_meta_value_as_string(&metas, "description").unwrap_or_else(|| panic!("the attribute {} is missing for {}", "description".red(), name));

                            if is_number {                                
                                //let default = get_mandatory_meta_value_as_isize(&metas, "default").unwrap_or_else(|| panic!("the attribute {} is missing for {}", "default".red(), name));
                                let min = get_mandatory_meta_value_as_isize(&metas, "min").unwrap_or_else(|| panic!("the attribute {} is missing for {}", "min".red(), name));
                                let max = get_mandatory_meta_value_as_isize(&metas, "max").unwrap_or_else(|| panic!("the attribute {} is missing for {}", "max".red(), name));
                                let default = get_mandatory_meta_value_as_isize(&metas, "default").unwrap_or_else(|| panic!("the attribute {} is missing for {}", "default".red(), name));

                                attrs.push(PluginAttributes::Number((Attribute { label, description }, NumberAttribute { default, min, max })))
                            } else if ty == "bool" {
                                let default = get_mandatory_meta_value_as_bool(&metas, "default").unwrap_or_else(|| panic!("the attribute {} is missing for {}", "default".red(), name));
                                attrs.push(PluginAttributes::Bool((Attribute { label, description }, BoolAttribute { default })))
                            } else if ty == "String" {
                                let default = get_mandatory_meta_value_as_string(&metas, "default").unwrap_or_else(|| panic!("the attribute {} is missing for {}", "default".red(), name));
                                attrs.push(PluginAttributes::Text((Attribute { label, description }, TextAttribute { default })))
                            }
                        }                        
                    }
                },
                _ => panic!("Only works on named attributes")
            }
        },
        _ => panic!("Derive macro \"Plugin\" can only applied to a structs. Use it like this:\n\n#[derive(Plugin)]\nstruct MyPlugin{{}};")
    }

    for attr in &ast.attrs {
        panic!("{:?}", attr);
    }

    // Build the impl
    let gen = produce(&ast, attrs);
    


    // Return the generated impl
    gen.into()
}

fn get_mandatory_meta_value<'a>(meta_attrs: &'a Vec<Meta>, identifier: &str) -> Option<&'a Lit> {
    if let Some(default_meta) = meta_attrs.iter().find(|meta| {
        meta.path().is_ident(identifier)
    }) {
        if let Meta::NameValue(e) = &default_meta {
            return Some(&e.lit);
        }
    }
    
    None
}

fn get_mandatory_meta_value_as_f64(meta_attrs: &Vec<Meta>, identifier: &str) -> Option<f64> {
    if let Some(lit) = get_mandatory_meta_value(meta_attrs, identifier) {
        if let Lit::Float(e) = lit {
            return Some(e.base10_parse::<f64>().unwrap_or_abort());
        }
    }

    None
}

fn get_mandatory_meta_value_as_isize(meta_attrs: &Vec<Meta>, identifier: &str) -> Option<i128> {
    if let Some(lit) = get_mandatory_meta_value(meta_attrs, identifier) {
        if let Lit::Int(e) = lit {            
            return Some(e.base10_parse::<i128>().unwrap_or_abort());
        }
    }

    None
}

fn get_mandatory_meta_value_as_string(meta_attrs: &Vec<Meta>, identifier: &str) -> Option<String> {
    if let Some(lit) = get_mandatory_meta_value(meta_attrs, identifier) {
        if let Lit::Str(e) = lit {
            return Some(e.value())
        }
    }

    None
}

fn get_mandatory_meta_value_as_bool(meta_attrs: &Vec<Meta>, identifier: &str) -> Option<bool> {
    if let Some(lit) = get_mandatory_meta_value(meta_attrs, identifier) {
        if let Lit::Bool(e) = lit {
            return Some(e.value())
        }
    }

    None
}


fn parse_attr(attr: &syn::Attribute) -> Vec<Meta> {
    use syn::{punctuated::Punctuated, Token};

    if attr.path.is_ident("option") {
        let last= attr
            .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            .unwrap_or_abort()
            .into_iter()
            .inspect(|meta| {
                if !(meta.path().is_ident("default")
                    || meta.path().is_ident("min")
                    || meta.path().is_ident("max")
                    || meta.path().is_ident("label")
                    || meta.path().is_ident("description"))
                {
                    abort!(meta.path().span(), "unknown parameter")
                }
            })
            .fold(vec![], |mut last, meta| {               
                last.push(meta);
                
                last
            });

        return last;
    }

    vec![]
}

fn produce(ast: &DeriveInput, attrs: Vec<PluginAttributes>) -> TokenStream2 {
    use yew::html;

    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();



    // Is it a struct?
    if let syn::Data::Struct(DataStruct { ref fields, .. }) = ast.data {
        let mut t = TokenStream2::new();
        t.extend_one(TokenTree::Ident(Ident::new("html",  Span::call_site())));
        t.extend_one(TokenTree::Punct(Punct::new('!', Spacing::Alone)));
        //
    
        let mut inner = TokenStream2::new();
        let mut muus: Vec<TokenStream2> = vec![
            quote! { <div> },

    
            
        ];
        
        for attr in attrs {
            match attr {
                PluginAttributes::Number((attr, number_attr)) => {
                    let min = number_attr.min;
                    let max = number_attr.max;
                    let label = attr.label;
                    let default = number_attr.default;
                        
                    muus.push(quote!{<div><label>{#label}</label><input type="number" min=#min max=#max value=#default /></div>});            
                },
                PluginAttributes::Text((attr, str_attr)) => {
                    let label = attr.label;
                    let default = str_attr.default;
                    muus.push(quote!{<div><label>{#label}</label><input type="text" value=#default /></div>});
                },
                PluginAttributes::Bool((attr, bool_attr)) => {
                    let label = attr.label;
                    let default = bool_attr.default;
                    muus.push(quote!{<div><label>{#label}</label><input type="checkbox" checked=#default /></div>})
                },
            }
        }

        
        
        muus.push(quote!{</div>});
        inner.append_all(muus);
        t.extend_one(TokenTree::Group(Group::new(Delimiter::Brace, inner)));
        

        let gen = quote! {           
            impl<T> PluginWithOptions<T> for Grid where T: Renderer + 'static {
                fn view_options(&self) -> Html {
                    #t
                }
            }            
        };

        gen

    } else {
        quote! {}
    }
}

/*
#[proc_macro_derive(Plugin)]
pub fn plugin(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let name = input.ident;

    let modified = quote! {
        use std::any::Any;
        use rust_internal::AsAny;

        impl AsAny for #name {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
    TokenStream::from(modified)
}
*/

#[proc_macro_derive(ElementId)]
pub fn derive_id_trait_functions(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let name = input.ident;

    let modified = quote! {
        impl SetId for #name {
            fn set_id(&mut self, id: Uuid) {
                self.id = id;
            }
        }

        impl Id for #name {
            fn id(&self) -> Uuid {
                self.id
            }
        }
    };
    TokenStream::from(modified)
}

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

#[proc_macro_derive(IsVariant)]
pub fn derive_is_variant(input: TokenStream) -> TokenStream {
    // See https://doc.servo.org/syn/derive/struct.DeriveInput.html
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    // get enum name
    let ref name = input.ident;
    let ref data = input.data;

    let mut variant_checker_functions;

    // data is of type syn::Data
    // See https://doc.servo.org/syn/enum.Data.html
    match data {
        // Only if data is an enum, we do parsing
        Data::Enum(data_enum) => {
            // data_enum is of type syn::DataEnum
            // https://doc.servo.org/syn/struct.DataEnum.html

            variant_checker_functions = TokenStream2::new();

            // Iterate over enum variants
            // `variants` if of type `Punctuated` which implements IntoIterator
            //
            // https://doc.servo.org/syn/punctuated/struct.Punctuated.html
            // https://doc.servo.org/syn/struct.Variant.html
            for variant in &data_enum.variants {
                // Variant's name
                let ref variant_name = variant.ident;

                if let Fields::Unnamed(e) = &variant.fields {
                    if let syn::Type::Path(e) = &e.unnamed[0].ty {
                        let return_type = &e.path.segments.first().unwrap().ident;

                        // construct an identifier named is_<variant_name> for function name
                        // We convert it to snake case using `to_case(Case::Snake)`
                        // For example, if variant is `HelloWorld`, it will generate `is_hello_world`
                        let mut get_as_func_name =
                            format_ident!("get_{}", variant_name.to_string().to_case(Case::Snake));
                        get_as_func_name.set_span(variant_name.span());

                        // Here we construct the function for the current variant
                        variant_checker_functions.extend(quote_spanned! {variant.span()=>


                            #[allow(dead_code)]
                            fn #get_as_func_name(&self) -> Option<&#return_type> {
                                match self {
                                    #name::#variant_name(e) => Some(e),
                                    _ => None,
                                }
                            }

                        });
                    }
                }

                // Above we are making a TokenStream using extend()
                // This is because TokenStream is an Iterator,
                // so we can keep extending it.
                //
                // proc_macro2::TokenStream:- https://docs.rs/proc-macro2/1.0.24/proc_macro2/struct.TokenStream.html

                // Read about
                // quote:- https://docs.rs/quote/1.0.7/quote/
                // quote_spanned:- https://docs.rs/quote/1.0.7/quote/macro.quote_spanned.html
                // spans:- https://docs.rs/syn/1.0.54/syn/spanned/index.html
            }
        }
        _ => return derive_error!("IsVariant is only implemented for enums"),
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            // variant_checker_functions gets replaced by all the functions
            // that were constructed above
            #variant_checker_functions
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn data_source(args: TokenStream, input: TokenStream) -> TokenStream {
    let args_parsed = syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated
        .parse(args)
        .unwrap(); // Better to turn it into a `compile_error!()`

    let mut var_names: Vec<Ident> = vec![];
    let mut var_types: Vec<Ident> = vec![];
    let mut ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    for data_type_arg in args_parsed {
                        let data_type_arg = data_type_arg.segments[0].ident.clone();

                        let name = Ident::new(
                            &format!("{}s", data_type_arg.to_string().to_lowercase()).to_string(),
                            Span::call_site(),
                        );

                        var_names.push(name.clone());
                        var_types.push(data_type_arg.clone());

                        fields.named.push(
                            syn::Field::parse_named
                                .parse2(quote! { pub #name: HashMap<Uuid, #data_type_arg> })
                                .unwrap_or_abort(),
                        );
                    }
                }
                _ => (),
            }

            /*
            return quote! {
                #ast
            }.into();
            */
        }
        _ => panic!("`add_field` has to be used with structs "),
    }

    let mut get_all_fn: Vec<TokenStream2> = vec![];
    let mut get_single_fn: Vec<TokenStream2> = vec![];
    let mut get_multiple_by_id_fn: Vec<TokenStream2> = vec![];
    let mut get_multiple_at_position_fn: Vec<TokenStream2> = vec![];
    for (i, _) in var_names.iter().enumerate() {
        get_all_fn.push(implement_get_all(
            var_names[i].clone(),
            var_types[i].clone(),
        ));
        get_single_fn.push(implement_get_single(
            var_names[i].clone(),
            var_types[i].clone(),
        ));
        get_multiple_by_id_fn.push(implement_get_multiple_by_id(
            var_names[i].clone(),
            var_types[i].clone(),
        ));

        get_multiple_at_position_fn.push(implement_get_at_position(
            var_names[i].clone(),
            var_types[i].clone(),
        ));
    }

    let result = quote! {
        use std::collections::HashMap;
        use uuid::Uuid;
        use geo_types::Coordinate;

        //#[derive(Debug)]
        #ast

        impl #struct_name {

            pub fn new() -> Self {
                #struct_name { #( #var_names: HashMap::new()),* }
            }

            #( #get_all_fn )*

            #( #get_single_fn )*

            #( #get_multiple_by_id_fn )*

            #( #get_multiple_at_position_fn )*
        }
    };

    result.into()
}

fn function_name(var_name: &Ident, suffix: String, mutable: bool) -> Ident {
    Ident::new(
        &format!(
            "{}_{}{}",
            var_name.to_string(),
            suffix,
            if mutable { "_mut" } else { "" }
        )
        .to_string(),
        Span::call_site(),
    )
}

fn implement_get_all(var_name: Ident, var_type: Ident) -> TokenStream2 {
    let var_name_mut = Ident::new(
        &format!("{}_mut", var_name.to_string()).to_string(),
        Span::call_site(),
    );
    quote! {
        pub fn #var_name(&self) -> &HashMap<Uuid, #var_type> {
            &self.#var_name
        }

        pub fn #var_name_mut(&mut self) -> &mut HashMap<Uuid, #var_type> {
            &mut self.#var_name
        }
    }
}

fn implement_get_single(var_name: Ident, var_type: Ident) -> TokenStream2 {
    let single_var_name = var_name.to_string();
    let single_var_name: &str = &single_var_name[0..single_var_name.len() - 1];

    let single_var_name = Ident::new(&single_var_name, Span::call_site());
    let single_var_name_mut = Ident::new(
        &format!("{}_mut", single_var_name).to_string(),
        Span::call_site(),
    );
    quote! {
        pub fn #single_var_name(&self, id: &Uuid) -> Option<&#var_type> {
            if self.#var_name.contains_key(id) {
                return Some(self.#var_name.get(id).unwrap());
            }

            None
        }

        pub fn #single_var_name_mut(&mut self, id: &Uuid) -> Option<&mut #var_type> {
            if self.#var_name.contains_key(id) {
                return Some(self.#var_name.get_mut(id).unwrap());
            }

            None
        }
    }
}

fn implement_get_multiple_by_id(var_name: Ident, var_type: Ident) -> TokenStream2 {
    let fn_name = function_name(&var_name, "by_ids".to_string(), false);
    let fn_name_mut = function_name(&var_name, "by_ids".to_string(), true);

    quote! {
        pub fn #fn_name <'a>(
            &'a self,
            ids: &'a Vec<Uuid>,
        ) -> impl Iterator<Item = &'a #var_type> {
            self.#var_name
                .values()
                .filter(|element| ids.contains(&element.id()))
        }


        pub fn #fn_name_mut <'a>(
            &'a mut self,
            ids: &'a Vec<Uuid>,
        ) -> impl Iterator<Item = &'a mut #var_type> {
            self.#var_name
                .values_mut()
                .filter(|element| ids.contains(&element.id()))
        }

    }
}

fn implement_get_at_position(var_name: Ident, var_type: Ident) -> TokenStream2 {
    let fn_name = function_name(&var_name, "at_position".to_string(), false);
    let fn_name_mut = function_name(&var_name, "at_position".to_string(), true);

    quote! {
        pub fn #fn_name <'a>(
            &'a self,
            position: &Coordinate<f64>,
            offset: f64
        ) -> Option<&'a #var_type> {
            for (id, element) in &self.#var_name {
                if element.position().euclidean_distance(position) < offset {
                    return Some(*id);
                }
            }

            None
        }

        pub fn #fn_name_mut <'a>(
            &'a mut self,
            position: &Coordinate<f64>,
            offset: f64
        ) -> Option<&'a mut #var_type> {
            for (id, element) in &mut self.#var_name {
                if element.position().euclidean_distance(position) < offset {
                    return Some(*id);
                }
            }

            None
        }
    }
}
