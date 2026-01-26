mod applet;
use applet::Applet;
use cosmic::iced::Result;

fn main() -> Result {
    tracing_subscriber::fmt::init();
    cosmic::applet::run::<Applet>(())?;

    Ok(())
}
