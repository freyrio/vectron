// Named colors for Vectron Render
//
// This module provides a comprehensive set of named colors for convenience.
// The names and values are based on commonly accepted web standards.

use crate::color::Color;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Gets a mapping of all named colors.
pub fn color_map() -> &'static HashMap<&'static str, Color> {
    static COLORS: OnceLock<HashMap<&'static str, Color>> = OnceLock::new();
    COLORS.get_or_init(|| {
        let mut m = HashMap::new();
        
        // Basic colors
        m.insert("transparent", Color::TRANSPARENT);
        m.insert("black", Color::BLACK);
        m.insert("white", Color::WHITE);
        m.insert("red", Color::RED);
        m.insert("green", Color::GREEN);
        m.insert("blue", Color::BLUE);
        
        // Extended basic colors
        m.insert("yellow", Color::from_rgb8(255, 255, 0));
        m.insert("cyan", Color::from_rgb8(0, 255, 255));
        m.insert("magenta", Color::from_rgb8(255, 0, 255));
        m.insert("silver", Color::from_rgb8(192, 192, 192));
        m.insert("gray", Color::from_rgb8(128, 128, 128));
        m.insert("maroon", Color::from_rgb8(128, 0, 0));
        m.insert("olive", Color::from_rgb8(128, 128, 0));
        m.insert("purple", Color::from_rgb8(128, 0, 128));
        m.insert("teal", Color::from_rgb8(0, 128, 128));
        m.insert("navy", Color::from_rgb8(0, 0, 128));
        
        // Extended web colors
        m.insert("aliceblue", Color::from_rgb8(240, 248, 255));
        m.insert("antiquewhite", Color::from_rgb8(250, 235, 215));
        m.insert("aqua", Color::from_rgb8(0, 255, 255));
        m.insert("aquamarine", Color::from_rgb8(127, 255, 212));
        m.insert("azure", Color::from_rgb8(240, 255, 255));
        m.insert("beige", Color::from_rgb8(245, 245, 220));
        m.insert("bisque", Color::from_rgb8(255, 228, 196));
        m.insert("blanchedalmond", Color::from_rgb8(255, 235, 205));
        m.insert("blueviolet", Color::from_rgb8(138, 43, 226));
        m.insert("brown", Color::from_rgb8(165, 42, 42));
        m.insert("burlywood", Color::from_rgb8(222, 184, 135));
        m.insert("cadetblue", Color::from_rgb8(95, 158, 160));
        m.insert("chartreuse", Color::from_rgb8(127, 255, 0));
        m.insert("chocolate", Color::from_rgb8(210, 105, 30));
        m.insert("coral", Color::from_rgb8(255, 127, 80));
        m.insert("cornflowerblue", Color::from_rgb8(100, 149, 237));
        m.insert("cornsilk", Color::from_rgb8(255, 248, 220));
        m.insert("crimson", Color::from_rgb8(220, 20, 60));
        m.insert("darkblue", Color::from_rgb8(0, 0, 139));
        m.insert("darkcyan", Color::from_rgb8(0, 139, 139));
        m.insert("darkgoldenrod", Color::from_rgb8(184, 134, 11));
        m.insert("darkgray", Color::from_rgb8(169, 169, 169));
        m.insert("darkgreen", Color::from_rgb8(0, 100, 0));
        m.insert("darkkhaki", Color::from_rgb8(189, 183, 107));
        m.insert("darkmagenta", Color::from_rgb8(139, 0, 139));
        m.insert("darkolivegreen", Color::from_rgb8(85, 107, 47));
        m.insert("darkorange", Color::from_rgb8(255, 140, 0));
        m.insert("darkorchid", Color::from_rgb8(153, 50, 204));
        m.insert("darkred", Color::from_rgb8(139, 0, 0));
        m.insert("darksalmon", Color::from_rgb8(233, 150, 122));
        m.insert("darkseagreen", Color::from_rgb8(143, 188, 143));
        m.insert("darkslateblue", Color::from_rgb8(72, 61, 139));
        m.insert("darkslategray", Color::from_rgb8(47, 79, 79));
        m.insert("darkturquoise", Color::from_rgb8(0, 206, 209));
        m.insert("darkviolet", Color::from_rgb8(148, 0, 211));
        m.insert("deeppink", Color::from_rgb8(255, 20, 147));
        m.insert("deepskyblue", Color::from_rgb8(0, 191, 255));
        m.insert("dimgray", Color::from_rgb8(105, 105, 105));
        m.insert("dodgerblue", Color::from_rgb8(30, 144, 255));
        m.insert("firebrick", Color::from_rgb8(178, 34, 34));
        m.insert("floralwhite", Color::from_rgb8(255, 250, 240));
        m.insert("forestgreen", Color::from_rgb8(34, 139, 34));
        m.insert("fuchsia", Color::from_rgb8(255, 0, 255));
        m.insert("gainsboro", Color::from_rgb8(220, 220, 220));
        m.insert("ghostwhite", Color::from_rgb8(248, 248, 255));
        m.insert("gold", Color::from_rgb8(255, 215, 0));
        m.insert("goldenrod", Color::from_rgb8(218, 165, 32));
        m.insert("greenyellow", Color::from_rgb8(173, 255, 47));
        m.insert("honeydew", Color::from_rgb8(240, 255, 240));
        m.insert("hotpink", Color::from_rgb8(255, 105, 180));
        m.insert("indianred", Color::from_rgb8(205, 92, 92));
        m.insert("indigo", Color::from_rgb8(75, 0, 130));
        m.insert("ivory", Color::from_rgb8(255, 255, 240));
        m.insert("khaki", Color::from_rgb8(240, 230, 140));
        m.insert("lavender", Color::from_rgb8(230, 230, 250));
        m.insert("lavenderblush", Color::from_rgb8(255, 240, 245));
        m.insert("lawngreen", Color::from_rgb8(124, 252, 0));
        m.insert("lemonchiffon", Color::from_rgb8(255, 250, 205));
        m.insert("lightblue", Color::from_rgb8(173, 216, 230));
        m.insert("lightcoral", Color::from_rgb8(240, 128, 128));
        m.insert("lightcyan", Color::from_rgb8(224, 255, 255));
        m.insert("lightgoldenrodyellow", Color::from_rgb8(250, 250, 210));
        m.insert("lightgray", Color::from_rgb8(211, 211, 211));
        m.insert("lightgreen", Color::from_rgb8(144, 238, 144));
        m.insert("lightpink", Color::from_rgb8(255, 182, 193));
        m.insert("lightsalmon", Color::from_rgb8(255, 160, 122));
        m.insert("lightseagreen", Color::from_rgb8(32, 178, 170));
        m.insert("lightskyblue", Color::from_rgb8(135, 206, 250));
        m.insert("lightslategray", Color::from_rgb8(119, 136, 153));
        m.insert("lightsteelblue", Color::from_rgb8(176, 196, 222));
        m.insert("lightyellow", Color::from_rgb8(255, 255, 224));
        m.insert("lime", Color::from_rgb8(0, 255, 0));
        m.insert("limegreen", Color::from_rgb8(50, 205, 50));
        m.insert("linen", Color::from_rgb8(250, 240, 230));
        m.insert("mediumaquamarine", Color::from_rgb8(102, 205, 170));
        m.insert("mediumblue", Color::from_rgb8(0, 0, 205));
        m.insert("mediumorchid", Color::from_rgb8(186, 85, 211));
        m.insert("mediumpurple", Color::from_rgb8(147, 112, 219));
        m.insert("mediumseagreen", Color::from_rgb8(60, 179, 113));
        m.insert("mediumslateblue", Color::from_rgb8(123, 104, 238));
        m.insert("mediumspringgreen", Color::from_rgb8(0, 250, 154));
        m.insert("mediumturquoise", Color::from_rgb8(72, 209, 204));
        m.insert("mediumvioletred", Color::from_rgb8(199, 21, 133));
        m.insert("midnightblue", Color::from_rgb8(25, 25, 112));
        m.insert("mintcream", Color::from_rgb8(245, 255, 250));
        m.insert("mistyrose", Color::from_rgb8(255, 228, 225));
        m.insert("moccasin", Color::from_rgb8(255, 228, 181));
        m.insert("navajowhite", Color::from_rgb8(255, 222, 173));
        m.insert("oldlace", Color::from_rgb8(253, 245, 230));
        m.insert("olivedrab", Color::from_rgb8(107, 142, 35));
        m.insert("orange", Color::from_rgb8(255, 165, 0));
        m.insert("orangered", Color::from_rgb8(255, 69, 0));
        m.insert("orchid", Color::from_rgb8(218, 112, 214));
        m.insert("palegoldenrod", Color::from_rgb8(238, 232, 170));
        m.insert("palegreen", Color::from_rgb8(152, 251, 152));
        m.insert("paleturquoise", Color::from_rgb8(175, 238, 238));
        m.insert("palevioletred", Color::from_rgb8(219, 112, 147));
        m.insert("papayawhip", Color::from_rgb8(255, 239, 213));
        m.insert("peachpuff", Color::from_rgb8(255, 218, 185));
        m.insert("peru", Color::from_rgb8(205, 133, 63));
        m.insert("pink", Color::from_rgb8(255, 192, 203));
        m.insert("plum", Color::from_rgb8(221, 160, 221));
        m.insert("powderblue", Color::from_rgb8(176, 224, 230));
        m.insert("rosybrown", Color::from_rgb8(188, 143, 143));
        m.insert("royalblue", Color::from_rgb8(65, 105, 225));
        m.insert("saddlebrown", Color::from_rgb8(139, 69, 19));
        m.insert("salmon", Color::from_rgb8(250, 128, 114));
        m.insert("sandybrown", Color::from_rgb8(244, 164, 96));
        m.insert("seagreen", Color::from_rgb8(46, 139, 87));
        m.insert("seashell", Color::from_rgb8(255, 245, 238));
        m.insert("sienna", Color::from_rgb8(160, 82, 45));
        m.insert("skyblue", Color::from_rgb8(135, 206, 235));
        m.insert("slateblue", Color::from_rgb8(106, 90, 205));
        m.insert("slategray", Color::from_rgb8(112, 128, 144));
        m.insert("snow", Color::from_rgb8(255, 250, 250));
        m.insert("springgreen", Color::from_rgb8(0, 255, 127));
        m.insert("steelblue", Color::from_rgb8(70, 130, 180));
        m.insert("tan", Color::from_rgb8(210, 180, 140));
        m.insert("thistle", Color::from_rgb8(216, 191, 216));
        m.insert("tomato", Color::from_rgb8(255, 99, 71));
        m.insert("turquoise", Color::from_rgb8(64, 224, 208));
        m.insert("violet", Color::from_rgb8(238, 130, 238));
        m.insert("wheat", Color::from_rgb8(245, 222, 179));
        m.insert("whitesmoke", Color::from_rgb8(245, 245, 245));
        m.insert("yellowgreen", Color::from_rgb8(154, 205, 50));
        
        m
    })
}

