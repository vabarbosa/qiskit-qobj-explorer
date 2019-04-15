use yew::prelude::*;
use yew::html;
use yew::virtual_dom::VNode;
use stdweb::web::Node;
use stdweb::unstable::TryFrom;
use crate::qobj::{Qobj, QobjHeader, QobjConfig};

pub enum Msg {
    DoIt,
}

impl Component for Qobj {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Qobj {
            qobj_id: "ThisIsMyId".to_string(),
            config: QobjConfig::new(0, 0, 5, 5, 1024),
            experiments: Vec::new(),
            header: QobjHeader::new("qasm_simulator".to_string(), "0.2.0".to_string()),
            r#type: "type".to_string(),
            schema_version: "latest".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Qobj> for Qobj {
    fn view(&self) -> Html<Self> {
        html!{
           <>
            <p></p>
           </>
        }
    }
}

pub fn show() {
    yew::initialize();
    App::<Qobj>::new().mount_to_body();
    yew::run_loop();
}