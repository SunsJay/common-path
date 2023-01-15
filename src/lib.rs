use std::path::Component;

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
            // Path::Home => {}
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
}
