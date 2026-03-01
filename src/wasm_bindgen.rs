use rlibphonenumber::{
    MatchType, NumberLengthType, PHONE_NUMBER_UTIL, PhoneNumber, PhoneNumberFormat, PhoneNumberType,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum JsPhoneNumberFormat {
    E164 = 0,
    International = 1,
    National = 2,
    RFC3966 = 3,
}

impl From<JsPhoneNumberFormat> for PhoneNumberFormat {
    fn from(f: JsPhoneNumberFormat) -> Self {
        match f {
            JsPhoneNumberFormat::E164 => PhoneNumberFormat::E164,
            JsPhoneNumberFormat::International => PhoneNumberFormat::International,
            JsPhoneNumberFormat::National => PhoneNumberFormat::National,
            JsPhoneNumberFormat::RFC3966 => PhoneNumberFormat::RFC3966,
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum JsPhoneNumberType {
    FixedLine = 0,
    Mobile = 1,
    FixedLineOrMobile = 2,
    TollFree = 3,
    PremiumRate = 4,
    SharedCost = 5,
    Voip = 6,
    PersonalNumber = 7,
    Pager = 8,
    Uan = 9,
    Voicemail = 10,
    Unknown = 99,
}

impl From<PhoneNumberType> for JsPhoneNumberType {
    fn from(t: PhoneNumberType) -> Self {
        match t {
            PhoneNumberType::FixedLine => JsPhoneNumberType::FixedLine,
            PhoneNumberType::Mobile => JsPhoneNumberType::Mobile,
            PhoneNumberType::FixedLineOrMobile => JsPhoneNumberType::FixedLineOrMobile,
            PhoneNumberType::TollFree => JsPhoneNumberType::TollFree,
            PhoneNumberType::PremiumRate => JsPhoneNumberType::PremiumRate,
            PhoneNumberType::SharedCost => JsPhoneNumberType::SharedCost,
            PhoneNumberType::VoIP => JsPhoneNumberType::Voip,
            PhoneNumberType::PersonalNumber => JsPhoneNumberType::PersonalNumber,
            PhoneNumberType::Pager => JsPhoneNumberType::Pager,
            PhoneNumberType::UAN => JsPhoneNumberType::Uan,
            PhoneNumberType::VoiceMail => JsPhoneNumberType::Voicemail,
            PhoneNumberType::Unknown => JsPhoneNumberType::Unknown,
        }
    }
}

#[wasm_bindgen]
pub struct JsPhoneNumber {
    inner: PhoneNumber,
}

#[wasm_bindgen]
impl JsPhoneNumber {
    #[wasm_bindgen(js_name = formatAs)]
    pub fn format_as(&self, format: JsPhoneNumberFormat) -> String {
        self.inner.format_as(format.into()).into_owned()
    }

    #[wasm_bindgen(js_name = formatInOriginalFormat)]
    pub fn format_in_original_format(&self, region_calling_from: &str) -> String {
        self.inner
            .format_in_original_format(region_calling_from)
            .into_owned()
    }

    #[wasm_bindgen(js_name = formatNationalWithCarrierCode)]
    pub fn format_national_with_carrier_code(&self, carrier_code: &str) -> String {
        self.inner.format_national_with_carrier_code(carrier_code)
    }

    #[wasm_bindgen(js_name = formatForMobileDialing)]
    pub fn format_for_mobile_dialing(
        &self,
        region_calling_from: &str,
        with_formatting: bool,
    ) -> Option<String> {
        self.inner
            .format_for_mobile_dialing(region_calling_from, with_formatting)
            .map(|s| s.into_owned())
    }

    #[wasm_bindgen(js_name = formatOutOfCountryCallingNumber)]
    pub fn format_out_of_country_calling_number(&self, region_calling_from: &str) -> String {
        self.inner
            .format_out_of_country_calling_number(region_calling_from)
            .into_owned()
    }

    #[wasm_bindgen(js_name = formatOutOfCountryKeepingAlphaChars)]
    pub fn format_out_of_country_keeping_alpha_chars(&self, region_calling_from: &str) -> String {
        self.inner
            .format_out_of_country_keeping_alpha_chars(region_calling_from)
            .into_owned()
    }

    #[wasm_bindgen(js_name = getRegionCode)]
    pub fn get_region_code(&self) -> Option<String> {
        self.inner.get_region_code().map(|s| s.to_string())
    }

    #[wasm_bindgen(js_name = getType)]
    pub fn get_type(&self) -> JsPhoneNumberType {
        self.inner.get_type().into()
    }

    #[wasm_bindgen(js_name = canBeInternationallyDialled)]
    pub fn can_be_internationally_dialled(&self) -> bool {
        self.inner.can_be_internationally_dialled()
    }

    #[wasm_bindgen(js_name = isGeographical)]
    pub fn is_geographical(&self) -> bool {
        self.inner.is_geographical()
    }

    #[wasm_bindgen(js_name = isValid)]
    pub fn is_valid(&self) -> bool {
        self.inner.is_valid()
    }

    #[wasm_bindgen(js_name = isValidForRegion)]
    pub fn is_valid_for_region(&self, region: &str) -> bool {
        self.inner.is_valid_for_region(region)
    }

    #[wasm_bindgen(js_name = isPossibleWithReason)]
    pub fn is_possible_with_reason(&self) -> JsNumberLengthType {
        match self.inner.is_possible_with_reason() {
            Ok(length_type) => length_type.into(),
            Err(e) => {
                wasm_bindgen::throw_str(&e.to_string());
            }
        }
    }

    #[wasm_bindgen(js_name = truncateTooLongNumber)]
    pub fn truncate_too_long_number(&mut self) -> bool {
        self.inner.truncate_too_long_number()
    }

    #[wasm_bindgen(js_name = getLengthOfGeographicalAreaCode)]
    pub fn get_length_of_geographical_area_code(&self) -> usize {
        self.inner.get_length_of_geographical_area_code()
    }

    #[wasm_bindgen(js_name = getLengthOfNationalDestinationCode)]
    pub fn get_length_of_national_destination_code(&self) -> usize {
        self.inner.get_length_of_national_destination_code()
    }

    #[wasm_bindgen(js_name = getNationalSignificantNumber)]
    pub fn get_national_significant_number(&self) -> String {
        self.inner.get_national_significant_number()
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum JsMatchType {
    NotANumber = 0,
    NoMatch = 1,
    ShortNsnMatch = 2,
    NsnMatch = 3,
    ExactMatch = 4,
}

impl From<MatchType> for JsMatchType {
    fn from(m: MatchType) -> Self {
        match m {
            MatchType::NoMatch => JsMatchType::NoMatch,
            MatchType::ShortNsnMatch => JsMatchType::ShortNsnMatch,
            MatchType::NsnMatch => JsMatchType::NsnMatch,
            MatchType::ExactMatch => JsMatchType::ExactMatch,
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum JsNumberLengthType {
    IsPossible = 0,
    IsPossibleLocalOnly = 1,
    InvalidCountryCode = 2,
    TooShort = 3,
    InvalidLength = 4,
    TooLong = 5,
}

impl From<NumberLengthType> for JsNumberLengthType {
    fn from(l: NumberLengthType) -> Self {
        match l {
            NumberLengthType::IsPossible => JsNumberLengthType::IsPossible,
            NumberLengthType::IsPossibleLocalOnly => JsNumberLengthType::IsPossibleLocalOnly,
        }
    }
}

#[wasm_bindgen(js_name = canBeInternationallyDialled)]
pub fn util_can_be_internationally_dialled(phone_number: &JsPhoneNumber) -> bool {
    PHONE_NUMBER_UTIL.can_be_internationally_dialled(&phone_number.inner)
}

#[wasm_bindgen(js_name = convertAlphaCharactersInNumber)]
pub fn convert_alpha_characters_in_number(number: &str) -> String {
    PHONE_NUMBER_UTIL.convert_alpha_characters_in_number(number)
}

#[wasm_bindgen(js_name = formatInOriginalFormat)]
pub fn util_format_in_original_format(
    phone_number: &JsPhoneNumber,
    region_calling_from: &str,
) -> String {
    PHONE_NUMBER_UTIL
        .format_in_original_format(&phone_number.inner, region_calling_from)
        .into_owned()
}

#[wasm_bindgen(js_name = formatNationalNumberWithCarrierCode)]
pub fn format_national_number_with_carrier_code(
    phone_number: &JsPhoneNumber,
    carrier_code: &str,
) -> String {
    PHONE_NUMBER_UTIL.format_national_number_with_carrier_code(&phone_number.inner, carrier_code)
}

#[wasm_bindgen(js_name = formatNumberForMobileDialing)]
pub fn format_number_for_mobile_dialing(
    phone_number: &JsPhoneNumber,
    region_calling_from: &str,
    with_formatting: bool,
) -> Option<String> {
    PHONE_NUMBER_UTIL
        .format_number_for_mobile_dialing(&phone_number.inner, region_calling_from, with_formatting)
        .map(|s| s.into_owned())
}

#[wasm_bindgen(js_name = formatOutOfCountryCallingNumber)]
pub fn util_format_out_of_country_calling_number(
    phone_number: &JsPhoneNumber,
    region_calling_from: &str,
) -> String {
    PHONE_NUMBER_UTIL
        .format_out_of_country_calling_number(&phone_number.inner, region_calling_from)
        .into_owned()
}

#[wasm_bindgen(js_name = formatOutOfCountryKeepingAlphaChars)]
pub fn util_format_out_of_country_keeping_alpha_chars(
    phone_number: &JsPhoneNumber,
    region_calling_from: &str,
) -> String {
    PHONE_NUMBER_UTIL
        .format_out_of_country_keeping_alpha_chars(&phone_number.inner, region_calling_from)
        .into_owned()
}

#[wasm_bindgen(js_name = getCountryCodeForRegion)]
pub fn get_country_code_for_region(region_code: &str) -> Option<i32> {
    PHONE_NUMBER_UTIL.get_country_code_for_region(region_code)
}

#[wasm_bindgen(js_name = getExampleNumber)]
pub fn get_example_number(region_code: &str) -> JsPhoneNumber {
    match PHONE_NUMBER_UTIL.get_example_number(region_code) {
        Ok(inner) => JsPhoneNumber { inner },
        Err(e) => wasm_bindgen::throw_str(&e.to_string()),
    }
}

#[wasm_bindgen(js_name = getExampleNumberForType)]
pub fn get_example_number_for_type(number_type: JsPhoneNumberType) -> JsPhoneNumber {
    let r_type = match number_type {
        JsPhoneNumberType::FixedLine => PhoneNumberType::FixedLine,
        JsPhoneNumberType::Mobile => PhoneNumberType::Mobile,
        JsPhoneNumberType::FixedLineOrMobile => PhoneNumberType::FixedLineOrMobile,
        JsPhoneNumberType::TollFree => PhoneNumberType::TollFree,
        JsPhoneNumberType::PremiumRate => PhoneNumberType::PremiumRate,
        JsPhoneNumberType::SharedCost => PhoneNumberType::SharedCost,
        JsPhoneNumberType::Voip => PhoneNumberType::VoIP,
        JsPhoneNumberType::PersonalNumber => PhoneNumberType::PersonalNumber,
        JsPhoneNumberType::Pager => PhoneNumberType::Pager,
        JsPhoneNumberType::Uan => PhoneNumberType::UAN,
        JsPhoneNumberType::Voicemail => PhoneNumberType::VoiceMail,
        JsPhoneNumberType::Unknown => PhoneNumberType::Unknown,
    };

    match PHONE_NUMBER_UTIL.get_example_number_for_type(r_type) {
        Ok(inner) => JsPhoneNumber { inner },
        Err(e) => wasm_bindgen::throw_str(&e.to_string()),
    }
}

#[wasm_bindgen(js_name = getInvalidExampleNumber)]
pub fn get_invalid_example_number(region_code: &str) -> JsPhoneNumber {
    match PHONE_NUMBER_UTIL.get_invalid_example_number(region_code) {
        Ok(inner) => JsPhoneNumber { inner },
        Err(e) => wasm_bindgen::throw_str(&e.to_string()),
    }
}

#[wasm_bindgen(js_name = getExampleNumberForNonGeoEntity)]
pub fn get_example_number_for_non_geo_entity(country_calling_code: i32) -> JsPhoneNumber {
    match PHONE_NUMBER_UTIL.get_example_number_for_non_geo_entity(country_calling_code) {
        Ok(inner) => JsPhoneNumber { inner },
        Err(e) => wasm_bindgen::throw_str(&e.to_string()),
    }
}

#[wasm_bindgen(js_name = isPossibleNumberForString)]
pub fn is_possible_number_for_string(phone_number: &str, region_dialing_from: &str) -> bool {
    PHONE_NUMBER_UTIL.is_possible_number_for_string(phone_number, region_dialing_from)
}

#[wasm_bindgen(js_name = getSupportedGlobalNetworkCallingCodes)]
pub fn get_supported_global_network_calling_codes() -> Vec<i32> {
    PHONE_NUMBER_UTIL
        .get_supported_global_network_calling_codes()
        .collect()
}

#[wasm_bindgen(js_name = getSupportedCallingCodes)]
pub fn get_supported_calling_codes() -> Vec<i32> {
    PHONE_NUMBER_UTIL.get_supported_calling_codes().collect()
}

#[wasm_bindgen(js_name = trimUnwantedEndChars)]
pub fn trim_unwanted_end_chars(phone_number: &str) -> String {
    PHONE_NUMBER_UTIL
        .trim_unwanted_end_chars(phone_number)
        .to_string()
}

#[wasm_bindgen(js_name = normalizeDigitsOnly)]
pub fn normalize_digits_only(phone_number: &str) -> String {
    PHONE_NUMBER_UTIL.normalize_digits_only(phone_number)
}

#[wasm_bindgen(js_name = normalizeDiallableCharsOnly)]
pub fn normalize_diallable_chars_only(phone_number: &str) -> String {
    PHONE_NUMBER_UTIL.normalize_diallable_chars_only(phone_number)
}

#[wasm_bindgen(js_name = getLengthOfGeographicalAreaCode)]
pub fn util_get_length_of_geographical_area_code(phone_number: &JsPhoneNumber) -> usize {
    PHONE_NUMBER_UTIL.get_length_of_geographical_area_code(&phone_number.inner)
}

#[wasm_bindgen(js_name = getLengthOfNationalDestinationCode)]
pub fn util_get_length_of_national_destination_code(phone_number: &JsPhoneNumber) -> usize {
    PHONE_NUMBER_UTIL.get_length_of_national_destination_code(&phone_number.inner)
}

#[wasm_bindgen(js_name = getNationalSignificantNumber)]
pub fn util_get_national_significant_number(phone_number: &JsPhoneNumber) -> String {
    PHONE_NUMBER_UTIL.get_national_significant_number(&phone_number.inner)
}

#[wasm_bindgen(js_name = getRegionCodeForCountryCode)]
pub fn get_region_code_for_country_code(country_code: i32) -> Option<String> {
    PHONE_NUMBER_UTIL
        .get_region_code_for_country_code(country_code)
        .map(|s| s.to_string())
}

#[wasm_bindgen(js_name = getRegionCodesForCountryCode)]
pub fn get_region_codes_for_country_code(country_code: i32) -> Vec<String> {
    match PHONE_NUMBER_UTIL.get_region_codes_for_country_code(country_code) {
        Some(iter) => iter.map(|s| s.to_string()).collect(),
        None => vec![],
    }
}

#[wasm_bindgen(js_name = getSupportedRegions)]
pub fn get_supported_regions() -> Vec<String> {
    PHONE_NUMBER_UTIL
        .get_supported_regions()
        .map(|s| s.to_string())
        .collect()
}

#[wasm_bindgen(js_name = isAlphaNumber)]
pub fn is_alpha_number(number: &str) -> bool {
    PHONE_NUMBER_UTIL.is_alpha_number(number)
}

#[wasm_bindgen(js_name = isNanpaCountry)]
pub fn is_nanpa_country(region_code: &str) -> bool {
    PHONE_NUMBER_UTIL.is_nanpa_country(region_code)
}

#[wasm_bindgen(js_name = isNumberGeographical)]
pub fn is_number_geographical(phone_number: &JsPhoneNumber) -> bool {
    PHONE_NUMBER_UTIL.is_number_geographical(&phone_number.inner)
}

#[wasm_bindgen(js_name = isNumberMatch)]
pub fn is_number_match(first_number: &JsPhoneNumber, second_number: &JsPhoneNumber) -> JsMatchType {
    PHONE_NUMBER_UTIL
        .is_number_match(&first_number.inner, &second_number.inner)
        .into()
}

#[wasm_bindgen(js_name = isMultipleStringsNumberMatch)]
pub fn is_number_match_with_two_strings(first_number: &str, second_number: &str) -> JsMatchType {
    PHONE_NUMBER_UTIL
        .is_number_match_with_two_strings(first_number, second_number)
        .into()
}

#[wasm_bindgen(js_name = isPossibleNumber)]
pub fn is_possible_number(phone_number: &JsPhoneNumber) -> bool {
    PHONE_NUMBER_UTIL.is_possible_number(&phone_number.inner)
}

#[wasm_bindgen(js_name = isPossibleNumberWithReason)]
pub fn is_possible_number_with_reason(phone_number: &JsPhoneNumber) -> JsNumberLengthType {
    match PHONE_NUMBER_UTIL.is_possible_number_with_reason(&phone_number.inner) {
        Ok(length_type) => length_type.into(),
        Err(e) => wasm_bindgen::throw_str(&e.to_string()),
    }
}

#[wasm_bindgen(js_name = isValidNumberForRegion)]
pub fn is_valid_number_for_region(phone_number: &JsPhoneNumber, region: &str) -> bool {
    PHONE_NUMBER_UTIL.is_valid_number_for_region(&phone_number.inner, region)
}

#[wasm_bindgen(js_name = parseAndKeepRawInputWithDefaultRegion)]
pub fn parse_and_keep_raw_input_with_default_region(
    number_to_parse: &str,
    default_region: &str,
) -> JsPhoneNumber {
    match PHONE_NUMBER_UTIL
        .parse_and_keep_raw_input_with_default_region(number_to_parse, default_region)
    {
        Ok(inner) => JsPhoneNumber { inner },
        Err(e) => wasm_bindgen::throw_str(&e.to_string()),
    }
}

#[wasm_bindgen(js_name = parseAndKeepRawInput)]
pub fn parse_and_keep_raw_input(number_to_parse: &str) -> JsPhoneNumber {
    match PHONE_NUMBER_UTIL.parse_and_keep_raw_input(number_to_parse) {
        Ok(inner) => JsPhoneNumber { inner },
        Err(e) => wasm_bindgen::throw_str(&e.to_string()),
    }
}

#[wasm_bindgen(js_name = truncateTooLongNumber)]
pub fn util_truncate_too_long_number(phone_number: &mut JsPhoneNumber) -> bool {
    PHONE_NUMBER_UTIL.truncate_too_long_number(&mut phone_number.inner)
}
