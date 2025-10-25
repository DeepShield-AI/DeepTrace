// use std::str::FromStr;

// pub trait FromValueUnit: Sized {
// 	fn from_value_unit(value: u64, unit: Option<&str>) -> Self;
// }

// #[derive(Debug, Clone)]
// pub struct SimpleQuantity {
// 	pub bytes: f64,
// 	pub unit_raw: Option<String>,
// }

// impl FromValueUnit for SimpleQuantity {
// 	fn from_value_unit(value: u64, unit: Option<&str>) -> Self {
// 		let bytes = match unit.map(|s| s.trim().to_ascii_lowercase()) {
// 			Some(u) if u == "kb" => (value as f64) * 1024.0,
// 			Some(u) if u == "b" => value as f64,
// 			Some(u) if u.ends_with("mb") => (value as f64) * 1024.0 * 1024.0,
// 			Some(_) => value as f64,
// 			None => value as f64,
// 		};
// 		SimpleQuantity { bytes, unit_raw: unit.map(|s| s.to_string()) }
// 	}
// }

// use uom::si::{f64::Information, information::byte};
// impl FromValueUnit for Information {
// 	fn from_value_unit(value: u64, unit: Option<&str>) -> Self {
// 		match unit.map(|s| s.trim()) {
// 			Some(u) if u.eq_ignore_ascii_case("kB") =>
// 				Information::new::<byte>((value as f64) * 1024.0),
// 			Some(u) if u.eq_ignore_ascii_case("B") => Information::new::<byte>(value as f64),
// 			_ => Information::new::<byte>(value as f64),
// 		}
// 	}
// }

// #[derive(Debug, Default, Clone)]
// pub struct Parsed<Q: FromValueUnit> {
// 	pub num: Option<Q>,       // 如果是数字就放这里（Q 类型）
// 	pub text: Option<String>, // 否则放原始字符串
// }

// impl<Q: FromValueUnit> Parsed<Q> {
// 	pub fn set_num(&mut self, v: Q) {
// 		self.num = Some(v);
// 	}

// 	pub fn set_text<S: Into<String>>(&mut self, s: S) {
// 		self.text = Some(s.into());
// 	}
// }

// /// ---------- 工具函数：normalize key（lower + replace 非字母数字为 underscore） ----------
// fn normalize_key(s: &str) -> String {
// 	s.trim()
// 		.chars()
// 		.map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
// 		.collect()
// }

// /// ---------- 工具函数：解析 KV 行 "Key:  123 kB" -> (key, maybe_number_str, maybe_unit) ----------
// fn parse_kv_line_raw(line: &str, sep: char) -> Option<(&str, &str, Option<&str>)> {
// 	let mut parts = line.splitn(2, sep);
// 	let key = parts.next()?.trim();
// 	let rest = parts.next()?.trim();
// 	// rest 的第一个 token 应该是 number（如果不是，也返回整个 rest 作为 text）
// 	let mut toks = rest.split_whitespace();
// 	let first = toks.next()?;
// 	let unit = toks.next();
// 	Some((key, first, unit))
// }

// #[macro_export]
// macro_rules! kv {
//     (
//         $(#[$struct_meta:meta])*
//         struct $name:ident < $Q:ident : FromValueUnit > {
//             $(
//                 $( #[$field_meta:meta] )*
//                 $field:ident => $key:expr $( => $comment:expr )?
//             ),* $(,)?
//         }
//     ) => {
//         $(#[$struct_meta])*
//         #[derive(Debug, Default, Clone)]
//         pub struct $name<$Q: FromValueUnit> {
//             $(
//                 $( #[$field_meta] )*
//                 pub $field: Parsed<$Q>,
//             )*
//         }

//         impl<$Q: FromValueUnit> $name<$Q> {

