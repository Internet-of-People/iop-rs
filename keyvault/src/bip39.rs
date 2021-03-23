use ::bip39::{Mnemonic, MnemonicType};
use getrandom::getrandom;

use super::*;

/// Tool for generating, validating and parsing BIP39 phrases in different supported languages.
#[derive(Clone, Copy, Debug, Default)]
pub struct Bip39 {
    lang: Bip39Language,
}

impl Bip39 {
    /// Number of words in the BIP39 mnemonics we accept and generate
    pub const MNEMONIC_WORDS: usize = 24;

    /// Creates the right sized entropy using the CSPRNG available on the platform.
    pub fn generate_entropy() -> Result<[u8; 32]> {
        let mut entropy = [0u8; 256 / 8];
        getrandom(&mut entropy)?;
        Ok(entropy)
    }

    /// Creates a new English BIP39 phrase handler
    pub fn new() -> Self {
        Self { lang: Bip39Language::English }
    }

    /// Creates a new BIP39 phrase handler with the given language
    pub fn language(lang: Bip39Language) -> Self {
        Self { lang }
    }

    /// Creates a new BIP39 phrase handler with the given language code
    ///
    /// # Example
    ///
    /// ```
    /// # use iop_keyvault::Bip39;
    /// let bip39 = Bip39::language_code("fr").unwrap();
    /// assert!(bip39.check_word("aménager"));
    /// ```
    pub fn language_code(code: impl AsRef<str>) -> Result<Self> {
        let lang = Bip39Language::from_language_code(code.as_ref())
            .ok_or_else(|| anyhow!("Invalid BIP39 language code"))?;
        Ok(Self { lang })
    }

    /// Creates a new phrase from hardware entropy. Cannot be used from WASM because it uses the system random generator.
    ///
    /// # Example
    /// ```
    /// # use iop_keyvault::{Bip39, Seed};
    /// let phrase: &str = Bip39::language_code("fr").unwrap().generate().as_phrase();
    /// let seed: Seed = Bip39::new().generate().password("");
    pub fn generate(self) -> Bip39Phrase {
        let size = MnemonicType::for_word_count(Self::MNEMONIC_WORDS).unwrap();
        let mnemonic = Mnemonic::new(size, self.lang);
        Bip39Phrase { mnemonic }
    }

    /// Checks if a word is present in the BIP39 dictionary
    ///
    /// # Example
    ///
    /// ```
    /// # use iop_keyvault::Bip39;
    /// let bip39 = Bip39::new();
    /// assert!(bip39.check_word("abandon"));
    /// assert!(!bip39.check_word("Abandon"));
    /// assert!(!bip39.check_word("avalon"));
    /// ```
    pub fn check_word(self, word: &str) -> bool {
        self.lang.wordmap().get_bits(word).is_ok()
    }

    /// Lists all words in the BIP39 dictionary, which start with the given prefix.
    ///
    /// Can be used in 3 different ways:
    /// - When the prefix is empty, the sorted list of all words are returned
    /// - When the prefix is a partial word, the returned list can be used for auto-completion
    /// - When the returned list is empty, the prefix is not a valid word in the dictionary
    ///
    /// # Example
    ///
    /// ```
    /// # use iop_keyvault::{Bip39, Bip39Language};
    /// let bip39 = Bip39::language(Bip39Language::English);
    /// assert_eq!(bip39.list_words("").len(), 2048);
    /// assert_eq!(bip39.list_words("woo"), ["wood", "wool"]);
    /// assert!(bip39.list_words("woof").is_empty());
    /// ```
    pub fn list_words(self, prefix: impl AsRef<str>) -> &'static [&'static str] {
        self.lang.wordlist().get_words_by_prefix(prefix.as_ref())
    }

    /// Validates a whole BIP39 mnemonic phrase. Because the phrase contains some checksum, the
    /// whole phrase can be invalid even when each word itself is valid.
    ///
    /// # Example
    ///
    /// ```
    /// # use iop_keyvault::Bip39;
    /// let bip39 = Bip39::new();
    /// assert!(bip39.validate("type shield target dream feature surface search flee tenant cake taxi shrug").is_ok());
    /// assert!(bip39.validate("abandon abandon about").unwrap_err().to_string().contains("invalid number of words"));
    /// ```
    pub fn validate(self, phrase: impl AsRef<str>) -> Result<()> {
        Mnemonic::validate(phrase.as_ref(), self.lang)
    }

    /// Validates a whole BIP39 mnemonic phrase and returns an intermediate object that can be
    /// later converted into a [`Seed`] with an optional password.
    ///
    /// [`Seed`]: struct.Seed.html
    pub fn phrase(self, phrase: impl AsRef<str>) -> Result<Bip39Phrase> {
        if phrase.as_ref().split(' ').count() != Bip39::MNEMONIC_WORDS {
            bail!("Only {}-word mnemonics are supported", Bip39::MNEMONIC_WORDS)
        }
        self.short_phrase(phrase)
    }

    /// Use the [`phrase`] method whenever possible. This method allows some legacy use-cases to
    /// provide mnemonics shorter than [`MNEMONIC_WORDS`] words.
    ///
    /// [`phrase`]: #method.phrase
    /// [`MNEMONIC_WORDS`]: ../constant.MNEMONIC_WORDS
    pub fn short_phrase(self, phrase: impl AsRef<str>) -> Result<Bip39Phrase> {
        let mnemonic = Mnemonic::from_phrase(phrase.as_ref(), self.lang)?;
        Ok(Bip39Phrase { mnemonic })
    }

    /// Creates a BIP39 phrase based on the 256 bits of entropy provided. This method is needed from WASM
    /// because [`generate`] uses system resources unavailable from WASM.
    pub fn entropy(self, entropy: impl AsRef<[u8]>) -> Result<Bip39Phrase> {
        let size = MnemonicType::for_word_count(Self::MNEMONIC_WORDS).unwrap();
        if entropy.as_ref().len() * 8 != size.entropy_bits() {
            bail!("Only {}-bit entropy is supported", size.entropy_bits())
        }
        self.short_entropy(entropy)
    }

    /// Use the ['entropy'] method whenever possible. This method allows some legacy use-cases to
    /// provide mnemonics with low entropy.
    pub fn short_entropy(self, entropy: impl AsRef<[u8]>) -> Result<Bip39Phrase> {
        let mnemonic = Mnemonic::from_entropy(entropy.as_ref(), self.lang)?;
        Ok(Bip39Phrase { mnemonic })
    }
}

/// A thin wrapper on top of a BIP39 phrase with a known language
pub struct Bip39Phrase {
    mnemonic: Mnemonic,
}

impl Bip39Phrase {
    /// Creates a [`Seed`] from the phrase with the given password. Give empty string when the user did not provide any password.
    pub fn password(&self, password: impl AsRef<str>) -> Seed {
        Seed::from_bip39(&self.mnemonic, password.as_ref())
    }

    /// Returns the phrase as a readable string
    pub fn as_phrase(&self) -> &str {
        self.mnemonic.phrase()
    }
}
