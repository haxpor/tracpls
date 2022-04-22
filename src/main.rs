use ::bscscan::bscscan;
use ::bscscan::environ::Context;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author="Wasin Thonkaew (wasin@wasin.io)")]
#[clap(name="tracpls")]
#[clap(about="cli tool to get smart contract code and its ABI for ease of viewing on terminal")]
struct CommandlineArgs {
    /// Target contract address to get its smart contract code or ABI from
    #[clap(long="address", short='a', required=true)]
    pub address: String,

    /// Make sure to clean CR/LF character codes to make it suitable to view
    /// the content on the platform running the application.
    #[clap(long="no-clean-crlf", multiple_values=false, default_missing_value="true", takes_value=false)]
    pub no_clean_crlf: bool,

    /// Get only contract ABI
    #[clap(long="abi-only", multiple_values=false, default_missing_value="true", takes_value=false)]
    pub abi_only: bool,

    /// Pretty print output for contract ABI. It can only be used if --abi-only exists.
    #[clap(long="no-abi-pretty-print", multiple_values=false, default_missing_value="true", takes_value=false)]
    pub no_abi_pretty_print: bool,

    /// Output directory path to write content of files to. In case of --abi-only,
    /// it will output into fixed filename of "abi.json" but at the supplied
    /// output directory. For JSON-based code, it will use the contract name of
    /// each file as the filename to write its content to.
    #[clap(long="out-dir", required=false)]
    pub out_dir_path: Option<String>,

    /// Whether or not to print meta information during execution.
    #[clap(long="silence", short='s', multiple_values=false, default_missing_value="true", takes_value=false)]
    pub silence: bool,
}

/// Clean CR/LF as necessary as per platform running the application.
///
/// # Arguments
/// * `text` - text to be cleaned if necessary
///
/// # Returned
/// New instance of `String`.
fn clean_crlf(text: &str) -> String {
    let os = std::env::consts::OS;

    // actually this would depend on the upstream source file itself
    // for which platform developers edit file on, but we can clean it
    // in (all) cases.

    // on Linux/Unix uses only line feed (\n)
    if os == "linux" {
        let cleaned_text = str::replace(&str::replace(text, "\r\n", "\n"), "\r", "\n");
        cleaned_text
    }
    // on macOS, it uses only carriage return (\r)
    else if os == "macos" {
        let cleaned_text = str::replace(&str::replace(text, "\r\n", "\r"), "\n", "\r");
        cleaned_text
    }
    // otherwise don't clean anything
    // e.g. Windows uses both CR/LF
    else {
        text.to_owned()
    }
}

/// Combine two path components together and return str version of it.
///
/// # Arguments
/// * `path_a` - first path component
/// * `path_b` - second path component
fn combine_two_path_components(path_a: &str, path_b: &str) -> Result<String, String> {
    let mut path = PathBuf::from(path_a);
    path.push(path_b);

    match path.as_path().to_str() {
        Some(res) => Ok(res.to_owned()),
        None => {
            let err_msg = format!("Error converting PathBuf to str from result of concatenation of {} and {}", path_a, path_b);
            return Err(err_msg);
        }
    }
}

/// Create intermediate directories.
/// It internally handles whether the path is file, or directory. So supplying
/// the actual filepath here is fine.
///
/// # Arguments
/// * `path` - path to create intermerdiate directories
fn create_intermediate_dirs(path: &str) -> Result<(), String> {
    let mut ppath = PathBuf::from(path);
    // pop the last component out to get only directory path
    if ppath.file_name().is_some() {
        ppath.pop();
    }

    // get path string
    let ppath_str = match ppath.as_path().to_str() {
        Some(res) => res,
        None => {
            let err_msg = format!("Error getting path string from PathBuf ('{}')", path);
            return Err(err_msg);
        }
    };

    // create all directories leading up to what we will
    match std::fs::create_dir_all(ppath_str) {
        Ok(_) => (),
        Err(e) => {
            let err_msg = format!("Error creating intermediate directories; err={}", e);
            return Err(err_msg);
        }
    }

    Ok(())
}

/// Write content to file.
///
/// # Arguments
/// * `filepath` - filepath to write file to, ensure path includes the filename
/// * `content` - content of file
fn write_file(filepath: &str, content: &str) -> Result<(), String> {
    match std::fs::write(filepath, content) {
        Ok(_) => (),
        Err(e) => {
            let err_msg = format!("Error writing file at '{}'; err={}", filepath, e);
            return Err(err_msg);
        }
    }

    Ok(())
}

