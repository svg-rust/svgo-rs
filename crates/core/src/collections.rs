use std::collections::HashMap;

// https://www.w3.org/TR/SVG11/intro.html#Definitions
pub fn get_elems_groups() -> HashMap<&'static str, Vec<&'static str>> {
    HashMap::from([
        ("animation", vec![
            "animate",
            "animateColor",
            "animateMotion",
            "animateTransform",
            "set",
        ]),
        ("descriptive", vec!["desc", "metadata", "title"]),
        ("shape", vec!["circle", "ellipse", "line", "path", "polygon", "polyline", "rect"]),
        ("structural", vec!["defs", "g", "svg", "symbol", "use"]),
        ("paintServer", vec![
            "solidColor",
            "linearGradient",
            "radialGradient",
            "meshGradient",
            "pattern",
            "hatch",
        ]),
        ("nonRendering", vec![
            "linearGradient",
            "radialGradient",
            "pattern",
            "clipPath",
            "mask",
            "marker",
            "symbol",
            "filter",
            "solidColor",
        ]),
        ("container", vec![
            "a",
            "defs",
            "g",
            "marker",
            "mask",
            "missing-glyph",
            "pattern",
            "svg",
            "switch",
            "symbol",
            "foreignObject",
        ]),
        ("textContent", vec![
            "altGlyph",
            "altGlyphDef",
            "altGlyphItem",
            "glyph",
            "glyphRef",
            "textPath",
            "text",
            "tref",
            "tspan",
        ]),
        ("textContentChild", vec!["altGlyph", "textPath", "tref", "tspan"]),
        ("lightSource", vec![
            "feDiffuseLighting",
            "feSpecularLighting",
            "feDistantLight",
            "fePointLight",
            "feSpotLight",
        ]),
        ("filterPrimitive", vec![
            "feBlend",
            "feColorMatrix",
            "feComponentTransfer",
            "feComposite",
            "feConvolveMatrix",
            "feDiffuseLighting",
            "feDisplacementMap",
            "feDropShadow",
            "feFlood",
            "feFuncA",
            "feFuncB",
            "feFuncG",
            "feFuncR",
            "feGaussianBlur",
            "feImage",
            "feMerge",
            "feMergeNode",
            "feMorphology",
            "feOffset",
            "feSpecularLighting",
            "feTile",
            "feTurbulence",
        ]),
    ])
}

pub fn get_text_elems() -> Vec<&'static str> {
    let mut elems_groups = get_elems_groups();
    let text_elems = elems_groups.get_mut("textContent").unwrap();
    text_elems.push("title");
    text_elems.to_vec()
}

// https://www.w3.org/TR/SVG11/propidx.html
pub fn get_inheritable_attrs() -> Vec<&'static str> {
    vec![
        "clip-rule",
        "color",
        "color-interpolation",
        "color-interpolation-filters",
        "color-profile",
        "color-rendering",
        "cursor",
        "direction",
        "dominant-baseline",
        "fill",
        "fill-opacity",
        "fill-rule",
        "font",
        "font-family",
        "font-size",
        "font-size-adjust",
        "font-stretch",
        "font-style",
        "font-variant",
        "font-weight",
        "glyph-orientation-horizontal",
        "glyph-orientation-vertical",
        "image-rendering",
        "letter-spacing",
        "marker",
        "marker-end",
        "marker-mid",
        "marker-start",
        "paint-order",
        "pointer-events",
        "shape-rendering",
        "stroke",
        "stroke-dasharray",
        "stroke-dashoffset",
        "stroke-linecap",
        "stroke-linejoin",
        "stroke-miterlimit",
        "stroke-opacity",
        "stroke-width",
        "text-anchor",
        "text-rendering",
        "transform",
        "visibility",
        "word-spacing",
        "writing-mode",
    ]
}

// https://www.w3.org/TR/SVG11/linking.html#processingIRI
pub fn get_references_props() -> Vec<&'static str> {
    vec![
        "clip-path",
        "color-profile",
        "fill",
        "filter",
        "marker-start",
        "marker-mid",
        "marker-end",
        "mask",
        "stroke",
        "style",
    ]
}

