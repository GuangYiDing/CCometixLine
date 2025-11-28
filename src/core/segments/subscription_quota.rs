use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct QuotaCache {
    percentage: f64,
    secondary_text: Option<String>,
    cached_at: String,
}

#[derive(Default)]
pub struct SubscriptionQuotaSegment;

impl SubscriptionQuotaSegment {
    pub fn new() -> Self {
        Self
    }

    fn get_circle_icon(percentage: f64) -> String {
        let percent = percentage.round() as u8;
        match percent {
            0..=12 => "\u{f0a9e}".to_string(),  // circle_slice_1
            13..=25 => "\u{f0a9f}".to_string(), // circle_slice_2
            26..=37 => "\u{f0aa0}".to_string(), // circle_slice_3
            38..=50 => "\u{f0aa1}".to_string(), // circle_slice_4
            51..=62 => "\u{f0aa2}".to_string(), // circle_slice_5
            63..=75 => "\u{f0aa3}".to_string(), // circle_slice_6
            76..=87 => "\u{f0aa4}".to_string(), // circle_slice_7
            _ => "\u{f0aa5}".to_string(),       // circle_slice_8
        }
    }

    fn get_cache_path() -> Option<std::path::PathBuf> {
        let home = dirs::home_dir()?;
        Some(
            home.join(".claude")
                .join("ccline")
                .join(".subscription_quota_cache.json"),
        )
    }

    fn load_cache(&self) -> Option<QuotaCache> {
        let cache_path = Self::get_cache_path()?;
        if !cache_path.exists() {
            return None;
        }

        let content = std::fs::read_to_string(&cache_path).ok()?;
        serde_json::from_str(&content).ok()
    }

    fn save_cache(&self, cache: &QuotaCache) {
        if let Some(cache_path) = Self::get_cache_path() {
            if let Some(parent) = cache_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(json) = serde_json::to_string_pretty(cache) {
                let _ = std::fs::write(&cache_path, json);
            }
        }
    }

    fn is_cache_valid(&self, cache: &QuotaCache, cache_duration: u64) -> bool {
        if let Ok(cached_at) = DateTime::parse_from_rfc3339(&cache.cached_at) {
            let now = Utc::now();
            let elapsed = now.signed_duration_since(cached_at.with_timezone(&Utc));
            elapsed.num_seconds() < cache_duration as i64
        } else {
            false
        }
    }

