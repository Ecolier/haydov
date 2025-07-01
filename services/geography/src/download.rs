use std::fs::File;
use std::io::copy;
use std::path::Path;
use reqwest::blocking::get;
use std::error::Error;
use std::fs;

/// Downloads the France OSM PBF map and saves it to the specified path.
/// Returns Ok(()) on success, or an error.
pub fn download_france_osm_pbf<P: AsRef<Path>>(output_path: P) -> Result<(), Box<dyn Error>> {
  let url = "https://download.geofabrik.de/europe/france-latest.osm.pbf";
  let response = get(url)?;
  let mut dest = File::create(output_path)?;
  let mut content = response;
  copy(&mut content, &mut dest)?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[ignore] // Remove this line to actually run the download test
  fn test_download_france_osm_pbf() {
    let path = "france-latest.osm.pbf";
    let result = download_france_osm_pbf(path);
    assert!(result.is_ok());
    assert!(fs::metadata(path).is_ok());
    // Clean up
    let _ = fs::remove_file(path);
  }
}