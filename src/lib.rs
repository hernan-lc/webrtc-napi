use napi::{bindgen_prelude::*, JsObject};
use napi_derive::napi;

#[napi]
pub struct PeerConnection {
  id: String,
  config: RTCConfiguration,
}

#[napi(object)]
pub struct RTCConfiguration {
  pub ice_servers: Vec<String>,
}

#[napi]
impl PeerConnection {
  #[napi(constructor)]
  pub fn new(config: RTCConfiguration) -> Self {
    let id = uuid::Uuid::new_v4().to_string();
    PeerConnection { id, config }
  }

  #[napi]
  pub fn get_id(&self) -> String {
    self.id.clone()
  }

  #[napi]
  pub fn get_stats(&self) -> String {
    format!("Connection {} with {} ICE servers", self.id, self.config.ice_servers.len())
  }

  #[napi]
  pub fn close(&self) {
    println!("Closing connection: {}", self.id);
  }

  #[napi]
  pub fn add_ice_candidate(&mut self, candidate: String) {
    println!("Added ICE candidate: {}", candidate);
  }
}

#[napi]
pub fn init_webrtc() {
  println!("WebRTC NAPI initialized");
}

#[napi]
pub fn get_version() -> String {
  "0.1.0".to_string()
}