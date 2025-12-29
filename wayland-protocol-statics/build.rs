use std::env;

fn main() {
    let mut build = cc::Build::new();

    if env::var_os("CARGO_FEATURE_EXPORT_PRESENTATION_TIME").is_some() {
        build.file("src/presentation-time/export-presentation-time.c");
    } else if env::var_os("CARGO_FEATURE_PRIVATE_PRESENTATION_TIME").is_some() {
        build.file("src/presentation-time/private-presentation-time.c");
    }

    if env::var_os("CARGO_FEATURE_EXPORT_VIEWPORTER").is_some() {
        build.file("src/viewporter/export-viewporter.c");
    } else if env::var_os("CARGO_FEATURE_PRIVATE_VIEWPORTER").is_some() {
        build.file("src/viewporter/private-viewporter.c");
    }

    if env::var_os("CARGO_FEATURE_EXPORT_XDG_SHELL").is_some() {
        build.file("src/xdg-shell/export-xdg-shell.c");
    } else if env::var_os("CARGO_FEATURE_PRIVATE_XDG_SHELL").is_some() {
        build.file("src/xdg-shell/private-xdg-shell.c");
    }

    if build.get_files().next().is_some() {
        build.compile("wayland-protocol-statics");
    }
}
