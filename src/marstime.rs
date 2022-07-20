use std::time::{SystemTime, UNIX_EPOCH, Duration};
use tokio;
use tokio::sync::Mutex;
use std::{sync::Arc, thread};

const LEAP_SECONDS: f64 = 37.0;
const MARS_YEAR: f64 = 668.5991;

pub struct MarsTime {
    msd: f64,
    mtc: f64,

    sol_year: f64,
    sol_day: f64,

    sol_hour: f64,
    sol_minute: f64,
    sol_second: f64,

    hour: f64,
    minute: f64,
    second: f64,
}

impl MarsTime {
    pub fn new() -> MarsTime {
        return MarsTime {
            msd: 0.0,
            mtc: 0.0,
            sol_year: 0.0,
            sol_day: 0.0,
            sol_hour: 0.0,
            sol_minute: 0.0,
            sol_second: 0.0,
            hour: 0.0,
            minute: 0.0,
            second: 0.0
        }
    }

    #[tokio::main]
    pub async fn update_loop(mt: Arc<Mutex<MarsTime>>) {
        info!("Starting update loop");
        let t = Duration::from_secs(1);
        loop {
            let mut _mt = mt.lock().await;
            _mt.update();
            drop(_mt);
            thread::sleep(t);
        }
    }

    pub fn update(&mut self) {
        let now = SystemTime::now();
        let timestamp = now.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs_f64() + LEAP_SECONDS;
        self.msd = timestamp / 88775.244147 + 34127.2954262;
        self.mtc = self.msd.fract();

        self.sol_year = self.msd / MARS_YEAR;
        self.sol_day = self.sol_year.fract() * MARS_YEAR;

        self.sol_hour = self.mtc * 24.0;
        self.sol_minute = self.sol_hour.fract() * 60.0;
        self.sol_second = self.sol_minute.fract() * 60.0;

        self.hour = self.mtc * 24.659790040800004;
        self.minute = self.hour.fract() * 60.0;
        self.second = self.minute.fract() * 60.0;
    }

    pub fn get_msd(&self) -> f64 {
        return self.msd
    }

    pub fn get_sol_year(&self) -> u16 { return self.sol_year as u16 }

    pub fn get_sol_day(&self) -> u16 { return self.sol_day as u16 }

    pub fn get_sol_hour(&self) -> u8 { return self.sol_hour as u8 }

    pub fn get_sol_minute(&self) -> u8 { return self.sol_minute as u8 }

    pub fn get_sol_second(&self) -> u8 { return self.sol_second as u8 }

    pub fn get_hour(&self) -> u8 { return self.hour as u8 }

    pub fn get_minute(&self) -> u8 { return self.minute as u8 }

    pub fn get_second(&self) -> u8 { return self.second as u8 }
}
