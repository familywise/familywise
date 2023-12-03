use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct HomeTileProps<'a> {
    #[props(default = "/")]
    href: &'a str,
    #[props(default = "FamilyWise")]
    title: &'a str,
}

pub fn HomeTile<'a>(cx: Scope<'a, HomeTileProps<'a>>) -> Element {
    cx.render(rsx!(
    a {
        class: "bg-slate-300 p-2 m-3 rounded-full border-2 border-slate-100 shadow text-blue-800",
        href: cx.props.href,
        cx.props.title
    }
                  ))
}
