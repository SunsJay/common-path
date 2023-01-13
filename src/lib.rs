pub enum Path {
    Root,
    Home,
    Desktop,
    CurrentProject,
}


impl Path {
    fn get(&self) -> String {
        "1".to_string()
    }
}


#[cfg(test)]
mod tests {
    use crate::Path;

    #[test]
    fn test_get_path() {
        assert_eq!("1".to_string(), Path::Home.get());
    }
}