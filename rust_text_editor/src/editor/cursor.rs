use super::Position;
use std::time::Duration;
use yew::prelude::*;
use yew::services::interval::{IntervalService, IntervalTask};
pub struct Cursor {
    link: ComponentLink<Self>,
    _interval_task: IntervalTask,
    position: Position,
    blinking: bool,
    visible: bool,
}

#[derive(Debug)]
pub enum CursorMsg {
    Blink,
}

#[derive(Properties, Clone)]
pub struct CursorProperties {
    pub visible: bool,
    pub blinking: bool,
    pub position: Position,
}

impl Component for Cursor {
    type Message = CursorMsg;
    type Properties = CursorProperties;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut interval = IntervalService::new();
        let _interval_task = interval.spawn(
            Duration::from_millis(600),
            link.callback(|_| CursorMsg::Blink),
        );
        Cursor {
            link,
            _interval_task,
            position: props.position,
            blinking: props.blinking,
            visible: props.visible,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::trace!("msg: {:?}", msg);
        if self.blinking {
            self.visible = !self.visible
        } else {
            self.visible = true
        }
        true
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.position = props.position;
        self.blinking = props.blinking;
        self.visible = props.visible;
        // TODO handle all those different events
        true
    }
    fn view(&self) -> Html {
        let mut class_names = String::from("absolute bg-gray-900 w-1 h-4");
        let visibility_class = if self.visible {
            " visible"
        } else {
            " invisible"
        };
        class_names.push_str(visibility_class);
        html! {
            <div class=class_names/>
        }
    }
}
