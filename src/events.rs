#[derive(Debug)]
pub enum Event {
    Ping(Vec<u8>),
    Custom(String),
    Unknown(Vec<u8>),
}

pub fn parse_event(data: Vec<u8>) -> Event {
    if data == b"ping" {
        Event::Ping(data)
    } else if let Ok(text) = String::from_utf8(data.clone()) {
        Event::Custom(text)
    } else {
        Event::Unknown(data)
    }
}
