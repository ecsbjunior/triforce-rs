pub struct Parser {

}

impl Parser {
  pub fn from_xml<R: std::io::Read>(read: R) {
    xml::EventReader::new(read)
      .into_iter()
      .map(Result::unwrap)
      .filter_map(ParseEvent::from_xml)
  }
}

enum ParseEvent {
  Start(String, Vec<xml::attribute::Attribute>),
  End(String),
  Text(String),
}

impl ParseEvent {
  fn from_xml(event: xml::reader::XmlEvent) -> Option<ParseEvent> {
    match event {
      xml::reader::XmlEvent::StartDocument { .. } => None,
      xml::reader::XmlEvent::EndDocument => None,
      xml::reader::XmlEvent::StartElement {
        name, attributes, ..
      } => {
        let attributes = attributes.into_iter().map(Attribute::from).collect();
        Some(ParseEvent::Start(name.local_name, attributes))
      },
      xml::reader::XmlEvent::EndElement { name } => Some(ParseEvent::End(name.local_name)),
      xml::reader::XmlEvent::Characters(chars) => Some(ParseEvent::Text(chars)),
      xml::reader::XmlEvent::ProcessingInstruction { .. } => None,
      xml::reader::XmlEvent::CData(_) => None,
      xml::reader::XmlEvent::Comment(_) => None,
      xml::reader::XmlEvent::Whitespace(_) => None,
    }
  }
}
