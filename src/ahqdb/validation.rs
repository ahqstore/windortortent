use std::path::{Component, Path};

#[derive(Debug)]
pub enum PathError {
  InvalidPath,
  InvalidEncoding,
  PathTraversalAttempt,
  AbsolutePathsNotAllowed,
}

pub fn secure_logical_resolve(virtual_root: &Path, user_uri: &str) -> Result<String, PathError> {
  // 1. Strip the scheme
  let path_str = user_uri
    .strip_prefix("root://")
    .map_or(Err(PathError::InvalidPath), |x| Ok(x))?;

  let user_path = Path::new(path_str);

  // 2. maintain a stack of components relative to the root
  let mut stack = Vec::new();

  for component in user_path.components() {
    match component {
      Component::Normal(c) => stack.push(c),
      Component::CurDir => {}

      // The parent directory ".."
      Component::ParentDir => {
        // SECURITY CHECK: If the stack is empty, the user is trying
        // to go 'above' the virtual root.
        if stack.pop().is_none() {
          return Err(PathError::PathTraversalAttempt);
        }
      }
      Component::RootDir | Component::Prefix(_) => {
        return Err(PathError::AbsolutePathsNotAllowed);
      }
    }
  }

  let mut final_path = virtual_root.to_path_buf();
  for part in stack {
    final_path.push(part);
  }

  Ok(final_path.to_string_lossy().into_owned())
}
