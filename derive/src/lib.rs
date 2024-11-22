use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Field};

#[proc_macro_derive(DeviceDestroyable, attributes(skip, skip_remaining))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = match syn::parse(input) {
        Ok(data) => data,
        Err(err) => return err.to_compile_error().into(),
    };

    // Build the trait implementation
    impl_macro(&ast).unwrap_or_else(|err| err.to_compile_error().into())
}

#[derive(Debug, Default)]
struct FieldAttributes {
    pub skip: bool,
    pub skip_remaining: bool,
}

fn parse_attributes<'a>(
    input_name: &syn::Ident,
    fields: &mut impl ExactSizeIterator<Item = &'a Field>,
    errors: &mut Vec<syn::Error>,
) -> Vec<FieldAttributes> {
    let mut field_attrs = Vec::with_capacity(fields.len());
    let mut skip_remaining_reached = false;

    for field in fields {
        let mut attrs = FieldAttributes::default();
        for attr in field.attrs.iter() {
            if attr.path().is_ident("skip") {
                if skip_remaining_reached {
                    errors.push(syn::Error::new(
                        attr.span(),
                        "Attribute #[skip] is not allowed after a #[skip_remaining] attribute declaration",
                    ));
                }
                if attrs.skip {
                    errors.push(syn::Error::new(
                        field.span(),
                        "Multiple #[skip] attributes on a single field",
                    ));
                }
                if let Err(err) = attr.meta.require_path_only() {
                    errors.push(err);
                }
                attrs.skip = true;
            } else if attr.path().is_ident("skip_remaining") {
                if skip_remaining_reached {
                    errors.push(syn::Error::new(
                        attr.span(),
                        format!("Multiple #[skip_remaining] attributes in {:?}", input_name.to_string()),
                    ));
                }
                if let Err(err) = attr.meta.require_path_only() {
                    errors.push(err);
                }
                attrs.skip_remaining = true;
                skip_remaining_reached = true;
            }
        }
        field_attrs.push(attrs);
    }

    field_attrs
}

struct FunctionDestroyStmtsFieldIterator<'a> {
    fields_iter: &'a mut dyn ExactSizeIterator<Item = (usize, &'a Field)>,
    field_attributes: &'a Vec<FieldAttributes>
}

impl<'a> Iterator for FunctionDestroyStmtsFieldIterator<'a> {
    type Item = TokenStream;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some((i, field)) = self.fields_iter.next() else {
                return None;
            };
            let attrs = &self.field_attributes[i];
            
            if attrs.skip_remaining {
                return None;
            }

            if !attrs.skip {
                return Some(if let Some(ident) = field.ident.as_ref() {
                    quote::quote_spanned! {field.span() =>
                        ash_destructor::DeviceDestroyable::destroy_self(&self.#ident, device);
                    }
                } else {
                    let tuple_i = syn::Index::from(i);
                    quote::quote_spanned! {field.span() =>
                        ash_destructor::DeviceDestroyable::destroy_self(&self.#tuple_i, device);
                    }
                });
            }
        }
    }
}

fn impl_macro(ast: &syn::DeriveInput) -> Result<proc_macro::TokenStream, syn::Error> {
    let name = &ast.ident;

    let fields = match &ast.data {
        syn::Data::Struct(data) => &data.fields,
        syn::Data::Enum(_) => {
            return Err(syn::Error::new(
                ast.span(),
                "Enums are currently unsupported",
            ))
        }
        syn::Data::Union(_) => {
            return Err(syn::Error::new(
                ast.span(),
                "Unions are currently unsupported",
            ))
        }
    };

    let mut errors = Vec::new();
    let field_attributes = parse_attributes(name, &mut fields.iter(), &mut errors);

    let function_destroy_stmts_iter = FunctionDestroyStmtsFieldIterator {
        fields_iter: &mut fields.iter().enumerate(),
        field_attributes: &field_attributes
    };

    let stream_errors = errors.iter().map(syn::Error::to_compile_error);
    let gen = quote::quote! {
        impl ash_destructor::DeviceDestroyable for #name {
            unsafe fn destroy_self(&self, device: &ash::Device) {
                #(#function_destroy_stmts_iter)*
            }

            #(#stream_errors)*
        }
    };

    Ok(gen.into())
}
