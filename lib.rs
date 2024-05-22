#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Node<S> {
    Text(S),
    // cannot tell the difference is intended at least for now
    Html(S),
    Element(
        ElementTag,
        #[cfg_attr(feature = "serde", serde(with = "tuple_vec_map"))]
        Vec<(AttrKey, S)>,
        Vec<Node<S>>,
    )
}

// #[macro_export]
macro_rules! enum_str_impl {
    ($name:ident {$($variant:ident $value:tt)*}) => {
        #[derive(Clone, Copy, Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[allow(non_camel_case_types)]
        pub enum $name {
            $($variant,)*
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => stringify!($value),)*
                }
            }
        }
    };
}

enum_str_impl! {
    ElementTag {
        E_A         a
        E_BR        br
        E_HR        hr
        E_DIV       div
        E_SVG       svg
        E_IMG       img
        E_TEMPLATE  template
        E_SPAN      span
        E_H1        h1
        E_H2        h2
        E_H3        h3
        E_H4        h4
        E_H5        h5
        E_I         i
        E_B         b
        E_P         p
        E_USE       use
        E_HTML      html
        E_HEAD      head
        E_BODY      body
        E_SCRIPT    script
        E_STYLE     style
        E_LINK      link
    }
}

enum_str_impl! {
    AttrKey {
        A_HREF      href
        A_SRC       src
        A_CLASS     class
        A_ALT       alt
        A_TARGET    target
        A_ONCLICK   onclick
        A_ID        id
        A_STYLE     style
        A_REL       rel
        A_CROSSORIGIN   crossorigin
        A_INTEGRITY     integrity
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
            E_BR | E_HR => false,
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
            Text(text) | Html(text) => {
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
