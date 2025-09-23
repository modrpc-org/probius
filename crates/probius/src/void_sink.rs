#[cfg(not(feature = "enabled"))]
pub fn init_void_sink() { }

#[cfg(feature = "enabled")]
pub fn init_void_sink() {
    let buffer_pool = bab::HeapBufferPool::new(8192, 16, 16);
    let headroom = 0;
    crate::init(headroom, buffer_pool.clone());

    spawn_void_sink_flusher(buffer_pool);
}

#[cfg(feature = "enabled")]
pub(crate) fn spawn_void_sink_flusher(buffer_pool: bab::HeapBufferPool) {
    std::thread::spawn(move || {
        loop {
            for buffer in crate::flush() {
                unsafe { buffer_pool.release(buffer); }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}
