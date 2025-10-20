use super::{
	ObservConfig,
	agent::AgentConfig,
	config,
	ebpf::EbpfConfig,
	metric::MetricConfig,
	sender::{ElasticSenderConfig, FileSenderConfig, SenderConfig},
	trace::{SpanConfig, TraceConfig},
};
use arc_swap::{
	ArcSwap,
	access::{Access as _, Map},
};
use rustc_hash::FxHashMap;
use std::sync::Arc;

type Access<C> = Map<Arc<ArcSwap<ObservConfig>>, ObservConfig, fn(&ObservConfig) -> &C>;

type AgentAccess = Access<AgentConfig>;
type SenderAccess = Access<SenderConfig>;
type FileSenderAccess = Access<FxHashMap<String, FileSenderConfig>>;
type ElasticAccess = Access<FxHashMap<String, ElasticSenderConfig>>;
type EbpfAccess = Access<FxHashMap<String, EbpfConfig>>;
type MetricAccess = Access<MetricConfig>;
pub type SpanAccess = Access<SpanConfig>;
pub type TraceAccess = Access<TraceConfig>;

pub fn agent_config() -> AgentAccess {
	Map::new(config(), |config: &ObservConfig| -> &AgentConfig { &config.agent })
}

pub fn sender_config() -> SenderAccess {
	Map::new(config(), |config: &ObservConfig| -> &SenderConfig {
		config.sender.as_ref().expect("sender config not found")
	})
}

fn file_sender_map() -> FileSenderAccess {
	Map::new(config(), |config: &ObservConfig| -> &FxHashMap<String, FileSenderConfig> {
		config
			.sender
			.as_ref()
			.expect("file sender config not found")
			.file
			.as_ref()
			.expect("file sender config not found")
	})
}

// TODO: handle the case where the key does not exist
pub fn file_sender_config(key: &str) -> FileSenderConfig {
	file_sender_map().load().get(key).expect("file sender config not found").clone()
}

fn elastic_sender_map() -> ElasticAccess {
	Map::new(config(), |config: &ObservConfig| -> &FxHashMap<String, ElasticSenderConfig> {
		config
			.sender
			.as_ref()
			.expect("elastic sender config not found")
			.elastic
			.as_ref()
			.expect("elastic sender config not found")
	})
}

pub fn elastic_sender_config(key: &str) -> ElasticSenderConfig {
	elastic_sender_map()
		.load()
		.get(key)
		.expect("elastic sender config not found")
		.clone()
}

fn ebpf_config_map() -> EbpfAccess {
	Map::new(config(), |config: &ObservConfig| -> &FxHashMap<String, EbpfConfig> {
		config.ebpf.as_ref().expect("ebpf config not found")
	})
}

pub fn ebpf_config(key: &str) -> EbpfConfig {
	ebpf_config_map().load().get(key).expect("ebpf config not found").clone()
}

pub fn metric_config() -> MetricAccess {
	Map::new(config(), |config: &ObservConfig| -> &MetricConfig {
		config.metric.as_ref().expect("metric config not found")
	})
}

pub fn span_config() -> SpanAccess {
	Map::new(config(), |config: &ObservConfig| -> &SpanConfig {
		&config.trace.as_ref().expect("span config not found").span
	})
}

pub fn trace_config() -> TraceAccess {
	Map::new(config(), |config: &ObservConfig| -> &TraceConfig {
		config.trace.as_ref().expect("trace config not found")
	})
}
