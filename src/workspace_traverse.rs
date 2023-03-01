use crate::workspace::Workspace;
use std::path::PathBuf;

impl Workspace {
    pub fn files(&self) -> Vec<PathBuf> {
        walkdir::WalkDir::new(&self.path)
            .into_iter()
            .filter(|e| {
                e.is_ok()
                    && !e
                        .as_ref()
                        .unwrap()
                        .file_name()
                        .to_str()
                        .unwrap_or(".")
                        .starts_with('.')
            })
            .map(|file| file.unwrap().into_path())
            .collect()
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    // TODO: Tests be failing 😭
    fn test_files() {
        let workspace = Workspace {
            name: "example workspace".to_string(),
            path: PathBuf::from("test/example_workspace"),
        };

        let files = workspace.files();
        assert_eq!(files.len(), 2);
    }
}
