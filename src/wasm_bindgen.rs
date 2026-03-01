use crate::{MatchType, PHONE_NUMBER_UTIL, PhoneNumber, PhoneNumberFormat, PhoneNumberType};
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
#[derive(Copy, Clone)]
pub enum JsMatchType {
    NoMatch = 1,
    ShortNSNMatch = 2,
    NSNMatch = 3,
    ExactMatch = 4,
}

impl From<MatchType> for JsMatchType {
    fn from(m: MatchType) -> Self {
        match m {
            MatchType::NoMatch => JsMatchType::NoMatch,
            MatchType::ShortNsnMatch => JsMatchType::ShortNSNMatch,
            MatchType::NsnMatch => JsMatchType::NSNMatch,
            MatchType::ExactMatch => JsMatchType::ExactMatch,
        }
    }
}

#[wasm_bindgen]
pub struct JsPhoneNumber {
    inner: PhoneNumber,
}

#[wasm_bindgen]
impl JsPhoneNumber {
    #[wasm_bindgen(getter)]
    pub fn country_code(&self) -> Option<i32> {
        self.inner.country_code
    }

    #[wasm_bindgen(getter)]
    pub fn national_number(&self) -> Option<u64> {
        self.inner.national_number
    }

    #[wasm_bindgen(getter)]
    pub fn extension(&self) -> Option<String> {
        self.inner.extension.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn italian_leading_zero(&self) -> bool {
        self.inner.italian_leading_zero.unwrap_or(false)
    }

    #[wasm_bindgen(getter)]
    pub fn raw_input(&self) -> Option<String> {
        self.inner.raw_input.clone()
    }
}

#[wasm_bindgen]
pub struct PhoneUtil {}

#[wasm_bindgen]
impl PhoneUtil {
    pub fn parse(
        &self,
        number_to_parse: &str,
        default_region: &str,
    ) -> Result<JsPhoneNumber, JsError> {
        match PHONE_NUMBER_UTIL.parse_with_default_region(number_to_parse, default_region) {
            Ok(phone_number) => Ok(JsPhoneNumber {
                inner: phone_number,
            }),
            Err(e) => Err(JsError::new(&e.to_string())),
        }
    }

    #[wasm_bindgen(js_name = isValidNumber)]
    pub fn is_valid_number(&self, number: &JsPhoneNumber) -> bool {
        PHONE_NUMBER_UTIL.is_valid_number(&number.inner)
    }

    #[wasm_bindgen(js_name = isValidNumberForRegion)]
    pub fn is_valid_number_for_region(&self, number: &JsPhoneNumber, region_code: &str) -> bool {
        PHONE_NUMBER_UTIL.is_valid_number_for_region(&number.inner, region_code)
    }

    #[wasm_bindgen(js_name = isPossibleNumber)]
    pub fn is_possible_number(&self, number: &JsPhoneNumber) -> bool {
        PHONE_NUMBER_UTIL.is_possible_number(&number.inner)
    }

    pub fn format(&self, number: &JsPhoneNumber, format: JsPhoneNumberFormat) -> String {
        PHONE_NUMBER_UTIL
            .format(&number.inner, format.into())
            .to_string()
    }

    #[wasm_bindgen(js_name = formatOutOfCountryCallingNumber)]
    pub fn format_out_of_country_calling_number(
        &self,
        number: &JsPhoneNumber,
        region_calling_from: &str,
    ) -> String {
        PHONE_NUMBER_UTIL
            .format_out_of_country_calling_number(&number.inner, region_calling_from)
            .to_string()
    }

    #[wasm_bindgen(js_name = getNumberType)]
    pub fn get_number_type(&self, number: &JsPhoneNumber) -> JsPhoneNumberType {
        PHONE_NUMBER_UTIL.get_number_type(&number.inner).into()
    }

    #[wasm_bindgen(js_name = getRegionCodeForNumber)]
    pub fn get_region_code_for_number(&self, number: &JsPhoneNumber) -> Option<String> {
        PHONE_NUMBER_UTIL
            .get_region_code_for_number(&number.inner)
            .map(|s| s.to_string())
    }

    #[wasm_bindgen(js_name = getCountryCodeForRegion)]
    pub fn get_country_code_for_region(&self, region_code: &str) -> Option<i32> {
        PHONE_NUMBER_UTIL.get_country_code_for_region(region_code)
    }

    #[wasm_bindgen(js_name = getRegionCodeForCountryCode)]
    pub fn get_region_code_for_country_code(&self, country_code: i32) -> Option<String> {
        PHONE_NUMBER_UTIL
            .get_region_code_for_country_code(country_code)
            .map(|s| s.to_string())
    }

    #[wasm_bindgen(js_name = getExampleNumber)]
    pub fn get_example_number(&self, region_code: &str) -> Option<JsPhoneNumber> {
        PHONE_NUMBER_UTIL
            .get_example_number(region_code)
            .ok()
            .map(|pn| JsPhoneNumber { inner: pn })
    }

    #[wasm_bindgen(js_name = getExampleNumberForType)]
    pub fn get_example_number_for_type(
        &self,
        number_type: JsPhoneNumberType,
    ) -> Option<JsPhoneNumber> {
        let rust_type = match number_type {
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

        PHONE_NUMBER_UTIL
            .get_example_number_for_type(rust_type)
            .ok()
            .map(|pn| JsPhoneNumber { inner: pn })
    }

    #[wasm_bindgen(js_name = convertAlphaCharactersInNumber)]
    pub fn convert_alpha_characters_in_number(&self, number: &str) -> String {
        PHONE_NUMBER_UTIL.convert_alpha_characters_in_number(number)
    }

    #[wasm_bindgen(js_name = isAlphaNumber)]
    pub fn is_alpha_number(&self, number: &str) -> bool {
        PHONE_NUMBER_UTIL.is_alpha_number(number)
    }
}
