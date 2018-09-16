#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub const ETH_UNITS : &'static [&'static str] = &["wei",
    "kwei",
    "Mwei",
    "Gwei",
    "szabo",
    "finney",
    "femtoether",
    "picoether",
    "nanoether",
    "microether",
    "milliether",
    "nano",
    "micro",
    "milli",
    "ether",
    "grand",
    "Mether",
    "Gether",
    "Tether",
    "Pether",
    "Eether",
    "Zether",
    "Yether",
    "Nether",
    "Dether",
    "Vether",
    "Uether"];

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct Config {
    ETH_PADDING: u32,
    ETH_SIGNATURE_LENGTH: u32,
    ETH_UNITS: &'static [&'static str],
    ETH_POLLING_TIMEOUT: u32,
    defaultBlock: &'static str,
    defaultAccount: Option<String>,
    // ETH_BIGNUMBER_ROUNDING_MODE: { ROUNDING_MODE: BigNumber.ROUND_DOWN },

}
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
static mut config: Config = Config{
    ETH_PADDING: 32 ,
    ETH_SIGNATURE_LENGTH: 4,
    ETH_UNITS: ETH_UNITS ,
    ETH_POLLING_TIMEOUT: 1000/2,
    defaultBlock: "latest",
    defaultAccount:None,
    // ETH_BIGNUMBER_ROUNDING_MODE: { ROUNDING_MODE: BigNumber.ROUND_DOWN },
};
