#[macro_use]
extern crate yew;
extern crate yew_svg;

use yew::prelude::*;
use yew_svg::*;

fn main() {
    yew::initialize();
    App::<u32, Model>::new(0).mount_to_body();
    yew::run_loop();
}

struct Model;

impl<CTX: 'static> Component<CTX> for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model
    }

    fn update(&mut self, _msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        false
    }
}

impl<CTX: 'static> Renderable<CTX, Model> for Model {
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div>
                <h1>{ "SVG by remote defs:" }</h1>
                <code>{ r#"<SVG: href="/svg-defs.svg#shape", height="50", width="50", />"# }</code>
                <div><SVG: href="/svg-defs.svg#shape", height="50", width="50", /></div>
                <h1>{ "SVG by svg file:" }</h1>
                <code>{ r#"<SVG: content=include_str!("../svgs/satellite.svg"), height="50", width="50",/>"# }</code>
                <div><SVG: content=include_str!("../svgs/satellite.svg"), height="50", width="50",/></div>
                <h1>{ "Attribute support:" }</h1>
                <code> { "width, height, class, style, view_box" } </code>
            </div>
        }
    }
}