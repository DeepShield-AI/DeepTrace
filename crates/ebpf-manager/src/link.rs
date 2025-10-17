use crate::{EbpfError, Result};
use aya::programs::{
	kprobe::KProbeLinkId, perf_event::PerfEventLinkId, trace_point::TracePointLinkId,
};

pub enum LinkId {
	KProbe(KProbeLinkId),
	Tracepoint(TracePointLinkId),
	PerfEvent(PerfEventLinkId),
}

impl TryFrom<LinkId> for KProbeLinkId {
	type Error = EbpfError;
	fn try_from(value: LinkId) -> Result<Self> {
		match value {
			LinkId::KProbe(l) => Ok(l),
			_ => Err(EbpfError::WrongLinkId),
		}
	}
}

impl TryFrom<LinkId> for TracePointLinkId {
	type Error = EbpfError;
	fn try_from(value: LinkId) -> Result<Self> {
		match value {
			LinkId::Tracepoint(l) => Ok(l),
			_ => Err(EbpfError::WrongLinkId),
		}
	}
}

impl TryFrom<LinkId> for PerfEventLinkId {
	type Error = EbpfError;
	fn try_from(value: LinkId) -> Result<Self> {
		match value {
			LinkId::PerfEvent(l) => Ok(l),
			_ => Err(EbpfError::WrongLinkId),
		}
	}
}
