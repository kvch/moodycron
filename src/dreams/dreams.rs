pub struct Dreamer {
    server: String,
    prompt: String,
}

impl Dreamer {
    pub fn new() -> Dreamer {
        return Dreamer {
            server: "".into(),
            prompt: "".into(),
        };
    }

    pub fn dream(self) -> String {
        "this is my dream".into()
    }
}
