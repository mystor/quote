use std::{f32, f64};

#[macro_use]
extern crate quote;

struct X;

impl quote::ToTokens for X {
    fn to_tokens(&self, tokens: &mut quote::Tokens) {
        tokens.append("X");
    }
}

#[test]
fn test_quote_impl() {
    let tokens = quote!(
        impl<'a, T: ToTokens> ToTokens for &'a T {
            fn to_tokens(&self, tokens: &mut Tokens) {
                (**self).to_tokens(tokens)
            }
        }
    );

    let expected = concat!(
        "impl < 'a , T : ToTokens > ToTokens for & 'a T { ",
            "fn to_tokens ( & self , tokens : & mut Tokens ) { ",
                "( * * self ) . to_tokens ( tokens ) ",
            "} ",
        "} "
    );

    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_substitution() {
    let x = X;
    let tokens = quote!(#x <#x> (#x) [#x] {#x});

    let expected = "X < X > ( X ) [ X ] { X } ";

    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_iter() {
    let primes = vec![X, X, X, X];

    assert_eq!("X X X X ", quote!(#(&primes)*).to_string());

    assert_eq!("X , X , X , X , ", quote!(#(&primes,)*).to_string());

    assert_eq!("X , X , X , X ", quote!(#(&primes),*).to_string());
}

#[test]
fn test_iter_with_non_vec() {
    let primes: &[X] = &[X, X, X, X];

    assert_eq!("X X X X ", quote!(#(primes)*).to_string());

    assert_eq!("X , X , X , X , ", quote!(#(primes,)*).to_string());

    assert_eq!("X , X , X , X ", quote!(#(primes),*).to_string());
}

#[test]
fn test_advanced() {
    let generics = quote!( <'a, T> );

    let where_clause = quote!( where T: Serialize );

    let field_ty = quote!( String );

    let item_ty = quote!( Cow<'a, str> );

    let path = quote!( SomeTrait::serialize_with );

    let value = quote!( self.x );

    let tokens = quote! {
        struct SerializeWith #generics #where_clause {
            value: &'a #field_ty,
            phantom: ::std::marker::PhantomData<#item_ty>,
        }

        impl #generics ::serde::Serialize for SerializeWith #generics #where_clause {
            fn serialize<S>(&self, s: &mut S) -> Result<(), S::Error>
                where S: ::serde::Serializer
            {
                #path(self.value, s)
            }
        }

        SerializeWith {
            value: #value,
            phantom: ::std::marker::PhantomData::<#item_ty>,
        }
    };

    let expected = concat!(
        "struct SerializeWith < 'a , T >  where T : Serialize  { ",
            "value : & 'a String  , ",
            "phantom : :: std :: marker :: PhantomData < Cow < 'a , str >  > , ",
        "} ",
        "impl < 'a , T >  :: serde :: Serialize for SerializeWith < 'a , T >  where T : Serialize  { ",
            "fn serialize < S > ( & self , s : & mut S ) -> Result < ( ) , S :: Error > ",
                "where S : :: serde :: Serializer ",
            "{ ",
                "SomeTrait :: serialize_with  ( self . value , s ) ",
            "} ",
        "} ",
        "SerializeWith { ",
            "value : self . x  , ",
            "phantom : :: std :: marker :: PhantomData :: < Cow < 'a , str >  > , ",
        "} "
    );

    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_integer() {
    let ii8 = -1i8;
    let ii16 = -1i16;
    let ii32 = -1i32;
    let ii64 = -1i64;
    let iisize = -1isize;
    let uu8 = 1u8;
    let uu16 = 1u16;
    let uu32 = 1u32;
    let uu64 = 1u64;
    let uusize = 1usize;

    let tokens = quote! {
        #ii8 #ii16 #ii32 #ii64 #iisize
        #uu8 #uu16 #uu32 #uu64 #uusize
    };
    let expected = "-1i8 -1i16 -1i32 -1i64 -1isize 1u8 1u16 1u32 1u64 1usize ";
    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_floating() {
    let e32 = 2.71828f32;
    let nan32 = f32::NAN;
    let inf32 = f32::INFINITY;
    let neginf32 = f32::NEG_INFINITY;

    let e64 = 2.71828f64;
    let nan64 = f64::NAN;
    let inf64 = f64::INFINITY;
    let neginf64 = f64::NEG_INFINITY;

    let tokens = quote! {
        #e32 @ #nan32 @ #inf32 @ #neginf32
        #e64 @ #nan64 @ #inf64 @ #neginf64
    };
    let expected = concat!(
        "2.71828f32 @ :: std :: f32 :: NAN @ :: std :: f32 :: INFINITY @ :: std :: f32 :: NEG_INFINITY ",
        "2.71828f64 @ :: std :: f64 :: NAN @ :: std :: f64 :: INFINITY @ :: std :: f64 :: NEG_INFINITY ",
    );
    assert_eq!(expected, tokens.to_string());
}
