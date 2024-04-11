use std::fmt::Display;

pub fn invert<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
  x.map_or(Ok(None), |v| v.map(Some))
}

pub fn invert2<T, E>(x: Result<Option<T>, E>) -> Option<Result<T, E>> {
  x.map_or_else(|e| Some(Err(e)), |v| v.map(Ok))
}

pub fn map_join<T, U: Display, F: FnMut(&T) -> &U>(arr: &[T], mut f: F, sep: &str) -> String {
  arr.into_iter().map(|v| f(v).to_string()).collect::<Vec<_>>().join(sep)
}

pub fn join<T: Display>(arr: &[T], sep: &str) -> String {
  arr.into_iter().map(|v| v.to_string()).collect::<Vec<_>>().join(sep)
}