use std::{fs, thread};
use std::sync::Mutex;
use std::time::Duration;

use crate::persist::GpuJson;
use crate::settings::generic::Gpu as GenericGpu;
use crate::settings::MinMax;
use crate::settings::TGpu;
use crate::settings::{OnResume, OnSet, SettingError, SettingVariant};

fn create_lock() -> Option<Mutex<i64>> {
    Some(Mutex::new(0))
}

unsafe impl Send for Gpu {}

//#[derive(Debug)]
pub struct Gpu {
    generic: GenericGpu,
    lock: Option<Mutex<i64>>,
    state: crate::state::generic::Gpu, // NOTE this is re-used for simplicity
}

impl std::fmt::Debug for Gpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gpu")
            .field("generic", &self.generic)
            .field("state", &self.state)
            .finish_non_exhaustive()
    }
}

impl Gpu {
    pub fn from_limits(limits: limits_core::json::GenericGpuLimit) -> Self {
        Self {
            generic: GenericGpu::from_limits(limits),
            lock: create_lock(),
            state: Default::default(),
        }
    }

    pub fn from_json_and_limits(
        other: GpuJson,
        version: u64,
        limits: limits_core::json::GenericGpuLimit,
    ) -> Self {
        Self {
            generic: GenericGpu::from_json_and_limits(other, version, limits),
            lock: create_lock(),
            state: Default::default(),
        }
    }

    fn get_thermal_policy(&self) -> u64 {
        let tdp = if let Some(stapm_ppt) = &self.generic.stapm_ppt {
            *stapm_ppt
        } else {
            return 0;
        };

        match tdp {
            val if val < 9_000 => 2,
            val if (9_000..=18_000).contains(&val) => 0,
            _ => 1,
        }
    }

    fn set_thermal_policy(&self, thermal_policy: u64) {
        log::info!("Setting thermal policy: {}", thermal_policy);
        let file_path = "/sys/devices/platform/asus-nb-wmi/throttle_thermal_policy";
        match fs::read_to_string(file_path) {
            Ok(content) if content.trim() != thermal_policy.to_string() => {
                fs::write(file_path, thermal_policy.to_string())
                    .expect("Couldn't change thermal policy");
                
                // kernel will automatically change TDP settings when we write to throttle_thermal_policy. 
                // This might race if we modify TDP ourself. Wait a bit for kernel to do first.
                thread::sleep(Duration::from_millis(2000));
            }
            _ => {}
        }
        log::info!("New thermal policy: {}", thermal_policy);
    }

    fn set_stapm_ppt(&self, limit: i32) {
        log::info!("Setting stamp ppt to {}", limit);
        let file_path = "/sys/devices/platform/asus-nb-wmi/ppt_pl1_spl";
        fs::write(file_path, limit.to_string()).expect("Couldn't change STAMP PPT");
    }

    fn set_slow_ppt(&self, limit: i32) {
        log::info!("Setting slow ppt to {}", limit);
        let file_path = "/sys/devices/platform/asus-nb-wmi/ppt_pl2_sppt";
        fs::write(file_path, limit.to_string()).expect("Couldn't change SLOW PPT");
    }

    fn set_fast_ppt(&self, limit: i32) {
        log::info!("Setting fast ppt to {}", limit);
        let file_path = "/sys/devices/platform/asus-nb-wmi/ppt_fppt";
        fs::write(file_path, limit.to_string()).expect("Couldn't change FAST PPT");
    }

    fn set_all(&mut self) -> Result<(), Vec<SettingError>> {
        let mutex = match &self.lock {
            Some(x) => x,
            None => {
                return Err(vec![SettingError {
                    msg: "Lock unavailable".to_owned(),
                    setting: SettingVariant::Gpu,
                }]);
            }
        };
        let _lock = match mutex.lock() {
            Ok(x) => x,
            Err(e) => {
                return Err(vec![SettingError {
                    msg: format!("Lock lock acquire failed: {}", e),
                    setting: SettingVariant::Gpu,
                }]);
            }
        };

        // Set thermal policy under lock.
        // This might make UI a bit unresponsive, but it's safer.
        let thermal_policy = self.get_thermal_policy();
        self.set_thermal_policy(thermal_policy);

        if let Some(fast_ppt) = &self.generic.fast_ppt {
            let value = *fast_ppt as i32;
            self.set_fast_ppt(value / 1000);
        } else {
            self.set_fast_ppt(17);
        }

        // Set slow limit
        if let Some(slow_ppt) = &self.generic.slow_ppt {
            let value = *slow_ppt as i32;
            self.set_slow_ppt(value / 1000);
        } else {
            self.set_slow_ppt(15);
        }

        // Set STAPM limit
        if let Some(stapm_ppt) = &self.generic.stapm_ppt {
            let value = *stapm_ppt as i32;
            self.set_stapm_ppt(value / 1000);
        } else {
            self.set_stapm_ppt(15);
        }

        if let Some(clock_limits) = &self.generic.clock_limits {
            self.state.clock_limits_set = true;
            self.set_clock_mode(self.state.clock_limits_set);
            if let Some(max) = clock_limits.max {
                if let Some(min) = clock_limits.min  {
                    self.set_max_min_clock(max, min);
                }
            }
        } else {
            self.state.clock_limits_set = false;
            self.set_clock_mode(self.state.clock_limits_set);
        }
        Ok(())
    }