/// Gets a named color from the predefined set.
///
/// Returns None if the color name is not recognized.
pub fn get_color(name: &str) -> Option<Color> {
    color_map().get(name.to_lowercase().as_str()).copied()
}

/// Tries to parse a color from a string, supporting various formats:
/// - Named colors ("red", "blue", etc.)
/// - Hex codes ("#ff0000", "#f00")
/// - RGB/RGBA functions ("rgb(255, 0, 0)", "rgba(255, 0, 0, 0.5)")
pub fn parse_color(color_str: &str) -> Option<Color> {
    let color_str = color_str.trim();
    
    // Try as a named color
    if let Some(color) = get_color(color_str) {
        return Some(color);
    }
    
    // Try as a hex code
    if color_str.starts_with('#') {
        return parse_hex_color(color_str);
    }
    
    // Try as RGB/RGBA function
    if color_str.starts_with("rgb(") || color_str.starts_with("rgba(") {
        return parse_rgb_function(color_str);
    }
    
    None
}

fn parse_hex_color(hex_str: &str) -> Option<Color> {
    let hex_str = hex_str.trim_start_matches('#');
    
    // Normalize shorthand form (#rgb) to expanded form (#rrggbb)
    let expanded_hex = if hex_str.len() == 3 {
        format!(
            "{}{}{}{}{}{}",
            hex_str.chars().nth(0)?,
            hex_str.chars().nth(0)?,
            hex_str.chars().nth(1)?,
            hex_str.chars().nth(1)?,
            hex_str.chars().nth(2)?,
            hex_str.chars().nth(2)?
        )
    } else {
        hex_str.to_string()
    };
    
    // Parse as #RRGGBB
    if expanded_hex.len() == 6 {
        let r = u8::from_str_radix(&expanded_hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&expanded_hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&expanded_hex[4..6], 16).ok()?;
        return Some(Color::from_rgb8(r, g, b));
    }
    
    // Parse as #RRGGBBAA
    if expanded_hex.len() == 8 {
        let r = u8::from_str_radix(&expanded_hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&expanded_hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&expanded_hex[4..6], 16).ok()?;
        let a = u8::from_str_radix(&expanded_hex[6..8], 16).ok()?;
        return Some(Color::from_rgba8(r, g, b, a));
    }
    
    None
}

