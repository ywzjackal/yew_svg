#![recursion_limit = "256"]
extern crate yew;
#[macro_use]
extern crate stdweb;

use stdweb::web::Node;
use yew::prelude::*;

/// SVG icon Properties
///
/// # example
/// ```rust
///
///
/// ```
#[derive(PartialEq, Clone)]
pub struct SvgProps {
    pub href: String,
    pub content: String,
    pub class: String,
    pub style: String,
    pub width: String,
    pub height: String,
    pub view_box: String,
}

impl Default for SvgProps {
    fn default() -> Self {
        SvgProps {
            href: Default::default(),
            content: Default::default(),
            class: Default::default(),
            style: Default::default(),
            width: Default::default(),
            height: Default::default(),
            view_box: Default::default(),
        }
    }
}

pub struct SVG {
    props: SvgProps,
}

impl<U> Renderable<U> for SVG
    where
        U: Component
{
    fn view(&self) -> Html<U> {
        use stdweb::unstable::TryFrom;
        let mut html = String::new();
        let props = &self.props;
        if !props.href.is_empty() {
            html += "<svg";
            if !props.class.is_empty() {
                html += &format!(" class={}", props.class);
            }
            if !props.style.is_empty() {
                html += &format!(" style={}", props.style);
            }
            if !props.view_box.is_empty() {
                html += &format!(" viewBox={}", props.view_box);
            }
            if !props.width.is_empty() {
                html += &format!(" width={}", props.width);
            }
            if !props.height.is_empty() {
                html += &format!(" height={}", props.height);
            }
            html += &format!(r#"><use xlink:href="{}"></use></svg>"#, props.href);
        } else if !props.content.is_empty() {
            html = props.content.clone();
        }
        let js_svg = js! {
            var div = document.createElement("div");
            div.innerHTML = @{html};
            var es = div.getElementsByTagName("svg");
            for (var i = 0; i < es.length; i++) {
                if @{!props.class.is_empty()} {
                    es[i].setAttribute("class", @{&props.class});
                }
                if @{!props.style.is_empty()} {
                    es[i].setAttribute("style", @{&props.style});
                }
                if @{!props.width.is_empty()} {
                    es[i].setAttribute("width", @{&props.width});
                }
                if @{!props.height.is_empty()} {
                    es[i].setAttribute("height", @{&props.height});
                }
                if @{!props.view_box.is_empty()} {
                    es[i].setAttribute("viewBox", @{&props.view_box});
                }
            }
            return div;
        };
        let node = Node::try_from(js_svg).expect("convert js_svg");
        let vnode = yew::virtual_dom::VNode::VRef(node);
        vnode
    }
}

pub enum Msg {
    SetHref(String),
    SetContent(String),
    SetWidth(String),
    SetHeight(String),
    SetClass(String),
    SetStyle(String),
    SetViewBox(String),
}

impl Component for SVG {
    type Message = Msg;
    type Properties = SvgProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        SVG {
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            SetHref(v) => {
                self.props.content = Default::default();
                self.props.href = v
            }
            SetContent(v) => {
                self.props.href = Default::default();
                self.props.content = v;
            }
            SetWidth(v) => self.props.width = v,
            SetHeight(v) => self.props.height = v,
            SetClass(v) => self.props.class = v,
            SetStyle(v) => self.props.style = v,
            SetViewBox(v) => self.props.view_box = v,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.props != props;
        self.props = props;
        changed
    }
}