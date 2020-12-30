#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum StoreError {
    FileError(String),
    DirectoryError(String),
    WriteError(String),
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub struct Store {
    base_path: String,
}

impl Store {
    pub fn new(base_path: String) -> Result<Store, StoreError> {
        std::fs::create_dir_all(base_path.clone())
            .map_err(|e| StoreError::FileError(e.to_string()))
            .map(|_| Store { base_path })
    }

    fn path_for(&self, key: String) -> String {
        self.base_path
            .strip_suffix('/')
            .map(|s| s.to_string())
            .unwrap_or(self.base_path.clone())
            + "/"
            + key.strip_prefix('/').map(|s| s).unwrap_or(key.as_str())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Store {
    pub async fn load(&self, key: String) -> Result<String, StoreError> {
        use async_std::prelude::*;
        let mut contents = String::new();

        let mut file = async_std::fs::File::open(self.path_for(key))
            .await
            .map_err(|e| StoreError::FileError(e.to_string()))?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|e| StoreError::FileError(e.to_string()))?;

        Ok(contents)
    }

    pub async fn save(&self, key: String, content: String) -> Result<(), StoreError> {
        use async_std::prelude::*;

        let path = self.path_for(key).split('/').into_iter().fold(
            std::path::PathBuf::new(),
            |mut path, part| {
                path.push(part);
                path
            },
        );

        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|e| StoreError::DirectoryError(e.to_string()))?;
        }

        {
            let mut file = async_std::fs::OpenOptions::new()
                .append(false)
                // .read(true)
                .write(true)
                .create(true)
                // .truncate(true)
                .open(path)
                .await
                .map_err(|e| StoreError::FileError(e.to_string()))?;

            file.write_all(content.as_bytes())
                .await
                .map_err(|e| StoreError::WriteError(e.to_string()))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Store;
    #[actix_rt::test]
    async fn file_persistence() {
        let content = "hello, I am the content you are looking for!".to_string();
        let key = "my-thing".to_string();
        let test_directory = "store-test".to_string();
        let store = Store::new(test_directory.clone()).unwrap();
        let first = store.load(key.clone()).await;
        assert!(first.is_err());

        let save = store.save(key.clone(), content.clone()).await;
        assert_eq!(save, Ok(()));

        let load = store.load(key.clone()).await;
        assert_eq!(load, Ok(content));
        std::fs::remove_dir_all(test_directory);
    }
}
