use anyhow::{Context, Result};
use include_dir::{include_dir, Dir};
use std::path::Path;

static CARGO_MANIFEST_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR");

/// Unpack all contents within crate assets dir
pub fn unpack_assets(path: impl AsRef<Path>) -> Result<(), anyhow::Error> {
    let path_ref = path.as_ref();

    fs_extra::dir::create_all(path_ref, true)?;

    CARGO_MANIFEST_DIR
        .extract(path_ref)
        .with_context(|| format!("Could not unpack into '{path_ref:?}'"))?;

    Ok(())
}

