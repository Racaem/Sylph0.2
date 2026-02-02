use std::time::{Duration, Instant};
use std::collections::HashMap;

// 性能分析事件
enum ProfilingEvent {
    Start(Instant),
    End(Instant),
}

// 性能分析统计
trait ProfilingStats {
    fn add_duration(&mut self, duration: Duration);
    fn get_average(&self) -> Duration;
    fn get_min(&self) -> Duration;
    fn get_max(&self) -> Duration;
    fn get_total(&self) -> Duration;
    fn get_count(&self) -> usize;
}

// 简单统计实现
#[derive(Debug)]
struct SimpleStats {
    total: Duration,
    count: usize,
    min: Duration,
    max: Duration,
}

impl SimpleStats {
    fn new() -> Self {
        SimpleStats {
            total: Duration::from_nanos(0),
            count: 0,
            min: Duration::from_secs(u64::MAX),
            max: Duration::from_nanos(0),
        }
    }
}

impl ProfilingStats for SimpleStats {
    fn add_duration(&mut self, duration: Duration) {
        self.total += duration;
        self.count += 1;
        if duration < self.min {
            self.min = duration;
        }
        if duration > self.max {
            self.max = duration;
        }
    }
    
    fn get_average(&self) -> Duration {
        if self.count == 0 {
            Duration::from_nanos(0)
        } else {
            self.total / self.count as u32
        }
    }
    
    fn get_min(&self) -> Duration {
        self.min
    }
    
    fn get_max(&self) -> Duration {
        self.max
    }
    
    fn get_total(&self) -> Duration {
        self.total
    }
    
    fn get_count(&self) -> usize {
        self.count
    }
}

// 详细性能分析器
pub struct Profiler {
    start_times: HashMap<String, Instant>,
    stats: HashMap<String, SimpleStats>,
    events: Vec<(String, ProfilingEvent)>,
    enabled: bool,
}

impl Profiler {
    pub fn new() -> Self {
        Profiler {
            start_times: HashMap::new(),
            stats: HashMap::new(),
            events: Vec::new(),
            enabled: true,
        }
    }
    
    pub fn new_disabled() -> Self {
        Profiler {
            start_times: HashMap::new(),
            stats: HashMap::new(),
            events: Vec::new(),
            enabled: false,
        }
    }

    pub fn start(&mut self, name: &str) {
        if !self.enabled {
            return;
        }
        
        let now = Instant::now();
        self.start_times.insert(name.to_string(), now);
        self.events.push((name.to_string(), ProfilingEvent::Start(now)));
    }

    pub fn end(&mut self, name: &str) {
        if !self.enabled {
            return;
        }
        
        let now = Instant::now();
        if let Some(start_time) = self.start_times.remove(name) {
            let duration = start_time.elapsed();
            self.stats.entry(name.to_string())
                .or_insert_with(SimpleStats::new)
                .add_duration(duration);
            self.events.push((name.to_string(), ProfilingEvent::End(now)));
        }
    }

    pub fn print(&self) {
        if !self.enabled {
            return;
        }
        
        println!("==== Detailed Profiling Results ====");
        
        // 按总时间排序
        let mut items: Vec<_> = self.stats.iter().collect();
        items.sort_by(|a, b| b.1.get_total().cmp(&a.1.get_total()));
        
        for (name, stats) in items {
            println!("{}", name);
            println!("  Total: {:?}", stats.get_total());
            println!("  Average: {:?}", stats.get_average());
            println!("  Min: {:?}", stats.get_min());
            println!("  Max: {:?}", stats.get_max());
            println!("  Count: {}", stats.get_count());
            println!("  Avg/Count: {:?}", stats.get_total() / stats.get_count().max(1) as u32);
            println!();
        }
        
        println!("==== Summary ====");
        let total_time = self.stats.values()
            .map(|s| s.get_total())
            .fold(Duration::from_nanos(0), |a, b| a + b);
        println!("Total profiling time: {:?}", total_time);
        println!("Number of events: {}", self.events.len());
        println!("Number of metrics: {}", self.stats.len());
        println!("================");
    }
    
