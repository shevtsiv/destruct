pub trait VectorBasedDataStructure<T> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
}