pub fn get_colors_names() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("aliceblue", "#f0f8ff"),
        ("antiquewhite", "#faebd7"),
        ("aqua", "#0ff"),
        ("aquamarine", "#7fffd4"),
        ("azure", "#f0ffff"),
        ("beige", "#f5f5dc"),
        ("bisque", "#ffe4c4"),
        ("black", "#000"),
        ("blanchedalmond", "#ffebcd"),
        ("blue", "#00f"),
        ("blueviolet", "#8a2be2"),
        ("brown", "#a52a2a"),
        ("burlywood", "#deb887"),
        ("cadetblue", "#5f9ea0"),
        ("chartreuse", "#7fff00"),
        ("chocolate", "#d2691e"),
        ("coral", "#ff7f50"),
        ("cornflowerblue", "#6495ed"),
        ("cornsilk", "#fff8dc"),
        ("crimson", "#dc143c"),
        ("cyan", "#0ff"),
        ("darkblue", "#00008b"),
        ("darkcyan", "#008b8b"),
        ("darkgoldenrod", "#b8860b"),
        ("darkgray", "#a9a9a9"),
        ("darkgreen", "#006400"),
        ("darkgrey", "#a9a9a9"),
        ("darkkhaki", "#bdb76b"),
        ("darkmagenta", "#8b008b"),
        ("darkolivegreen", "#556b2f"),
        ("darkorange", "#ff8c00"),
        ("darkorchid", "#9932cc"),
        ("darkred", "#8b0000"),
        ("darksalmon", "#e9967a"),
        ("darkseagreen", "#8fbc8f"),
        ("darkslateblue", "#483d8b"),
        ("darkslategray", "#2f4f4f"),
        ("darkslategrey", "#2f4f4f"),
        ("darkturquoise", "#00ced1"),
        ("darkviolet", "#9400d3"),
        ("deeppink", "#ff1493"),
        ("deepskyblue", "#00bfff"),
        ("dimgray", "#696969"),
        ("dimgrey", "#696969"),
        ("dodgerblue", "#1e90ff"),
        ("firebrick", "#b22222"),
        ("floralwhite", "#fffaf0"),
        ("forestgreen", "#228b22"),
        ("fuchsia", "#f0f"),
        ("gainsboro", "#dcdcdc"),
        ("ghostwhite", "#f8f8ff"),
        ("gold", "#ffd700"),
        ("goldenrod", "#daa520"),
        ("gray", "#808080"),
        ("green", "#008000"),
        ("greenyellow", "#adff2f"),
        ("grey", "#808080"),
        ("honeydew", "#f0fff0"),
        ("hotpink", "#ff69b4"),
        ("indianred", "#cd5c5c"),
        ("indigo", "#4b0082"),
        ("ivory", "#fffff0"),
        ("khaki", "#f0e68c"),
        ("lavender", "#e6e6fa"),
        ("lavenderblush", "#fff0f5"),
        ("lawngreen", "#7cfc00"),
        ("lemonchiffon", "#fffacd"),
        ("lightblue", "#add8e6"),
        ("lightcoral", "#f08080"),
        ("lightcyan", "#e0ffff"),
        ("lightgoldenrodyellow", "#fafad2"),
        ("lightgray", "#d3d3d3"),
        ("lightgreen", "#90ee90"),
        ("lightgrey", "#d3d3d3"),
        ("lightpink", "#ffb6c1"),
        ("lightsalmon", "#ffa07a"),
        ("lightseagreen", "#20b2aa"),
        ("lightskyblue", "#87cefa"),
        ("lightslategray", "#789"),
        ("lightslategrey", "#789"),
        ("lightsteelblue", "#b0c4de"),
        ("lightyellow", "#ffffe0"),
        ("lime", "#0f0"),
        ("limegreen", "#32cd32"),
        ("linen", "#faf0e6"),
        ("magenta", "#f0f"),
        ("maroon", "#800000"),
        ("mediumaquamarine", "#66cdaa"),
        ("mediumblue", "#0000cd"),
        ("mediumorchid", "#ba55d3"),
        ("mediumpurple", "#9370db"),
        ("mediumseagreen", "#3cb371"),
        ("mediumslateblue", "#7b68ee"),
        ("mediumspringgreen", "#00fa9a"),
        ("mediumturquoise", "#48d1cc"),
        ("mediumvioletred", "#c71585"),
        ("midnightblue", "#191970"),
        ("mintcream", "#f5fffa"),
        ("mistyrose", "#ffe4e1"),
        ("moccasin", "#ffe4b5"),
        ("navajowhite", "#ffdead"),
        ("navy", "#000080"),
        ("oldlace", "#fdf5e6"),
        ("olive", "#808000"),
        ("olivedrab", "#6b8e23"),
        ("orange", "#ffa500"),
        ("orangered", "#ff4500"),
        ("orchid", "#da70d6"),
        ("palegoldenrod", "#eee8aa"),
        ("palegreen", "#98fb98"),
        ("paleturquoise", "#afeeee"),
        ("palevioletred", "#db7093"),
        ("papayawhip", "#ffefd5"),
        ("peachpuff", "#ffdab9"),
        ("peru", "#cd853f"),
        ("pink", "#ffc0cb"),
        ("plum", "#dda0dd"),
        ("powderblue", "#b0e0e6"),
        ("purple", "#800080"),
        ("rebeccapurple", "#639"),
        ("red", "#f00"),
        ("rosybrown", "#bc8f8f"),
        ("royalblue", "#4169e1"),
        ("saddlebrown", "#8b4513"),
        ("salmon", "#fa8072"),
        ("sandybrown", "#f4a460"),
        ("seagreen", "#2e8b57"),
        ("seashell", "#fff5ee"),
        ("sienna", "#a0522d"),
        ("silver", "#c0c0c0"),
        ("skyblue", "#87ceeb"),
        ("slateblue", "#6a5acd"),
        ("slategray", "#708090"),
        ("slategrey", "#708090"),
        ("snow", "#fffafa"),
        ("springgreen", "#00ff7f"),
        ("steelblue", "#4682b4"),
        ("tan", "#d2b48c"),
        ("teal", "#008080"),
        ("thistle", "#d8bfd8"),
        ("tomato", "#ff6347"),
        ("turquoise", "#40e0d0"),
        ("violet", "#ee82ee"),
        ("wheat", "#f5deb3"),
        ("white", "#fff"),
        ("whitesmoke", "#f5f5f5"),
        ("yellow", "#ff0"),
        ("yellowgreen", "#9acd32"),
    ])
}

