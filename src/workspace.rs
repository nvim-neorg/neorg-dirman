use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Workspace {
    pub name: String,
    pub path: PathBuf,
}

pub struct WorkspaceManager<'a> {
    pub workspaces: HashMap<&'a String, &'a Workspace>,
    current_workspace: &'a String,
}

#[derive(Debug)]
pub struct WorkspaceNotFound {
    pub workspace: String,
}

impl<'a> WorkspaceManager<'a> {
    /// Creates a new workspace manager with a single workspace, setting it as the default.
    ///
    /// * `workspace`: The single workspace to use
    pub fn from_single_workspace(workspace: &'a Workspace) -> WorkspaceManager<'a> {
        WorkspaceManager {
            current_workspace: &workspace.name,
            workspaces: HashMap::from([(&workspace.name, workspace)]),
        }
    }

    /// Creates a new workspace manager from a list of workspaces.
    /// If the default workspace is not found, an error is returned.
    ///
    /// * `workspaces`: A list of workspaces to add to the workspace manager.
    /// * `default_workspace`: The name of the default workspace.
    pub fn new(
        workspaces: Vec<&'a Workspace>,
        default_workspace: &'a String,
    ) -> Result<WorkspaceManager<'a>, WorkspaceNotFound> {
        if !workspaces.iter().any(|w| &w.name == default_workspace) {
            Err(WorkspaceNotFound {
                workspace: default_workspace.to_string(),
            })
        } else {
            Ok(WorkspaceManager {
                current_workspace: default_workspace,
                workspaces: workspaces
                    .into_iter()
                    .map(|w| (&w.name, w))
                    .collect::<HashMap<_, _>>(),
            })
        }
    }

    /// Returns a workspace with the given name, or None if it doesn't exist.
    ///
    /// * `name`: The name of the workspace.
    pub fn get_workspace(&self, name: &String) -> Option<&Workspace> {
        self.workspaces.get(name).cloned()
    }

    /// Sets the current workspace to the workspace with the given name.
    /// Returns unit if the workspace was set, else returns a WorkspaceNotFound error.
    ///
    /// * `name`: The name of the workspace to set as the current workspace.
    pub fn set_current_workspace(&mut self, name: &'a String) -> Result<(), WorkspaceNotFound> {
        if self.workspaces.get(name).is_none() {
            Err(WorkspaceNotFound {
                workspace: name.to_string(),
            })
        } else {
            self.current_workspace = name;
            Ok(())
        }
    }

    /// Returns the current workspace.
    pub fn get_current_workspace(&self) -> &'a Workspace {
        self.workspaces.get(self.current_workspace).unwrap()
    }

    /// Adds a workspace to the list of workspaces.
    /// Overwrites any existing workspace with the same name.
    ///
    /// * `workspace`: The workspace to add to the list of workspaces.
    pub fn add_workspace(&mut self, workspace: &'a Workspace) {
        self.workspaces.insert(&workspace.name, workspace);
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_workspace_manager_from_single_workspace() {
        let workspace = Workspace {
            name: "example name".to_string(),
            path: "~/some/path".into(),
        };

        let workspace_manager = WorkspaceManager::from_single_workspace(&workspace);
        assert_eq!(workspace_manager.current_workspace, &workspace.name);
    }

    #[test]
    fn test_workspace_manager_new() {
        let workspace1 = Workspace {
            name: "example name".to_string(),
            path: "~/some/path".into(),
        };

        let workspace2 = Workspace {
            name: "another example name".to_string(),
            path: "~/another/path".into(),
        };

        let workspace_manager =
            WorkspaceManager::new(vec![&workspace1, &workspace2], &workspace1.name).expect("");
        assert_eq!(workspace_manager.current_workspace, &workspace1.name);
    }

    #[test]
    fn test_workspace_manager_current_workspace() {
        let workspace1 = Workspace {
            name: "example name".to_string(),
            path: "~/some/path".into(),
        };

        let workspace2 = Workspace {
            name: "another example name".to_string(),
            path: "~/another/path".into(),
        };

        let mut workspace_manager =
            WorkspaceManager::new(vec![&workspace1, &workspace2], &workspace1.name)
                .expect("Workspace 1 not found!");
        workspace_manager
            .set_current_workspace(&workspace2.name)
            .expect("Unable to set the current workspace");

        assert_eq!(
            &workspace_manager.get_current_workspace().name,
            &workspace2.name
        );
    }
}
