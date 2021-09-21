use design_pattern_generator::{CodeGenerator, InputParser};

fn main() {
    let mut cpp_header = design_pattern_generator::CppHeaderBuilderDesignPatternCodeGenerator::new();
    let mut cpp_source = design_pattern_generator::CppSourceBuilderDesignPatternCodeGenerator::new();
    {
        let mut parser = design_pattern_generator::JsonParser::new();
        parser.attach(&mut cpp_header);
        parser.attach(&mut cpp_source);
        let json = r#"{
            "name":"MyClass",
            "members":[
                {
                    "the_type":"int",
                    "name":"counter"
                }
            ],
            "functions":[
                {
                    "return_type":"int",
                    "name":"increment"
                }
            ]
        }"#;
        parser.parse(&json);
    }
    cpp_header.generate(&mut std::io::stdout());
    cpp_source.generate(&mut std::io::stdout());
}