pub fn get_colors_short_names() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("#f0ffff", "azure"),
        ("#f5f5dc", "beige"),
        ("#ffe4c4", "bisque"),
        ("#a52a2a", "brown"),
        ("#ff7f50", "coral"),
        ("#ffd700", "gold"),
        ("#808080", "gray"),
        ("#008000", "green"),
        ("#4b0082", "indigo"),
        ("#fffff0", "ivory"),
        ("#f0e68c", "khaki"),
        ("#faf0e6", "linen"),
        ("#800000", "maroon"),
        ("#000080", "navy"),
        ("#808000", "olive"),
        ("#ffa500", "orange"),
        ("#da70d6", "orchid"),
        ("#cd853f", "peru"),
        ("#ffc0cb", "pink"),
        ("#dda0dd", "plum"),
        ("#800080", "purple"),
        ("#f00", "red"),
        ("#ff0000", "red"),
        ("#fa8072", "salmon"),
        ("#a0522d", "sienna"),
        ("#c0c0c0", "silver"),
        ("#fffafa", "snow"),
        ("#d2b48c", "tan"),
        ("#008080", "teal"),
        ("#ff6347", "tomato"),
        ("#ee82ee", "violet"),
        ("#f5deb3", "wheat"),
    ])
}

// https://www.w3.org/TR/SVG11/single-page.html#types-DataTypeColor
pub fn get_colors_props() -> Vec<&'static str> {
    vec![
        "color",
        "fill",
        "stroke",
        "stop-color",
        "flood-color",
        "lighting-color",
    ]
}
