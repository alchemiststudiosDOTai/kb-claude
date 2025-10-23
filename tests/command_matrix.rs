use std::fs;

use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;

#[test]
fn command_matrix_behaviour() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;

    // init → directories exist
    Command::cargo_bin("kb-claude")?
        .args([
            "init",
            "--directory",
            temp.path().to_str().expect("utf8 path"),
        ])
        .assert()
        .success();
    let node = temp.child(".claude/metadata");
    node.assert(predicate::path::exists());

    // new → markdown file with front matter
    let alpha_input = "tag-alpha\nrel-one\nAlpha body\n\n";
    Command::cargo_bin("kb-claude")?
        .current_dir(temp.path())
        .args(["new", "Alpha Summary", "-t", "metadata"])
        .write_stdin(alpha_input)
        .assert()
        .success();

    let beta_input = "\n\nBeta body\n\n";
    Command::cargo_bin("kb-claude")?
        .current_dir(temp.path())
        .args(["new", "Beta Summary", "-t", "metadata"])
        .write_stdin(beta_input)
        .assert()
        .success();

    let alpha_path = temp.child(".claude/metadata/alpha-summary.md");
    alpha_path.assert(predicate::path::exists());
    alpha_path.assert(predicate::str::contains("---"));

    // search → returns match
    Command::cargo_bin("kb-claude")?
        .current_dir(temp.path())
        .args(["search", "alpha"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha-summary.md"));

    // link → both files updated
    Command::cargo_bin("kb-claude")?
        .current_dir(temp.path())
        .args(["link", "alpha-summary", "beta-summary"])
        .assert()
        .success();

    let alpha_contents = fs::read_to_string(alpha_path.path())?;
    assert!(alpha_contents.contains("relates_to: beta-summary"));
    let beta_contents = fs::read_to_string(temp.child(".claude/metadata/beta-summary.md").path())?;
    assert!(beta_contents.contains("relates_to: alpha-summary"));

    // manifest → table regenerated
    Command::cargo_bin("kb-claude")?
        .current_dir(temp.path())
        .arg("manifest")
        .assert()
        .success();
    temp.child(".claude/manifest.md")
        .assert(predicate::str::contains("| Title | Type | Path |"));

    // validate → passes
    Command::cargo_bin("kb-claude")?
        .current_dir(temp.path())
        .arg("validate")
        .assert()
        .success()
        .stdout(predicate::str::contains("no issues found"));

    Ok(())
}
