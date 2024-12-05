use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Field};

#[proc_macro_derive(DeviceDestroyable, attributes(skip, skip_remaining))]
pub fn derive_device_destroyable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
}

fn parse_attributes<'a>(
    input_name: &syn::Ident,
    fields: &mut impl ExactSizeIterator<Item = &'a Field>,
    errors: &mut Vec<syn::Error>,
) -> (Option<usize>, Vec<FieldAttributes>) {
    let mut field_attrs = Vec::with_capacity(fields.len());
    let mut skip_remaining_index = None;

    for (f_i, field) in fields.enumerate() {
        let mut attrs = FieldAttributes::default();
        for attr in field.attrs.iter() {
            if attr.path().is_ident("skip_remaining") {
                if skip_remaining_index.is_some() {
                    errors.push(syn::Error::new(
                        attr.span(),
                        format!(
                            "Multiple #[skip_remaining] attributes in {:?}",
                            input_name.to_string()
                        ),
                    ));
                    continue;
                }
                if let Err(err) = attr.meta.require_path_only() {
                    errors.push(err);
                }
                skip_remaining_index = Some(f_i);
            }
        }

        for (attr_i, attr) in field.attrs.iter().enumerate() {
            if attr.path().is_ident("skip") {
                if let Some(skip_i) = skip_remaining_index {
                    if skip_i >= attr_i {
                        errors.push(syn::Error::new(
                            attr.span(),
                            "Attribute #[skip] is not allowed after a #[skip_remaining] attribute declaration",
                        ));
                    }
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
            }
        }

        field_attrs.push(attrs);
    }

    (skip_remaining_index, field_attrs)
}

struct FunctionDestroyStmtsFieldIterator<
    'a,
    T: ExactSizeIterator<Item = &'a Field> + DoubleEndedIterator<Item = &'a Field>,
> {
    fields_iter: std::iter::Rev<std::iter::Enumerate<&'a mut T>>,
    field_attributes: &'a Vec<FieldAttributes>,
}

impl<'a, T: ExactSizeIterator<Item = &'a Field> + DoubleEndedIterator<Item = &'a Field>>
    FunctionDestroyStmtsFieldIterator<'a, T>
{
    fn new(
        fields: &'a mut T,
        field_attributes: &'a Vec<FieldAttributes>,
        skip_everything_after: usize,
    ) -> Self {
        let fields_len = fields.len();
        let mut fields_iter = fields.enumerate().rev();
        // skip all elements after skip_everything_after
        for _ in 0..(fields_len - skip_everything_after) {
            let _ = fields_iter.next();
        }

        Self {
            fields_iter,
            field_attributes,
        }
    }
}

impl<'a, T: ExactSizeIterator<Item = &'a Field> + DoubleEndedIterator<Item = &'a Field>> Iterator
    for FunctionDestroyStmtsFieldIterator<'a, T>
{
    type Item = TokenStream;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (i, field) = self.fields_iter.next()?;
            let attrs = &self.field_attributes[i];

            if !attrs.skip {
                return Some(if let Some(ident) = field.ident.as_ref() {
                    quote::quote_spanned! {field.span() =>
                        ash_destructor::DeviceDestroyable::destroy_self_alloc(&self.#ident, device, allocation_callbacks);
                    }
                } else {
                    let tuple_i = syn::Index::from(i);
                    quote::quote_spanned! {field.span() =>
                        ash_destructor::DeviceDestroyable::destroy_self_alloc(&self.#tuple_i, device, allocation_callbacks);
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
    let (skip_after, field_attributes) = parse_attributes(name, &mut fields.iter(), &mut errors);

    let function_fields_iter = &mut fields.iter();
    let function_destroy_stmts_iter = FunctionDestroyStmtsFieldIterator::new(
        function_fields_iter,
        &field_attributes,
        skip_after.unwrap_or(fields.len()),
    );

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let stream_errors = errors.iter().map(syn::Error::to_compile_error);
    let gen = quote::quote! {
        impl #impl_generics ash_destructor::DeviceDestroyable for #name #ty_generics #where_clause {
            unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: std::option::Option<&ash::vk::AllocationCallbacks<'_>>) {
                #(#function_destroy_stmts_iter)*
            }

            #(#stream_errors)*
        }
    };

    Ok(gen.into())
}
