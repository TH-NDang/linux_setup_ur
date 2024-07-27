pub trait Repository<T> {
    fn new() -> Self;
    fn add(&mut self, item: T);
}
