use std::process::Command;
use cosmic::{Task, app::Core};
use tracing::info;

#[derive(Clone, Debug)]
pub enum AppletMessage {
    ToggleKeyboard,
}

pub struct Applet {
    core: Core,
    is_toggled: bool,
}

impl cosmic::Application for Applet {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = AppletMessage;

    const APP_ID: &str = "com.github.bukutsu.cosmic-applet-toggle-keyboard";

    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::app::Core,
        _flags: Self::Flags,
    ) -> (Self, cosmic::app::Task<Self::Message>) {
        (
            Applet {
                core,
                is_toggled: false,
            },
            Task::none(),
        )
    }

    fn view(&self) -> cosmic::Element<Self::Message> {
        self.core
            .applet
            .icon_button("input-keyboard-symbolic")
            .on_press(AppletMessage::ToggleKeyboard)
            .into()
    }

    fn update(&mut self, message: Self::Message) -> cosmic::app::Task<Self::Message> {
        info!("Received {:?}", message);
        match message {
            AppletMessage::ToggleKeyboard => {
                let _ = Command::new("/home/bukutsu/.scripts/toggle-keyboard.sh")
                    .spawn();
                self.is_toggled = !self.is_toggled;
            }
        }

        Task::none()
    }
}