//             /// 从 kv 风格文本解析（sep 默认为 ':'）
//             pub fn parse_kv(input: &str, sep: char) -> Result<Self, String> {
//                 let mut out: Self = Default::default();
//                 for line in input.lines() {
//                     let l = line.trim();
//                     if l.is_empty() { continue; }
//                     if let Some((raw_key, first_token, unit_opt)) = crate::parse_kv_line_raw(l, sep) {
//                         let key_norm = crate::normalize_key(raw_key);
//                         match key_norm.as_str() {
//                             $(
//                                 key if key == crate::normalize_key($key).as_str() => {
//                                     // 若 first_token 可解析为 u64 则当 number
//                                     if let Ok(v) = u64::from_str(first_token) {
//                                         let q = $Q::from_value_unit(v, unit_opt);
//                                         out.$field.set_num(q);
//                                     } else {
//                                         // 不是数字，就把后半段原样当 text
//                                         out.$field.set_text(first_token.to_string() + unit_opt.map(|u| format!(" {}", u)).as_deref().unwrap_or(""));
//                                     }
//                                 }
//                             )*
//                             _ => {
//                                 // 未命中任何字段，忽略
//                             }
//                         }
//                     } else {
//                         // 解析失败 -> 忽略
//                     }
//                 }
//                 Ok(out)
//             }
//         }
//     };
// }

// /// ---------- 宏：cols_struct! 用于列式文件（每行若干列），支持字符串列与数值列 ----------
// /// 每个字段后给出列索引（0-based），若想把第一列中的 "iface:" 这种带冒号的名称解析为文本，请指定 index 为 "iface" (macro 内用 string "iface" 区分)
// #[macro_export]
// macro_rules! cols {
//     (
//         $(#[$struct_meta:meta])*
//         struct $name:ident < $Q:ident : FromValueUnit > {
//             $(
//                 $( #[$field_meta:meta] )*
//                 $field:ident => $idx:tt $( => $comment:expr )?
//             ),* $(,)?
//         }
//     ) => {
//         $(#[$struct_meta])*
//         #[derive(Debug, Default, Clone)]
//         pub struct $name<$Q: FromValueUnit> {
//             $(
//                 $( #[$field_meta] )*
//                 pub $field: Parsed<$Q>,
//             )*
//         }

//         impl<$Q: FromValueUnit> $name<$Q> {
//             pub const FIELD_COMMENTS: &'static [(&'static str, &'static str)] = &[
//                 $(
//                     $( ( stringify!($field), $comment ), )?
//                 )*
//             ];

//             /// 解析按空白分割的列（skip_header_lines 表示跳过前 N 行）
//             pub fn parse_cols_whitespace(input: &str, skip_header_lines: usize) -> Result<Vec<Self>, String> {
//                 let mut out = Vec::new();
//                 for (i, line) in input.lines().enumerate() {
//                     if i < skip_header_lines { continue; }
//                     let l = line.trim();
//                     if l.is_empty() { continue; }
//                     // 处理类似 " iface: rest..." 的情况：先尝试找 ':'，把 iface 单独处理
//                     let (maybe_iface, rest_line) = if let Some(pos) = l.find(':') {
//                         (Some(l[..pos].trim()), l[pos+1..].trim())
//                     } else {
//                         (None, l)
//                     };
//                     // parts 是对 rest_line 的 split_whitespace
//                     let parts: Vec<&str> = rest_line.split_whitespace().collect();
//                     let mut row: Self = Default::default();
//                     $(
//                         // match either numeric column index or special 'iface' token
//                         {
//                             let filled = false;
//                             // special token: if $idx is "iface" (string literal), fill from maybe_iface
//                             let handled = {
//                                 // Using stringify! to detect token "iface"
//                                 if stringify!($idx) == "iface" {
//                                     if let Some(iface_raw) = maybe_iface {
//                                         // iface is text
//                                         row.$field.set_text(iface_raw.to_string());
//                                     }
//                                     true
//                                 } else {
//                                     false
//                                 }
//                             };
//                             if !handled {
//                                 // treat $idx as usize
//                                 let idx_usize: usize = { $idx };
//                                 if let Some(tok) = parts.get(idx_usize) {
//                                     if let Ok(v) = u64::from_str(tok) {
//                                         let q = $Q::from_value_unit(v, None);
//                                         row.$field.set_num(q);
//                                     } else {
//                                         // not numeric -> store as text
//                                         row.$field.set_text(tok.to_string());
//                                     }
//                                 } else {
//                                     // column missing -> leave default
//                                 }
//                             }
//                         }
//                     )*
//                     out.push(row);
//                 }
//                 Ok(out)
//             }
//         }
//     };
// }

