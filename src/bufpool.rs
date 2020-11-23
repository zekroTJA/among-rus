use async_std::sync::Mutex;

/// Object pool for fixed size buffers.
///
/// *I have lierally no idea if this works right like this. xD*
pub struct BufPool<T> {
    initial_size: usize,
    pool: Mutex<Vec<Vec<T>>>,
}

impl<T: Default + Clone> BufPool<T> {
    /// Creates a new BufPool with the given Type and
    /// the given initial size of the buffers created.
    pub fn new(initial_size: usize) -> BufPool<T> {
        BufPool {
            initial_size,
            pool: Mutex::new(vec![]),
        }
    }

    /// Takes a buffer from the pool. If the pool is
    /// empty, a new buffer is created with the
    /// previously specified initial size.
    pub async fn take(&mut self) -> Vec<T> {
        let mut pool = self.pool.lock().await;
        match (*pool).pop() {
            Some(v) => v,
            None => vec![T::default(); self.initial_size],
        }
    }

    /// Returns a buffer back to the pool so it is
    /// available again to be taken.
    pub async fn back(&mut self, v: Vec<T>) {
        let mut pool = self.pool.lock().await;
        (*pool).push(v);
    }
}
