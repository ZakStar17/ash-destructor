use proc_macro::TokenStream;

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

    let gen = quote::quote! {
        impl ash_destructor::DeviceDestroyable for #name {
            unsafe fn destroy_self(&self, device: &ash::Device) {
                // todo
            }
        }
    };

    Ok(gen.into())
}
