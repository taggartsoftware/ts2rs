// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "perf_hooks")]
extern "C" {
    pub type PerformanceEntry;
    #[doc = "The total number of milliseconds elapsed for this entry."]
    #[doc = "This value will not be meaningful for all Performance Entry types."]
    #[wasm_bindgen(method, getter)]
    pub fn duration(this: &PerformanceEntry) -> f64;
    #[doc = "The name of the performance entry."]
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &PerformanceEntry) -> String;
    #[doc = "The high resolution millisecond timestamp marking the starting time of the Performance Entry."]
    # [ wasm_bindgen ( method , getter , js_name = startTime ) ]
    pub fn start_time(this: &PerformanceEntry) -> f64;
    #[doc = "The type of the performance entry."]
    #[doc = "Currently it may be one of: 'node', 'mark', 'measure', 'gc', or 'function'."]
    # [ wasm_bindgen ( method , getter , js_name = entryType ) ]
    pub fn entry_type(this: &PerformanceEntry) -> String;
    #[doc = "When performanceEntry.entryType is equal to 'gc', the performance.kind property identifies"]
    #[doc = "the type of garbage collection operation that occurred."]
    #[doc = "The value may be one of perf_hooks.constants."]
    #[wasm_bindgen(method, getter)]
    pub fn kind(this: &PerformanceEntry) -> i32;
    pub type PerformanceNodeTiming;
    #[doc = "The high resolution millisecond timestamp at which the Node.js process completed bootstrap."]
    # [ wasm_bindgen ( method , getter , js_name = bootstrapComplete ) ]
    pub fn bootstrap_complete(this: &PerformanceNodeTiming) -> f64;
    #[doc = "The high resolution millisecond timestamp at which cluster processing ended."]
    # [ wasm_bindgen ( method , getter , js_name = clusterSetupEnd ) ]
    pub fn cluster_setup_end(this: &PerformanceNodeTiming) -> f64;
    #[doc = "The high resolution millisecond timestamp at which cluster processing started."]
    # [ wasm_bindgen ( method , getter , js_name = clusterSetupStart ) ]
    pub fn cluster_setup_start(this: &PerformanceNodeTiming) -> f64;
    #[doc = "The high resolution millisecond timestamp at which the Node.js event loop exited."]
    # [ wasm_bindgen ( method , getter , js_name = loopExit ) ]
    pub fn loop_exit(this: &PerformanceNodeTiming) -> f64;
    #[doc = "The high resolution millisecond timestamp at which the Node.js event loop started."]
    # [ wasm_bindgen ( method , getter , js_name = loopStart ) ]
    pub fn loop_start(this: &PerformanceNodeTiming) -> f64;
    #[doc = "The high resolution millisecond timestamp at which main module load ended."]
    # [ wasm_bindgen ( method , getter , js_name = moduleLoadEnd ) ]
    pub fn module_load_end(this: &PerformanceNodeTiming) -> f64;
    #[doc = "The high resolution millisecond timestamp at which main module load started."]
    # [ wasm_bindgen ( method , getter , js_name = moduleLoadStart ) ]
    pub fn module_load_start(this: &PerformanceNodeTiming) -> f64;
    #[doc = "The high resolution millisecond timestamp at which the Node.js process was initialized."]
    # [ wasm_bindgen ( method , getter , js_name = nodeStart ) ]
    pub fn node_start(this: &PerformanceNodeTiming) -> f64;
    #[doc = "The high resolution millisecond timestamp at which preload module load ended."]
    # [ wasm_bindgen ( method , getter , js_name = preloadModuleLoadEnd ) ]
    pub fn preload_module_load_end(this: &PerformanceNodeTiming) -> f64;
    #[doc = "The high resolution millisecond timestamp at which preload module load started."]
    # [ wasm_bindgen ( method , getter , js_name = preloadModuleLoadStart ) ]
    pub fn preload_module_load_start(this: &PerformanceNodeTiming) -> f64;
    #[doc = "The high resolution millisecond timestamp at which third_party_main processing ended."]
    # [ wasm_bindgen ( method , getter , js_name = thirdPartyMainEnd ) ]
    pub fn third_party_main_end(this: &PerformanceNodeTiming) -> f64;
    #[doc = "The high resolution millisecond timestamp at which third_party_main processing started."]
    # [ wasm_bindgen ( method , getter , js_name = thirdPartyMainStart ) ]
    pub fn third_party_main_start(this: &PerformanceNodeTiming) -> f64;
    #[doc = "The high resolution millisecond timestamp at which the V8 platform was initialized."]
    # [ wasm_bindgen ( method , getter , js_name = v8Start ) ]
    pub fn v8_start(this: &PerformanceNodeTiming) -> f64;
    pub type Performance;
    # [ wasm_bindgen ( method , js_name = clearFunctions ) ]
    pub fn clear_functions(this: &Performance, name: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = clearFunctions ) ]
    pub fn set_clear_functions(this: &Performance, value: &Function);
    # [ wasm_bindgen ( method , js_name = clearMarks ) ]
    pub fn clear_marks(this: &Performance, name: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = clearMarks ) ]
    pub fn set_clear_marks(this: &Performance, value: &Function);
    # [ wasm_bindgen ( method , js_name = clearMeasures ) ]
    pub fn clear_measures(this: &Performance, name: Option<&str>);
    # [ wasm_bindgen ( method , setter , js_name = clearMeasures ) ]
    pub fn set_clear_measures(this: &Performance, value: &Function);
    # [ wasm_bindgen ( method , js_name = getEntries ) ]
    pub fn get_entries(this: &Performance) -> Array;
    # [ wasm_bindgen ( method , setter , js_name = getEntries ) ]
    pub fn set_get_entries(this: &Performance, value: &Function);
    # [ wasm_bindgen ( method , js_name = getEntriesByName ) ]
    pub fn get_entries_by_name(this: &Performance, name: &str, type_: Option<&str>) -> Array;
    # [ wasm_bindgen ( method , setter , js_name = getEntriesByName ) ]
    pub fn set_get_entries_by_name(this: &Performance, value: &Function);
    # [ wasm_bindgen ( method , js_name = getEntriesByType ) ]
    pub fn get_entries_by_type(this: &Performance, type_: &str) -> Array;
    # [ wasm_bindgen ( method , setter , js_name = getEntriesByType ) ]
    pub fn set_get_entries_by_type(this: &Performance, value: &Function);
    #[wasm_bindgen(method)]
    pub fn mark(this: &Performance, name: Option<&str>);
    #[wasm_bindgen(method, setter)]
    pub fn set_mark(this: &Performance, value: &Function);
    #[wasm_bindgen(method)]
    pub fn measure(this: &Performance, name: &str, start_mark: &str, end_mark: &str);
    #[wasm_bindgen(method, setter)]
    pub fn set_measure(this: &Performance, value: &Function);
    #[doc = "An instance of the PerformanceNodeTiming class that provides performance metrics for specific Node.js operational milestones."]
    # [ wasm_bindgen ( method , getter , js_name = nodeTiming ) ]
    pub fn node_timing(this: &Performance) -> PerformanceNodeTiming;
    #[wasm_bindgen(method)]
    pub fn now(this: &Performance) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_now(this: &Performance, value: &Function);
    #[doc = "The timeOrigin specifies the high resolution millisecond timestamp from which all performance metric durations are measured."]
    # [ wasm_bindgen ( method , getter , js_name = timeOrigin ) ]
    pub fn time_origin(this: &Performance) -> f64;
    #[wasm_bindgen(method)]
    pub fn timerify(this: &Performance, fn_: &JsValue) -> JsValue;
    #[wasm_bindgen(method, setter)]
    pub fn set_timerify(this: &Performance, value: &Function);
    pub type PerformanceObserverEntryList;
    # [ wasm_bindgen ( method , js_name = getEntries ) ]
    pub fn get_entries(this: &PerformanceObserverEntryList) -> Array;
    # [ wasm_bindgen ( method , setter , js_name = getEntries ) ]
    pub fn set_get_entries(this: &PerformanceObserverEntryList, value: &Function);
    # [ wasm_bindgen ( method , js_name = getEntriesByName ) ]
    pub fn get_entries_by_name(
        this: &PerformanceObserverEntryList,
        name: &str,
        type_: Option<&str>,
    ) -> Array;
    # [ wasm_bindgen ( method , setter , js_name = getEntriesByName ) ]
    pub fn set_get_entries_by_name(this: &PerformanceObserverEntryList, value: &Function);
    # [ wasm_bindgen ( method , js_name = getEntriesByType ) ]
    pub fn get_entries_by_type(this: &PerformanceObserverEntryList, type_: &str) -> Array;
    # [ wasm_bindgen ( method , setter , js_name = getEntriesByType ) ]
    pub fn set_get_entries_by_type(this: &PerformanceObserverEntryList, value: &Function);
    pub type PerformanceObserverCallback;
    pub type PerformanceObserver;
    #[wasm_bindgen(constructor)]
    pub fn new_performance_observer(callback: &PerformanceObserverCallback) -> PerformanceObserver;
    #[wasm_bindgen(method)]
    pub fn disconnect(this: &PerformanceObserver);
    #[wasm_bindgen(method, setter)]
    pub fn set_disconnect(this: &PerformanceObserver, value: &Function);
    #[wasm_bindgen(method)]
    pub fn observe(this: &PerformanceObserver, options: &JsValue);
    #[wasm_bindgen(method, setter)]
    pub fn set_observe(this: &PerformanceObserver, value: &Function);
    #[wasm_bindgen(js_name = "performance")]
    pub static PERFORMANCE: Performance;
    pub type EventLoopMonitorOptions;
    #[doc = "The sampling rate in milliseconds."]
    #[doc = "Must be greater than zero."]
    #[wasm_bindgen(method, getter)]
    pub fn resolution(this: &EventLoopMonitorOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_resolution(this: &EventLoopMonitorOptions, value: Option<f64>);
    pub type EventLoopDelayMonitor;
    #[wasm_bindgen(method)]
    pub fn enable(this: &EventLoopDelayMonitor) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_enable(this: &EventLoopDelayMonitor, value: &Function);
    #[wasm_bindgen(method)]
    pub fn disable(this: &EventLoopDelayMonitor) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_disable(this: &EventLoopDelayMonitor, value: &Function);
    #[wasm_bindgen(method)]
    pub fn reset(this: &EventLoopDelayMonitor);
    #[wasm_bindgen(method, setter)]
    pub fn set_reset(this: &EventLoopDelayMonitor, value: &Function);
    #[wasm_bindgen(method)]
    pub fn percentile(this: &EventLoopDelayMonitor, percentile: f64) -> f64;
    #[wasm_bindgen(method, setter)]
    pub fn set_percentile(this: &EventLoopDelayMonitor, value: &Function);
    #[doc = "A `Map` object detailing the accumulated percentile distribution."]
    #[wasm_bindgen(method, getter)]
    pub fn percentiles(this: &EventLoopDelayMonitor) -> Map;
    #[doc = "The number of times the event loop delay exceeded the maximum 1 hour eventloop delay threshold."]
    #[wasm_bindgen(method, getter)]
    pub fn exceeds(this: &EventLoopDelayMonitor) -> f64;
    #[doc = "The minimum recorded event loop delay."]
    #[wasm_bindgen(method, getter)]
    pub fn min(this: &EventLoopDelayMonitor) -> f64;
    #[doc = "The maximum recorded event loop delay."]
    #[wasm_bindgen(method, getter)]
    pub fn max(this: &EventLoopDelayMonitor) -> f64;
    #[doc = "The mean of the recorded event loop delays."]
    #[wasm_bindgen(method, getter)]
    pub fn mean(this: &EventLoopDelayMonitor) -> f64;
    #[doc = "The standard deviation of the recorded event loop delays."]
    #[wasm_bindgen(method, getter)]
    pub fn stddev(this: &EventLoopDelayMonitor) -> f64;
    # [ wasm_bindgen ( js_name = monitorEventLoopDelay ) ]
    pub fn monitor_event_loop_delay(
        options: Option<&EventLoopMonitorOptions>,
    ) -> EventLoopDelayMonitor;
}
