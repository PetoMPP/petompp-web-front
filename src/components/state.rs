#[derive(Debug, Clone, PartialEq)]
pub enum State<T, E> {
    Loading,
    Ok(T),
    Err(E),
}
