#[macro_use]
extern crate yew;
extern crate yew_svg;

use yew::prelude::*;
use yew_svg::*;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{ "SVG by remote defs:" }</h1>
                <code>{ r#"<SVG: href="/svg-defs.svg#shape", height="50", width="50", />"# }</code>
                <SVG: href="/svg-defs.svg#shape", height="50", width="50", />
                <h1>{ "SVG by svg file:" }</h1>
                <code>{ r#"<SVG: content=include_str!("../svgs/satellite.svg"), height="50", width="50",/>"# }</code>
                <SVG: content=include_str!("../svgs/satellite.svg"), height="50", width="50",/>
                <h1>{ "Attribute support:" }</h1>
                <code> { "width, height, class, style, view_box" } </code>
            </div>
        }
    }
}