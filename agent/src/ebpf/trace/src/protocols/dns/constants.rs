pub(super) const DNS_HEADER_SIZE: u32 = 12;

/// This is the typical maximum size for DNS.
pub(super) const DNS_MSG_MAX_SIZE: u32 = 512;

/// ref: <https://stackoverflow.com/questions/6794926/how-many-a-records-can-fit-in-a-single-dns-response>
pub(super) const MAX_RECOURSE_RECORD_NUM: u16 = 25;
