use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JsonResponse<T> {
    pub jsonrpc: String,
    pub id: u32,
    pub result: T,
}