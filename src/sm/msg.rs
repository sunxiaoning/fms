use std::collections::HashMap;
pub struct Message<T> {
    payload: T,
    headers: Option<HashMap<String, String>>,
}

impl<T> Message<T> {
    pub fn get_header(&self, key: &str) -> Option<&String> {
        if let Some(headers) = &self.headers {
            headers.get(&String::from(key))
        } else {
            None
        }
    }

    pub fn get_payload(&self) -> &T {
        &self.payload
    }

    pub fn new(payload: T) -> Message<T> {
        Message {
            payload,
            headers: None,
        }
    }

    pub fn new_with_header(payload: T, headers: HashMap<String, String>) -> Message<T> {
        assert!(headers.len() > 0);
        Message {
            payload,
            headers: Some(headers),
        }
    }

    pub fn headers(&self) -> Option<&HashMap<String, String>> {
        self.headers.as_ref()
    }
}

pub struct MessageBuilder<T> {
    payload: Option<T>,
    headers: Option<HashMap<String, String>>,
}

impl<T> MessageBuilder<T> {
    pub fn build(&mut self) -> Message<T> {
        let payload = self.payload.take().expect("payload absent!");
        if let None = self.headers {
            Message::new(payload)
        } else {
            Message::new_with_header(payload, self.headers.take().unwrap())
        }
    }

    pub fn add_header(&mut self, key: String, val: String) -> &mut MessageBuilder<T> {
        if let None = self.headers {
            self.headers = Some(HashMap::new());
        }
        self.headers.as_mut().unwrap().insert(key, val);
        self
    }

    pub fn payload(&mut self, payload: T) -> &mut MessageBuilder<T> {
        self.payload = Some(payload);
        self
    }

    pub fn new() -> MessageBuilder<T> {
        MessageBuilder {
            payload: None,
            headers: None,
        }
    }
}
