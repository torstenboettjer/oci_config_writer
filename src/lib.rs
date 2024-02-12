//! This is a small library to manage an Oracle Cloud Infrastructure (OCI) config file. 
//! The library checks, whether a file already exists, before it writes the config into the sub-directory within the user's home directory.
//! It also checks the permissions before adding content.
//! 
//! More information about the config file itself can be found in the official documentation under: <https://docs.oracle.com/en-us/iaas/Content/API/Concepts/sdkconfig.htm>
//! # Example
//! ```rust
//! use oci_config_writer::{profile, credentials, report};
//! 
//! fn main() {
//!    profile(
//!     "ocid1.user.oc1..aaaaaaaaxxxxxx",
//!     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
//!     "path/to/private/key",
//!     "ocid1.tenancy.oc1..aaaaaaaaxxxxxx",
//!     "IAD"
//!    );
//!    credentials(
//!     "ocid1.user.oc1..aaaaaaaaxxxxxx",
//!     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
//!     "path/to/private/key",
//!     "passphrase"
//!    );
//!    report();
//! }
//! ```
pub mod file;
pub mod region;
pub mod log;
pub mod account;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io;
use std::path::PathBuf;
use directories::UserDirs;
use account::{default, admin};
use file::{create, permissions, read};
use region::{identifier, identifiers};

static DIR: &str = ".oci";
static NAME: &str = "config";

// Define the struct representing a file entry
#[derive(Debug)]
pub struct Profile {
    user: &'static str,
    fingerprint: &'static str,
    key_file: &'static str,
    tenancy: &'static str,
    region: String, // selection of active regions
}

impl Profile {
    // Function to format the Profile struct as a string
    fn profile_entry(&self) -> String {
        format!("[DEFAULT]\nuser: {}\nfingerprint: {}\nkey_file: {}\ntenancy: {}\nregion: {}\n\n", 
        self.user, self.fingerprint, self.key_file, self.tenancy, self.region)
    }
    
    // Function to write the struct to the config file
    fn write_to_config(&self, path: &str) -> io::Result<()> {
        // define directory directory
        let config_path = UserDirs::new().unwrap().home_dir().join(path);
        let path_to_str = config_path.to_str().expect("Failed to convert path to str");

        // set modification properties
        let config = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path_to_str);
        match config {
            Ok(mut config) => {
                match config.write_all(
                    self.profile_entry().as_bytes(),
                ) {
                    Ok(_) => println!("Tenancy data written to file successfully"),
                    Err(e) => println!("Failed to write tenancy data to file: {}", e),
                }
            }
            Err(e) => println!("Failed to create file: {}", e),
        }
    
        Ok(())
    }
}

/// writes an account profile to the config file, the values are used as defaults for admin users.
/// # Example
/// ```rust
/// use oci_config_writer::profile;
/// 
/// fn main() {
///    profile(
///     "ocid1.user.oc1..aaaaaaaaxxxxxx",
///     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
///     "path/to/private/key",
///     "ocid1.tenancy.oc1..aaaaaaaaxxxxxx",
///     "IAD"
///    );
/// }
/// ```
pub fn profile(user: &str, fingerprint: &str, key_file: &str, tenancy: &str, home: String) {
    let default_profile = Profile {
        user,
        fingerprint,
        key_file,
        tenancy,
        region: identifier(home)
    };
    let mut path = PathBuf::from(DIR);
    path.push(NAME);

    if !path.exists() {
        create(DIR, NAME);
        // Call the write_to_config method to write the struct to the file
        if let Err(err) = default_profile.write_to_config(path.to_str().unwrap()) {
            eprintln!("Error writing to file: {}", err);
        } else {
            println!("Profile successfully written to {}", path.to_str().unwrap());
        }
    } else {
        permissions(path.to_str().unwrap());
        // Call the write_to_config method to write the struct to the file
        if let Err(err) = default_profile.write_to_config(path.to_str().unwrap()) {
            eprintln!("Error writing to file: {}", err);
        } else {
            println!("Profile successfully written to {}", path.to_str().unwrap());
        }
    }
}

/// adds user credentials to the config file to authenticate the user and to provide access to a defined tenancy.
/// # Example
/// ```rust
/// use oci_config_writer::credentials;
/// 
/// fn main() {
///    credentials(
///     "ocid1.user.oc1..aaaaaaaaxxxxxx",
///     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
///     "path/to/private/key",
///     "passphrase"
///    );
/// }
/// ```
pub fn credentials(user: &str, fingerprint: &str, key_file: &str, pass_phrase: &str) {
    let file_path: String = format!("{}/{}", DIR, NAME); 

    permissions(file_path.as_str());
    admin(
        user, 
        fingerprint, 
        key_file, 
        pass_phrase
    );
}

/// reads and returns the content of a config file as a string.
/// # Example
/// ```rust
/// use oci_config_writer::report;
/// 
/// fn main() {
///   report();
/// }
/// ```
pub fn report() {
    let file_path: String = format!("{}/{}", DIR, NAME); 
    read(file_path.as_str());
}