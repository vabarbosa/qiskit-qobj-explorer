use yew::prelude::*;
use yew::html;
use crate::qobj::Qobj;

struct Model {
    value: i64,
}

enum Msg {
    DoIt,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => self.value = self.value + 1
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
          <script>
            (function () {
                var myjson, inter;

                myjson = document.querySelector("#myjson");
                inter = setInterval(function () {
                    // Wait for dataChanged observer; Polymer and IE10 have this weird thing.
                    if (myjson.dataChanged) {
                        myjson.data = {
                            "Hola": "Cara de bola"
                        };
                        clearInterval(inter);
                    }
                }, 1);
            }());
          </script>
        }
    }
}

pub fn show(qobj: Qobj) {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}