    // 打印热点分析
    pub fn print_hotspots(&self) {
        if !self.enabled {
            return;
        }
        
        println!("==== Hotspot Analysis ====");
        
        let mut items: Vec<_> = self.stats.iter().collect();
        items.sort_by(|a, b| b.1.get_total().cmp(&a.1.get_total()));
        
        let total_time: Duration = items.iter()
            .map(|(_, stats)| stats.get_total())
            .sum();
        
        for (name, stats) in items.iter().take(10) {
            let percentage = (stats.get_total().as_nanos() as f64 / total_time.as_nanos() as f64) * 100.0;
            println!("{:<30} {:>15?} ({:.2}%)", name, stats.get_total(), percentage);
        }
        
        println!("========================");
    }
    
    // 导出分析数据
    pub fn export_json(&self) -> String {
        if !self.enabled {
            return "{}".to_string();
        }
        
        let mut json = String::from("{");
        json.push_str("\"stats\": {");
        
        let mut first = true;
        for (name, stats) in &self.stats {
            if !first {
                json.push_str(",");
            }
            first = false;
            
            json.push_str(&format!("\"{}\": {{", name));
            json.push_str(&format!("\"total\": {},", stats.get_total().as_nanos()));
            json.push_str(&format!("\"average\": {},", stats.get_average().as_nanos()));
            json.push_str(&format!("\"min\": {},", stats.get_min().as_nanos()));
            json.push_str(&format!("\"max\": {},", stats.get_max().as_nanos()));
            json.push_str(&format!("\"count\": {}", stats.get_count()));
            json.push_str("}");
        }
        
        json.push_str("}, ");
        json.push_str("\"events\": [");
        
        let mut first_event = true;
        for (name, event) in &self.events {
            if !first_event {
                json.push_str(",");
            }
            first_event = false;
            
            match event {
                ProfilingEvent::Start(instant) => {
                    json.push_str(&format!("{{\"name\": \"{}\", \"type\": \"start\", \"time\": {}}}", 
                                         name, instant.elapsed().as_nanos()));
                }
                ProfilingEvent::End(instant) => {
                    json.push_str(&format!("{{\"name\": \"{}\", \"type\": \"end\", \"time\": {}}}", 
                                         name, instant.elapsed().as_nanos()));
                }
            }
        }
        
        json.push_str("]}");
        json
    }
    
    // 启用/禁用分析
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    // 清除数据
    pub fn clear(&mut self) {
        self.start_times.clear();
        self.stats.clear();
        self.events.clear();
    }
}

// 全局默认分析器
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref GLOBAL_PROFILER: Mutex<Profiler> = Mutex::new(Profiler::new_disabled());
}

// 获取全局分析器
pub fn get_global_profiler() -> &'static Mutex<Profiler> {
    &GLOBAL_PROFILER
}

// 便捷分析函数
pub fn profile<F, R>(name: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let mut profiler = get_global_profiler().lock().unwrap();
    profiler.start(name);
    let result = f();
    profiler.end(name);
    result
}

// 启用全局分析器
pub fn enable_profiling() {
    let mut profiler = get_global_profiler().lock().unwrap();
    profiler.set_enabled(true);
}

// 禁用全局分析器
pub fn disable_profiling() {
    let mut profiler = get_global_profiler().lock().unwrap();
    profiler.set_enabled(false);
}

// 打印全局分析结果
pub fn print_profiling_results() {
    let profiler = get_global_profiler().lock().unwrap();
    profiler.print();
    profiler.print_hotspots();
}

// 导出全局分析数据
pub fn export_profiling_json() -> String {
    let profiler = get_global_profiler().lock().unwrap();
    profiler.export_json()
}

// 性能分析作用域
pub struct ProfilingScope {
    name: String,
}

impl ProfilingScope {
    pub fn new(name: &str) -> Self {
        let mut profiler = get_global_profiler().lock().unwrap();
        profiler.start(name);
        ProfilingScope {
            name: name.to_string(),
        }
    }
}

impl Drop for ProfilingScope {
    fn drop(&mut self) {
        let mut profiler = get_global_profiler().lock().unwrap();
        profiler.end(&self.name);
    }
}

// 便捷宏
#[macro_export]
macro_rules! profile_scope {
    ($name:expr, $body:expr) => {
        {
            let _scope = $crate::profiler::ProfilingScope::new($name);
            $body
        }
    };
}

#[macro_export]
macro_rules! profile_fn {
    ($name:expr) => {
        #[inline]
        fn wrapped($($args:tt)*) -> $ret:ty {
            profile_scope!($name, {
                original($($args)*)
            })
        }
        
        original
    };
}
