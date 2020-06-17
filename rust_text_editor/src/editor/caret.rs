use super::Position;
use std::time::Duration;
use yew::prelude::*;
use yew::services::interval::{IntervalService, IntervalTask};

pub struct Caret {
    node_ref: NodeRef,
    _interval_task: IntervalTask,
    position: Position,
    visible: bool,
}

#[derive(Debug)]
pub enum CaretMsg {
    Blink,
}

#[derive(Properties, Clone)]
pub struct CaretProperties {
    pub position: Position,
}

impl Component for Caret {
    type Message = CaretMsg;
    type Properties = CaretProperties;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut interval = IntervalService::new();
        let _interval_task = interval.spawn(
            Duration::from_millis(600),
            link.callback(|_| CaretMsg::Blink),
        );
        Caret {
            node_ref: NodeRef::default(),
            _interval_task,
            position: props.position,
            visible: false,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::trace!("msg: {:?}", msg);
        // if self.blinking {
        self.visible = !self.visible;
        // } else {
        // self.visible = true
        // }
        true
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.position = props.position;
        true
    }
    fn view(&self) -> Html {
        let mut class_names = String::from("relative bg-gray-900 w-px-2 h-4 leading-tight");

        let visibility_class = if self.visible {
            " visible"
        } else {
            " invisible"
        };
        class_names.push_str(visibility_class);
        html! {
            <div ref=self.node_ref.clone() class=class_names style={format!("left:{}px;top:{}px", self.position.column, self.position.line)}/>
        }
    }
}
