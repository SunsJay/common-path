use std::path::Component;
use std::process::Command;

pub enum CommonPath {
    Root,
    Home,
    Desktop,
    CurrentProject,
}

impl CommonPath {
    fn get(&self) -> Option<&str> {
        match self {
            CommonPath::Root => Component::RootDir.as_os_str().to_str(),
            CommonPath::Home => {
                Some("home")
            }
            // Path::Desktop => {}
            CommonPath::CurrentProject => Component::CurDir.as_os_str().to_str(),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CommonPath;

    #[test]
    fn test_get__root_path() {
        assert_eq!(None, CommonPath::Root.get());
    }

    #[test]
    fn test_get__current_path() {
        assert_eq!(None, CommonPath::CurrentProject.get());
    }

    #[test]
    fn test_get__home_path() {
        assert_eq!(None, CommonPath::Home.get());
    }
}
