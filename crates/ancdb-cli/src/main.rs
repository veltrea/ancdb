use ancdb_core::{Database, AncError};
use ancdb_protocol::{ProtocolCommand, ProtocolResponse, CommandResult, encode_response, decode_command};
use clap::Parser;
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    db_path: PathBuf,

    #[arg(long)]
    stdio: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let db = Database::open(&cli.db_path).map_err(|e| format!("DB Open Error: {:?}", e))?;

    if cli.stdio {
        run_stdio_mode(&db).await?;
    } else {
        println!("ANC-DB CLI v{} (Database: {:?})", env!("CARGO_PKG_VERSION"), cli.db_path);
        println!("Currently, only --stdio mode supports full protocol interactions.");
    }

    Ok(())
}

async fn run_stdio_mode(db: &Database) -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut len_buf = [0u8; 4];
        if stdin.read_exact(&mut len_buf).is_err() {
            break;
        }
        let len = u32::from_be_bytes(len_buf) as usize;
        let mut payload = vec![0u8; len];
        stdin.read_exact(&mut payload)?;

        let command = match decode_command(&payload) {
            Ok(cmd) => cmd,
            Err(e) => {
                send_error(&mut stdout, format!("Decode error: {:?}", e))?;
                continue;
            }
        };

        let result = match execute_command(db, command) {
            Ok(res) => res,
            Err(AncError::TransactionConflict) => ProtocolResponse::Error { 
                id: 0, 
                message: "Transaction conflict".into() 
            },
            Err(e) => ProtocolResponse::Error { 
                id: 0, 
                message: format!("Runtime error: {:?}", e) 
            },
        };

        let response_payload = encode_response(&result).expect("encode");
        let resp_len = (response_payload.len() as u32).to_be_bytes();
        stdout.write_all(&resp_len)?;
        stdout.write_all(&response_payload)?;
        stdout.flush()?;
    }
    Ok(())
}

fn execute_command(db: &Database, cmd: ProtocolCommand) -> Result<ProtocolResponse, AncError> {
    match cmd {
        ProtocolCommand::CreateTable { id, table_id, name } => {
            db.begin_tx(ancdb_core::TxMode::Write)?;
            db.create_table(table_id, name)?;
            db.commit_tx()?;
            Ok(ProtocolResponse::Ok { id, result: CommandResult::Success })
        }
        ProtocolCommand::Put { id, table_id, key, value } => {
            db.begin_tx(ancdb_core::TxMode::Write)?;
            db.put(table_id, key, &value)?;
            db.commit_tx()?;
            Ok(ProtocolResponse::Ok { id, result: CommandResult::Success })
        }
        _ => Ok(ProtocolResponse::Error { id: 0, message: "Not implemented".into() })
    }
}

fn send_error(stdout: &mut io::Stdout, msg: String) -> io::Result<()> {
    let resp = ProtocolResponse::Error { id: 0, message: msg };
    let payload = encode_response(&resp).expect("encode");
    let len = (payload.len() as u32).to_be_bytes();
    stdout.write_all(&len)?;
    stdout.write_all(&payload)?;
    stdout.flush()
}
