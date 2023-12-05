use std::fs::File; // importa o módulo de arquivos
use std::io::{self, BufWriter, Write}; // importa o módulo de entrada e saída
use std::thread; // importa o módulo de thread
use std::sync::mpsc; // importa o módulo de canais de comunicação

// define uma constante com a URL do arquivo que queremos baixar
const URL: &str = "https://www.football-data.co.uk/mmz4281/2324/E0.csv";

// define uma função que baixa o arquivo usando a biblioteca reqwest
// essa função retorna um Result com um vetor de bytes ou um erro
fn download_file(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    // cria um cliente HTTP usando o método new
    let client = reqwest::blocking::Client::new();
    // faz uma requisição GET para a URL usando o método get
    let response = client.get(url).send()?;
    // lê o corpo da resposta como um vetor de bytes usando o método bytes
    let bytes = response.bytes()?;
    // converte o vetor de bytes em um vetor de u8 usando o método to_vec
    let data = bytes.to_vec();
    // retorna o vetor de u8 como um Ok
    Ok(data)
}

fn main() -> io::Result<()> {
    // cria um canal de comunicação usando a função channel
    // esse canal vai permitir enviar e receber dados entre as threads
    let (sender, receiver) = mpsc::channel();
    // cria uma nova thread usando a função thread::spawn
    // e passa uma closure com o código que queremos executar na thread
    let handle = thread::spawn(move || {
        // chama a função download_file e passa a URL como argumento
        // se a função retornar um Ok, envia os dados pelo canal usando o método send
        // se a função retornar um Err, imprime o erro na tela usando o macro eprintln!
        match download_file(URL) {
            Ok(data) => {
                sender.send(data).unwrap();
            }
            Err(e) => {
                eprintln!("Erro ao baixar o arquivo: {}", e);
            }
        }
    });

    // cria um arquivo usando a função File::create e passa o nome do arquivo como argumento
    let file = File::create("Premier League 23-24.txt")?;
    // cria um buffer de escrita usando a função BufWriter::new e passa o arquivo como argumento
    let mut writer = BufWriter::new(file);
    // recebe os dados pelo canal usando o método recv
    // se o canal retornar um Ok, escreve os dados no arquivo usando o método write_all
    // se o canal retornar um Err, imprime o erro na tela usando o macro eprintln!
    match receiver.recv() {
        Ok(data) => {
            writer.write_all(&data)?;
        }
        Err(e) => {
            eprintln!("Erro ao receber os dados: {}", e);
        }
    }

    // espera a thread terminar antes de sair do programa usando o método join
    handle.join().unwrap();

    // retorna um Ok vazio
    Ok(())
}