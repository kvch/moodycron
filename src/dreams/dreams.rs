use futures::executor;
use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;

pub struct Dreamer {
    server: Ollama,
    model: String,
    prompt: String,
}

impl Dreamer {
    pub fn new() -> Dreamer {
        return Dreamer {
            server: Ollama::default(),
            model: "tinydolphin".into(),
            prompt: "Act as someone who just woke up.".into(),
        };
    }

    pub fn dream(self) -> String {
        let dream = executor::block_on(
            self.server
                .generate(GenerationRequest::new(self.model, self.prompt)),
        );
        return match dream {
            Ok(dream) => dream.response,
            Err(e) => e.to_string(),
        };
    }
}
