use std::sync::Arc;

use log::error;
use serde::{Deserialize, Serialize};

use crate::{
  device_info::{self, DeviceInfo},
  state_storage::StateStorage,
  utils::LogAndForget,
};

/// Longest channel label accepted, as in Dante (31 characters).
pub const MAX_CHANNEL_NAME_BYTES: usize = 31;

/// Whether a channel label (from a rename request or saved state) is usable: it
/// becomes part of an mDNS label `<name>@<hostname>` (at most 63 bytes, a longer
/// one used to panic the mDNS responder) and of `<name>@<hostname>` subscription
/// addresses, so it must be 1..=31 bytes without `.`, `=`, `@` or control characters.
pub fn is_valid_channel_name(name: &str) -> bool {
  !name.is_empty()
    && name.len() <= MAX_CHANNEL_NAME_BYTES
    && !name.chars().any(|c| c.is_control() || matches!(c, '.' | '=' | '@'))
}

#[derive(Deserialize, Serialize, Default)]
pub struct ChannelSettings {
  id: usize,
  friendly_name: String,
}

#[derive(Deserialize, Serialize)]
struct SavedChannels {
  channels: Vec<ChannelSettings>,
}

pub struct SavedChannelsSettings {
  state_storage: Arc<StateStorage>,
  self_info: Arc<DeviceInfo>,
  rx_channels: SavedChannels,
  tx_channels: SavedChannels,
}

impl SavedChannelsSettings {
  pub fn load(state_storage: Arc<StateStorage>, self_info: Arc<DeviceInfo>) -> Self {
    let mut r = Self {
      rx_channels: state_storage.load("rx_channels").unwrap_or(SavedChannels { channels: vec![] }),
      tx_channels: state_storage.load("tx_channels").unwrap_or(SavedChannels { channels: vec![] }),
      state_storage,
      self_info,
    };
    Self::load_and_init(&mut r.rx_channels.channels, &r.self_info.rx_channels);
    Self::load_and_init(&mut r.tx_channels.channels, &r.self_info.tx_channels);
    r
  }
  fn load_and_init(src: &mut Vec<ChannelSettings>, dst: &Vec<device_info::Channel>) {
    for (index, cs) in src.iter().enumerate().take(dst.len()) {
      if cs.id == 0 || cs.friendly_name.is_empty() {
        continue;
      }
      if index != (cs.id - 1) {
        error!("corrupted saved channels: id {}", cs.id);
        continue;
      }
      if !is_valid_channel_name(&cs.friendly_name) {
        error!("ignoring invalid saved name for channel id {}: {:?}", cs.id, cs.friendly_name);
        continue;
      }
      *dst[index].friendly_name.write().unwrap() = cs.friendly_name.clone();
    }
    if dst.len() > src.len() {
      src.resize_with(dst.len(), || Default::default());
    };
  }
  fn rename(
    local: &mut Vec<ChannelSettings>,
    device_channels: &Vec<device_info::Channel>,
    index: usize,
    name: String,
  ) {
    local[index].friendly_name = name.clone();
    local[index].id = index + 1;
    *device_channels[index].friendly_name.write().unwrap() = name;
  }
  pub fn rename_rx_channel(&mut self, index: usize, name: String) {
    Self::rename(&mut self.rx_channels.channels, &self.self_info.rx_channels, index, name);
    self.state_storage.save("rx_channels", &self.rx_channels).log_and_forget();
  }
  pub fn rename_tx_channel(&mut self, index: usize, name: String) {
    Self::rename(&mut self.tx_channels.channels, &self.self_info.tx_channels, index, name);
    self.state_storage.save("tx_channels", &self.tx_channels).log_and_forget();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn channel_names() {
    for ok in ["TX 1", "Kick In", "01", "a", &"x".repeat(31), "Gesang Ä"] {
      assert!(is_valid_channel_name(ok), "{ok:?}");
    }
    for bad in ["", &"x".repeat(32), "a.b", "a=b", "a@b", "tab\there", "nl\n", &"Ä".repeat(16)] {
      assert!(!is_valid_channel_name(bad), "{bad:?}");
    }
  }
}
