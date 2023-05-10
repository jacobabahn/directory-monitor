use walkdir::WalkDir;
use notify_rust::Notification;

fn get_directory_size(path: &str) -> f64 {

    let total_size = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.metadata().unwrap().len())
        .sum::<u64>();
    
    let gigabytes = total_size as f64 / 1024.0 / 1024.0 / 1024.0;
    println!("Total size: {} gigabytes", gigabytes);
    return gigabytes;
}

fn send_notification(directory: &str, size: f64) -> Result<(), Box<dyn std::error::Error>> {
    Notification::new()
        .summary("Large Folder Size")
        .body(&format!("{} is {} gigabytes", directory, size))
        .show()?;
    Ok(())
}

fn main() {
    let downloads_size: f64 = get_directory_size("/Users/jbahn/Downloads");
    let trash_size: f64 = get_directory_size("/Users/jbahn/.Trash");
    let desktop_size: f64 = get_directory_size("/Users/jbahn/Desktop");
    let documents_size: f64 = get_directory_size("/Users/jbahn/Documents");
     
    if downloads_size > 1.0 {
        send_notification("/Users/jbahn/Downloads", downloads_size).unwrap();
    }
    
    if trash_size > 1.0 {
        send_notification("/Users/jbahn/.Trash", trash_size).unwrap();
    }

    if desktop_size > 1.0 {
        send_notification("/Users/jbahn/Desktop", desktop_size).unwrap();
    }

    if documents_size > 1.0 {
        send_notification("/Users/jbahn/Documents", documents_size).unwrap();
    }
}
