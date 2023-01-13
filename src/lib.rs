use std::path::Component;

pub enum CommonPath {
    Root,
    Home,
    Desktop,
    CurrentProject,
}

impl CommonPath {
    fn get(&self) -> &str {
        match self {
            CommonPath::Root => Component::RootDir.as_os_str().to_str().unwrap(),
            // Path::Home => {}
            // Path::Desktop => {}
            CommonPath::CurrentProject => Component::CurDir.as_os_str().to_str().unwrap(),
            _ => "1",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CommonPath;

    #[test]
    fn test_get__root_path() {
        assert_eq!("/", CommonPath::Root.get());
    }

#[test]
    fn test_get__current_path() {
        assert_eq!("/", CommonPath::CurrentProject.get());
    }
}
