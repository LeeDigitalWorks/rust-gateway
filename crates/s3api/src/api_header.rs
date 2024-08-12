use quick_xml::se::to_writer;
use serde::Serialize;

pub fn encode_response<T: Serialize>(response: &T) -> Vec<u8> {
    // Create a buffer to hold the XML data
    // Add the XML header
    // Use to_writer to write the XML data to the buffer
    let mut buf = String::new();

    buf.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");

    to_writer(&mut buf, response).unwrap();

    buf.as_bytes().to_vec()
}
