use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ProtocolCommand {
    CreateTable { id: u32, table_id: u32, name: String },
    Put { id: u32, table_id: u32, key: i64, value: Vec<u8> },
    DirectRead { id: u32, table_id: u32, key: i64 },
    RangeScan { 
        id: u32, 
        table_id: u32, 
        start_key: i64, 
        end_key: i64, 
        desc: bool, 
        limit: usize 
    },
    BeginTransaction { id: u32, mode: String },
    CommitTransaction { id: u32 },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProtocolResponse {
    Ok { id: u32, result: CommandResult },
    Error { id: u32, message: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandResult {
    Success,
    Value(Vec<u8>),
    ScanResult(Vec<(i64, Vec<u8>)>),
}

pub fn encode_command(cmd: &ProtocolCommand) -> Result<Vec<u8>, rmp_serde::encode::Error> {
    rmp_serde::to_vec(cmd)
}

pub fn decode_command(buf: &[u8]) -> Result<ProtocolCommand, rmp_serde::decode::Error> {
    rmp_serde::from_slice(buf)
}

pub fn encode_response(res: &ProtocolResponse) -> Result<Vec<u8>, rmp_serde::encode::Error> {
    rmp_serde::to_vec(res)
}

pub fn decode_response(buf: &[u8]) -> Result<ProtocolResponse, rmp_serde::decode::Error> {
    rmp_serde::from_slice(buf)
}
