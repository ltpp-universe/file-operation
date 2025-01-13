#[tokio::test]
async fn test_write() {
    use crate::*;
    static FILE_DIR: &str = "./log";
    static NEW_FILE_DIR: &str = "./new_log";
    static NEW_TEST_DIR: &str = "./test_log";
    static FILE_PATH: &str = "./log/test.txt";
    let _ = write_to_file(FILE_PATH, "test".as_bytes());
    let res: Vec<u8> = read_from_file(FILE_PATH).unwrap_or_default();
    let size: Option<u64> = get_file_size(FILE_PATH);
    println!("read_from_file => {:?}", String::from_utf8_lossy(&res));
    println!("get_file_size => {:?}", size);
    let _ = std::thread::sleep(std::time::Duration::from_secs(2));
    let res: Result<(), std::io::Error> = copy_dir_files(FILE_DIR, NEW_FILE_DIR);
    println!("copy_dir_files => {:?}", res);
    let _ = std::thread::sleep(std::time::Duration::from_secs(2));
    let res: Result<(), std::io::Error> = delete_file(FILE_PATH);
    println!("delete_file => {:?}", res);
    let _ = std::thread::sleep(std::time::Duration::from_secs(2));
    let res: Result<(), std::io::Error> = move_dir(FILE_DIR, NEW_TEST_DIR);
    println!("move_dir => {:?}", res);
    let _ = std::thread::sleep(std::time::Duration::from_secs(2));
    let res: Result<(), std::io::Error> = delete_dir(NEW_TEST_DIR);
    println!("delete_dir => {:?}", res);
    let _ = std::thread::sleep(std::time::Duration::from_secs(2));
    let res: Result<(), std::io::Error> = delete_dir(NEW_FILE_DIR);
    println!("delete_dir => {:?}", res);
    let _ = std::thread::sleep(std::time::Duration::from_secs(2));
    let _ = async_write_to_file(FILE_PATH, "test".as_bytes()).await;
    let res: Vec<u8> = async_read_from_file(FILE_PATH).await.unwrap_or_default();
    let size: Option<u64> = async_get_file_size(FILE_PATH).await;
    println!("read_from_file => {:?}", String::from_utf8_lossy(&res));
    println!("get_file_size => {:?}", size);
    let _ = std::thread::sleep(std::time::Duration::from_secs(2));
    let res: Result<(), std::io::Error> = async_copy_dir_files(FILE_DIR, NEW_FILE_DIR).await;
    println!("copy_dir_files => {:?}", res);
    let _ = std::thread::sleep(std::time::Duration::from_secs(2));
    let res: Result<(), std::io::Error> = async_delete_file(FILE_PATH).await;
    let _ = std::thread::sleep(std::time::Duration::from_secs(2));
    println!("delete_file => {:?}", res);
    let _ = std::thread::sleep(std::time::Duration::from_secs(2));
    let res: Result<(), std::io::Error> = async_move_dir(FILE_DIR, NEW_TEST_DIR).await;
    println!("move_dir => {:?}", res);
    let _ = std::thread::sleep(std::time::Duration::from_secs(2));
    let res: Result<(), std::io::Error> = async_delete_dir(NEW_TEST_DIR).await;
    println!("delete_dir => {:?}", res);
    let _ = std::thread::sleep(std::time::Duration::from_secs(2));
    let res: Result<(), std::io::Error> = async_delete_dir(NEW_FILE_DIR).await;
    println!("delete_dir => {:?}", res);
    let _ = std::thread::sleep(std::time::Duration::from_secs(2));
}
