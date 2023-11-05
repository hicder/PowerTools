use std::{fs, thread};
use libryzenadj::RyzenAdj;
use std::sync::Mutex;
use std::time::Duration;

use crate::persist::GpuJson;
use crate::settings::generic::Gpu as GenericGpu;
use crate::settings::MinMax;
use crate::settings::TGpu;
use crate::settings::{OnResume, OnSet, SettingError, SettingVariant};

fn ryzen_adj_or_log() -> Option<Mutex<RyzenAdj>> {
    match RyzenAdj::new() {
        Ok(x) => Some(Mutex::new(x)),
        Err(e) => {
            log::error!("RyzenAdj init error: {}", e);
            None
        }
    }
}

unsafe impl Send for Gpu {} // implementor (RyzenAdj) may be unsafe

//#[derive(Debug)]
pub struct Gpu {
    generic: GenericGpu,
    implementor: Option<Mutex<RyzenAdj>>,
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
            implementor: ryzen_adj_or_log(),
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
            implementor: ryzen_adj_or_log(),
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
            val if val < 12_000 => 2,
            val if (12_000..=24_000).contains(&val) => 0,
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

    fn set_all(&mut self) -> Result<(), Vec<SettingError>> {
        let mutex = match &self.implementor {
            Some(x) => x,
            None => {
                return Err(vec![SettingError {
                    msg: "RyzenAdj unavailable".to_owned(),
                    setting: SettingVariant::Gpu,
                }]);
            }
        };
        let lock = match mutex.lock() {
            Ok(x) => x,
            Err(e) => {
                return Err(vec![SettingError {
                    msg: format!("RyzenAdj lock acquire failed: {}", e),
                    setting: SettingVariant::Gpu,
                }]);
            }
        };

        // Set thermal policy under lock.
        // This might make UI a bit unresponsive, but it's safer.
        let thermal_policy = self.get_thermal_policy();
        self.set_thermal_policy(thermal_policy);

        let mut errors = Vec::new();
        if let Some(fast_ppt) = &self.generic.fast_ppt {
            if self.state.old_fast_ppt.is_none() {
                match lock.get_fast_value() {
                    Ok(val) => self.state.old_fast_ppt = Some(val as _),
                    Err(e) => errors.push(SettingError {
                        msg: format!("RyzenAdj get_fast_value() err: {}", e),
                        setting: SettingVariant::Gpu,
                    }),
                }
            }
            lock.set_fast_limit(*fast_ppt as _)
                .map_err(|e| SettingError {
                    msg: format!("RyzenAdj set_fast_limit({}) err: {}", *fast_ppt, e),
                    setting: SettingVariant::Gpu,
                })
                .unwrap_or_else(|e| errors.push(e));
        } else if let Some(fast_ppt) = &self.state.old_fast_ppt {
            lock.set_fast_limit(*fast_ppt as _)
                .map_err(|e| SettingError {
                    msg: format!("RyzenAdj set_fast_limit({}) err: {}", *fast_ppt, e),
                    setting: SettingVariant::Gpu,
                })
                .unwrap_or_else(|e| errors.push(e));
            self.state.old_fast_ppt = None;
        }

        // Set slow limit
        if let Some(slow_ppt) = &self.generic.slow_ppt {
            if self.state.old_slow_ppt.is_none() {
                match lock.get_slow_value() {
                    Ok(val) => self.state.old_fast_ppt = Some(val as _),
                    Err(e) => errors.push(SettingError {
                        msg: format!("RyzenAdj get_slow_value() err: {}", e),
                        setting: SettingVariant::Gpu,
                    }),
                }
            }
            lock.set_slow_limit(*slow_ppt as _)
                .map_err(|e| SettingError {
                    msg: format!("RyzenAdj set_slow_limit({}) err: {}", *slow_ppt, e),
                    setting: SettingVariant::Gpu,
                })
                .unwrap_or_else(|e| errors.push(e));
        } else if let Some(slow_ppt) = &self.state.old_slow_ppt {
            lock.set_slow_limit(*slow_ppt as _)
                .map_err(|e| SettingError {
                    msg: format!("RyzenAdj set_slow_limit({}) err: {}", *slow_ppt, e),
                    setting: SettingVariant::Gpu,
                })
                .unwrap_or_else(|e| errors.push(e));
            self.state.old_slow_ppt = None;
        }

        // Set STAPM limit
        if let Some(stapm_ppt) = &self.generic.stapm_ppt {
            if self.state.old_stapm_ppt.is_none() {
                match lock.get_stapm_value() {
                    Ok(val) => self.state.old_stapm_ppt = Some(val as _),
                    Err(e) => errors.push(SettingError {
                        msg: format!("RyzenAdj get_stapm_value() err: {}", e),
                        setting: SettingVariant::Gpu,
                    }),
                }
            }

            lock.set_stapm_limit(*stapm_ppt as _)
                .map_err(|e| SettingError {
                    msg: format!("RyzenAdj set_stapm_limit({}) err: {}", *stapm_ppt, e),
                    setting: SettingVariant::Gpu,
                })
                .unwrap_or_else(|e| errors.push(e));
        } else if let Some(stapm_ppt) = &self.state.old_stapm_ppt {
            lock.set_stapm_limit(*stapm_ppt as _)
                .map_err(|e| SettingError {
                    msg: format!("RyzenAdj set_stapm_limit({}) err: {}", *stapm_ppt, e),
                    setting: SettingVariant::Gpu,
                })
                .unwrap_or_else(|e| errors.push(e));
            self.state.old_stapm_ppt = None;
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
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn set_max_min_clock(&self, max: u64, min: u64) {
        log::info!("Setting max: {}, min: {}", max, min);
        std::process::Command::new("/usr/local/bin/set-clock")
            .arg(min.to_string())
            .arg(max.to_string())
            .arg("card1")
            .output()
            .expect("Failed to execute set-clock");
    }

    fn set_clock_mode(&self, clock_limits_set: bool) {
        if !clock_limits_set {
            log::info!("Setting clock mode to auto");
            // Execute /usr/local/bin/set-clock-mode auto card1
            std::process::Command::new("/usr/local/bin/set-clock-mode")
                .arg("auto")
                .arg("card1")
                .output()
                .expect("Failed to execute set-clock-mode");
        } else {
            log::info!("Setting clock mode to manual");
            // Execute /usr/local/bin/set-clock-mode manual card1
            std::process::Command::new("/usr/local/bin/set-clock-mode")
                .arg("manual")
                .arg("card1")
                .output()
                .expect("Failed to execute set-clock-mode");
        }
    }

    fn resume_all(&self) -> Result<(), Vec<SettingError>> {
        // like set_all() but without updating state
        // -- assumption: state is already up to date
        let mutex = match &self.implementor {
            Some(x) => x,
            None => {
                return Err(vec![SettingError {
                    msg: "RyzenAdj unavailable".to_owned(),
                    setting: SettingVariant::Gpu,
                }]);
            }
        };
        let lock = match mutex.lock() {
            Ok(x) => x,
            Err(e) => {
                return Err(vec![SettingError {
                    msg: format!("RyzenAdj lock acquire failed: {}", e),
                    setting: SettingVariant::Gpu,
                }]);
            }
        };
        let mut errors = Vec::new();
        if let Some(fast_ppt) = &self.generic.fast_ppt {
            lock.set_fast_limit(*fast_ppt as _)
                .map_err(|e| SettingError {
                    msg: format!("RyzenAdj set_fast_limit({}) err: {}", *fast_ppt, e),
                    setting: SettingVariant::Gpu,
                })
                .unwrap_or_else(|e| errors.push(e));
        }
        if let Some(slow_ppt) = &self.generic.slow_ppt {
            lock.set_slow_limit(*slow_ppt as _)
                .map_err(|e| SettingError {
                    msg: format!("RyzenAdj set_slow_limit({}) err: {}", *slow_ppt, e),
                    setting: SettingVariant::Gpu,
                })
                .unwrap_or_else(|e| errors.push(e));
        }
        if let Some(stapm_ppt) = &self.generic.stapm_ppt {
            lock.set_stapm_limit(*stapm_ppt as _)
                .map_err(|e| SettingError {
                    msg: format!("RyzenAdj set_stapm_limit({}) err: {}", *stapm_ppt, e),
                    setting: SettingVariant::Gpu,
                })
                .unwrap_or_else(|e| errors.push(e));
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
