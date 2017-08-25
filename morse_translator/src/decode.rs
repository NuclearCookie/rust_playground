pub fn decode(input:String) -> String {
    let text = input.replace("*", ".").replace("-","_").trim().to_string();
    let mut result = String::new();
    let lines = text.split("///");
    for line in lines {
        let words = line.split("//");
        for word in words {
            let characters = word.split("/");
            for c in characters {
                let letter = match c {
                "._" => 'a',
                "_..." => 'b',
                "_._." => 'c',
                "_.." => 'd',
                "." => 'e',
                ".._." => 'f',
                "__." => 'g',
                "...." => 'h',
                ".." => 'i',
                ".___" => 'j',
                "_._" => 'k',
                "._.." => 'l',
                "__" => 'm',
                "_." => 'n',
                "___" => 'o',
                ".__." => 'p',
                "__._" => 'q',
                "._." => 'r',
                "..." => 's',
                "_" => 't',
                ".._" => 'u',
                "..._" => 'v',
                ".__" => 'w',
                "_.._" => 'x',
                "_.__" => 'y',
                "__.." => 'z',
                "_____" => '0',
                ".____" => '1',
                "..___" => '2',
                "...__" => '3',
                "...._" => '4',
                "....." => '5',
                "_...." => '6',
                "__..." => '7',
                "___.." => '8',
                "____." => '9',
                "" => ' ',
                "/" => '.',
                "//" => '\n',
                _ => panic!("No translation found for char {}", c)
                };
                result.push(letter);
            }
        result.push(' ');
        }
    result.push('\n');
    }
    result
}