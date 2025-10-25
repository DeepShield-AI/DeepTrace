#[doc(hidden)]
#[macro_export]
macro_rules! metric_tags {
    () => {
        None
    };
    ($($key:expr => $value:expr,)+) => {
        metric_tags!($($key => $value),+)
    };
    ($($key:expr => $value:expr),* $(,)?) => {
        Some(
            [$( ($key.into(), $crate::metric::String::from($value)), )*]
            .into_iter().collect::<$crate::metric::MetricTags>()
        )
    };
}

/// Gauges represent a single value that can go up or down over time, and always starts out with an
/// initial value of zero.
///
/// Metrics can be registered, which provides a handle to directly update that metric.  For gauges,
/// [`Gauge`](crate::metric::Gauge) is provided which can be incremented, decremented, or set to an absolute value.
///
/// Metric names are shown below using string literals, but they can also be owned `String` values,
/// which includes using macros such as `format!` directly at the callsite. String literals are
/// preferred for performance where possible.
#[macro_export]
macro_rules! gauge {
    ($name:expr, $value:expr, $namespace:expr $(, $key:expr => $val:expr)* $(,)?) => {{
        let tags = $crate::metric_tags!($($key => $val),*);
        let mut gauge = $crate::metric::Gauge::new();
        gauge.set($value);
        $crate::metric::Metric::new(
            $name,
            $crate::metric::MetricValue::Gauge(gauge)
        )
        .with_namespace($namespace.into())
        .with_tags(tags)
    }};

    ($name:expr, $value:expr) => {
        $crate::gauge!($name, $value, ::std::path::Path::new(file!()).file_stem().and_then(|s| s.to_str()).unwrap_or("unknown"))
    };

    ($name:expr, $value:expr, $namespace:expr) => {{
        let mut gauge = $crate::metric::Gauge::new();
        gauge.set($value);
        $crate::metric::Metric::new(
            $name,
            $crate::metric::MetricValue::Gauge(gauge)
        ).with_namespace($namespace.into())
    }};

    ($name:expr, $value:expr $(, $key:expr => $val:expr)* $(,)?) => {
        $crate::gauge!($name, $value, ::std::path::Path::new(file!()).file_stem().and_then(|s| s.to_str()).unwrap_or("unknown") $(, $key => $val)*)
    };
}
