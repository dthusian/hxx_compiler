
pub fn invert<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
  x.map_or(Ok(None), |v| v.map(Some))
}

pub fn invert2<T, E>(x: Result<Option<T>, E>) -> Option<Result<T, E>> {
  x.map_or_else(|e| Some(Err(e)), |v| v.map(Ok))
}