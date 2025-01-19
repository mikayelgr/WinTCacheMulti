use std::fs::DirEntry;

#[inline]
pub fn get_entries() -> Vec<DirEntry> {
    let path = dunce::canonicalize("./benches/imgdata").unwrap();
    // Unwrapping the entries in parallel
    let mut entries: Vec<DirEntry> = std::fs::read_dir(&path)
        .unwrap()
        .map(|e| e.unwrap())
        .collect();

    // .gitkeep will be removed so that all the files are valid. In case an error occurs
    // that usually means that the file format is not supported. So, one assumption that
    // we have made for this benchmark is that all the files are going to be in a format
    // that makes them valid for extraction of thumbnail, which in our case are 36 MP4
    // video files in 4K format.
    if let Some(gitkeep_index) = entries.iter().position(|e| e.file_name() == ".gitkeep") {
        entries.remove(gitkeep_index);
    }
    
    entries
}
