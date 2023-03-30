use tempfile::{NamedTempFile, TempDir};

use esp::Plugin;

fn create_temp_file() -> (TempDir, NamedTempFile) {
    let dir = TempDir::new().unwrap();
    let file = NamedTempFile::new_in(&dir).unwrap();
    (dir, file)
}

#[test]
fn load_save() -> std::io::Result<()> {
    let (_dir, file) = create_temp_file();

    let src_path = "tests/assets/all_types.esp";
    let dst_path = file.into_temp_path();

    let mut plugin = Plugin::new();
    plugin.load_path(src_path)?;
    plugin.save_path(&dst_path)?;

    let src_bytes = std::fs::read(src_path)?;
    let dst_bytes = std::fs::read(&dst_path)?;
    assert_eq!(src_bytes, dst_bytes);

    Ok(())
}
