use async_std::sync::Mutex;

pub struct BufPool<T> {
    initial_size: usize,
    pool: Mutex<Vec<Vec<T>>>,
}

impl<T: Default + Clone> BufPool<T> {
    pub fn new(initial_size: usize) -> BufPool<T> {
        BufPool {
            initial_size,
            pool: Mutex::new(vec![]),
        }
    }

    pub async fn take(&mut self) -> Vec<T> {
        let mut pool = self.pool.lock().await;
        let v = match (*pool).pop() {
            Some(v) => v,
            None => vec![T::default(); self.initial_size],
        };

        v
    }

    pub async fn back(&mut self, v: Vec<T>) {
        let mut pool = self.pool.lock().await;
        (*pool).push(v);
    }
}
