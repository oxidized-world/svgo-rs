use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use bumpalo::collections::String as BumpString;
use bumpalo::Bump;
use derive_builder::Builder;
use lazy_static::lazy_static;
use regex::Regex;

use crate::optimizer::{Plugin, VisitAction}; // Assuming these paths
use crate::parser::XMLAstElement; // Assuming this path

// --- Static Collections ---
lazy_static! {
    static ref COLORS_NAMES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("aliceblue", "#f0f8ff");
        m.insert("antiquewhite", "#faebd7");
        m.insert("aqua", "#0ff"); // or #00ffff
        m.insert("aquamarine", "#7fffd4");
        m.insert("azure", "#f0ffff");
        m.insert("beige", "#f5f5dc");
        m.insert("bisque", "#ffe4c4");
        m.insert("black", "#000"); // or #000000
        m.insert("blanchedalmond", "#ffebcd");
        m.insert("blue", "#00f"); // or #0000ff
        m.insert("blueviolet", "#8a2be2");
        m.insert("brown", "#a52a2a");
        m.insert("burlywood", "#deb887");
        m.insert("cadetblue", "#5f9ea0");
        m.insert("chartreuse", "#7fff00");
        m.insert("chocolate", "#d2691e");
        m.insert("coral", "#ff7f50");
        m.insert("cornflowerblue", "#6495ed");
        m.insert("cornsilk", "#fff8dc");
        m.insert("crimson", "#dc143c");
        m.insert("cyan", "#0ff"); // or #00ffff, same as aqua
        m.insert("darkblue", "#00008b");
        m.insert("darkcyan", "#008b8b");
        m.insert("darkgoldenrod", "#b8860b");
        m.insert("darkgray", "#a9a9a9");
        m.insert("darkgreen", "#006400");
        m.insert("darkgrey", "#a9a9a9"); // same as darkgray
        m.insert("darkkhaki", "#bdb76b");
        m.insert("darkmagenta", "#8b008b");
        m.insert("darkolivegreen", "#556b2f");
        m.insert("darkorange", "#ff8c00");
        m.insert("darkorchid", "#9932cc");
        m.insert("darkred", "#8b0000");
        m.insert("darksalmon", "#e9967a");
        m.insert("darkseagreen", "#8fbc8f");
        m.insert("darkslateblue", "#483d8b");
        m.insert("darkslategray", "#2f4f4f");
        m.insert("darkslategrey", "#2f4f4f"); // same as darkslategray
        m.insert("darkturquoise", "#00ced1");
        m.insert("darkviolet", "#9400d3");
        m.insert("deeppink", "#ff1493");
        m.insert("deepskyblue", "#00bfff");
        m.insert("dimgray", "#696969");
        m.insert("dimgrey", "#696969"); // same as dimgray
        m.insert("dodgerblue", "#1e90ff");
        m.insert("firebrick", "#b22222");
        m.insert("floralwhite", "#fffaf0");
        m.insert("forestgreen", "#228b22");
        m.insert("fuchsia", "#f0f"); // or #ff00ff, same as magenta
        m.insert("gainsboro", "#dcdcdc");
        m.insert("ghostwhite", "#f8f8ff");
        m.insert("gold", "#ffd700");
        m.insert("goldenrod", "#daa520");
        m.insert("gray", "#808080");
        m.insert("green", "#008000");
        m.insert("greenyellow", "#adff2f");
        m.insert("grey", "#808080"); // same as gray
        m.insert("honeydew", "#f0fff0");
        m.insert("hotpink", "#ff69b4");
        m.insert("indianred", "#cd5c5c");
        m.insert("indigo", "#4b0082");
        m.insert("ivory", "#fffff0");
        m.insert("khaki", "#f0e68c");
        m.insert("lavender", "#e6e6fa");
        m.insert("lavenderblush", "#fff0f5");
        m.insert("lawngreen", "#7cfc00");
        m.insert("lemonchiffon", "#fffacd");
        m.insert("lightblue", "#add8e6");
        m.insert("lightcoral", "#f08080");
        m.insert("lightcyan", "#e0ffff");
        m.insert("lightgoldenrodyellow", "#fafad2");
        m.insert("lightgray", "#d3d3d3");
        m.insert("lightgreen", "#90ee90");
        m.insert("lightgrey", "#d3d3d3"); // same as lightgray
        m.insert("lightpink", "#ffb6c1");
        m.insert("lightsalmon", "#ffa07a");
        m.insert("lightseagreen", "#20b2aa");
        m.insert("lightskyblue", "#87cefa");
        m.insert("lightslategray", "#778899"); // JS has #789, which is short. CSS spec has #778899
        m.insert("lightslategrey", "#778899"); // JS has #789
        m.insert("lightsteelblue", "#b0c4de");
        m.insert("lightyellow", "#ffffe0");
        m.insert("lime", "#0f0"); // or #00ff00
        m.insert("limegreen", "#32cd32");
        m.insert("linen", "#faf0e6");
        m.insert("magenta", "#f0f"); // or #ff00ff, same as fuchsia
        m.insert("maroon", "#800000");
        m.insert("mediumaquamarine", "#66cdaa");
        m.insert("mediumblue", "#0000cd");
        m.insert("mediumorchid", "#ba55d3");
        m.insert("mediumpurple", "#9370db");
        m.insert("mediumseagreen", "#3cb371");
        m.insert("mediumslateblue", "#7b68ee");
        m.insert("mediumspringgreen", "#00fa9a");
        m.insert("mediumturquoise", "#48d1cc");
        m.insert("mediumvioletred", "#c71585");
        m.insert("midnightblue", "#191970");
        m.insert("mintcream", "#f5fffa");
        m.insert("mistyrose", "#ffe4e1");
        m.insert("moccasin", "#ffe4b5");
        m.insert("navajowhite", "#ffdead");
        m.insert("navy", "#000080");
        m.insert("oldlace", "#fdf5e6");
        m.insert("olive", "#808000");
        m.insert("olivedrab", "#6b8e23");
        m.insert("orange", "#ffa500");
        m.insert("orangered", "#ff4500");
        m.insert("orchid", "#da70d6");
        m.insert("palegoldenrod", "#eee8aa");
        m.insert("palegreen", "#98fb98");
        m.insert("paleturquoise", "#afeeee");
        m.insert("palevioletred", "#db7093");
        m.insert("papayawhip", "#ffefd5");
        m.insert("peachpuff", "#ffdab9");
        m.insert("peru", "#cd853f");
        m.insert("pink", "#ffc0cb");
        m.insert("plum", "#dda0dd");
        m.insert("powderblue", "#b0e0e6");
        m.insert("purple", "#800080");
        m.insert("rebeccapurple", "#663399"); // JS has #639
        m.insert("red", "#f00"); // or #ff0000
        m.insert("rosybrown", "#bc8f8f");
        m.insert("royalblue", "#4169e1");
        m.insert("saddlebrown", "#8b4513");
        m.insert("salmon", "#fa8072");
        m.insert("sandybrown", "#f4a460");
        m.insert("seagreen", "#2e8b57");
        m.insert("seashell", "#fff5ee");
        m.insert("sienna", "#a0522d");
        m.insert("silver", "#c0c0c0");
        m.insert("skyblue", "#87ceeb");
        m.insert("slateblue", "#6a5acd");
        m.insert("slategray", "#708090");
        m.insert("slategrey", "#708090"); // same as slategray
        m.insert("snow", "#fffafa");
        m.insert("springgreen", "#00ff7f");
        m.insert("steelblue", "#4682b4");
        m.insert("tan", "#d2b48c");
        m.insert("teal", "#008080");
        m.insert("thistle", "#d8bfd8");
        m.insert("tomato", "#ff6347");
        m.insert("turquoise", "#40e0d0");
        m.insert("violet", "#ee82ee");
        m.insert("wheat", "#f5deb3");
        m.insert("white", "#fff"); // or #ffffff
        m.insert("whitesmoke", "#f5f5f5");
        m.insert("yellow", "#ff0"); // or #ffff00
        m.insert("yellowgreen", "#9acd32");
        m
    };

    static ref COLORS_SHORT_NAMES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("#f0ffff", "azure");
        m.insert("#f5f5dc", "beige");
        m.insert("#ffe4c4", "bisque");
        m.insert("#a52a2a", "brown");
        m.insert("#ff7f50", "coral");
        m.insert("#ffd700", "gold");
        m.insert("#808080", "gray"); // and grey
        m.insert("#008000", "green");
        m.insert("#4b0082", "indigo");
        m.insert("#fffff0", "ivory");
        m.insert("#f0e68c", "khaki");
        m.insert("#faf0e6", "linen");
        m.insert("#800000", "maroon");
        m.insert("#000080", "navy");
        m.insert("#808000", "olive");
        m.insert("#ffa500", "orange");
        m.insert("#da70d6", "orchid");
        m.insert("#cd853f", "peru");
        m.insert("#ffc0cb", "pink");
        m.insert("#dda0dd", "plum");
        m.insert("#800080", "purple");
        m.insert("#f00", "red"); // Short hex for red
        m.insert("#ff0000", "red"); // Long hex for red
        m.insert("#fa8072", "salmon");
        m.insert("#a0522d", "sienna");
        m.insert("#c0c0c0", "silver");
        m.insert("#fffafa", "snow");
        m.insert("#d2b48c", "tan");
        m.insert("#008080", "teal");
        m.insert("#ff6347", "tomato");
        m.insert("#ee82ee", "violet");
        m.insert("#f5deb3", "wheat");
        // Add other short hex names if necessary, e.g. #000 for black, #fff for white
        m.insert("#000", "black");
        m.insert("#000000", "black");
        m.insert("#fff", "white");
        m.insert("#ffffff", "white");
        m.insert("#0ff", "aqua"); // or cyan
        m.insert("#00ffff", "aqua");
        m.insert("#f0f", "fuchsia"); // or magenta
        m.insert("#ff00ff", "fuchsia");
        m
    };

    static ref COLORS_PROPS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        s.insert("color");
        s.insert("fill");
        s.insert("flood-color");
        s.insert("lighting-color");
        s.insert("stop-color");
        s.insert("stroke");
        s
    };

    // Regexes
    // rNumber = '([+-]?(?:\d*\.\d+|\d+\.?)%?)'
    // rComma = '(?:\s*,\s*|\s+)'
    // regRGB = new RegExp('^rgb\\(\\s*' + rNumber + rComma + rNumber + rComma + rNumber + '\\s*\\)$')
    static ref REG_RGB: Regex = Regex::new(&format!(
        r"^rgb\(\s*({0}){1}({0}){1}({0})\s*\)$",
        r"[+-]?(?:(?:\d*\.\d+|\d+\.?))%?", // Simplified rNumber for capture
        r"(?:\s*,\s*|\s+)" // rComma
    )).unwrap();

    // regHEX = /^#(([a-fA-F0-9])\2){3}$/;
    // This regex checks if a long hex can be shortened, e.g., #AABBCC
    // It captures the *last* pair and its constituent character in JS.
    // For Rust, to get the components for shortening, a different regex is more direct:
    static ref REG_HEX_SHORTENABLE: Regex = Regex::new(r"^#([0-9a-fA-F])\1([0-9a-fA-F])\2([0-9a-fA-F])\3$").unwrap();

    // For includesUrlReference, based on `url(#...)`
    static ref REG_URL_REFERENCE: Regex = Regex::new(r"url\(#.+?\)").unwrap();
}

