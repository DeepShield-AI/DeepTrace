use super::TraceError;
use aya::{
	Ebpf,
	maps::{HashMap, MapData},
};

pub(super) fn config_pids(ebpf: &mut Ebpf, pids: Vec<u32>) -> Result<(), TraceError> {
	let mut pids_map: HashMap<&mut MapData, u32, u32> =
		HashMap::try_from(ebpf.map_mut("pids").unwrap())?;

	for pid in pids {
		pids_map.insert(pid, 0, 0)?;
	}
	Ok(())
}
