use std::fs;

use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;

#[test]
fn end_to_end_flow() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;

    Command::cargo_bin("kb-claude")?
        .args([
            "init",
            "--directory",
            temp.path().to_str().expect("utf8 path"),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialized .claude hierarchy"));

    assert_layout(&temp);

    let new_input = "tag-one,tag-two\nrel-one\nAlpha body\n\n";
    Command::cargo_bin("kb-claude")?
        .current_dir(temp.path())
        .args(["new", "Alpha Node", "-t", "metadata"])
        .write_stdin(new_input)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Created ./.claude/metadata/alpha-node.md",
        ));

    let beta_input = "\n\nBeta body\n\n";
    Command::cargo_bin("kb-claude")?
        .current_dir(temp.path())
        .args(["new", "Beta Node", "-t", "metadata"])
        .write_stdin(beta_input)
        .assert()
        .success();

    let alpha_path = temp.child(".claude/metadata/alpha-node.md");
    alpha_path.assert(predicate::path::exists());
    alpha_path.assert(predicate::str::contains("Alpha body"));

    Command::cargo_bin("kb-claude")?
        .current_dir(temp.path())
        .args(["link", "alpha-node", "beta-node"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Linked"));

    let alpha_content = fs::read_to_string(alpha_path.path())?;
    assert!(alpha_content.contains("relates_to: beta-node"));

    temp.child(".claude/other").create_dir_all()?;
    temp.child(".claude/other/ignore-me.md")
        .write_str("this file is intentionally invalid")?;

    Command::cargo_bin("kb-claude")?
        .current_dir(temp.path())
        .arg("manifest")
        .assert()
        .success();
    temp.child(".claude/manifest.md")
        .assert(predicate::str::contains("| Title | Type | Path |"));

    Command::cargo_bin("kb-claude")?
        .current_dir(temp.path())
        .arg("validate")
        .assert()
        .success();

    Command::cargo_bin("kb-claude")?
        .current_dir(temp.path())
        .args(["search", "alpha"])
        .assert()
        .success()
        .stdout(predicate::str::contains("./.claude/metadata/alpha-node.md"));

    Command::cargo_bin("kb-claude")?
        .current_dir(temp.path())
        .args(["search", "alpha", "--tag", "missing"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No matching entries found."));

    Ok(())
}

fn assert_layout(temp: &TempDir) {
    let expected = [
        ".claude",
        ".claude/metadata",
        ".claude/debug_history",
        ".claude/qa",
        ".claude/code_index",
        ".claude/patterns",
        ".claude/plans",
        ".claude/cheatsheets",
        ".claude/memory_anchors",
    ];

    for entry in expected {
        temp.child(entry).assert(predicate::path::exists());
    }
}
