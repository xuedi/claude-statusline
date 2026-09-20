use std::path::Path;

use crate::input::Input;

/// Directory names that describe a role inside a project rather than the project
/// itself. Walking up past these yields the name a human would actually say.
/// Kept lowercase and sorted; lookup is case-insensitive.
const GENERIC_DIRS: &[&str] = &[
    ".agents",
    ".claude",
    ".git",
    ".github",
    "admin",
    "api",
    "app",
    "application",
    "apps",
    "assets",
    "back-end",
    "backend",
    "bin",
    "build",
    "cache",
    "cli",
    "client",
    "clients",
    "cmd",
    "code",
    "common",
    "conf",
    "config",
    "configs",
    "core",
    "daemon",
    "data",
    "database",
    "db",
    "demo",
    "deploy",
    "deployment",
    "desktop",
    "dev",
    "dist",
    "doc",
    "docker",
    "docs",
    "documentation",
    "dotfiles",
    "e2e",
    "engine",
    "env",
    "etc",
    "example",
    "examples",
    "export",
    "files",
    "front-end",
    "frontend",
    "gradle",
    "gui",
    "hooks",
    "import",
    "infra",
    "infrastructure",
    "internal",
    "lib",
    "libs",
    "locales",
    "main",
    "migrations",
    "mobile",
    "modules",
    "node_modules",
    "out",
    "output",
    "packages",
    "packaging",
    "pkg",
    "plugins",
    "project",
    "protocol",
    "public",
    "release",
    "repo",
    "res",
    "resources",
    "script",
    "scripts",
    "server",
    "service",
    "services",
    "settings",
    "shared",
    "site",
    "source",
    "spec",
    "specs",
    "src",
    "static",
    "systemd",
    "target",
    "temp",
    "templates",
    "test",
    "tests",
    "tmp",
    "tool",
    "tooling",
    "tools",
    "translations",
    "ui",
    "util",
    "utils",
    "var",
    "vendor",
    "venv",
    "web",
    "webapp",
    "website",
    "worker",
    "workers",
    "workspace",
    "www",
];

pub fn render(input: &Input) -> Option<String> {
    let dir = input
        .workspace
        .as_ref()
        .and_then(|w| w.project_dir.as_deref().or(w.current_dir.as_deref()))
        .or(input.cwd.as_deref())?;
    project_name(dir)
}

fn project_name(dir: &str) -> Option<String> {
    let mut path = Path::new(dir.trim_end_matches('/'));
    loop {
        let name = path.file_name()?.to_str()?;
        if !is_generic(name) {
            return Some(name.to_string());
        }
        path = path.parent()?;
    }
}

fn is_generic(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    GENERIC_DIRS.binary_search(&lower.as_str()).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_dirs_is_sorted_and_unique() {
        let mut sorted = GENERIC_DIRS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted, GENERIC_DIRS, "GENERIC_DIRS must be sorted + unique");
    }

    #[test]
    fn project_name_matrix() {
        let cases: &[(&str, Option<&str>)] = &[
            // --- plain project roots ---
            (
                "/home/xuedi/Projects/claude-statusline",
                Some("claude-statusline"),
            ),
            (
                "/home/xuedi/Projects/2_Websites/MeetAgain",
                Some("MeetAgain"),
            ),
            (
                "/home/xuedi/Projects/2_Websites/SimpleSites/starraid.de",
                Some("starraid.de"),
            ),
            // --- one generic level ---
            ("/home/xuedi/Projects/FileFin/app", Some("FileFin")),
            ("/home/xuedi/Projects/Starraid/server", Some("Starraid")),
            // --- several generic levels stacked ---
            ("/home/xuedi/Projects/FileFin/app/.claude", Some("FileFin")),
            ("/home/xuedi/Projects/armdash/internal/web", Some("armdash")),
            // --- case-insensitive matching ---
            ("/home/xuedi/Projects/vellum/SRC", Some("vellum")),
            // --- trailing slash ---
            ("/home/xuedi/Projects/FileFin/app/", Some("FileFin")),
            // --- nothing but generic components ---
            ("/src/app", None),
            ("/", None),
            ("", None),
        ];
        for &(dir, expected) in cases {
            assert_eq!(
                project_name(dir).as_deref(),
                expected,
                "project_name({dir:?})"
            );
        }
    }
}
