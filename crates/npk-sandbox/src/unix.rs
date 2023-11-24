#[cfg(target_os = "linux")]
pub mod linux;

use serde::Deserialize;
use std::time::Duration;

use crc::Crc;
#[cfg(target_os = "linux")]
pub use linux as flavor;

pub use flavor::{Controller, Sandbox};

const SOCKET_TIMEOUT: Duration = Duration::from_secs(2);
const USIZE_SIZE: usize = std::mem::size_of::<usize>();
const U64_SIZE: usize = std::mem::size_of::<usize>();
const ZYGOTE_HEADER_SIZE: usize = USIZE_SIZE + U64_SIZE;
static CRC: Crc<u64> = Crc::<u64>::new(&crc::CRC_64_REDIS);
