use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashSet;

pub fn extract_css_selectors(svg: &str) -> HashSet<String> {
  let mut reader = Reader::from_str(svg);
  reader.config_mut().trim_text(true);

  let mut selectors = HashSet::new();
  let mut buffer = Vec::new();

  let mut current_css = String::new();
  let mut in_style = false;

  loop {
    match reader.read_event_into(&mut buffer) {
      Ok(Event::Start(e)) => {
        if e.name().as_ref() == b"style" {
          in_style = true;
          current_css.clear();
        }
      }
      Ok(Event::Text(e)) => {
        if in_style {
          if let Ok(text) = e.unescape() {
            current_css.push_str(&text);
          }
        }
      }
      Ok(Event::CData(e)) => {
        if in_style {
          // Converts a byte slice to a String using UTF-8 lossy conversion
          //
          // Takes a byte slice reference and returns a new owned String. If the input contains
          // invalid UTF-8 sequences, they are replaced with the Unicode replacement character (U+FFFD).
          let text = String::from_utf8_lossy(e.as_ref()).to_string();
          current_css.push_str(&text);
        }
      }
      Ok(Event::End(e)) => {
        if e.name().as_ref() == b"style" {
          in_style = false;
          // parse_css_selectors(&current_css, &mut selectors);
          !todo!("parse_css_selectors(&current_css, &mut selectors)");
          current_css.clear();
        }
      }
      Ok(Event::Eof) => break,
      _ => {}
    }
    buffer.clear();
  }

  selectors
}
