use winit::{
    dpi::{PhysicalSize, PhysicalPosition},
    event::{
        Event, WindowEvent, DeviceEvent,
        MouseButton, MouseScrollDelta,
        KeyboardInput, VirtualKeyCode, ModifiersState,
        ElementState, ButtonId
    }
};

#[allow(unused)]
pub enum ManagedEvent {
    Button { button: ButtonId, state: ElementState },
    CloseRequested,
    CursorEntered,
    CursorLeft,
    CursorMoved(PhysicalPosition<f64>),
    Destroyed,
    KeyboardInput { state: ElementState, vk: VirtualKeyCode },
    LoopDestroyed,
    MainEventsCleared,
    ModifiersChanged(ModifiersState),
    MouseInput { state: ElementState, button: MouseButton },
    MouseMotion(f64, f64),
    MouseWheelLines(f32, f32),
    MouseWheelPixels(PhysicalPosition<f64>),
    Moved(PhysicalPosition<i32>),
    ReceivedCharacter(char),
    RedrawEventsCleared,
    RedrawRequested,
    Resized(PhysicalSize<u32>),
    Resumed,
    Suspended,

    /// Means that the event wasn't important (the control flow should continue)
    Ignored,
    /// Means that the event is not yet implemented (the handling of this event is left to the developper)
    NotImplemented
}

#[allow(unused)]
pub fn manage<T>(event: Event<T>) -> ManagedEvent {
    use Event as E;
    use WindowEvent as W;
    use DeviceEvent as D;
    use ManagedEvent as M;
    match event {
        E::Suspended             => M::Suspended,
        E::Resumed               => M::Resumed,
        E::MainEventsCleared     => M::MainEventsCleared,
        E::RedrawRequested(_)    => M::RedrawRequested,
        E::RedrawEventsCleared   => M::RedrawEventsCleared,
        E::LoopDestroyed         => M::LoopDestroyed,
        E::WindowEvent { event, .. } => match event {
            W::AxisMotion { .. }          => M::NotImplemented,
            W::CloseRequested             => M::CloseRequested,
            W::CursorEntered { .. }       => M::CursorEntered,
            W::CursorLeft { .. }          => M::CursorLeft,
            W::CursorMoved { position, .. } => M::CursorMoved(position),
            W::Destroyed                  => M::Destroyed,
            W::DroppedFile(_)             => M::NotImplemented,
            W::Focused(_)                 => M::NotImplemented,
            W::HoveredFile(_)             => M::NotImplemented,
            W::HoveredFileCancelled       => M::NotImplemented,
            W::Ime(_)                     => M::NotImplemented,
            W::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state,
                    virtual_keycode: Some(vk),
                    ..
                } => M::KeyboardInput{state, vk},
                _ => M::Ignored
            },
            W::ModifiersChanged(m) => M::ModifiersChanged(m),
            W::MouseInput { state, button, .. } => M::MouseInput{state, button},
            W::MouseWheel { .. }          => M::NotImplemented,
            W::Moved(pos) => M::Moved(pos),
            W::Occluded(_)                => M::NotImplemented,
            W::ReceivedCharacter(c) => M::ReceivedCharacter(c),
            W::Resized(size) => M::Resized(size),
            W::ScaleFactorChanged { .. }  => M::NotImplemented,
            W::ThemeChanged(_)            => M::NotImplemented,
            W::Touch(_)                   => M::NotImplemented,
            W::TouchpadPressure { .. }    => M::NotImplemented
        },
        E::DeviceEvent { event, .. } => match event {
            D::Added                => M::NotImplemented,
            D::Removed              => M::NotImplemented,
            D::MouseMotion { delta: (dx, dy) } => M::MouseMotion(dx, dy),
            D::MouseWheel { delta } => match delta {
                MouseScrollDelta::LineDelta(dx, dy) => M::MouseWheelLines(dx, dy),
                MouseScrollDelta::PixelDelta(dpos) => M::MouseWheelPixels(dpos)
            }
            D::Motion { .. }        => M::NotImplemented,
            D::Button { button, state } => M::Button{button, state},
            D::Key(_)               => M::NotImplemented,
            D::Text { .. }          => M::NotImplemented,
        },
        E::NewEvents(_) => M::NotImplemented,
        E::UserEvent(_) => M::NotImplemented,
    }
}
