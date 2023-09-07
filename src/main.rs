use base64_decoder_cli::decode_base64::decode_base64;
use base64_decoder_cli::read_file::read_file;
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
    let base64_content = read_file(input_file);
    let utf8_content = decode_base64(&base64_content);
    // // Imprimir la salida decodificada
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
