extern crate clap;
use clap::{App, Arg};

fn main() {
    // Definir la interfaz de línea de comandos usando clap
    let matches = App::new("Base64 Decoder CLI")
        .version("1.0")
        .author("Tu Nombre")
        .about("Decodifica una cadena en base64 desde un archivo y extrae una palabra específica")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("ARCHIVO")
                .required(true)
                .help("Archivo en base64 para decodificar"),
        )
        .arg(
            Arg::with_name("word")
                .short("w")
                .long("word")
                .value_name("PALABRA")
                .help("Palabra a extraer de la salida decodificada")
                .takes_value(true),
        )
        .get_matches();

    // Leer el nombre del archivo de entrada desde los argumentos
    let input_file = matches.value_of("input").unwrap();

    // Leer el contenido del archivo en base64
    let base64_content = std::fs::read_to_string(input_file).unwrap();

    // Decodificar el contenido en base64
    let decoded_content = base64::decode(&base64_content).unwrap();

    // Convertir el contenido decodificado en una cadena UTF-8
    let utf8_content = String::from_utf8_lossy(&decoded_content);

    // Imprimir la salida decodificada
    println!("Contenido decodificado:\n{}", utf8_content);

    // Si se proporcionó el argumento 'word', extraer la palabra específica
    if let Some(word) = matches.value_of("word") {
        let extracted_word = utf8_content
            .split_whitespace()
            .find(|&w| w == word)
            .unwrap_or("La palabra no fue encontrada");
        println!("Palabra extraída: {}", extracted_word);
    }
}
