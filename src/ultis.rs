use std::{path::Path, fs, io::{Cursor, SeekFrom, Seek}};

use rodio::{OutputStream, Decoder, Sink};

// 遍历文件夹删除
pub fn delete_folder_contents(folder_path: &str) -> std::io::Result<()> {
    let path = Path::new(folder_path);

    // 遍历文件夹内的内容
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        // 如果是文件，则直接删除
        if entry_path.is_file() {
            fs::remove_file(entry_path)?;
        }
        // 如果是文件夹，则递归调用自身进行删除
        else if entry_path.is_dir() {
            delete_folder_contents(entry_path.to_str().unwrap())?;
            fs::remove_dir(entry_path)?;
        }
    }

    Ok(())
}

pub fn musicplay(stream:&[u8]){
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    //let file = BufReader::new(File::open(path).unwrap());
    let mut file = Cursor::new(stream.to_vec());
    file.seek(SeekFrom::Start(2)).unwrap();
    let source = Decoder::new(file).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}