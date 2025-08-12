use aes::Aes256;
use aes::cipher::{KeyIvInit, StreamCipher};
use ctr::Ctr128BE;
use eframe::App;
use eframe::egui::{CentralPanel, Color32, Context, RichText};
use rand::Rng;
use rfd::FileDialog;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::{Path, PathBuf};

type Aes256Ctr = Ctr128BE<Aes256>;

#[derive(Debug, Clone, PartialEq)]
enum AppState {
    MainMenu,
    ShowResult(String),
    ShowError(String),
}

struct EzEncryptApp {
    state: AppState,
    password: String,
    executable_dir: PathBuf,
}

impl EzEncryptApp {
    fn new() -> Self {
        Self {
            state: AppState::MainMenu,
            password: String::new(),
            executable_dir: Self::get_executable_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    fn get_executable_dir() -> Result<PathBuf, Error> {
        std::env::current_exe().map(|exe_path| {
            exe_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf()
        })
    }

    fn derive_key(password: &str, salt: &[u8; 16]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        hasher.update(salt);
        let result = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(&result);
        key
    }

    fn encrypt_file(&self, input_path: &Path, password: &str) -> Result<String, String> {
        // Read input file
        let mut input_file =
            File::open(input_path).map_err(|e| format!("Failed to open input file: {}", e))?;

        let mut plaintext = Vec::new();
        input_file
            .read_to_end(&mut plaintext)
            .map_err(|e| format!("Failed to read input file: {}", e))?;

        // Generate random salt and IV
        let mut rng = rand::thread_rng();
        let mut salt = [0u8; 16];
        let mut iv = [0u8; 16];
        rng.fill(&mut salt);
        rng.fill(&mut iv);

        // Derive key from password and salt
        let key = Self::derive_key(password, &salt);

        // Encrypt the data
        let mut cipher = Aes256Ctr::new(&key.into(), &iv.into());
        let mut ciphertext = plaintext.clone();
        cipher.apply_keystream(&mut ciphertext);

        // Create output file path
        let output_path = input_path.with_extension(format!(
            "{}.enc",
            input_path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("file")
        ));

        // Write encrypted file: salt + iv + encrypted_data
        let mut output_file = File::create(&output_path)
            .map_err(|e| format!("Failed to create output file: {}", e))?;

        output_file
            .write_all(&salt)
            .map_err(|e| format!("Failed to write salt: {}", e))?;
        output_file
            .write_all(&iv)
            .map_err(|e| format!("Failed to write IV: {}", e))?;
        output_file
            .write_all(&ciphertext)
            .map_err(|e| format!("Failed to write encrypted data: {}", e))?;

        Ok(format!(
            "File encrypted successfully!\nOutput: {}",
            output_path.display()
        ))
    }

    fn decrypt_file(&self, input_path: &Path, password: &str) -> Result<String, String> {
        // Read encrypted file
        let mut input_file =
            File::open(input_path).map_err(|e| format!("Failed to open encrypted file: {}", e))?;

        let mut file_data = Vec::new();
        input_file
            .read_to_end(&mut file_data)
            .map_err(|e| format!("Failed to read encrypted file: {}", e))?;

        // Check minimum file size (salt + iv = 32 bytes)
        if file_data.len() < 32 {
            return Err("Invalid encrypted file: too small".to_string());
        }

        // Extract salt, IV, and encrypted data
        let salt: [u8; 16] = file_data[0..16]
            .try_into()
            .map_err(|_| "Failed to extract salt")?;
        let iv: [u8; 16] = file_data[16..32]
            .try_into()
            .map_err(|_| "Failed to extract IV")?;
        let mut ciphertext = file_data[32..].to_vec();

        // Derive key from password and salt
        let key = Self::derive_key(password, &salt);

        // Decrypt the data
        let mut cipher = Aes256Ctr::new(&key.into(), &iv.into());
        cipher.apply_keystream(&mut ciphertext);

        // Create output file path (remove .enc extension)
        let output_path = if input_path.extension().and_then(|s| s.to_str()) == Some("enc") {
            input_path.with_extension("")
        } else {
            input_path.with_extension("decrypted")
        };

        // Write decrypted file
        let mut output_file = File::create(&output_path)
            .map_err(|e| format!("Failed to create output file: {}", e))?;

        output_file
            .write_all(&ciphertext)
            .map_err(|e| format!("Failed to write decrypted data: {}", e))?;

        Ok(format!(
            "File decrypted successfully!\nOutput: {}",
            output_path.display()
        ))
    }

    fn select_file_and_encrypt(&mut self) {
        if let Some(path) = FileDialog::new()
            .set_directory(&self.executable_dir)
            .pick_file()
        {
            if self.password.is_empty() {
                self.state = AppState::ShowError("Please enter a password first!".to_string());
                return;
            }

            match self.encrypt_file(&path, &self.password) {
                Ok(message) => {
                    self.state = AppState::ShowResult(message);
                    self.password.clear();
                }
                Err(error) => {
                    self.state = AppState::ShowError(error);
                }
            }
        }
    }

    fn select_file_and_decrypt(&mut self) {
        if let Some(path) = FileDialog::new()
            .set_directory(&self.executable_dir)
            .add_filter("Encrypted files", &["enc"])
            .pick_file()
        {
            if self.password.is_empty() {
                self.state = AppState::ShowError("Please enter a password first!".to_string());
                return;
            }

            match self.decrypt_file(&path, &self.password) {
                Ok(message) => {
                    self.state = AppState::ShowResult(message);
                    self.password.clear();
                }
                Err(error) => {
                    self.state = AppState::ShowError(error);
                }
            }
        }
    }

    fn reset_to_main_menu(&mut self) {
        self.state = AppState::MainMenu;
        self.password.clear();
    }
}

impl App for EzEncryptApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading(RichText::new("🔐 Ez-Encrypt").size(32.0));
                ui.add_space(10.0);
                ui.label("Simple File Encryption & Decryption");
                ui.add_space(30.0);
            });

            match &self.state {
                AppState::MainMenu => {
                    ui.vertical_centered(|ui| {
                        ui.label("Enter your password:");
                        ui.add_space(5.0);

                        let password_response = ui.add_sized(
                            [300.0, 25.0],
                            eframe::egui::TextEdit::singleline(&mut self.password)
                                .password(true)
                                .hint_text("Enter encryption password..."),
                        );

                        ui.add_space(20.0);

                        // Enable/disable buttons based on password
                        let buttons_enabled = !self.password.is_empty();

                        ui.horizontal(|ui| {
                            ui.add_space(50.0);

                            ui.add_enabled_ui(buttons_enabled, |ui| {
                                if ui
                                    .add_sized(
                                        [120.0, 40.0],
                                        eframe::egui::Button::new(
                                            RichText::new("🔒 Encrypt").size(16.0),
                                        ),
                                    )
                                    .clicked()
                                {
                                    self.select_file_and_encrypt();
                                }
                            });

                            ui.add_space(20.0);

                            ui.add_enabled_ui(buttons_enabled, |ui| {
                                if ui
                                    .add_sized(
                                        [120.0, 40.0],
                                        eframe::egui::Button::new(
                                            RichText::new("🔓 Decrypt").size(16.0),
                                        ),
                                    )
                                    .clicked()
                                {
                                    self.select_file_and_decrypt();
                                }
                            });

                            ui.add_space(20.0);

                            if ui
                                .add_sized(
                                    [120.0, 40.0],
                                    eframe::egui::Button::new(RichText::new("❌ Exit").size(16.0)),
                                )
                                .clicked()
                            {
                                frame.close();
                            }
                        });

                        if !buttons_enabled {
                            ui.add_space(10.0);
                            ui.label(
                                RichText::new(
                                    "⚠ Please enter a password to enable encryption/decryption",
                                )
                                .color(Color32::DARK_GRAY),
                            );
                        }

                        // Handle Enter key in password field
                        if password_response.lost_focus()
                            && ui.input(|i| i.key_pressed(eframe::egui::Key::Enter))
                        {
                            if !self.password.is_empty() {
                                // Could default to encrypt mode or show file picker
                            }
                        }
                    });
                }

                AppState::ShowResult(message) => {
                    let message_clone = message.clone();
                    ui.vertical_centered(|ui| {
                        ui.label(
                            RichText::new("✅ Success!")
                                .size(24.0)
                                .color(Color32::GREEN),
                        );
                        ui.add_space(15.0);

                        ui.label(RichText::new(&message_clone).size(14.0));
                        ui.add_space(30.0);
                    });

                    ui.vertical_centered(|ui| {
                        if ui
                            .add_sized(
                                [200.0, 40.0],
                                eframe::egui::Button::new(
                                    RichText::new("🏠 Back to Main Menu").size(16.0),
                                ),
                            )
                            .clicked()
                        {
                            self.reset_to_main_menu();
                        }
                    });
                }

                AppState::ShowError(error) => {
                    let error_clone = error.clone();
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("❌ Error").size(24.0).color(Color32::RED));
                        ui.add_space(15.0);

                        ui.label(RichText::new(&error_clone).size(14.0).color(Color32::RED));
                        ui.add_space(30.0);
                    });

                    ui.vertical_centered(|ui| {
                        if ui
                            .add_sized(
                                [200.0, 40.0],
                                eframe::egui::Button::new(
                                    RichText::new("🏠 Back to Main Menu").size(16.0),
                                ),
                            )
                            .clicked()
                        {
                            self.reset_to_main_menu();
                        }
                    });
                }
            }

            ui.add_space(50.0);
            ui.vertical_centered(|ui| {
                ui.separator();
                ui.add_space(10.0);
                ui.label(
                    RichText::new("💡 Tip: Place this executable in the same folder as your files")
                        .size(12.0)
                        .color(Color32::GRAY),
                );
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(500.0, 400.0)),
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "Ez-Encrypt",
        options,
        Box::new(|_cc| Box::new(EzEncryptApp::new())),
    )
}