    fn execute_script(script_path: &str, timeout_secs: u64) -> Result<String, String> {
        use std::sync::{Arc, Mutex};
        use std::sync::atomic::{AtomicBool, Ordering};

        let script_path_owned = script_path.to_owned();
        let result = Arc::new(Mutex::new(None));
        let result_clone = Arc::clone(&result);
        let finished = Arc::new(AtomicBool::new(false));
        let finished_clone = Arc::clone(&finished);

        // Spawn a thread to execute the script
        let handle = std::thread::spawn(move || {
            let output = Command::new("sh")
                .args(["-c", &script_path_owned])
                .output()
                .map_err(|e| format!("Failed to execute script: {}", e));

            *result_clone.lock().unwrap() = Some(output);
            finished_clone.store(true, Ordering::Relaxed);
        });

        // Wait for either completion or timeout
        let start = std::time::Instant::now();
        while start.elapsed() < std::time::Duration::from_secs(timeout_secs) {
            if finished.load(Ordering::Relaxed) {
                // Join the thread to ensure it completes
                let _ = handle.join();
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        // Check if finished
        if finished.load(Ordering::Relaxed) {
            if let Some(Ok(output)) = result.lock().unwrap().as_ref() {
                if !output.status.success() {
                    return Err(format!(
                        "Script exited with status {}",
                        output.status
                    ));
                }
                let stdout = String::from_utf8_lossy(&output.stdout);
                return Ok(stdout.trim().to_string());
            } else if let Some(Err(e)) = result.lock().unwrap().as_ref() {
                return Err(e.clone());
            }
        }

        // Timeout occurred
        Err(format!("Script execution timed out after {} seconds", timeout_secs))
    }

    fn parse_output(output: &str, output_format: &str, _parse_rules: Option<&str>) -> Result<(f64, Option<String>), String> {
        match output_format {
            "text" => {
                // For text format, return the output directly as primary text
                // Let the script output be displayed as-is without any parsing
                let percentage = 0.0; // Use 0% as default since we're not parsing percentage
                Ok((percentage, Some(output.to_string())))
            }
            "json" => {
                let json_value: serde_json::Value = serde_json::from_str(output)
                    .map_err(|_| "Failed to parse JSON output".to_string())?;

                let percentage = json_value
                    .get("percentage")
                    .and_then(|v| v.as_f64())
                    .ok_or("No 'percentage' field in JSON".to_string())?;

                if percentage < 0.0 || percentage > 100.0 {
                    return Err("Percentage out of range (0-100)".to_string());
                }

                // Extract optional secondary text fields
                let secondary = json_value
                    .get("remaining")
                    .or_else(|| json_value.get("secondary"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                Ok((percentage, secondary))
            }
            "key_value" => {
                // Parse key=value pairs
                for line in output.lines() {
                    let parts: Vec<&str> = line.split('=').collect();
                    if parts.len() == 2 && parts[0].trim() == "percentage" {
                        let percentage = parts[1]
                            .trim()
                            .parse::<f64>()
                            .map_err(|_| "Failed to parse percentage value".to_string())?;

                        if percentage < 0.0 || percentage > 100.0 {
                            return Err("Percentage out of range (0-100)".to_string());
                        }

                        return Ok((percentage, None));
                    }
                }

                Err("No 'percentage' key found in key-value output".to_string())
            }
            _ => Err(format!("Unknown output format: {}", output_format)),
        }
    }
}

impl Segment for SubscriptionQuotaSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        // Load config from file to get segment options
        let config = crate::config::Config::load().ok()?;
        let segment_config = config.segments.iter().find(|s| s.id == SegmentId::SubscriptionQuota)?;

        // Get configuration options with defaults
        let script_path = segment_config
            .options
            .get("custom_script_path")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if script_path.is_empty() {
            eprintln!("SubscriptionQuota: custom_script_path is not configured");
            return None;
        }

        let output_format = segment_config
            .options
            .get("output_format")
            .and_then(|v| v.as_str())
            .unwrap_or("text");

        let cache_duration = segment_config
            .options
            .get("cache_duration")
            .and_then(|v| v.as_u64())
            .unwrap_or(300);

        let timeout = segment_config
            .options
            .get("timeout")
            .and_then(|v| v.as_u64())
            .unwrap_or(5);

        let parse_rules = segment_config
            .options
            .get("parse_rules")
            .and_then(|v| v.as_str());

        // Check cache first
        let cached_data = self.load_cache();
        let use_cached = cached_data
            .as_ref()
            .map(|cache| self.is_cache_valid(cache, cache_duration))
            .unwrap_or(false);

        let (percentage, secondary_text, script_output) = if use_cached {
            let cache = cached_data.unwrap();
            (cache.percentage, cache.secondary_text, None)
        } else {
            // Execute script and parse output
            match Self::execute_script(script_path, timeout) {
                Ok(output) => {
                    match Self::parse_output(&output, output_format, parse_rules) {
                        Ok((percentage, secondary)) => {
                            // Save to cache
                            let cache = QuotaCache {
                                percentage,
                                secondary_text: secondary.clone(),
                                cached_at: Utc::now().to_rfc3339(),
                            };
                            self.save_cache(&cache);
                            (percentage, secondary, Some(output))
                        }
                        Err(e) => {
                            eprintln!("Failed to parse script output: {}", e);
                            if let Some(cache) = cached_data {
                                (cache.percentage, cache.secondary_text, None)
                            } else {
                                return None;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Script execution failed: {}", e);
                    if let Some(cache) = cached_data {
                        (cache.percentage, cache.secondary_text, None)
                    } else {
                        return None;
                    }
                }
            }
        };

        // Format output based on output_format
        let (primary, secondary, dynamic_icon);

        if output_format == "text" {
            // For text format, display the script output directly as primary
            // Cache stores secondary_text, but we want the raw output
            primary = script_output.unwrap_or_else(|| {
                secondary_text.clone().unwrap_or_else(|| "0%".to_string())
            });
            secondary = None;
            dynamic_icon = String::new(); // No icon for custom text format
        } else {
            // For JSON and key_value formats, use percentage-based display
            dynamic_icon = Self::get_circle_icon(percentage);
            primary = format!("{:.0}%", percentage.round());
            secondary = secondary_text.map(|s| format!("· {}", s));
        }

        let mut metadata = HashMap::new();
        if !dynamic_icon.is_empty() {
            metadata.insert("dynamic_icon".to_string(), dynamic_icon);
        }
        metadata.insert("percentage".to_string(), percentage.to_string());

        Some(SegmentData {
            primary,
            secondary: secondary.unwrap_or_default(),
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::SubscriptionQuota
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeout_fast_script() {
        let fast_script = "echo '快速完成'; sleep 1; echo '完成'";
        let result = SubscriptionQuotaSegment::execute_script(fast_script, 3);
        assert!(result.is_ok(), "快速脚本应该成功执行");
        let output = result.unwrap();
        assert!(output.contains("完成"), "输出应包含 '完成'");
    }

    #[test]
    fn test_timeout_slow_script() {
        let slow_script = "echo '开始'; sleep 10; echo '结束'";
        let result = SubscriptionQuotaSegment::execute_script(slow_script, 2);
        assert!(result.is_err(), "慢速脚本应该超时");
        let error = result.unwrap_err();
        assert!(error.contains("timed out"), "错误信息应包含 'timed out'");
        assert!(error.contains("2 seconds"), "错误信息应包含超时时间");
    }
}
