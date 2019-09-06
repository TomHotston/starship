use ansi_term::Color;
use std::process::Command;
use std::str;

use super::{Context, Module};

/// A module which shows the latest (or pinned) version of the dotnet SDK
///
/// Will display if any of the following file extensions are present in
/// the current directory: .sln, .csproj, .fsproj, .xproj
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const DOTNET_SYMBOL: &str = "•NET ";

    let mut module = context.new_module("dotnet");

    let is_dotnet = context
        .try_begin_scan()?
        .set_extensions(&["sln", "csproj", "fsproj", "xproj"])
        .is_match();

    if !is_dotnet {
        return None;
    }
    let version = get_dotnet_version()?;
    module.set_style(Color::Blue.bold());
    module.new_segment("symbol", DOTNET_SYMBOL);
    module.new_segment("version", &version);

    Some(module)
}

fn get_dotnet_version() -> Option<String> {
    let version_output = Command::new("dotnet").arg("--version").output().ok()?;
    let version = str::from_utf8(version_output.stdout.as_slice())
        .ok()?
        .trim();

    let mut buffer = String::with_capacity(version.len() + 1);
    buffer.push('v');
    buffer.push_str(version);

    Some(buffer)
}