// use parsers::{SimpleQuantity, *}; // or use your uom-backed type

// kv! {
// 	/// meminfo 结构（示例）
// 	struct MemInfo<Q: FromValueUnit> {
// 		/// HugePages total pages
// 		HugePages_Total => "HugePages_Total" => "HugePages 总页数",
// 		HugePages_Free => "HugePages_Free" => "HugePages 空闲页数",
// 		HugePages_Rsvd => "HugePages_Rsvd" => "保留页",
// 		HugePages_Surp => "HugePages_Surp" => "备用页",
// 		Hugepagesize => "Hugepagesize" => "huge page size",
// 		Hugetlb => "Hugetlb" => "huge tlb bytes",
// 	}
// }

// cols! {
// 	/// /proc/net/dev 结构（示例）: 使用 iface 特殊 token 代表行首的 iface:
// 	struct NetDevRow<Q: FromValueUnit> {
// 		iface => iface => "interface name",
// 		rx_bytes => 0 => "receive bytes (first column after ':')",
// 		rx_packets => 1 => "receive packets",
// 		rx_errs => 2 => "receive errs",
// 		tx_bytes => 8 => "transmit bytes (tx columns start at index 8)",
// 		tx_packets => 9 => "transmit packets",
// 	}
// }

// cols! {
// 	/// vmstat/stat 风格：一行 key 后跟多个列（这里按空白切分取不同列）
// 	struct VmStatRow<Q: FromValueUnit> {
// 		key => iface => "key (or name) of vmstat line",
// 		v0 => 0 => "value 0",
// 		v1 => 1 => "value 1",
// 		v2 => 2 => "value 2",
// 		v3 => 3 => "value 3",
// 		// ... 如需更多列可继续添加
// 	}
// }

// fn main() {
// 	// ---------- meminfo 示例 ----------
// 	let meminfo_txt = "\
// HugePages_Total:       0
// HugePages_Free:        0
// HugePages_Rsvd:        0
// HugePages_Surp:        0
// Hugepagesize:       2048 kB
// Hugetlb:               0 kB
// ";
// 	let mem: MemInfo = MemInfo::parse_kv(meminfo_txt, ':').unwrap();
// 	println!("Parsed MemInfo: {:#?}", mem);

// 	// ---------- /proc/net/dev 示例 ----------
// 	let netdev_txt = "\
// Inter-|   Receive                                                |  Transmit
//  face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
//     lo: 702313499 3593273    0    0    0     0          0         0 702313499 3593273    0    0    0     0       0          0
// ens160: 2346622037 5597246    0    0    0     0          0         0 2217960254 4129395    0   70    0     0       0          0
// ";
// 	// skip 2 header lines
// 	let rows = NetDevRow::<SimpleQuantity>::parse_cols_whitespace(netdev_txt, 2).unwrap();
// 	println!("Parsed /proc/net/dev rows:");
// 	for r in rows {
// 		println!("{:#?}", r);
// 	}

