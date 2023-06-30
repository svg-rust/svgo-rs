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
