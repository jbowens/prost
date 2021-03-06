use syn::{
    Ident,
    Lit,
    MetaItem,
    NestedMetaItem,
};
use quote::Tokens;

use error::*;
use field::{
    tags_attr,
    set_option,
};

pub struct Field {
    pub ty: Ident,
    pub tags: Vec<u32>,
}

impl Field {
    pub fn new(attrs: &[MetaItem]) -> Result<Option<Field>> {
        let mut ty = None;
        let mut tags = None;
        let mut unknown_attrs = Vec::new();

        for attr in attrs {
            if attr.name() == "oneof" {
                let t = match *attr {
                    MetaItem::NameValue(_, Lit::Str(ref ident, _)) => {
                        Ident::new(ident.as_ref())
                    },
                    MetaItem::List(_, ref items) if items.len() == 1 => {
                        // TODO(rustlang/rust#23121): slice pattern matching would make this much nicer.
                        if let NestedMetaItem::MetaItem(MetaItem::Word(ref ident)) = items[0] {
                            ident.clone()
                        } else {
                            bail!("invalid oneof attribute: item must be an identifier");
                        }
                    },
                    _ => bail!("invalid oneof attribute: {:?}", attr),
                };
                set_option(&mut ty, t, "duplicate oneof attribute")?;
            } else if let Some(t) = tags_attr(attr)? {
                set_option(&mut tags, t, "duplicate tags attributes")?;
            } else {
                unknown_attrs.push(attr);
            }
        }

        let ty = match ty {
            Some(ty) => ty,
            None => return Ok(None),
        };

        match unknown_attrs.len() {
            0 => (),
            1 => bail!("unknown attribute for message field: {:?}", unknown_attrs[0]),
            _ => bail!("unknown attributes for message field: {:?}", unknown_attrs),
        }

        let tags = match tags {
            Some(tags) => tags,
            None => bail!("oneof field is missing a tags attribute"),
        };

        Ok(Some(Field {
            ty: ty,
            tags: tags,
        }))
    }

    /// Returns a statement which encodes the oneof field.
    pub fn encode(&self, ident: &Ident) -> Tokens {
        quote! {
            if let Some(ref oneof) = #ident {
                oneof.encode(buf)
            }
        }
    }

    /// Returns an expression which evaluates to the result of decoding the oneof field.
    pub fn merge(&self, ident: &Ident) -> Tokens {
        let ty = &self.ty;
        quote! {
            #ty::merge(&mut #ident, tag, wire_type, buf)
        }
    }

    /// Returns an expression which evaluates to the encoded length of the oneof field.
    pub fn encoded_len(&self, ident: &Ident) -> Tokens {
        let ty = &self.ty;
        quote! {
            #ident.as_ref().map_or(0, #ty::encoded_len)
        }
    }

    pub fn clear(&self, ident: &Ident) -> Tokens {
        quote!(#ident = ::std::option::Option::None)
    }
}
