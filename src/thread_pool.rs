use rayon::ThreadPool;
use rayon::ThreadPoolBuilder;

pub struct RayonThreadPool(ThreadPool);

impl RayonThreadPool {
    pub fn new(threads: u32) -> Result<Self, String> {
        let pool = ThreadPoolBuilder::new()
            .num_threads(threads as usize)
            .build()
            .map_err(|e| format!("{}", e))?;
        Ok(RayonThreadPool(pool))
    }
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.0.spawn(job)
    }
}

#[test]
pub fn new() {
    RayonThreadPool::new(21).unwrap();
    assert!(true)
}
