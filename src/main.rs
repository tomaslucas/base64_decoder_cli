use base64::{engine::general_purpose, Engine as _};
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
//let m = command!().get_matches();
/// Simple programa
struct Args {
    /// Input base64 file to decode
    #[arg(
        short,
        long,
        value_name("ARCHIVO"),
        required(true),
        help("Archivo en base64 para decodificar")
    )]
    input: String,

    /// Word to extract
    #[arg(
        short,
        long,
        value_name("PALABRA"),
        help("Palabra a extraer de la salida decodificada"),
        default_value_t = String::from("PRO"),
    )]
    word: String,
}

fn main() {
    //let m = command!().get_matches();
    let args = Args::parse();
    // Leer el nombre del archivo de entrada desde los argumentos
    let input_file = &args.input;
    // Leer el contenido del archivo en base64
    let base64_content = std::fs::read_to_string(input_file).unwrap();
    // Decodificar el contenido en base64
    let decoded_content = general_purpose::STANDARD.decode(base64_content).unwrap();
    // Convertir el contenido decodificado en una cadena UTF-8
    let utf8_content = String::from_utf8_lossy(&decoded_content);
    // Imprimir la salida decodificada
    println!("Contenido decodificado:\n{}", utf8_content);

    // Si se proporcionó el argumento 'word', extraer la palabra específica
    if !args.word.is_empty() {
        let extracted_word = utf8_content
            .split_whitespace()
            .find(|&w| w == args.word)
            .unwrap_or("KO");
        if extracted_word == "KO" {
            println!(
                "La palabra: {} -> NO se encuentra en el fichero.",
                &args.word
            );
        } else {
            println!("La palabra: {} -> existe en el fichero.", &args.word);
        }
    }
}
