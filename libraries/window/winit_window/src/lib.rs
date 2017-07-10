extern crate winit;
extern crate vulkano;
extern crate vulkano_win;
extern crate input;
extern crate window;

use std::time::{Duration};
use std::sync::{Arc};
use std::collections::{VecDeque};

use vulkano::swapchain::{Surface};
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano_win::{VkSurfaceBuild, Window as VulkanoWinWindow};
use winit::{EventsLoop, WindowBuilder, Event as WinitEvent, WindowEvent, ElementState, MouseButton as WinitMouseButton};
use input::{Input, EventId, CloseArgs, Motion, Button, MouseButton};
use window::{Window, Size};

pub fn required_extensions() -> InstanceExtensions {
    vulkano_win::required_extensions()
}

pub struct WinitWindow {
    pub window: VulkanoWinWindow,
    pub events_loop: EventsLoop,
    pub surface: Arc<Surface>,

    size: Size,
    should_close: bool,
    queued_events: VecDeque<WinitEvent>,
}

impl WinitWindow {
    pub fn new_vulkano(instance: Arc<Instance>, title: &str, size: Size) -> Self {
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_dimensions(size.width, size.height)
            .with_title(title)
            .build_vk_surface(&events_loop, instance)
            .unwrap();

        let surface = window.surface().clone();

        WinitWindow {
            window,
            events_loop,
            surface,

            size,
            should_close: false,
            queued_events: VecDeque::new(),
        }
    }
}

impl Window for WinitWindow {
    fn set_should_close(&mut self, value: bool) {
        self.should_close = value;
    }

    fn should_close(&self) -> bool {
        self.should_close
    }

    fn size(&self) -> Size {
        // TODO: Report outer size rather than inner size
        self.size
    }

    fn swap_buffers(&mut self) {
        // TODO: Unclear what to do here, we don't have buffers to swap
        unimplemented!()
    }

    fn wait_event(&mut self) -> Input {
        // TODO: Implement this
        unimplemented!()
    }

    fn wait_event_timeout(&mut self, _timeout: Duration) -> Option<Input> {
        // TODO: Implement this
        unimplemented!()
    }

    fn poll_event(&mut self) -> Option<Input> {
        // Add all events we got to the event queue, since winit only allows us to get all pending
        //  events at once.
        {
            let queued_events = &mut self.queued_events;
            self.events_loop.poll_events(|event| {
                queued_events.push_back(event);
            });
        }

        // Get the first event in the queue, and then map it to a pistoncore-input event
        let event = self.queued_events.pop_front().map(map_event);

        // Check if we got a close event, if we did we need to mark ourselves as should-close
        if let &Some(Input::Close(_)) = &event {
            self.set_should_close(true);
        }

        event
    }

    fn draw_size(&self) -> Size {
        self.size
    }
}

fn map_event(event: WinitEvent) -> Input {
    let unsupported_input = Input::Custom(EventId("Unsupported Winit Event"), Arc::new(0));

    match event {
        WinitEvent::WindowEvent { event: ev, .. } => {
            match ev {
                WindowEvent::Closed => Input::Close(CloseArgs),
                WindowEvent::MouseMoved { device_id: _, position } =>
                    Input::Move(Motion::MouseCursor(position.0, position.1)),
                WindowEvent::MouseInput { device_id: _, state, button } => {
                    let button = map_mouse_button(button);
                    if state == ElementState::Pressed {
                        Input::Press(Button::Mouse(button))
                    } else {
                        Input::Release(Button::Mouse(button))
                    }
                },
                _ => unsupported_input,
            }
        },
        _ => unsupported_input,
    }
}

fn map_mouse_button(button: WinitMouseButton) -> MouseButton {
    match button {
        WinitMouseButton::Left => MouseButton::Left,
        WinitMouseButton::Right => MouseButton::Right,
        WinitMouseButton::Middle => MouseButton::Middle,
        WinitMouseButton::Other(4) => MouseButton::X1,
        WinitMouseButton::Other(5) => MouseButton::X2,
        WinitMouseButton::Other(6) => MouseButton::Button6,
        WinitMouseButton::Other(7) => MouseButton::Button7,
        WinitMouseButton::Other(8) => MouseButton::Button8,
        _ => MouseButton::Unknown,
    }
}