fn parse_rgb_function(rgb_str: &str) -> Option<Color> {
    // Extract values between parentheses
    let start = rgb_str.find('(')?;
    let end = rgb_str.rfind(')')?;
    let values_str = &rgb_str[start + 1..end];
    
    // Split by commas
    let values: Vec<&str> = values_str.split(',').map(|s| s.trim()).collect();
    
    // Parse as rgb(r, g, b)
    if values.len() == 3 {
        let r = parse_rgb_value(values[0])?;
        let g = parse_rgb_value(values[1])?;
        let b = parse_rgb_value(values[2])?;
        return Some(Color::from_rgb8(r, g, b));
    }
    
    // Parse as rgba(r, g, b, a)
    if values.len() == 4 {
        let r = parse_rgb_value(values[0])?;
        let g = parse_rgb_value(values[1])?;
        let b = parse_rgb_value(values[2])?;
        let a = parse_alpha_value(values[3])?;
        return Some(Color::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a,
        ));
    }
    
    None
}

fn parse_rgb_value(value: &str) -> Option<u8> {
    // Parse value as 0-255 integer
    if let Ok(val) = value.parse::<u8>() {
        return Some(val);
    }
    
    // Parse value as percentage
    if value.ends_with('%') {
        let perc_str = value.trim_end_matches('%');
        if let Ok(percentage) = perc_str.parse::<f32>() {
            return Some(((percentage / 100.0) * 255.0).round() as u8);
        }
    }
    
    None
}

