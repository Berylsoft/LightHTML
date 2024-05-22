#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Node<S> {
    Text(S),
    Element(
        ElementTag,
        #[cfg_attr(feature = "serde", serde(with = "tuple_vec_map"))]
        Vec<(AttrKey, S)>,
        Vec<Node<S>>,
    )
}

fn cvt(_variant: &'static str) -> &'static str {
    match _variant {
        "_use" => "use",
        _ => unreachable!(),
    }
}

// #[macro_export]
macro_rules! enum_str_impl {
    ($name:ident {$($variant:ident)* $(! $_variant:ident)*}) => {
        #[derive(Clone, Copy, Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[allow(non_camel_case_types)]
        pub enum $name {
            $($variant,)*
            $($_variant,)*
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => stringify!($variant),)*
                    $(Self::$_variant => cvt(stringify!($_variant)),)*
                }
            }
        }
    };
}

enum_str_impl! {
    ElementTag {
        a
        br
        hr
        div
        svg
        img
        template
        span
        h1
        h2
        h3
        h4
        h5
        i
        b
        p
        ! _use
    }
}

enum_str_impl! {
    AttrKey {
        href
        src
        class
        alt
        target
        onclick
        id
        style
    }
}

pub mod prelude {
    pub use crate::Node::*;
    pub use crate::ElementTag::*;
    pub use crate::AttrKey::*;
}

use prelude::*;

impl ElementTag {
    pub fn has_content(&self) -> bool {
        match self {
            br | hr => false,
            _ => true
        }
    }
}

impl<S: AsRef<str>> Node<S> {
    pub fn render(&self, s: &mut String) {
        macro_rules! c {
            ($ch:expr) => {
                s.push($ch);
            };
        }

        macro_rules! s {
            ($string:expr) => {
                s.push_str($string);
            };
        }

        match self {
            Text(text) => {
                s!(text.as_ref().replace("\n", "").as_ref());
            },
            Element(tag, attrs, childs) => {
                c!('<');
                s!(tag.as_str());
                if tag.has_content() {
                    for (k, v) in attrs {
                        c!(' ');
                        s!(k.as_str());
                        if v.as_ref().len() != 0 {
                            c!('=');
                            c!('"');
                            s!(v.as_ref());
                            c!('"');
                        }
                    }
                    c!('>');
                    for child in childs {
                        child.render(s);
                    }
                    c!('<');
                    c!('/');
                    s!(tag.as_str());
                } else {
                    assert_eq!(attrs.len(), 0);
                    assert_eq!(childs.len(), 0);
                }
                c!('>');
            }
        }
    }
}


pub fn render_node<S: AsRef<str>>(node: Node<S>) -> String {
    let mut s = String::new();
    node.render(&mut s);
    s
}

pub fn render_nodes<I: IntoIterator<Item = Node<S>>, S: AsRef<str>>(nodes: I) -> String {
    let mut s = String::new();
    for node in nodes {
        node.render(&mut s);
    }
    s
}

#[macro_export]
macro_rules! attr {
    ($($k:ident: $v:expr),* $(,)?) => {
        vec![$((lighthtml::AttrKey::$k, $v),)*]
    };
}
