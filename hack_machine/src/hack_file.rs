pub fn encode_image(image: &[u16]) -> String {
    image
        .iter()
        .map(|u| format!("{:016b}", u))
        .collect::<Vec<String>>()
        .join("\n")
}
