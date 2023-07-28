# [FROM HERE](https://stackoverflow.com/questions/69540812/how-to-return-current-working-directory-from-function)

- ```rust
  fn get_current_working_dir() -> String {
      let res = env::current_dir();
      match res {
          Ok(path) => path.into_os_string().into_string().unwrap(),
          Err(_) => "FAILED".to_string()
      }
  }
  ```
