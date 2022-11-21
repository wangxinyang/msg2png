use anyhow::Result;
use chunk::Chunk;
use chunk_type::ChunkType;
use png::Png;
use std::fs;
use std::str::FromStr;

mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;

pub fn handle_cli() -> Result<()> {
    let cli = commands::parse_commands().unwrap();
    match cli.command {
        commands::PngMeArgs::Encode(cmd) => {
            // get basic info
            let input_file = &cmd.file_path;
            let chunk_type = ChunkType::from_str(cmd.chunk_type.as_str())?;
            let chunk_data = cmd.message.clone().into_bytes();
            let message_chunk = Chunk::new(chunk_type, chunk_data);
            // create Png from file path
            let mut png = Png::from_file(input_file)?;
            // add secret message chunk
            png.append_chunk(message_chunk);
            // output new file path
            if let Some(output_path) = &cmd.output_file {
                fs::write(output_path, png.as_bytes())?;
            }
        }
        commands::PngMeArgs::Decode(cmd) => {
            let input_file = &cmd.file_path;
            let chunk_type = cmd.chunk_type.as_str();
            let png = Png::from_file(input_file)?;
            let message = png.data_string_by_type(chunk_type);
            match message {
                None => {
                    println!("no such message for chunk_type: {}.", chunk_type)
                }
                Some(msg) => {
                    println!("secret msg for {} is: {}", chunk_type, msg)
                }
            }
        }
        commands::PngMeArgs::Remove(cmd) => {
            let input_file = &cmd.file_path;
            let chunk_type = cmd.chunk_type.as_str();
            let mut png = Png::from_file(input_file)?;
            let _chunk_removed = png.remove_chunk(chunk_type)?;
            fs::write(input_file, png.as_bytes())?;
            println!("remove chunk type: {}", chunk_type);
        }
        commands::PngMeArgs::Print(cmd) => {
            let input_file = &cmd.file_path;
            let png = Png::from_file(input_file)?;
            let v = &png.chunks;
            let num = v.len();
            println!("====================all chunk type({num})====================");
            // for i in v {
            // print!("{}:{};", i + 1, v[i].chunk_type().to_string());
            // }
        }
    }

    Ok(())
}
