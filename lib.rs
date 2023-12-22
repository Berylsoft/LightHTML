#[derive(Clone, Debug)]
pub enum Node<S> {
    Text(S),
    Element(ElementTag, Vec<(AttrKey, S)>, Vec<Node<S>>)
}

// #[macro_export]
macro_rules! enum_str_impl {
    ($name:ident {$($variant:ident)*} {$($r_use:ident)*}) => {
        #[derive(Clone, Copy, Debug)]
        #[allow(non_camel_case_types)]
        pub enum $name {
            $($variant,)*
            $($r_use,)*
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $($variant => stringify!($variant),)*
                    $($r_use => "use",)*
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
    } {
        r#use
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
    } {

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
