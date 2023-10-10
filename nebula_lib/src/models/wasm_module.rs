pub struct WasmModule<Input = String, Output = Option<String>> {
    pub module_name: String,
    pub input: Input,
    pub output: Option<Output>,
}

#[allow(dead_code)]
impl<Input, Output> WasmModule<Input, Output> {
    fn new(module_name: String, input: Input) -> Self {
        WasmModule {
            module_name,
            input,
            output: None,
        }
    }

    fn set_output(&mut self, output: Output) {
        self.output = Some(output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const IMAGE_NAME_EXAMPLE: &str = "docker/test_test";
    #[test]
    fn test_create_wasm_module() {
        let module =
            WasmModule::<String, String>::new(IMAGE_NAME_EXAMPLE.to_string(), "5".to_string());
        assert_eq!(module.module_name, IMAGE_NAME_EXAMPLE)
    }
}
