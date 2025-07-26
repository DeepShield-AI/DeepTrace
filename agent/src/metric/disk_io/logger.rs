use crate::metric::disk_io::{DiskMetric, DiskUsage, Ext4CacheStats};
use log::{error, info};
use std::{
	fs::{File, OpenOptions},
	io::{BufWriter, Write},
	path::PathBuf,
	sync::{Arc, Mutex},
};

pub struct DiskLogger {
	disk_metrics_file: BufWriter<File>,
	disk_usages_file: BufWriter<File>,
	ext4_cache_file: BufWriter<File>,
}

impl DiskLogger {
	pub fn new(base_path: &str) -> Result<Self, std::io::Error> {
		let path = PathBuf::from(base_path);

		// 创建目录如果不存在
		std::fs::create_dir_all(&path)?;

		// 打开或创建 disk_metrics.csv 并写入注释
		let mut disk_metrics_file = BufWriter::new(File::create(path.join("disk_metrics.csv"))?);
		writeln!(disk_metrics_file, "# timestamp: 时间戳 (毫秒)")?;
		writeln!(disk_metrics_file, "# device: 设备名称")?;
		writeln!(disk_metrics_file, "# read_completed: 读取完成次数")?;
		writeln!(disk_metrics_file, "# read_merged: 读取合并次数")?;
		writeln!(disk_metrics_file, "# sectors_read: 读取的扇区数")?;
		writeln!(disk_metrics_file, "# time_spent_read: 读取耗时 (毫秒)")?;
		writeln!(disk_metrics_file, "# write_completed: 写入完成次数")?;
		writeln!(disk_metrics_file, "# write_merged: 写入合并次数")?;
		writeln!(disk_metrics_file, "# sectors_written: 写入的扇区数")?;
		writeln!(disk_metrics_file, "# time_spent_writing: 写入耗时 (毫秒)")?;
		writeln!(disk_metrics_file, "# io_in_progress: 正在进行的 I/O 操作数")?;
		writeln!(disk_metrics_file, "# time_spent_io: I/O 操作总耗时 (毫秒)")?;
		writeln!(disk_metrics_file, "# weighted_time_spent_io: 加权 I/O 操作耗时 (毫秒)")?;
		writeln!(disk_metrics_file, "# aqu_sz: 平均I/O队列长度")?;
		writeln!(disk_metrics_file, "# await_time: I/O 操作平均等待时间 (毫秒)")?;
		writeln!(disk_metrics_file, "# svctm_time: I/O操作平均服务时间 (毫秒)")?;
		writeln!(
			disk_metrics_file,
			"timestamp,device,read_completed,read_merged,sectors_read,time_spent_read,write_completed,write_merged,sectors_written,time_spent_writing,io_in_progress,time_spent_io,weighted_time_spent_io,aqu_sz,await_time,svctm_time"
		)?;

		// 打开或创建 disk_usages.csv 并写入注释
		let mut disk_usages_file = BufWriter::new(File::create(path.join("disk_usages.csv"))?);
		writeln!(disk_usages_file, "# timestamp: 时间戳 (毫秒)")?;
		writeln!(disk_usages_file, "# filesystem: 文件系统名称")?;
		writeln!(disk_usages_file, "# size: 总大小 (字节)")?;
		writeln!(disk_usages_file, "# used: 已使用大小 (字节)")?;
		writeln!(disk_usages_file, "# available: 可用大小 (字节)")?;
		writeln!(disk_usages_file, "# use_percent: 使用百分比")?;
		writeln!(disk_usages_file, "timestamp,filesystem,size,used,available,use_percent")?;

		// 打开或创建 ext4_cache.csv 并写入注释
		let mut ext4_cache_file = BufWriter::new(File::create(path.join("ext4_cache.csv"))?);
		writeln!(ext4_cache_file, "# timestamp: 时间戳 (毫秒)")?;
		writeln!(ext4_cache_file, "# hits: 缓存命中次数")?;
		writeln!(ext4_cache_file, "# misses: 缓存未命中次数")?;
		writeln!(ext4_cache_file, "timestamp,hits,misses")?;

		Ok(Self { disk_metrics_file, disk_usages_file, ext4_cache_file })
	}

	pub fn write_metrics(&mut self, metrics: &[DiskMetric]) {
		for metric in metrics {
			if let Err(e) = writeln!(
				self.disk_metrics_file,
				"{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
				chrono::Local::now().timestamp_millis(),
				metric.device,
				metric.read_completed,
				metric.read_merged,
				metric.sectors_read,
				metric.time_spent_read,
				metric.write_completed,
				metric.write_merged,
				metric.sectors_written,
				metric.time_spent_writing,
				metric.io_in_progress,
				metric.time_spent_io,
				metric.weighted_time_spent_io,
				metric.aqu_sz,
				metric.await_time,
				metric.svctm_time
			) {
				error!("Failed to write disk metrics to CSV: {}", e);
			}
		}
	}

	pub fn write_usages(&mut self, usages: &[DiskUsage]) {
		for usage in usages {
			if let Err(e) = writeln!(
				self.disk_usages_file,
				"{},{},{},{},{},{}",
				chrono::Local::now().timestamp_millis(),
				usage.filesystem,
				usage.size,
				usage.used,
				usage.available,
				usage.use_percent
			) {
				error!("Failed to write disk usages to CSV: {}", e);
			}
		}
	}

	pub fn write_ext4_cache(&mut self, stats: &Ext4CacheStats) {
		if let Err(e) = writeln!(
			self.ext4_cache_file,
			"{},{},{}",
			chrono::Local::now().timestamp_millis(),
			stats.hits,
			stats.misses
		) {
			error!("Failed to write ext4 cache stats to CSV: {}", e);
		}
	}

	pub fn flush(&mut self) {
		if let Err(e) = self.disk_metrics_file.flush() {
			error!("Failed to flush disk metrics CSV: {}", e);
		}
		if let Err(e) = self.disk_usages_file.flush() {
			error!("Failed to flush disk usages CSV: {}", e);
		}
		if let Err(e) = self.ext4_cache_file.flush() {
			error!("Failed to flush ext4 cache CSV: {}", e);
		}
	}
}
