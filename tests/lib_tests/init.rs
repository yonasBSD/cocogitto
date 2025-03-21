use std::path::Path;

use crate::helpers::*;

use anyhow::Result;
use sealed_test::prelude::*;
use speculoos::prelude::*;

#[sealed_test]
fn should_init_a_cog_repository() -> Result<()> {
    // Arrange
    // Act
    cocogitto::command::init::init(".")?;

    // Assert
    assert_that!(Path::new("cog.toml")).exists();
    assert_that!(git_log_head_message()?).is_equal_to("chore: initial commit".to_string());
    Ok(())
}

#[sealed_test]
fn should_skip_initialization_if_repository_exists() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("The first commit")?;

    // Act
    cocogitto::command::init::init(".")?;

    // Assert
    assert_that!(Path::new("cog.toml")).exists();
    assert_that!(git_log_head_message()?).is_equal_to("The first commit\n".to_string());
    let status = git_status()?;
    assert_that!(status).contains("On branch master");
    assert_that!(status).contains("Changes to be committed:");
    assert_that!(status).contains("new file:   cog.toml");
    Ok(())
}
