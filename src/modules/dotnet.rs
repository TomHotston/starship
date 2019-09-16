use ansi_term::Color;
use std::path::Path;
use std::process::Command;
use std::str;

use std::ffi::OsStr;

use super::{Context, Module};

/// A module which shows the latest (or pinned) version of the dotnet SDK
///
/// Will display if any of the following file extensions are present in
/// the current directory: .sln, .csproj, .fsproj, .xproj
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const DOTNET_SYMBOL: &str = "•NET ";

    let mut module = context.new_module("dotnet");

    let dotnet_files = get_local_files_filtered(context).ok()?;

    if dotnet_files.len() == 0 {
        return None;
    }
    let version = get_dotnet_version()?;
    module.set_style(Color::Blue.bold());
    module.new_segment("symbol", DOTNET_SYMBOL);
    module.new_segment("version", &version);

    Some(module)
}

struct DotNetFile<'a> {
    path: &'a Path,
    file_type: FileType,
}

enum FileType {
    ProjectJson,
    ProjectFile,
    GlobalJson,
    SolutionFile,
}

fn get_local_files_filtered<'a>(
    context: &'a Context,
) -> Result<Vec<DotNetFile<'a>>, std::io::Error> {
    Ok(context
        .get_dir_files()?
        .iter()
        .filter_map(|p| {
            is_dotnet_relevant_file(p).map(|t| DotNetFile {
                path: p.as_ref(),
                file_type: t,
            })
        })
        .collect())
}

fn is_dotnet_relevant_file(path: &Path) -> Option<FileType> {
    let file_name_lower = map_to_lower(path.file_name());

    match file_name_lower.as_ref().map(|f| f.as_ref()) {
        Some("global.json") => return Some(FileType::GlobalJson),
        Some("project.json") => return Some(FileType::ProjectJson),
        _ => (),
    };

    let extension_lower = map_to_lower(path.extension());

    match file_name_lower.as_ref().map(|f| f.as_ref()) {
        Some("sln") => return Some(FileType::SolutionFile),
        Some("csproj") | Some("fsproj") | Some("xproj") => return Some(FileType::ProjectFile),
        _ => (),
    };

    None
}

fn map_to_lower(value: Option<&OsStr>) -> Option<String> {
    Some(value?.to_str()?.to_ascii_lowercase())
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
