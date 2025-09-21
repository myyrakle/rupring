/// SSE (Server-Sent Events) Payload Builder
#[derive(Debug, Clone, Default)]
pub struct Event {
    event: Option<String>,
    data_list: Vec<String>,
}

impl Event {
    /// Create a new SSE event builder.
    pub fn new() -> Self {
        Self {
            event: None,
            data_list: vec![],
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

    /// Build the final SSE event string.
    pub fn build(self) -> String {
        let mut event_string = String::new();

        if let Some(event_name) = self.event {
            event_string.push_str(&format!("event: {}\n", event_name));
        }

        for data in self.data_list {
            event_string.push_str(&format!("data: {}\n", data));
        }

        event_string.push('\n');

        event_string
    }
}
