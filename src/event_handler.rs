use crate::events::Event;

pub async fn handle_event(event: Event) {
    match event {
        Event::Ping(data) => {
            println!("Received Ping with data: {:?}", data);
            // Handle ping logic.
        }
        Event::Custom(message) => {
            println!("Received Custom event: {}", message);
            // Handle custom logic.
        }
        Event::Unknown(data) => {
            println!("Received Unknown data: {:?}", data);
            // Handle unknown data.
        }
    }
}