fn main() {
    let cmd_args = CommandlineArgs::parse();
    let has_out_dir_path = cmd_args.out_dir_path.is_some();

    // make sure flags are supplied and used only when it's proper
    if !cmd_args.abi_only && cmd_args.no_abi_pretty_print {
        eprintln!("Error --no-abi-pretty-print can ony be used when --abi-only exists");
        std::process::exit(1);
    }

    let ctx = Context { api_key: std::env::var("TRACPLS_BSCSCAN_APIKEY").expect("Required environment variable 'TRACPLS_BSCSCAN_APIKEY' to be defined") };
    let contracts = bscscan::contracts();

    if cmd_args.abi_only {
        match contracts.get_abi(&ctx, &cmd_args.address, !cmd_args.no_abi_pretty_print) {
            Ok(abi) => {
                if has_out_dir_path {
                    let out_dir_str = cmd_args.out_dir_path.unwrap();
                    let write_filepath = match combine_two_path_components(&out_dir_str, "abi.json") {
                        Ok(res) => res,
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    };

                    match create_intermediate_dirs(&write_filepath) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    }

                    let content = if !cmd_args.no_clean_crlf { clean_crlf(&abi) } else { abi };
                    match write_file(&write_filepath, &content) {
                        Ok(_) => if !cmd_args.silence { println!("{}", &write_filepath) },
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    }
                }
                else {
                    println!("{}", if !cmd_args.no_clean_crlf { clean_crlf(&abi) } else { abi });
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            },
        }
    }
    else {
        match contracts.get_verified_source_code(&ctx, &cmd_args.address) {
            Ok((contract_codes, is_submitted_as_json)) => {
                if is_submitted_as_json {
                    // we have more information about number of files, and
                    // separate content of code for each file now. So there can
                    // be options to handle this either
                    // 1. output all files altogether as a whole
                    // 2. output into target directory by writing into multiple files
                    for i in 1..contract_codes.len() {
                        if has_out_dir_path {
                            let out_dir_str = cmd_args.out_dir_path.as_ref().unwrap();
                            let write_filepath = match combine_two_path_components(&out_dir_str, &contract_codes[i].contract_name) {
                                Ok(res) => res,
                                Err(e) => {
                                    eprintln!("{}", e);
                                    std::process::exit(1);
                                }
                            };

                            match create_intermediate_dirs(&write_filepath) {
                                Ok(_) => (),
                                Err(e) => {
                                    eprintln!("{}", e);
                                    std::process::exit(1);
                                }
                            }

                            let content = if !cmd_args.no_clean_crlf { clean_crlf(&contract_codes[i].source_code) } else { contract_codes[i].source_code.clone() };
                            match write_file(&write_filepath, &content) {
                                Ok(_) => if !cmd_args.silence { println!("{}", &write_filepath) },
                                Err(e) => {
                                    eprintln!("{}", e);
                                    std::process::exit(1);
                                }
                            }
                        }
                        else {
                            println!("// ---------- {} ----------", contract_codes[i].contract_name);

                            if !cmd_args.no_clean_crlf {
                                println!("{}", clean_crlf(&contract_codes[i].source_code));
                            }
                            else {
                                println!("{}", &contract_codes[i].source_code);
                            }
                        }
                    }
                }
                else {
                    if has_out_dir_path {
                        let out_dir_str = cmd_args.out_dir_path.unwrap();
                        // use contract name as the filename also append with .sol if necessary
                        let mut filename = contract_codes[0].contract_name.clone();
                        if !filename.ends_with(".sol") {
                            filename.push_str(".sol");
                        }
                        let write_filepath = match combine_two_path_components(&out_dir_str, &filename) {
                            Ok(res) => res,
                            Err(e) => {
                                eprintln!("{}", e);
                                std::process::exit(1);
                            }
                        };

                        match create_intermediate_dirs(&write_filepath) {
                            Ok(_) => (),
                            Err(e) => {
                                eprintln!("{}", e);
                                std::process::exit(1);
                            }
                        }

                        let content = if !cmd_args.no_clean_crlf { clean_crlf(&contract_codes[0].source_code) } else { contract_codes[0].source_code.clone() };
                        match write_file(&write_filepath, &content) {
                            Ok(_) => if !cmd_args.silence { println!("{}", &write_filepath) },
                            Err(e) => {
                                eprintln!("{}", e);
                                std::process::exit(1);
                            }
                        }
                    }
                    else {
                        println!("{}", if !cmd_args.no_clean_crlf { clean_crlf(&contract_codes[0].source_code) } else { contract_codes[0].source_code.clone() });
                    }
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
