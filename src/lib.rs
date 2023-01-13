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
            // Path::CurrentProject => {}
            _ => "1",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CommonPath;

    #[test]
    fn test_get_path() {
        assert_eq!("1".to_string(), CommonPath::Root.get());
    }
}
