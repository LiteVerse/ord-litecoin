use {super::*, clap::ValueEnum};

#[derive(Default, ValueEnum, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Chain {
  #[default]
  #[clap(alias("main"))]
  Mainnet,
  #[clap(alias("test"))]
  Testnet,
  Signet,
  Regtest,
}

impl Chain {
  pub(crate) fn network(self) -> Network {
    match self {
      Self::Mainnet => Network::Bitcoin,
      Self::Testnet => Network::Testnet,
      Self::Signet => Network::Signet,
      Self::Regtest => Network::Regtest,
    }
  }

  pub(crate) fn default_rpc_port(self) -> u16 {
    match self {
      Self::Mainnet => 9332,
      Self::Regtest => 19443,
      Self::Signet => 39332,
      Self::Testnet => 19332,
    }
  }

  pub(crate) fn inscription_content_size_limit(self) -> Option<usize> {
    match self {
      Self::Mainnet | Self::Regtest => None,
      Self::Testnet | Self::Signet => Some(1024),
    }
  }

  pub(crate) fn first_inscription_height(self) -> u64 {
    match self {
      Self::Mainnet => 2424429,
      Self::Regtest => 0,
      Self::Signet => 112402,
      Self::Testnet => 2669127,
    }
  }

  pub(crate) fn genesis_block(self) -> Block {
    bitcoin::blockdata::constants::genesis_block(self.network())
  }

  pub(crate) fn address_from_script(
    self,
    script: &Script,
  ) -> Result<Address, bitcoin::util::address::Error> {
    Address::from_script(script, self.network())
  }

  pub(crate) fn join_with_data_dir(self, data_dir: &Path) -> PathBuf {
    match self {
      Self::Mainnet => data_dir.to_owned(),
      Self::Testnet => data_dir.join("testnet3"),
      Self::Signet => data_dir.join("signet"),
      Self::Regtest => data_dir.join("regtest"),
    }
  }

  pub(crate) fn check_address_is_valid_for_network(self, address: &Address) -> Result {
    if !address.is_valid_for_network(self.network()) {
      bail!("Address `{address}` is not valid for {self}");
    }
    Ok(())
  }
}

impl Display for Chain {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Mainnet => "mainnet",
        Self::Regtest => "regtest",
        Self::Signet => "signet",
        Self::Testnet => "testnet",
      }
    )
  }
}
