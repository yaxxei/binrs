use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Encode, attributes(bin))]
pub fn encode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let fields = match input.data {
        syn::Data::Struct(data_struct) => match data_struct.fields {
            syn::Fields::Named(fields_named) => fields_named.named,
            _ => return syn::Error::new(name.span(), "This macro works only for structs with named fields").to_compile_error().into()
        },
        _ => return syn::Error::new(name.span(), "This macro works only for structs").to_compile_error().into()
    };

    let field_names = fields.iter()
        .filter(|f| {
            !f.attrs.iter().any(|attr| {
                attr.path().is_ident("bin") &&
                attr.parse_args::<syn::Ident>().map_or(false, |ident| ident == "skip")
            })
        })
        .map(|f| &f.ident);

    let generated = quote! {
        impl binrs::encoder::Encode for #name {
            fn encode<E: binrs::encoder::Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
                #(self.#field_names.encode(encoder)?;)*                
                Ok(())
            }
        } 
    };

    generated.into()
}

#[proc_macro_derive(Decode, attributes(binrs))]
pub fn decode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let fields = match input.data {
        syn::Data::Struct(data_struct) => match data_struct.fields {
            syn::Fields::Named(fields_named) => fields_named.named,
            _ => return syn::Error::new(name.span(), "This macro works only for structs with named fields").to_compile_error().into()
        },
        _ => return syn::Error::new(name.span(), "This macro works only for structs").to_compile_error().into()
    };

    let (field_names, field_types): (Vec<_>, Vec<_>) = fields.iter()
        .filter(|f| {
            !f.attrs.iter().any(|attr| {
                attr.path().is_ident("bin") &&
                attr.parse_args::<syn::Ident>().map_or(false, |ident| ident == "skip")
            })
        })
        .map(|f| (&f.ident, &f.ty))
        .unzip();
    
    let skipped_fields = fields.iter()
        .filter(|f| {
            f.attrs.iter().any(|attr| {
                attr.path().is_ident("bin") &&
                attr.parse_args::<syn::Ident>().map_or(false, |ident| ident == "skip")
            })
        })
        .map(|f| &f.ident);

    let generated = quote! {
        impl binrs::decoder::Decode for #name {
            fn decode<D: binrs::decoder::Decoder>(decoder: &mut D) -> Result<Self, Error> {
                Ok(Self {
                    #(#field_names: <#field_types as binrs::decoder::Decode>::decode(decoder)?,)*
                    #(#skipped_fields: Default::default(),)*      
                })
            }
        } 
    };

    generated.into()
}