// 	// ---------- vmstat 示例（样本） ----------
// 	let vmstat_txt = "\
// zswpout 0
// zswpwb 0
// vma_lock_success 1072453240
// vma_lock_abort 1011380
// vma_lock_retry 11902492
// vma_lock_miss 1
// nr_unstable 0
// ";
// 	// For vmstat-like small-key lines we can reuse parse_cols_whitespace and treat first token as key
// 	let vm_rows = VmStatRow::<SimpleQuantity>::parse_cols_whitespace(vmstat_txt, 0).unwrap();
// 	println!("Parsed vmstat-like rows:");
// 	for r in vm_rows {
// 		println!("{:#?}", r);
// 	}

// 	// ---------- stat / proc/stat 示例（cpu lines） ----------
// 	let stat_txt = "\
// cpu  2669874 2875 1233376 178730740 119759 0 397676 0 0 0
// cpu0 672809 612 309333 44686078 31234 0 217614 0 0 0
// intr 188051724 0 3614843 55657159 0 0 0 20 0 0 0
// ";
// 	let stat_rows = VmStatRow::<SimpleQuantity>::parse_cols_whitespace(stat_txt, 0).unwrap();
// 	println!("Parsed stat rows:");
// 	for r in stat_rows {
// 		println!("{:#?}", r);
// 	}
// }

// pub struct MemInfo {
// 	/// Total usable RAM (i.e., physical RAM minus a few reserved bits and the
// 	/// kernel binary code).
// 	MemTotal: Information,
// 	/// The sum of [`LowFree`] + [`HighFree`]
// 	MemFree: Information,
// 	/// An estimate of how much memory is available for starting new
// 	/// applications, without swapping. Calculated from MemFree, SReclaimable,
// 	/// the size of the file LRU lists, and the low watermarks in each zone.
// 	/// The estimate takes into account that the system needs some page cache
// 	/// to function well, and that not all reclaimable slab will be
// 	/// reclaimable, due to items being in use. The impact of those factors
// 	/// will vary from system to system.
// 	MemAvailable: Information,
// 	/// Relatively temporary storage for raw disk blocks shouldn't get
// 	/// tremendously large (20MB or so)
// 	Buffers: Information,
// 	/// in-memory cache for files read from the disk (the pagecache). Doesn't
// 	/// include [`SwapCached`].
// 	Cached: Information,
// 	/// Memory that once was swapped out, is swapped back in but still also is
// 	/// in the swapfile (if memory is needed it doesn't need to be swapped out
// 	/// AGAIN because it is already in the swapfile. This saves I/O)
// 	SwapCached: Information,
// 	/// Memory that has been used more recently and usually not reclaimed
// 	/// unless absolutely necessary.
// 	Active: Information,
// 	/// Memory which has been less recently used.  It is more eligible to be
// 	/// reclaimed for other purposes
// 	Inactive: Information,
// 	/// (since Linux 2.6.28)
// 	Active_anon: Information,
// 	/// (since Linux 2.6.28)
// 	Inactive_anon: Information,
// 	/// (since Linux 2.6.28)
// 	Active_file: Information,
// 	/// (since Linux 2.6.28)
// 	Inactive_file: Information,
// 	/// Total amount of swap space available.
// 	SwapTotal: Information,
// 	/// Amount of swap space that is currently unused.
// 	SwapFree: Information,
// 	/// Memory which is waiting to get written back to the disk.
// 	Dirty: Information,
// 	/// Writeback
// 	Writeback: Information,
// }

// macro_rules! proc_table_struct {
//     ($name:ident {
//         $($field:ident),* $(,)?
//     }) => {
//         #[derive(Debug)]
//         pub struct $name {
//             pub iface: String,
//             $( pub $field: u64, )*
//         }
//         impl $name {
//             /// 解析表格，返回多行结果向量
//             pub fn parse(s: &str) -> Result<Vec<$name>, Box<dyn std::error::Error>> {
//                 let mut results = Vec::new();
//                 // 跳过前两行标题
//                 for line in s.lines().skip(2) {
//                     if line.trim().is_empty() { continue; }
//                     // 按空白分隔：第一列带冒号是接口名，其余为数字
//                     let parts: Vec<&str> = line.split_whitespace().collect();
//                     if parts.len() < 1 { continue; }
//                     // 接口名去掉结尾冒号
//                     let iface = parts[0].trim_end_matches(':').to_string();
//                     let mut iter = parts.into_iter().skip(1);
//                     $(
//                         let $field = iter.next().unwrap_or("0").parse::<u64>()?;
//                     )*
//                     results.push($name { iface, $($field),* });
//                 }
//                 Ok(results)
//             }
//         }
//     };
// }

