use super::*;

use iop_keyvault::Bip39;

#[no_mangle]
pub extern "C" fn Bip39_generate_phrase(lang: *const raw::c_char) -> CPtrResult<raw::c_char> {
    let fun = || {
        let lang_code = convert::str_in(lang)?;
        let bip39 = Bip39::language_code(lang_code)?;
        let phrase = bip39.generate();
        Ok(convert::string_out(phrase.as_phrase().to_string()))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn Bip39_validate_phrase(
    lang: *const raw::c_char, phrase: *const raw::c_char,
) -> CPtrResult<raw::c_void> {
    let fun = || {
        let lang_code = convert::str_in(lang)?;
        let phrase = convert::str_in(phrase)?;
        let bip39 = Bip39::language_code(lang_code)?;
        bip39.validate(phrase)?;
        Ok(())
    };
    cresult_void(fun())
}

#[no_mangle]
pub extern "C" fn Bip39_list_words(
    lang: *const raw::c_char, pref: *const raw::c_char,
) -> CPtrResult<CSlice<*mut raw::c_char>> {
    let fun = || {
        let lang_code = convert::str_in(lang)?;
        let prefix = convert::str_in(pref)?;
        let bip39 = Bip39::language_code(lang_code)?;
        let matching_words =
            bip39.list_words(prefix).iter().map(|word| (*word).to_string()).collect::<Vec<_>>();
        let raw_slice = convert::CSlice::from(matching_words);
        Ok(convert::move_out(raw_slice))
    };
    cresult(fun())
}
