use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

pub fn impl_byteunpack(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    match &ast.data {
        syn::Data::Struct(data) => {
            let mut fields_byteunpack = quote!();
            let mut new_self = quote!();
            match &data.fields {
                syn::Fields::Named(fields) => {
                    for field in fields.named.iter() {
                        let field_name = field.ident.as_ref().unwrap();
                        fields_byteunpack.extend(quote!(
                            let #field_name = ByteUnpack::unpack(buf)?;
                            let byte_size = ByteSize::byte_size(&#field_name);
                            let buf = &buf[byte_size..];
                        ));
                        new_self.extend(quote!(
                            #field_name,
                        ));
                    }
                }
                syn::Fields::Unnamed(_) => unimplemented!(),
                syn::Fields::Unit => {},
            }
            let gen = quote! {
                impl ByteUnpack for #name {
                    fn unpack(buf: &[u8]) -> Result<Self, ()> {
                        #fields_byteunpack
                        Ok(Self {
                            #new_self
                        })
                    }
                }
            };
            gen.into()
        },
        syn::Data::Enum(_) => unimplemented!(),
        // syn::Data::Enum(data) => {
        //     let mut enum_variants_into_bson = quote!();
        //     for variant in &data.variants {
        //         let variant_name = &variant.ident;
        //         match variant.fields.len() {
        //             0 => {
        //                 enum_variants_into_bson.extend(quote!(
        //                     Self::#variant_name => bson::Bson::String(stringify!(#variant_name).to_string()),
        //                 ));
        //             }
        //             1 => {
        //                 enum_variants_into_bson.extend(quote!(
        //                     Self::#variant_name(val) => bson::Bson::Document(bson::doc!(
        //                         stringify!(#variant_name): IntoBson::into_bson(val),
        //                     )),
        //                 ));
        //             }
        //             n => {
        //                 let mut destructure_quote = quote!();
        //                 let mut bson_vec_quote = quote!();
        //                 destructure_quote.extend(
        //                     (0..n)
        //                         .map(|i| format_ident!("val_{i}"))
        //                         .map(|val_i| quote!(#val_i,)),
        //                 );
        //                 bson_vec_quote.extend(
        //                     (0..n)
        //                         .map(|i| format_ident!("val_{i}"))
        //                         .map(|val_i| quote!(IntoBson::into_bson(#val_i),)),
        //                 );

        //                 enum_variants_into_bson.extend(quote!(
        //                     Self::#variant_name(#destructure_quote) => bson::Bson::Document(bson::doc!(
        //                         stringify!(#variant_name): IntoBson::into_bson(vec![#bson_vec_quote]),
        //                     )),
        //                 ));
        //             } // _ => panic!("Only implemented for unit or tuple kinded variants"),
        //         }
        //     }
        //     let gen = quote! {
        //         impl IntoBson for #name {
        //             fn into_bson(self) -> bson::Bson {
        //                 match self {
        //                     #enum_variants_into_bson
        //                 }
        //             }
        //         }
        //     };
        //     gen.into()
        // }
        syn::Data::Union(_) => unimplemented!(),
    }
}