// --- Configuration Enums ---
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum CaseConvention {
  Lower,
  Upper,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum CurrentColorType {
  Enabled,          // JS `true`
  Specific(String), // JS `string` to match
  Pattern(String),  // JS `RegExp` string to match
}

// --- Configuration Struct ---
#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option), default)] // `default` calls ConvertColorsPluginConfig::default()
pub struct ConvertColorsPluginConfig {
  pub current_color: Option<CurrentColorType>,
  pub names_to_hex: bool,
  pub rgb_to_hex: bool,
  pub convert_case: Option<CaseConvention>,
  pub short_hex: bool,
  pub short_name: bool,
}

impl Default for ConvertColorsPluginConfig {
  fn default() -> Self {
    ConvertColorsPluginConfig {
      current_color: None, // JS default `false`
      names_to_hex: true,
      rgb_to_hex: true,
      convert_case: Some(CaseConvention::Lower), // JS default `'lower'`
      short_hex: true,
      short_name: true,
    }
  }
}

// --- Plugin Struct ---
pub struct ConvertColorsPlugin<'a> {
  arena: &'a Bump,
  config: ConvertColorsPluginConfig,
  mask_counter: u32,
  // Compiled regex for current_color if it's a pattern
  current_color_pattern_regex: Option<Regex>,
}

