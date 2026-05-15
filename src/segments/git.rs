use std::process::Command;

use crate::input::Input;

pub fn render(input: &Input) -> Option<String> {
    let cwd = input.cwd.as_deref()?;
    git_segment(cwd)
}

fn git_segment(cwd: &str) -> Option<String> {
    let branch_out = run_git(cwd, &["rev-parse", "--abbrev-ref", "HEAD"])?;
    let branch = branch_out.trim();
    if branch.is_empty() || branch == "HEAD" {
        return None;
    }

    let numstat = run_git(cwd, &["diff", "--numstat"]).unwrap_or_default();
    let (added, deleted) = numstat
        .lines()
        .filter_map(parse_numstat_line)
        .fold((0u64, 0u64), |(ta, td), (a, d)| (ta + a, td + d));

    let mut seg = format!("git@{branch}");
    if added + deleted > 0 {
        seg.push_str(&format!(" (+{added} -{deleted})"));
    }
    Some(seg)
}

fn parse_numstat_line(line: &str) -> Option<(u64, u64)> {
    let mut cols = line.splitn(3, '\t');
    let added: u64 = cols.next()?.parse().ok()?;
    let deleted: u64 = cols.next()?.parse().ok()?;
    Some((added, deleted))
}

fn run_git(cwd: &str, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(cwd)
        .args(args)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}
