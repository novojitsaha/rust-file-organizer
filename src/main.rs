use std::fs;

fn main() {
    
    let dirPath = "./test";

    if let Ok(entries) = fs::read_dir(dirPath){
        for entry in entries{
            if let Ok(entry) = entry{
                let entryPath = entry.path();
                let entryName = entry.file_name();

                if entryPath.is_file(){
                    println!("File name: {:?}", entryName);
                } else if entryPath.is_dir(){
                    println!("Directory: {:?}", entryName );
                }

            }
        }
    } else {
        eprintln!("Failed to read directory: {:?}", dirPath);
    }
}


