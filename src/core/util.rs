macro_rules! lock_mutex {
    ($mutex:expr) => {{
        $mutex
            .lock()
            .map_err(|err| RtError::MutexLock(err.to_string()))?
    }};
}

pub(crate) use lock_mutex;
