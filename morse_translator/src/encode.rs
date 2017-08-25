pub fn encode(input:String) -> String {
    let text = input.to_lowercase().trim().to_string();
    let chars = text.chars();
    let mut result = String::new();
    for c in chars {
        let code = match c {
            'a' => "._",
            'b' => "_...",
            'c' => "_._.",
            'd' => "_..",
            'e' => ".",
            'f' => ".._.",
            'g' => "__.",
            'h' => "....",
            'i' => "..",
            'j' => ".___",
            'k' => "_._",
            'l' => "._..",
            'm' => "__",
            'n' => "_.",
            'o' => "___",
            'p' => ".__.",
            'q' => "__._",
            'r' => "._.",
            's' => "...",
            't' => "_",
            'u' => ".._",
            'v' => "..._",
            'w' => ".__",
            'x' => "_.._",
            'y' => "_.__",
            'z' => "__..",
            '0' => "_____",
            '1' => ".____",
            '2' => "..___",
            '3' => "...__",
            '4' => "...._",
            '5' => ".....",
            '6' => "_....",
            '7' => "__...",
            '8' => "___..",
            '9' => "____.",
            ' ' => "/",
            '.' => "//",
            '\n' => "///",
            _ => panic!("No translation found for morse character {}", c)
        };
        result.push_str(code);
        result.push('/');
    }
    result
}