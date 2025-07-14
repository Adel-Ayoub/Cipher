use crate::file::{self, CphFile};
use crate::ui::cli::{self, Action};
use crate::key::Key;
use crate::crypt;

pub fn main() {
    cli::welcome();
    loop {
        match cli::get_action() {
            Some(Action::Encrypt) => {
                let password = cli::get_password();
                let rounds = cli::select_rounds();

                let key: Key = Key::new(password.as_bytes(), rounds);
                drop(password);

                let plaintext_file = cli::select_file_checked();

                let data = match file::read_plain(plaintext_file) {
                    Ok(d) => d,
                    Err(e) => {
                        println!("reading the file failed: {}", e);
                        continue;
                    }
                };

                let crypted = crypt::encrypt(&key, data.as_slice());

                let mut crypted_file: file::CphFile = file::CphFile::new();
                crypted_file.set_content(crypted.data);
                crypted_file.nonce = crypted.nonce;
                crypted_file.rounds = key.rounds;
                crypted_file.salt = key.salt;

                println!("where do u want to save the file?");
                let mut save_location = cli::select_file();
                if save_location.extension().is_some() {
                    save_location.set_extension(format!("{}.{}", save_location.extension().unwrap().to_str().unwrap(), "cph"));
                } else {
                    save_location.set_extension("cph");
                }

                match crypted_file.write(save_location) {
                    Ok(_) => println!("file saved successfully!"),
                    Err(e) => println!("saving the file failed: {}", e)
                };
            },
            Some(Action::Decrypt) => {
                println!("what file do you want to decrypt?");
                let filepath = cli::select_file_checked();
                let crypted_file = match CphFile::read(filepath) {
                    Ok(f) => f,
                    Err(e) => {
                        println!("an error occured reading the file: {}", e);
                        continue;
                    },
                };

                let password = cli::get_password();

                let key = Key::from_file(&crypted_file, password.as_bytes());

                let data = crypt::EncryptedData { data: crypted_file.get_content(), nonce: crypted_file.nonce };
                let plain = match crypt::decrypt(&key, data) {
                    Ok(d) => d,
                    Err(_) => {
                        println!("decrypting the file failed (wrong password?)");
                        continue;
                    }
                };

                println!("where do u want to save the file?");
                let save_location = cli::select_file();

                match file::write_plain(save_location, plain) {
                    Ok(_) => println!("file saved successfully!"),
                    Err(e) => println!("saving the file failed: {}", e),
                };
            },
            Some(Action::Exit) => break,
            None => {
                println!("Invalid input. Please try again");
                continue;
            }
        }
    }
}