    fn set_max_min_clock(&self, max: u64, min: u64) {
        log::info!("Setting max: {}, min: {}", max, min);

        fs::write("/sys/class/drm/card1/device/pp_od_clk_voltage", format!("s 0 {}", min)).expect("cant write file");
        fs::write("/sys/class/drm/card1/device/pp_od_clk_voltage", format!("s 1 {}", max)).expect("cant write file");
        fs::write("/sys/class/drm/card1/device/pp_od_clk_voltage", "c").expect("cant write file");
    }

    fn set_clock_mode(&self, clock_limits_set: bool) {
        if !clock_limits_set {
            log::info!("Setting clock mode to auto");
            fs::write("/sys/class/drm/card1/device/power_dpm_force_performance_level", "auto").expect("cant write file /sys/class/drm/card1/device/power_dpm_force_performance_level");
        } else {
            log::info!("Setting clock mode to manual");
            fs::write("/sys/class/drm/card1/device/power_dpm_force_performance_level", "manual").expect("cant write file /sys/class/drm/card1/device/power_dpm_force_performance_level");
        }
    }

    fn resume_all(&self) -> Result<(), Vec<SettingError>> {
        // like set_all() but without updating state
        // -- assumption: state is already up to date
        let mutex = match &self.lock {
            Some(x) => x,
            None => {
                return Err(vec![SettingError {
                    msg: "Lock unavailable".to_owned(),
                    setting: SettingVariant::Gpu,
                }]);
            }
        };
        let _lock = match mutex.lock() {
            Ok(x) => x,
            Err(e) => {
                return Err(vec![SettingError {
                    msg: format!("Lock acquire failed: {}", e),
                    setting: SettingVariant::Gpu,
                }]);
            }
        };
        if let Some(fast_ppt) = &self.generic.fast_ppt {
            let value = *fast_ppt as i32;
            self.set_fast_ppt(value / 1000);
        }
        if let Some(slow_ppt) = &self.generic.slow_ppt {
            let value = *slow_ppt as i32;
            self.set_slow_ppt(value / 1000);
        }
        if let Some(stapm_ppt) = &self.generic.stapm_ppt {
            let value = *stapm_ppt as i32;
            self.set_stapm_ppt(value / 1000);
        }

        if let Some(clock_limits) = &self.generic.clock_limits {
            self.set_clock_mode(self.state.clock_limits_set);
            if let Some(max) = clock_limits.max {
                if let Some(min) = clock_limits.min  {
                    self.set_max_min_clock(max, min);
                }
            }
        }
        Ok(())
    }
}

impl OnResume for Gpu {
    fn on_resume(&self) -> Result<(), Vec<SettingError>> {
        self.generic.on_resume()?;
        self.resume_all()
    }
}

impl OnSet for Gpu {
    fn on_set(&mut self) -> Result<(), Vec<SettingError>> {
        self.generic.on_set()?;
        self.set_all()
    }
}

impl crate::settings::OnPowerEvent for Gpu {}

impl TGpu for Gpu {
    fn limits(&self) -> crate::api::GpuLimits {
        self.generic.limits()
    }

    fn json(&self) -> crate::persist::GpuJson {
        self.generic.json()
    }

    fn ppt(&mut self, fast: Option<u64>, slow: Option<u64>) {
        self.generic.ppt(fast, slow)
    }

    fn get_ppt(&self) -> (Option<u64>, Option<u64>) {
        self.generic.get_ppt()
    }

    fn get_ppt_tdp(&self) -> (Option<u64>, Option<u64>, Option<u64>) {
        self.generic.get_ppt_tdp()
    }

    fn clock_limits(&mut self, limits: Option<MinMax<u64>>) {
        self.generic.clock_limits(limits)
    }

    fn get_clock_limits(&self) -> Option<&MinMax<u64>> {
        self.generic.get_clock_limits()
    }

    fn slow_memory(&mut self) -> &mut bool {
        self.generic.slow_memory()
    }

    fn provider(&self) -> crate::persist::DriverJson {
        crate::persist::DriverJson::GenericAMD
    }

    fn ppt_tdp(&mut self, tdp: Option<u64>, fast: Option<u64>, slow: Option<u64>) {
        log::info!("ppt_tdp: tdp: {:?}, fast: {:?}, slow: {:?}", tdp, fast, slow);
        self.generic.ppt_tdp(tdp, fast, slow)
    }

    fn get_preset(&self) -> Option<u64> {
        self.generic.get_preset()
    }

    fn set_preset(&mut self, preset: Option<u64>) {
        self.generic.set_preset(preset)
    }
}
