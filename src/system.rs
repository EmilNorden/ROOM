use std::time::SystemTime;

pub struct System {
    base_time_secs: u64,
}

impl System {
    pub fn new() -> Self {
        let duration_since_epoch =
            SystemTime::now().
                duration_since(SystemTime::UNIX_EPOCH)
                .expect("Failed to get epoch time");

        System {
            base_time_secs: duration_since_epoch.as_secs(),
        }
    }

    /// Calculates the ticks since game start.
    /// Based on I_GetTime in i_system.c
    pub fn calculate_tics(&self) -> u64 {
        const TIC_RATE: u32 = 35;

        let duration_since_epoch =
            SystemTime::now().
                duration_since(SystemTime::UNIX_EPOCH)
                .expect("Failed to get epoch time");

        (duration_since_epoch.as_secs() - self.base_time_secs) * TIC_RATE as u64 +
            (duration_since_epoch.subsec_micros() * TIC_RATE / 1000000) as u64
    }
}