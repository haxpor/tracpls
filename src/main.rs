use ::bscscan::bscscan;
use ::bscscan::environ::Context;
use clap::Parser;

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

fn main() {
    let cmd_args = CommandlineArgs::parse();

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
                if !cmd_args.no_clean_crlf {
                    println!("{}", clean_crlf(&abi));
                }
                else {
                    println!("{}", abi);
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
            Ok(contract_codes) => {
                if !cmd_args.no_clean_crlf {
                    println!("{}", clean_crlf(&contract_codes[0].source_code));
                }
                else {
                    println!("{}", contract_codes[0].source_code);
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
