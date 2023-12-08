mod prelude;
use prelude::*;

fn main() {
    dioxus_desktop::launch_cfg(
        app::app,
        config::config(),
    );

}