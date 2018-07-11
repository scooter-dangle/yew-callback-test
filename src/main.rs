#![deny(warnings)]

#[macro_use]
extern crate yew;
use yew::prelude::*;

#[macro_use]
extern crate stdweb;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

struct Model {
    payload: String,
    // Pointless field just to have something that's been manipulated
    debugged_payload: String,
}

enum Msg {
    Payload(String),
    // Not sure whether this is actually necessary. I assume I need it to
    // express the return value of a button click that triggers an asynchronous
    // call.
    Deferred,
}

#[derive(Default, PartialEq, Eq, Clone)]
struct Props {
    payload: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(Props { payload }: Self::Properties, _: ComponentLink<Self>) -> Self {
        let debugged_payload = format!("{:?}", payload);
        Self { payload, debugged_payload }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            Payload(payload) => self.change(Self::Properties { payload }),
            Deferred => false,
        }
    }

    fn change(&mut self, Self::Properties { payload }: Self::Properties) -> ShouldRender {
        if payload == self.payload {
            false
        } else {
            self.debugged_payload = format!("{:?}", payload);
            self.payload = payload;
            true
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <textarea
                    oninput=|InputData { value }| Msg::Payload(value),
                    style="font-family: 'Monaco', monospace;",
                    value={ &self.payload },
                ></textarea>
                <button onclick=|_| Msg::Payload(get_payload()), >{
                    "Get the payload!"
                }</button>
                <button onclick=|_| {
                    get_payload_later();
                    Msg::Deferred
                }, >{
                    "Get the payload later!"
                }</button>
                <p style="font-family: 'Monaco', monospace;", >{
                    nbsp(self.debugged_payload.as_ref())
                }</p>
            </div>
        }
    }
}

fn nbsp<T>(string: T) -> String
where
    String: From<T>,
{
    String::from(string).replace(' ', "\u{00a0}")
}

fn get_payload() -> String {
    let payload = js! {
        return window.get_payload()
    };
    payload.into_string().unwrap()
}

fn get_payload_later() {
    // TODO
    ()

    // // Something like the following
    // js! {
    //     return window.get_payload_later(@{something})
    // };
}