// // 使用示例：定义 NetDev 结构体（按 /proc/net/dev 的列顺序）
// proc_table_struct! {
// 	NetDev {
// 		rx_bytes, rx_packets, rx_errs, rx_drop, rx_fifo, rx_frame, rx_compressed, rx_multicast,
// 		tx_bytes, tx_packets, tx_errs, tx_drop, tx_fifo, tx_colls, tx_carrier, tx_compressed
// 	}
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
// 	let data = "\
// Inter-|   Receive                                                |  Transmit
//  face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
//   lo: 2776770   11307    0    0    0     0          0         0  2776770   11307    0    0    0     0       0          0
// eth0: 1215645    2751    0    0    0     0          0         0  1782404    4324    0    0    0   427       0          0";
// 	let stats = NetDev::parse(data)?;
// 	for stat in stats {
// 		println!("{:?}", stat);
// 	}
// 	Ok(())
// }

// use nom::{
// 	IResult,
// 	bytes::complete::{tag, take_till, take_until},
// 	character::complete::{alpha1, digit1, multispace0, space0, space1},
// 	combinator::{map_res, opt},
// 	multi::fold_many0,
// 	sequence::{preceded, terminated},
// };

// /// 解析值字符串（包括单位）到 uom 信息量类型
// fn parse_information(value: f64, unit: &str) -> uom::si::Information {
// 	use uom::si::information::*;
// 	// 简单示例：将 "kB", "MB", "GB" 识别为相应单位
// 	match unit {
// 		"kB" => Information::new::<kilobyte>(value), // 1 kB = 1000 B
// 		"MB" => Information::new::<megabyte>(value),
// 		"GB" => Information::new::<gigabyte>(value),
// 		_ => Information::new::<byte>(value),
// 	}
// }

// /// kv_struct! 宏：针对 "Key: Value [Unit]" 格式的行
// #[macro_export]
// macro_rules! kv_struct {
//     (
//         $(#[$struct_meta:meta])*
//         $struct_vis:vis $StructName:ident {
//             $(
//                 $(#[$field_meta:meta])*
//                 $field_vis:vis $field:ident : $typ:ty
//             ),* $(,)?
//         }
//     ) => {
//         $(#[$struct_meta])*
//         $struct_vis struct $StructName {
//             $(
//                 $(#[$field_meta])*
//                 $field_vis $field: Option<$typ>,
//             )*
//         }

//         impl $StructName {
//             pub fn parse(input: &str) -> IResult<&str, $StructName> {
//                 let mut res = $StructName { $($field: None,)* };

//                 let (input, _) = fold_many0(
//                     nom::sequence::terminated(
//                         |i| {
//                             $( let (i, _) = tag(concat!(stringify!($field), ":"))(i)?;
//                                let (i, _) = space0(i)?;
//                                let (i, num) = map_res(digit1, |s: &str| s.parse::<f64>())(i)?;
//                                let (i, unit) = opt(preceded(space1, alpha1))(i)?;
//                                let val = if let Some(u) = unit { parse_information(num, u) } else { parse_information(num, "") };
//                                res.$field = Some(val.into());
//                                return Ok((i, ())); )*

