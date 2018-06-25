use Route;

use proc_macro2::TokenStream;
use syn;

#[derive(Debug)]
pub struct Service {
    pub self_ty: Box<syn::Type>,
    pub routes: Vec<Route>,
}

impl Service {
    pub fn new(self_ty: Box<syn::Type>) -> Service {
        Service {
            self_ty,
            routes: vec![],
        }
    }

    pub fn destination_ty(&self) -> TokenStream {
        match self.routes.len() {
            0 | 1 => quote! { () },
            2 => quote! { ::tower_web::util::tuple::Either2<(), ()> },
            3 => quote! { ::tower_web::util::tuple::Either3<(), (), ()> },
            4 => quote! { ::tower_web::util::tuple::Either4<(), (), (), ()> },
            5 => quote! { ::tower_web::util::tuple::Either5<(), (), (), (), ()> },
            6 => quote! { ::tower_web::util::tuple::Either6<(), (), (), (), (), ()> },
            7 => quote! { ::tower_web::util::tuple::Either7<(), (), (), (), (), (), ()> },
            8 => quote! { ::tower_web::util::tuple::Either8<(), (), (), (), (), (), (), ()> },
            9 => {
                quote! { ::tower_web::util::tuple::Either9<(), (), (), (), (), (), (), (), ()> }
            }
            _ => panic!("unimplemented; Service::destination_ty"),
        }
    }

    pub fn destination_ty_use(&self) -> TokenStream {
        match self.routes.len() {
            0 | 1 => quote!{},
            2 => quote! { use ::tower_web::util::tuple::Either2::*; },
            3 => quote! { use ::tower_web::util::tuple::Either3::*; },
            4 => quote! { use ::tower_web::util::tuple::Either4::*; },
            5 => quote! { use ::tower_web::util::tuple::Either5::*; },
            6 => quote! { use ::tower_web::util::tuple::Either6::*; },
            7 => quote! { use ::tower_web::util::tuple::Either7::*; },
            8 => quote! { use ::tower_web::util::tuple::Either8::*; },
            9 => quote! { use ::tower_web::util::tuple::Either9::*; },
            _ => panic!("unimplemented; Service::destination_ty"),
        }
    }
}