impl<'a> ConvertColorsPlugin<'a> {
  pub fn new(config: ConvertColorsPluginConfig, arena: &'a Bump) -> Self {
    let current_color_pattern_regex = match &config.current_color {
      Some(CurrentColorType::Pattern(s)) => Regex::new(s).ok(),
      _ => None,
    };
    Self {
      arena,
      config,
      mask_counter: 0,
      current_color_pattern_regex,
    }
  }

  // Helper: Convert [r, g, b] (u8) to #rrggbb string
  fn rgb_to_hex_string(&self, r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b) // Default to lowercase as per CSS and many tools
  }

  // Helper: checks if a string contains a url reference like url(#...)
  fn includes_url_reference(&self, s: &str) -> bool {
    REG_URL_REFERENCE.is_match(s)
  }
}

// --- Plugin Implementation ---
impl<'a> Plugin<'a> for ConvertColorsPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if el.name == "mask" {
      self.mask_counter += 1;
    }

    for (_attr_name, attr_value_ref) in el.attributes.iter_mut() {
      // Only process attributes that are known to accept color values
      // The JS version iterates all attributes and checks `colorsProps.has(name)`.
      // Assuming _attr_name is &'a str, we need to ensure it's comparable to keys in COLORS_PROPS.
      // For now, let's assume _attr_name is already suitable or convert it.
      // In the provided example, _attr_name is not used directly to filter,
      // but here we need to check against COLORS_PROPS.
      // Let's assume for now that this check happens *before* calling this plugin,
      // or that we should add it here if `el.attributes` is generic.
      // The JS plugin does `if (colorsProps.has(name))`. So we should too.
      let attr_name_str: &str = _attr_name; // Assuming _attr_name is &'a str
      if !COLORS_PROPS.contains(attr_name_str) {
        continue;
      }

      let original_value: &'a str = attr_value_ref;
      let mut current_val_cow: Cow<'a, str> = Cow::Borrowed(original_value);
      let mut modified = false;

      // 1. Convert to currentColor
      if self.mask_counter == 0 {
        if let Some(cc_type) = &self.config.current_color {
          let matched = match cc_type {
            CurrentColorType::Enabled => current_val_cow != "none",
            CurrentColorType::Specific(s_match) => current_val_cow == *s_match,
            CurrentColorType::Pattern(_) => {
              if let Some(ref regex) = self.current_color_pattern_regex {
                regex.is_match(&current_val_cow)
              } else {
                false // Pattern provided but regex compilation failed
              }
            }
          };
          if matched {
            current_val_cow = Cow::Borrowed("currentcolor");
            modified = true;
          }
        }
      }

      // If value is "currentcolor" or "none", further color conversions are usually skipped.
      // The JS plugin continues processing, which might be fine.
      // Let's see: if val is "currentcolor", names2hex/rgb2hex won't match.
      // convertCase could change "currentcolor" to "CURRENTCOLOR".
      // shorthex/shortname won't match. So it's probably fine.

      if current_val_cow == "currentcolor" || current_val_cow == "none" {
        // Update attribute if it was changed to currentcolor and skip further processing for it.
        if modified {
          *attr_value_ref =
            BumpString::from_str_in(current_val_cow.as_ref(), self.arena).into_bump_str();
        }
        continue; // Skip other conversions for "currentcolor" or "none"
      }

      // 2. Convert color name keyword to long hex
      if self.config.names_to_hex {
        // Lookup requires lowercase
        let potential_name_key = current_val_cow.to_lowercase();
        if let Some(hex_color) = COLORS_NAMES.get(potential_name_key.as_str()) {
          if current_val_cow != *hex_color {
            current_val_cow = Cow::Borrowed(hex_color); // Values from COLORS_NAMES are &'static str
            modified = true;
          }
        }
      }

      // 3. Convert rgb() to long hex
      if self.config.rgb_to_hex {
        if let Some(caps) = REG_RGB.captures(&current_val_cow) {
          let mut nums: [u8; 3] = [0; 3];
          let mut conversion_ok = true;
          for i in 0..3 {
            let m = caps.get(i + 1).unwrap().as_str(); // Captures are 1-indexed
            let val_str: Cow<str>;
            let is_percent = if m.ends_with('%') {
              val_str = Cow::Borrowed(&m[..m.len() - 1]);
              true
            } else {
              val_str = Cow::Borrowed(m);
              false
            };

            if let Ok(n_float) = val_str.parse::<f64>() {
              let n = if is_percent {
                (n_float * 2.55).round()
              } else {
                n_float.round()
              };
              // Clamp and convert to u8
              nums[i] = n.max(0.0).min(255.0) as u8;
            } else {
              conversion_ok = false; // Parsing failed
              break;
            }
          }
          if conversion_ok {
            let hex_color_str = self.rgb_to_hex_string(nums[0], nums[1], nums[2]);
            if current_val_cow != hex_color_str {
              current_val_cow = Cow::Owned(hex_color_str);
              modified = true;
            }
          }
        }
      }

      // 4. Convert case (upper/lower) for hex colors
      // The JS applies this more broadly; here we'll be more careful if it's a hex.
      // JS: `if (convertCase && !includesUrlReference(val))`
      if let Some(ref convention) = self.config.convert_case {
        if !self.includes_url_reference(&current_val_cow) {
          // Only apply to hex colors for safety, though JS is broader
          if current_val_cow.starts_with('#') {
            let original_case_val = current_val_cow.to_string(); // temp store
            let new_case_val = match convention {
              CaseConvention::Lower => current_val_cow.to_lowercase(),
              CaseConvention::Upper => current_val_cow.to_uppercase(),
            };
            if original_case_val != new_case_val {
              current_val_cow = Cow::Owned(new_case_val);
              modified = true;
            }
          }
        }
      }

      // 5. Convert long hex to short hex (e.g., #aabbcc -> #abc)
      // This should happen *after* case conversion if we want #ABC from #AABBCC with uppercase.
      // However, the output of shortener is usually lowercase: #abc
      if self.config.short_hex && current_val_cow.starts_with('#') && current_val_cow.len() == 7 {
        if let Some(caps) = REG_HEX_SHORTENABLE.captures(&current_val_cow) {
          // caps[1], caps[2], caps[3] are the chars for the short hex
          let r = caps.get(1).unwrap().as_str();
          let g = caps.get(2).unwrap().as_str();
          let b = caps.get(3).unwrap().as_str();

          // Preserve case from the original string if convert_case was 'upper'
          // Or, standardize to lower, which is more common for shorthex.
          // JS: '#' + match[0][1] + match[0][3] + match[0][5]; implies direct char copy.
          let short_hex_val = format!("#{}{}{}", r, g, b);

          if current_val_cow != short_hex_val {
            current_val_cow = Cow::Owned(short_hex_val);
            modified = true;
          }
        }
      }

      // 6. Convert hex to short name (e.g., #f00 -> red)
      if self.config.short_name && current_val_cow.starts_with('#') {
        let lower_hex_key = current_val_cow.to_lowercase(); // Lookup always with lowercase
        if let Some(short_name) = COLORS_SHORT_NAMES.get(lower_hex_key.as_str()) {
          if current_val_cow != *short_name {
            // Check if it's different before assigning
            current_val_cow = Cow::Borrowed(short_name); // short_name is &'static str
            modified = true;
          }
        }
      }

      if modified {
        if let Cow::Owned(owned_str) = current_val_cow {
          *attr_value_ref = BumpString::from_str_in(&owned_str, self.arena).into_bump_str();
        } else if let Cow::Borrowed(borrowed_str) = current_val_cow {
          // If it's borrowed but different from original_value, it means it came from a static source
          if borrowed_str != original_value {
            *attr_value_ref = BumpString::from_str_in(borrowed_str, self.arena).into_bump_str();
          }
          // If borrowed_str == original_value, no actual change happened, so no need to reassign.
        }
      }
    }
    VisitAction::Keep
  }

  fn element_exit(&mut self, el: &mut XMLAstElement<'a>) {
    if el.name == "mask" && self.mask_counter > 0 {
      // Should always be true if enter was paired
      self.mask_counter -= 1;
    }
  }
}
