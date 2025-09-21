/// SSE (Server-Sent Events) Payload Builder
#[derive(Debug, Clone, Default)]
pub struct Event {
    event: Option<String>,
    data_list: Vec<String>,
    id: Option<String>,
    retry: Option<u64>,
}

impl Event {
    /// Create a new SSE event builder.
    pub fn new() -> Self {
        Self {
            event: None,
            data_list: vec![],
            id: None,
            retry: None,
        }
    }

    /// Set the event name. (Optional)
    pub fn event(mut self, event_name: impl AsRef<str>) -> Self {
        self.event = Some(event_name.as_ref().to_string());

        self
    }

    /// Add a data. (append)
    pub fn data(mut self, data: impl AsRef<str>) -> Self {
        self.data_list.push(data.as_ref().to_string());

        self
    }

    /// Set the event ID. (Optional)
    /// If the client reconnects, it will send the last event ID to the server.
    /// The server can use this ID to resume sending events from where it left off.
    /// This is useful for ensuring that the client does not miss any events in case of a disconnection.
    pub fn id(mut self, id: impl AsRef<str>) -> Self {
        self.id = Some(id.as_ref().to_string());
        self
    }

    /// Set the retry time in milliseconds. (Optional)
    /// This tells the client how long to wait before attempting to reconnect after a disconnection.
    /// If not set, the client will use its default retry time.
    pub fn retry(mut self, millis: u64) -> Self {
        self.retry = Some(millis);
        self
    }

    /// Build the final SSE event string.
    pub fn build(self) -> String {
        let mut event_string = String::new();

        if let Some(event_name) = self.event {
            event_string.push_str(&format!("event: {}\n", event_name));
        }

        for data in self.data_list {
            event_string.push_str(&format!("data: {}\n", data));
        }

        if let Some(id) = self.id {
            event_string.push_str(&format!("id: {}\n", id));
        }

        if let Some(retry) = self.retry {
            event_string.push_str(&format!("retry: {}\n", retry));
        }

        event_string.push('\n');

        event_string
    }
}
