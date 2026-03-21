use super::*;

#[derive(Serialize, Deserialize)]
pub struct Output {
  pub message: String,
}

#[derive(Debug, Parser)]
pub(crate) struct Create {
  #[arg(
    long,
    default_value = "",
    help = "Use <PASSPHRASE> to derive wallet seed."
  )]
  pub(crate) passphrase: String,
}

impl Create {
  pub(crate) fn run(self, name: String, settings: &Settings) -> SubcommandResult {
    Wallet::initialize(name, settings, [0u8; 64])?;

    Ok(Some(Box::new(Output {
      message: "Ord wallet created. Descriptor wallets are not supported in Doriancoincore. \
        Please make a backup of the wallet.dat file and store it in a safe place."
        .to_string(),
    })))
  }
}
