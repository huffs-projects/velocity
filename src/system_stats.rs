use sysinfo::System;
use std::collections::{VecDeque, HashMap};
use std::time::Instant;
use std::process::Command;

pub struct SystemStats {
    system: System,
    network_history: NetworkHistory,
    cpu_history: CpuHistory,
    cpu_temp_history: CpuTempHistory,
    memory_history: MemoryHistory,
    disk_history: DiskHistory,
    load_history: LoadHistory,
    network_peaks: NetworkPeaks,
    ping_history: PingHistory,
    ping_servers: Vec<PingServer>,
    last_refresh: Instant,
    last_ping_time: Instant,
    last_network_update: Instant, // Separate timer for network rate calculation
    prev_network_totals: Option<(u64, u64)>, // (received, transmitted) for primary interface
    primary_interface: Option<String>,
    smoothed_download_rate: f64, // Exponential moving average for smooth display
    smoothed_upload_rate: f64,
}

#[derive(Debug, Clone)]
struct NetworkHistory {
    download_samples: VecDeque<f64>,  // Bytes per second
    upload_samples: VecDeque<f64>,
    max_samples: usize,
}

#[derive(Debug, Clone)]
struct CpuHistory {
    cpu_samples: VecDeque<f32>,
    max_samples: usize,
}

#[derive(Debug, Clone)]
struct CpuTempHistory {
    temp_samples: VecDeque<f32>,  // Temperature in Celsius
    max_samples: usize,
}

#[derive(Debug, Clone)]
struct MemoryHistory {
    used_samples: VecDeque<f64>,  // Used memory in bytes
    free_samples: VecDeque<f64>,  // Free memory in bytes
    max_samples: usize,
}

#[derive(Debug, Clone)]
struct DiskHistory {
    usage_samples: VecDeque<f64>,  // Percentage values 0-100
    max_samples: usize,
}

#[derive(Debug, Clone)]
struct LoadHistory {
    one_min_samples: VecDeque<f64>,  // 1-minute load average
    five_min_samples: VecDeque<f64>,  // 5-minute load average
    fifteen_min_samples: VecDeque<f64>,  // 15-minute load average
    max_samples: usize,
}

#[derive(Debug, Clone)]
struct NetworkPeaks {
    peak_download: f64,  // Bytes per second
    peak_upload: f64,
}

#[derive(Debug, Clone)]
struct PingServer {
    region: String,
    hostname: String,
}

#[derive(Debug, Clone)]
pub struct RegionPingHistory {
    latency_samples: VecDeque<f64>,  // Latency in milliseconds
    max_samples: usize,
}

impl RegionPingHistory {
    pub fn samples(&self) -> &VecDeque<f64> {
        &self.latency_samples
    }
}