//                             // 如果当前行键不是定义的字段，忽略整行
//                             let (i, _) = take_until("\n")(i)?;
//                             Ok((i, ()))
//                         },
//                         multispace0 // 行尾或空行
//                     ),
//                     res,
//                     |mut acc, _| { acc } // 直接在 acc 上修改，无需合并
//                 )(input)?;

//                 Ok((input, res))
//             }
//         }

//         pub fn parse_$StructName(input: &str) -> IResult<&str, $StructName> {
//             $StructName::parse(input)
//         }
//     }
// }

// /// space_struct! 宏：针对 "key val1 val2..." 格式
// #[macro_export]
// macro_rules! space_struct {
//     (
//         $(#[$struct_meta:meta])*
//         $StructName:ident {
//             $(
//                 $(#[$field_meta:meta])*
//                 $field:ident : $typ:ty
//             ),* $(,)?
//         }
//     ) => {
//         $(#[$struct_meta])*
//         pub struct $StructName {
//             $( $(#[$field_meta])* pub $field: Option<$typ>, )*
//         }

//         impl $StructName {
//             pub fn parse(input: &str) -> IResult<&str, $StructName> {
//                 // 跳过键名，只解析数字部分
//                 let mut res = $StructName { $($field: None,)* };

//                 // 先跳过字段名
//                 let (input, _) = alpha1(input)?;
//                 let (input, _) = space1(input)?;
//                 // 顺序解析预期的数值列
//                 $(
//                     let (input, num_str) = digit1(input)?;
//                     let val: $typ = num_str.parse().unwrap_or_default();
//                     res.$field = Some(val);
//                     let (input, _) = opt(space1)(input)?;
//                 )*

//                 Ok((input, res))
//             }
//         }

//         pub fn parse_$StructName(input: &str) -> IResult<&str, $StructName> {
//             $StructName::parse(input)
//         }
//     }
// }

// /// table_struct! 宏：针对表格格式（第一列名称，其余为固定列）
// /// for table-like lines with fixed number of columns
// #[macro_export]
// macro_rules! table_struct {
//     (
//         $(#[$struct_meta:meta])*
//         $StructName:ident {
//             $(
//                 $(#[$field_meta:meta])*
//                 $field:ident : $typ:ty
//             ),* $(,)?
//         }
//     ) => {
//         $(#[$struct_meta])*
//         pub struct $StructName {
//             #[doc = "接口/设备名"]
//             pub name: Option<String>,
//             $(
//                 $(#[$field_meta])*
//                 pub $field: Option<$typ>,
//             )*
//         }

//         impl $StructName {
//             pub fn parse(input: &str) -> IResult<&str, $StructName> {
//                 let mut res = $StructName { name: None, $($field: None,)* };
//                 // 提取冒号前的设备名（可能有空格前缀）
//                 let (input, _) = multispace0(input)?;
//                 let (input, name) = take_till(|c| c == ':')(input)?;
//                 let (input, _) = tag(":")(input)?;
//                 // 填充 name 字段
//                 res.name = Some(name.trim().to_string());
//                 // 解析剩下固定列
//                 $(
//                     let (input, num_str) = preceded(space1, digit1)(input)?;
//                     let val: $typ = num_str.parse().unwrap_or_default();
//                     res.$field = Some(val);
//                 )*
//                 Ok((input, res))
//             }

//             /// 解析所有行为 Vec<Self>
//             pub fn parse_all(input: &str) -> IResult<&str, Vec<$StructName>> {
//                 // 重复解析每一行
//                 nom::multi::many1(
//                     terminated(
//                         |i| $StructName::parse(i),
//                         opt(tag("\n"))
//                     )
//                 )(input)
//             }
//         }

//         pub fn parse_$StructName(input: &str) -> IResult<&str, $StructName> {
//             $StructName::parse(input)
//         }
//         pub fn parse_all_$StructName(input: &str) -> IResult<&str, Vec<$StructName>> {
//             $StructName::parse_all(input)
//         }
//     }
// }
