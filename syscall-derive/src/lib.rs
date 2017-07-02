#![feature(box_patterns)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use syn::{Ty, MutTy, Mutability, MetaItem, Ident, Lit};
use proc_macro::TokenStream;

fn num_to_ident(num: usize) -> quote::Ident {
    quote::Ident::from((('a' as u8 + num as u8) as char).to_string())
}

fn impl_syscall(ast: &syn::DeriveInput) -> quote::Tokens {
    let variants = match ast.body {
        syn::Body::Enum(ref v) => v,
        _ => panic!("Must be enum")
    };

    let enum_name = &ast.ident;
    let mut name_arms = Vec::new();
    let mut to_arms = Vec::new();
    let mut from_arms = Vec::new();

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    for variant in variants {
        let mut number = None;
        for attr in variant.attrs.iter() {
            if let &MetaItem::NameValue(ref ident, ref value) = &attr.value {
                if *ident == Ident::from("number") {
                    number = match *value {
                        Lit::Int(value, _) => Some(value as usize),
                        _ => panic!("Type not supported by 'number' attribute")
                    };
                }
            }
        }
        let number = number.expect("number attribute is mandatory");
        let name = &variant.ident;
        let fields = match variant.data {
            syn::VariantData::Tuple(ref fields) => fields,
            _ => panic!("Only tuple variants allowed")
        };

        let mut call = Vec::new();
        call.push(quote!(#number));
        let mut field_names = Vec::new();
        let mut from_names = Vec::new();

        let mut from_num = 1;
        for (idx, field) in fields.iter().enumerate() {
            let name = num_to_ident(idx);
            // For slices, push pointer and length
            if let Ty::Rptr(_, box MutTy { ty: Ty::Slice(..), mutability }) = field.ty {
                call.push(match mutability {
                    Mutability::Mutable => quote!(#name.as_mut_ptr() as usize),
                    Mutability::Immutable => quote!(#name.as_ptr() as usize)
                });
                call.push(quote!(#name.len() as usize));
                field_names.push(match mutability {
                    Mutability::Mutable => quote!(ref mut #name),
                    Mutability::Immutable => quote!(ref #name)
                });
                let from1 = num_to_ident(from_num);
                let from2 = num_to_ident(from_num + 1);
                from_names.push(match mutability {
                    Mutability::Mutable => quote!(T::to_slice_mut(#from1, #from2)),
                    Mutability::Immutable => quote!(T::to_slice(#from1, #from2))
                });
                from_num += 1;
            } else {
                call.push(quote!(#name as usize));
                field_names.push(quote!(#name));
                let from = num_to_ident(from_num);
                from_names.push(quote!(#from as _));
            }
            from_num += 1;
        }

        if call.len() > 6 {
            panic!("Variant {} results in more than 6 values", name);
        } else {
            while call.len() < 6 {
                call.push(quote!(0));
            }
        }

        name_arms.push(quote!{ #enum_name::#name(..) => stringify!(#name) });
        to_arms.push(quote!{ #enum_name::#name(#(#field_names),*) => (#(#call),*) });
        from_arms.push(quote!{ #number => #enum_name::#name(#(#from_names),*) });
    }

    from_arms.push(quote!{ _ => unreachable!() });

    quote! {
        impl #impl_generics #enum_name #ty_generics #where_clause {
            pub fn name(&self) -> &'static str {
                match *self {
                    #(#name_arms),*
                }
            }

            pub fn to_call(&mut self) -> (usize, usize, usize, usize, usize, usize) {
                match *self {
                    #(#to_arms),*
                }
            }

            pub unsafe fn from_call<T: SyscallMem>(a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> Self {
                match a {
                    #(#from_arms),*
                }
            }
        }
    }
}

#[proc_macro_derive(Syscall, attributes(number))]
pub fn syscall(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_syscall(&ast);
    gen.parse().unwrap()
}