#[derive(Debug, Clone)]
struct PingHistory {
    region_histories: HashMap<String, RegionPingHistory>,  // Keyed by region name
    max_samples: usize,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MemoryDetails {
    pub used: u64,
    pub free: u64,
    pub total: u64,
    pub cached: u64,
    pub buffers: u64,
    pub swap_used: u64,
    pub swap_total: u64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NetworkStats {
    pub interface: String,
    pub received: u64,
    pub transmitted: u64,
    pub received_per_sec: f64,
    pub transmitted_per_sec: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DiskStats {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LoadAverage {
    pub one_min: f64,
    pub five_min: f64,
    pub fifteen_min: f64,
}

impl SystemStats {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        // Initialize network totals so first refresh can calculate rates
        let mut prev_network_totals = None;
        let mut primary_interface = None;
        use sysinfo::Networks;
        let mut networks = Networks::new_with_refreshed_list();
        networks.refresh();
        
        // Find primary interface (highest total traffic)
        let mut primary_interface_name: Option<String> = None;
        let mut max_traffic = 0u64;
        for (interface_name, network) in networks.iter() {
            let total = network.total_received() + network.total_transmitted();
            if total > max_traffic {
                max_traffic = total;
                primary_interface_name = Some(interface_name.to_string());
            }
        }
        
        // Use primary interface if found, otherwise use first available
        if primary_interface_name.is_none() {
            for (interface_name, _) in networks.iter() {
                primary_interface_name = Some(interface_name.to_string());
                break;
            }
        }
        
        // Store initial totals for primary interface
        // Use total_received/total_transmitted (cumulative) instead of received/transmitted (delta)
        if let Some(ref primary_name) = primary_interface_name {
            primary_interface = Some(primary_name.clone());
            if let Some((_, network)) = networks.iter().find(|(name, _)| name.as_str() == primary_name.as_str()) {
                prev_network_totals = Some((network.total_received(), network.total_transmitted()));
            }
        }
        
        Self {
            system,
            network_history: NetworkHistory {
                download_samples: VecDeque::new(),
                upload_samples: VecDeque::new(),
                max_samples: 60, // 60 samples for history
            },
            cpu_history: CpuHistory {
                cpu_samples: VecDeque::new(),
                max_samples: 60,
            },
            cpu_temp_history: CpuTempHistory {
                temp_samples: VecDeque::new(),
                max_samples: 60,
            },
            memory_history: MemoryHistory {
                used_samples: VecDeque::new(),
                free_samples: VecDeque::new(),
                max_samples: 60,
            },
            disk_history: DiskHistory {
                usage_samples: VecDeque::new(),
                max_samples: 60,
            },
            load_history: LoadHistory {
                one_min_samples: VecDeque::new(),
                five_min_samples: VecDeque::new(),
                fifteen_min_samples: VecDeque::new(),
                max_samples: 60,
            },
            network_peaks: NetworkPeaks {
                peak_download: 0.0,
                peak_upload: 0.0,
            },
            ping_servers: vec![
                PingServer { region: "Africa".to_string(), hostname: "speedtest.joburg.linode.com".to_string() },
                PingServer { region: "Europe".to_string(), hostname: "speedtest.frankfurt.linode.com".to_string() },
                PingServer { region: "Asia".to_string(), hostname: "speedtest.tokyo2.linode.com".to_string() },
                PingServer { region: "North America".to_string(), hostname: "speedtest.newark.linode.com".to_string() },
                PingServer { region: "South America".to_string(), hostname: "speedtest.sao-paulo.linode.com".to_string() },
                PingServer { region: "Australia".to_string(), hostname: "speedtest.syd1.digitalocean.com".to_string() },
            ],
            ping_history: {
                let mut histories = HashMap::new();
                let regions = vec!["Africa", "Europe", "Asia", "North America", "South America", "Australia"];
                for region in regions {
                    histories.insert(region.to_string(), RegionPingHistory {
                        latency_samples: VecDeque::new(),
                        max_samples: 60,
                    });
                }
                PingHistory {
                    region_histories: histories,
                    max_samples: 60,
                }
            },
            last_refresh: Instant::now(),
            last_ping_time: Instant::now(),
            last_network_update: Instant::now(),
            prev_network_totals,
            primary_interface,
            smoothed_download_rate: 0.0,
            smoothed_upload_rate: 0.0,
        }
    }

    pub fn refresh(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refresh);
        
        self.system.refresh_all();
        
        // Update CPU history
        let cpu_usage = self.system.global_cpu_info().cpu_usage();
        self.cpu_history.cpu_samples.push_back(cpu_usage);
        if self.cpu_history.cpu_samples.len() > self.cpu_history.max_samples {
            self.cpu_history.cpu_samples.pop_front();
        }
        
        // Update CPU temperature history (try real temperature first, fallback to simulated)
        let temp = if let Some(real_temp) = self.get_cpu_temperature() {
            real_temp
        } else {
            // Fallback to simulated temperature if real reading is unavailable
            let base_temp = 35.0; // Base temperature in Celsius
            let cpu_influence = cpu_usage * 0.5; // CPU usage adds heat
            // Add some variation based on time (using elapsed seconds as seed)
            let variation = ((elapsed.as_millis() % 1000) as f32 / 1000.0) * 5.0 - 2.5;
            base_temp + cpu_influence + variation
        };
        self.cpu_temp_history.temp_samples.push_back(temp);
        if self.cpu_temp_history.temp_samples.len() > self.cpu_temp_history.max_samples {
            self.cpu_temp_history.temp_samples.pop_front();
        }
        
        // Update load average history
        if let Some(load_avg) = self.load_average() {
            self.load_history.one_min_samples.push_back(load_avg.one_min);
            if self.load_history.one_min_samples.len() > self.load_history.max_samples {
                self.load_history.one_min_samples.pop_front();
            }
            
            self.load_history.five_min_samples.push_back(load_avg.five_min);
            if self.load_history.five_min_samples.len() > self.load_history.max_samples {
                self.load_history.five_min_samples.pop_front();
            }
            
            self.load_history.fifteen_min_samples.push_back(load_avg.fifteen_min);
            if self.load_history.fifteen_min_samples.len() > self.load_history.max_samples {
                self.load_history.fifteen_min_samples.pop_front();
            }
        }
        
        // Update memory history
        let used_memory = self.system.used_memory();
        let free_memory = self.system.free_memory();
        
        // Store used memory (in bytes)
        self.memory_history.used_samples.push_back(used_memory as f64);
        if self.memory_history.used_samples.len() > self.memory_history.max_samples {
            self.memory_history.used_samples.pop_front();
        }
        
        // Store free memory (in bytes)
        self.memory_history.free_samples.push_back(free_memory as f64);
        if self.memory_history.free_samples.len() > self.memory_history.max_samples {
            self.memory_history.free_samples.pop_front();
        }
        
        // Update disk history (use primary disk)
        use sysinfo::Disks;
        let mut disks = Disks::new_with_refreshed_list();
        disks.refresh();
        if let Some(disk) = disks.iter().next() {
            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let used_space = total_space.saturating_sub(available_space);
            let disk_percent = if total_space > 0 {
                (used_space as f64 / total_space as f64) * 100.0
            } else {
                0.0
            };
            self.disk_history.usage_samples.push_back(disk_percent);
            if self.disk_history.usage_samples.len() > self.disk_history.max_samples {
                self.disk_history.usage_samples.pop_front();
            }
        }
        
        // Update network stats with rate calculation
        // Use a separate timer (every ~50ms) for stable rate measurements
        // This prevents spurts while still providing frequent updates
        let network_elapsed = now.duration_since(self.last_network_update);
        if network_elapsed.as_secs_f64() >= 0.05 { // 50ms interval
            use sysinfo::Networks;
            let mut networks = Networks::new_with_refreshed_list();
            networks.refresh();
            
            // Find primary interface (highest total traffic)
            let mut primary_interface_name: Option<String> = None;
            let mut max_traffic = 0u64;
            for (interface_name, network) in networks.iter() {
                let total = network.total_received() + network.total_transmitted();
                if total > max_traffic {
                    max_traffic = total;
                    primary_interface_name = Some(interface_name.to_string());
                }
            }
            
            // Use primary interface if found, otherwise use first available
            if primary_interface_name.is_none() {
                for (interface_name, _) in networks.iter() {
                    primary_interface_name = Some(interface_name.to_string());
                    break;
                }
            }
            
            // Track primary interface for rate calculation
            if let Some(ref primary_name) = primary_interface_name {
                // Update primary interface if changed
                if self.primary_interface.as_ref() != Some(primary_name) {
                    self.primary_interface = Some(primary_name.clone());
                    // Reset totals when switching interfaces
                    if let Some((_, network)) = networks.iter().find(|(name, _)| name.as_str() == primary_name.as_str()) {
                        self.prev_network_totals = Some((network.total_received(), network.total_transmitted()));
                        self.smoothed_download_rate = 0.0;
                        self.smoothed_upload_rate = 0.0;
                    }
                } else {
                    // Same interface - calculate rates if we have previous totals
                    if let Some((prev_received, prev_transmitted)) = self.prev_network_totals {
                        if let Some((_, network)) = networks.iter().find(|(name, _)| name.as_str() == primary_name.as_str()) {
                            let received = network.total_received();
                            let transmitted = network.total_transmitted();
                            
                            // Calculate rates over the network update interval (100ms+)
                            let elapsed_secs = network_elapsed.as_secs_f64().max(0.001);
                            let download_rate = (received.saturating_sub(prev_received)) as f64 / elapsed_secs;
                            let upload_rate = (transmitted.saturating_sub(prev_transmitted)) as f64 / elapsed_secs;
                            
                            // Ensure rates are non-negative
                            let download_rate = download_rate.max(0.0);
                            let upload_rate = upload_rate.max(0.0);
                            
                            // Store raw rates in history - IMMUTABLE once stored
                            // Historical data represents actual measured rates at that point in time
                            // Never modify existing history entries, only append new ones
                            self.network_history.download_samples.push_back(download_rate);
                            self.network_history.upload_samples.push_back(upload_rate);
                            
                            // Trim history to max size (only removes oldest entries, never modifies)
                            if self.network_history.download_samples.len() > self.network_history.max_samples {
                                self.network_history.download_samples.pop_front();
                            }
                            if self.network_history.upload_samples.len() > self.network_history.max_samples {
                                self.network_history.upload_samples.pop_front();
                            }
                            
                            // Use exponential moving average ONLY for displayed current rate
                            // This smoothing does NOT affect historical data - history remains raw/immutable
                            let alpha = 0.6; // 60% new value, 40% old - responsive but smooth
                            self.smoothed_download_rate = alpha * download_rate + (1.0 - alpha) * self.smoothed_download_rate;
                            self.smoothed_upload_rate = alpha * upload_rate + (1.0 - alpha) * self.smoothed_upload_rate;
                            
                            // Update peaks
                            if download_rate > self.network_peaks.peak_download {
                                self.network_peaks.peak_download = download_rate;
                            }
                            if upload_rate > self.network_peaks.peak_upload {
                                self.network_peaks.peak_upload = upload_rate;
                            }
                            
                            // Store current totals for next network update
                            self.prev_network_totals = Some((received, transmitted));
                            self.last_network_update = now;
                        } else {
                            // Primary interface disappeared - reset tracking
                            self.prev_network_totals = None;
                            self.smoothed_download_rate = 0.0;
                            self.smoothed_upload_rate = 0.0;
                        }
                    } else {
                        // No previous totals - initialize them
                        if let Some((_, network)) = networks.iter().find(|(name, _)| name.as_str() == primary_name.as_str()) {
                            self.prev_network_totals = Some((network.total_received(), network.total_transmitted()));
                            self.last_network_update = now;
                        }
                    }
                }
            }
        }
        
        // Ping execution disabled - we're using network throughput instead
        // Uncomment below if you want to re-enable ping latency tracking
        /*
        let ping_elapsed = now.duration_since(self.last_ping_time);
        if ping_elapsed.as_secs_f64() >= 0.9 {
            // Ping all servers (sequentially for simplicity)
            for server in &self.ping_servers {
                let ping_result = self.execute_ping(&server.hostname);
                if let Some(latency_ms) = ping_result {
                    if let Some(history) = self.ping_history.region_histories.get_mut(&server.region) {
                        history.latency_samples.push_back(latency_ms);
                        if history.latency_samples.len() > history.max_samples {
                            history.latency_samples.pop_front();
                        }
                    }
                }
            }
            self.last_ping_time = now;
        }
        */
        
        self.last_refresh = now;
    }
    
    /// Attempts to read the actual CPU temperature from the system.
    /// Returns None if temperature cannot be read, allowing fallback to simulated values.
    fn get_cpu_temperature(&self) -> Option<f32> {
        #[cfg(target_os = "linux")]
        {
            self.get_cpu_temperature_linux()
        }
        
        #[cfg(target_os = "macos")]
        {
            self.get_cpu_temperature_macos()
        }
        
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            // Unsupported platform - return None to use simulated temperature
            None
        }
    }
    
    #[cfg(target_os = "linux")]
    fn get_cpu_temperature_linux(&self) -> Option<f32> {
        use std::fs;
        use std::path::Path;
        
        let thermal_base = Path::new("/sys/class/thermal");
        if !thermal_base.exists() {
            return None;
        }
        
        // Find CPU thermal zones
        let mut cpu_temps = Vec::new();
        
        // Iterate through thermal zones
        if let Ok(entries) = fs::read_dir(thermal_base) {
            for entry in entries.flatten() {
                let path = entry.path();
                let zone_name = path.file_name()?.to_string_lossy();
                
                // Check if it's a thermal zone directory (thermal_zone*)
                if !zone_name.starts_with("thermal_zone") {
                    continue;
                }
                
                // Read the type file to check if it's a CPU sensor
                let type_path = path.join("type");
                if let Ok(zone_type) = fs::read_to_string(&type_path) {
                    let zone_type = zone_type.trim().to_lowercase();
                    // Check if this zone represents CPU temperature
                    if zone_type.contains("cpu") || zone_type.contains("processor") || 
                       zone_type.contains("x86_pkg_temp") || zone_type.contains("k10temp") {
                        // Read the temperature file
                        let temp_path = path.join("temp");
                        if let Ok(temp_str) = fs::read_to_string(&temp_path) {
                            if let Ok(temp_millidegrees) = temp_str.trim().parse::<i32>() {
                                // Convert from millidegrees Celsius to degrees Celsius
                                let temp_celsius = temp_millidegrees as f32 / 1000.0;
                                // Sanity check: reasonable CPU temperature range
                                if temp_celsius > -50.0 && temp_celsius < 150.0 {
                                    cpu_temps.push(temp_celsius);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Return average of all CPU temperatures found, or None if none found
        if cpu_temps.is_empty() {
            None
        } else {
            let sum: f32 = cpu_temps.iter().sum();
            Some(sum / cpu_temps.len() as f32)
        }
    }
    
    #[cfg(target_os = "macos")]
    fn get_cpu_temperature_macos(&self) -> Option<f32> {
        // macOS doesn't expose CPU temperature through standard sysctl keys.
        // We can try a few approaches:
        // 1. Try sysctl with common temperature keys (usually not available)
        // 2. Try IOKit (requires additional dependencies)
        // For now, we'll try sysctl and fall back to None (which triggers simulated temp)
        
        // Try sysctl approach - some Macs may have temperature sensors accessible this way
        // Common keys to try (though most Macs don't expose CPU temp via sysctl)
        let sysctl_keys = [
            "machdep.xcpm.cpu_thermal_level",
            "machdep.xcpm.cpu_thermal_pressure",
        ];
        
        for key in &sysctl_keys {
            if let Ok(output) = Command::new("sysctl")
                .arg("-n")
                .arg(key)
                .output()
            {
                if output.status.success() {
                    if let Ok(stdout) = String::from_utf8(output.stdout) {
                        if let Ok(_value) = stdout.trim().parse::<f32>() {
                            // These sysctl values are typically not direct temperatures,
                            // but thermal pressure/levels. Skip for now.
                            // In a real implementation, you'd need IOKit to get actual temps.
                        }
                    }
                }
            }
        }
        
        // macOS doesn't easily expose CPU temperature without IOKit or third-party tools.
        // Return None to use simulated temperature as fallback.
        // To get real temperatures on macOS, you would need:
        // - IOKit bindings (like iokit-sys crate)
        // - Or use a tool like osx-cpu-temp
        // For now, we fall back to simulated temperature.
        None
    }
    
    fn execute_ping(&self, hostname: &str) -> Option<f64> {
        #[cfg(target_os = "macos")]
        {
            let output = Command::new("ping")
                .arg("-c")
                .arg("1")
                .arg("-W")
                .arg("1000")
                .arg(hostname)
                .output()
                .ok()?;
            
            if !output.status.success() {
                return None;
            }
            
            let stdout = String::from_utf8_lossy(&output.stdout);
            self.parse_ping_output(&stdout)
        }
        
        #[cfg(target_os = "linux")]
        {
            let output = Command::new("ping")
                .arg("-c")
                .arg("1")
                .arg("-W")
                .arg("1")
                .arg(hostname)
                .output()
                .ok()?;
            
            if !output.status.success() {
                return None;
            }
            
            let stdout = String::from_utf8_lossy(&output.stdout);
            self.parse_ping_output(&stdout)
        }
        
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            // Fallback for other platforms
            None
        }
    }
    
    fn parse_ping_output(&self, output: &str) -> Option<f64> {
        // Parse ping output to extract latency
        // macOS format: "time=15.234 ms" or "time=15.234ms"
        // Linux format: "time=15.234 ms" or "time=15.234ms"
        
        // Try to find "time=" pattern
        for line in output.lines() {
            if let Some(time_pos) = line.find("time=") {
                let after_time = &line[time_pos + 5..];
                // Extract number (may have decimal point)
                let mut num_str = String::new();
                for ch in after_time.chars() {
                    if ch.is_ascii_digit() || ch == '.' {
                        num_str.push(ch);
                    } else if !num_str.is_empty() {
                        // Stop at first non-numeric character after number
                        break;
                    }
                }
                if let Ok(latency) = num_str.parse::<f64>() {
                    return Some(latency);
                }
            }
            
            // Also try "time " pattern (some ping versions)
            if let Some(time_pos) = line.find("time ") {
                let after_time = &line[time_pos + 5..];
                let mut num_str = String::new();
                for ch in after_time.chars() {
                    if ch.is_ascii_digit() || ch == '.' {
                        num_str.push(ch);
                    } else if !num_str.is_empty() {
                        break;
                    }
                }
                if let Ok(latency) = num_str.parse::<f64>() {
                    return Some(latency);
                }
            }
        }
        
        None
    }

    pub fn cpu_usage(&self) -> f32 {
        self.system.global_cpu_info().cpu_usage()
    }

    #[allow(dead_code)]
    pub fn memory_usage(&self) -> (u64, u64) {
        let total = self.system.total_memory();
        let used = self.system.used_memory();
        (used, total)
    }

    #[allow(dead_code)]
    pub fn disk_usage(&self) -> Option<(u64, u64)> {
        // Get root filesystem usage
        // Note: sysinfo 0.30 API may differ - this is a simplified version
        // For now, return None if disks() is not available
        None
    }

    pub fn username(&self) -> String {
        std::env::var("USER").unwrap_or_else(|_| "user".to_string())
    }

    #[allow(dead_code)]
    pub fn hostname(&self) -> String {
        std::env::var("HOSTNAME")
            .or_else(|_| std::env::var("COMPUTERNAME"))
            .unwrap_or_else(|_| "localhost".to_string())
    }

    #[allow(dead_code)]
    pub fn cpu_cores(&self) -> Vec<f32> {
        self.system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect()
    }

    pub fn memory_detailed(&self) -> MemoryDetails {
        MemoryDetails {
            used: self.system.used_memory(),
            free: self.system.free_memory(),
            total: self.system.total_memory(),
            cached: self.system.used_memory().saturating_sub(self.system.used_memory()),
            buffers: 0, // sysinfo 0.30 doesn't expose buffers directly
            swap_used: self.system.used_swap(),
            swap_total: self.system.total_swap(),
        }
    }

    pub fn network_stats(&mut self) -> Vec<NetworkStats> {
        // sysinfo 0.30: Networks needs to be created separately
        use sysinfo::Networks;
        let mut networks = Networks::new_with_refreshed_list();
        networks.refresh();
        let mut stats = Vec::new();
        
        // Use smoothed rates directly (already calculated with EMA in refresh())
        // This provides continuous, smooth updates without spurts
        let (download_rate, upload_rate) = (
            self.smoothed_download_rate,
            self.smoothed_upload_rate,
        );
        
        // Find primary interface (highest total traffic) - same logic as in refresh()
        let mut primary_interface_name: Option<String> = None;
        let mut max_traffic = 0u64;
        for (interface_name, network) in networks.iter() {
            let total = network.total_received() + network.total_transmitted();
            if total > max_traffic {
                max_traffic = total;
                primary_interface_name = Some(interface_name.to_string());
            }
        }
        
        // Use primary interface if found, otherwise use first available
        if primary_interface_name.is_none() {
            for (interface_name, _) in networks.iter() {
                primary_interface_name = Some(interface_name.to_string());
                break;
            }
        }
        
        // Assign rates only to primary interface, 0.0 for others
        // If no interfaces found, create a dummy entry so UI can display something
        if networks.iter().count() == 0 {
            stats.push(NetworkStats {
                interface: "none".to_string(),
                received: 0,
                transmitted: 0,
                received_per_sec: 0.0,
                transmitted_per_sec: 0.0,
            });
        } else {
            for (interface_name, network) in networks.iter() {
                let is_primary = primary_interface_name.as_ref().map_or(false, |name| name == interface_name.as_str());
                stats.push(NetworkStats {
                    interface: interface_name.to_string(),
                    received: network.total_received(),
                    transmitted: network.total_transmitted(),
                    received_per_sec: if is_primary { download_rate } else { 0.0 },
                    transmitted_per_sec: if is_primary { upload_rate } else { 0.0 },
                });
            }
        }
        
        stats.sort_by(|a, b| (b.received + b.transmitted).cmp(&(a.received + a.transmitted)));
        stats
    }
    
    pub fn network_history(&self) -> (&VecDeque<f64>, &VecDeque<f64>) {
        (&self.network_history.download_samples, &self.network_history.upload_samples)
    }
    
    pub fn cpu_history(&self) -> &VecDeque<f32> {
        &self.cpu_history.cpu_samples
    }
    
    pub fn cpu_temp_history(&self) -> &VecDeque<f32> {
        &self.cpu_temp_history.temp_samples
    }
    
    pub fn cpu_temperature(&self) -> f32 {
        self.cpu_temp_history.temp_samples.back().copied().unwrap_or(35.0)
    }
    
    pub fn memory_history(&self) -> (&VecDeque<f64>, &VecDeque<f64>) {
        (&self.memory_history.used_samples, &self.memory_history.free_samples)
    }
    
    pub fn load_history(&self) -> (&VecDeque<f64>, &VecDeque<f64>, &VecDeque<f64>) {
        (&self.load_history.one_min_samples, 
         &self.load_history.five_min_samples, 
         &self.load_history.fifteen_min_samples)
    }
    
    pub fn cpu_core_count(&self) -> usize {
        self.system.cpus().len()
    }
    
    pub fn ping_history(&self) -> &HashMap<String, RegionPingHistory> {
        &self.ping_history.region_histories
    }
    
    pub fn ping_history_for_region(&self, region: &str) -> Option<&VecDeque<f64>> {
        self.ping_history.region_histories.get(region)
            .map(|h| &h.latency_samples)
    }
    
    pub fn current_ping(&self) -> HashMap<String, Option<f64>> {
        let mut result = HashMap::new();
        for (region, history) in &self.ping_history.region_histories {
            result.insert(region.clone(), history.latency_samples.back().copied());
        }
        result
    }
    
    pub fn get_regions(&self) -> Vec<String> {
        self.ping_servers.iter().map(|s| s.region.clone()).collect()
    }
    
    #[allow(dead_code)]
    pub fn disk_history(&self) -> &VecDeque<f64> {
        &self.disk_history.usage_samples
    }
    
    #[allow(dead_code)]
    pub fn network_peaks(&self) -> (f64, f64) {
        (self.network_peaks.peak_download, self.network_peaks.peak_upload)
    }

    pub fn disk_stats(&self) -> Vec<DiskStats> {
        // sysinfo 0.30: Disks needs to be created separately
        use sysinfo::Disks;
        let mut disks = Disks::new_with_refreshed_list();
        disks.refresh();
        disks.iter().map(|disk| {
            DiskStats {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
                read_bytes: 0, // sysinfo 0.30 doesn't expose read_bytes directly
                write_bytes: 0, // sysinfo 0.30 doesn't expose write_bytes directly
            }
        }).collect()
    }

    #[allow(dead_code)]
    pub fn top_processes(&self, limit: usize) -> Vec<ProcessInfo> {
        let mut processes: Vec<ProcessInfo> = self.system.processes()
            .iter()
            .map(|(pid, process)| {
                ProcessInfo {
                    pid: pid.as_u32(),
                    name: process.name().to_string(),
                    cpu_usage: process.cpu_usage(),
                    memory_usage: process.memory(),
                }
            })
            .collect();
        
        processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));
        processes.truncate(limit);
        processes
    }

    pub fn uptime(&self) -> u64 {
        System::uptime()
    }

    pub fn load_average(&self) -> Option<LoadAverage> {
        let load_avg = System::load_average();
        Some(LoadAverage {
            one_min: load_avg.one,
            five_min: load_avg.five,
            fifteen_min: load_avg.fifteen,
        })
    }
}
