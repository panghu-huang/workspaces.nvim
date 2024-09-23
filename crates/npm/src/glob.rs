use glob::glob;
use std::path::{Path, PathBuf};

pub fn find_package_json_paths(dir: &Path, pattern: &str) -> anyhow::Result<Vec<PathBuf>> {
  let pattern = dir.join(pattern).join("package.json");
  let pattern = pattern
    .to_str()
    .ok_or_else(|| anyhow::anyhow!("Invalid pattern: {:?}", pattern))?;

  Ok(glob(pattern).unwrap().filter_map(Result::ok).collect())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_find_package_json_paths() {
    let dir = Path::new("tests/fixtures/pnpm");
    let pattern = "packages/*";
    let paths = find_package_json_paths(dir, pattern).unwrap();

    assert_eq!(paths.len(), 2);
    assert!(paths.iter().all(|path| path.ends_with("package.json")));
  }
}
