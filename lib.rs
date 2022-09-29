pub enum Node<'a> {
    Text(&'a str),
    Element(ElementTag, Vec<(AttrKey, &'a str)>, Vec<Node<'a>>)
}

macro_rules! enum_str_impl {
    ($name:ident {$($variant:ident),*,}) => {
        #[allow(non_camel_case_types)]
        pub enum $name {
            $($variant,)*
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $($variant => stringify!($variant),)*
                }
            }
        }
    };
}

enum_str_impl! {
    ElementTag {
        a,
        br,
        hr,
    }
}

enum_str_impl! {
    AttrKey {
        href,
        src,
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

impl<'a> Node<'a> {
    pub fn render(&'a self, s: &mut String) {
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
                // s!(text.replace("\n", "").as_ref());
                s!(text);
            },
            Element(tag, attrs, childs) => {
                c!('<');
                s!(tag.as_str());
                if tag.has_content() {
                    for (k, v) in attrs {
                        c!(' ');
                        s!(k.as_str());
                        if v.len() != 0 {
                            c!('=');
                            c!('"');
                            s!(v);
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


pub fn render_node<'a>(node: Node<'a>) -> String {
    let mut s = String::new();
    node.render(&mut s);
    s
}

pub fn render_nodes<'a>(nodes: Vec<Node<'a>>) -> String {
    let mut s = String::new();
    for node in nodes {
        node.render(&mut s);
    }
    s
}
