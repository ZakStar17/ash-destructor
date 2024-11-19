use proc_macro::TokenStream;
use syn::{spanned::Spanned, Fields};

#[proc_macro_derive(DeviceDestroyable, attributes(builder))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = match syn::parse(input) {
        Ok(data) => data,
        Err(err) => return err.to_compile_error().into(),
    };

    // Build the trait implementation
    impl_macro(&ast).unwrap_or_else(|err| err.to_compile_error().into())
}

fn impl_macro(ast: &syn::DeriveInput) -> Result<proc_macro::TokenStream, syn::Error> {
    let name = &ast.ident;

    let fields = if let syn::Data::Struct(data) = &ast.data {
        &data.fields
    } else {
        return Err(syn::Error::new(
            ast.span(),
            "Enums are currently unsupported",
        ));
    };

    let function_destroy_stmts = match &fields {
        Fields::Named(fields) => fields.named.iter().map(|f| {
            let name = f.ident.as_ref().unwrap();
            quote::quote_spanned! {f.span()=>
                ash_destructor::DeviceDestroyable::destroy_self(&self.#name, device);
            }
        }).collect(),
        Fields::Unnamed(fields) => fields.unnamed.iter().enumerate().map(|(i, f)| {
            let tuple_i = syn::Index::from(i);
            quote::quote_spanned! {f.span()=>
                ash_destructor::DeviceDestroyable::destroy_self(&self.#tuple_i, device);
            }
        }).collect(),
        Fields::Unit => Vec::new()
    };

    let gen = quote::quote! {
        impl ash_destructor::DeviceDestroyable for #name {
            unsafe fn destroy_self(&self, device: &ash::Device) {
                #(#function_destroy_stmts)*
            }
        }
    };

    Ok(gen.into())
}
