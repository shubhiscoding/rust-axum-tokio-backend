use dashmap::DashMap;
pub struct AppState {
   pub tokens: DashMap<String, u64>,
}