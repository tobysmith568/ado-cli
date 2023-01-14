use urlencoding::encode;

pub fn url_encode(content: &str) -> String {
    encode(content).to_string()
}