fn parse_alpha_value(value: &str) -> Option<f32> {
    // Parse value as float
    if let Ok(val) = value.parse::<f32>() {
        return Some(val.clamp(0.0, 1.0));
    }
    
    // Parse value as percentage
    if value.ends_with('%') {
        let perc_str = value.trim_end_matches('%');
        if let Ok(percentage) = perc_str.parse::<f32>() {
            return Some((percentage / 100.0).clamp(0.0, 1.0));
        }
    }
    
    None
}

/// Common named colors as constants
pub mod colors {
    use crate::color::Color;
    
    // Basic colors
    pub const TRANSPARENT: Color = Color::TRANSPARENT;
    pub const BLACK: Color = Color::BLACK;
    pub const WHITE: Color = Color::WHITE;
    pub const RED: Color = Color::RED;
    pub const GREEN: Color = Color::GREEN;
    pub const BLUE: Color = Color::BLUE;
    
    // Extended basic colors
    pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const MAGENTA: Color = Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const SILVER: Color = Color { r: 0.75, g: 0.75, b: 0.75, a: 1.0 };
    pub const GRAY: Color = Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 };
    pub const MAROON: Color = Color { r: 0.5, g: 0.0, b: 0.0, a: 1.0 };
    pub const OLIVE: Color = Color { r: 0.5, g: 0.5, b: 0.0, a: 1.0 };
    pub const PURPLE: Color = Color { r: 0.5, g: 0.0, b: 0.5, a: 1.0 };
    pub const TEAL: Color = Color { r: 0.0, g: 0.5, b: 0.5, a: 1.0 };
    pub const NAVY: Color = Color { r: 0.0, g: 0.0, b: 0.5, a: 1.0 };
    
    // Additional common colors
    pub const ORANGE: Color = Color { r: 1.0, g: 0.65, b: 0.0, a: 1.0 };
    pub const PINK: Color = Color { r: 1.0, g: 0.75, b: 0.8, a: 1.0 };
    pub const GOLD: Color = Color { r: 1.0, g: 0.84, b: 0.0, a: 1.0 };
    pub const LIME: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const INDIGO: Color = Color { r: 0.29, g: 0.0, b: 0.51, a: 1.0 };
    pub const VIOLET: Color = Color { r: 0.93, g: 0.51, b: 0.93, a: 1.0 };
} 