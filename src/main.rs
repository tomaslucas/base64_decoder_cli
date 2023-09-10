use base64_decoder_cli::decode_base64::decode_base64;
use base64_decoder_cli::read_file::read_file;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(before_help = "License MIT OR Apache-2.0")]
#[command(name = "BASE64_DECODER_CLI")]
#[command(version = "0.0.1")]
#[command(author = "Tomás Lucas <tomaslucas@gmail.com>")]
#[command(
    about = "Decodifica una cadena en base64 desde un archivo y extrae una palabra específica"
)]
#[command(help_template("
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}
{all-args}{after-help}
"))]
#[command(after_help = "Esta es una prueba con CLAP.")]
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
        //default_value_t = String::from("PRO"),
        
    )]
    word: Option<String>,
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
    if args.word.is_some() {
        let extracted_word = utf8_content
            .split_whitespace()
            .find(|&w| w == args.word.as_ref().unwrap())
            .unwrap_or("KO");
        if extracted_word == "KO" {
            println!(
                "La palabra: {} -> NO se encuentra en el fichero.",
                &args.word.unwrap()
            );
        } else {
            println!("La palabra: {} -> existe en el fichero.", &args.word.unwrap());
        }
    }
}
