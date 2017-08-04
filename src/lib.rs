#[macro_use]
extern crate error_chain;

pub mod error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Denomination {
    pub value: u32,
    pub currency: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DenominationCount {
    pub count: u16,
    pub value: u32,
    pub currency: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncompleteCoinValue {
    pub value: u32,
    pub value_requested: u32,
    pub country_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CashboxPayoutData {
    known: Vec<DenominationCount>,
    unknown_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceType {
    BillValidator,
    SmartHopper,
}

#[derive(Debug)]
pub enum DevicePollResult {
    Progress { event: String, repeat: bool },
    Credit(u32),
    Dispensing(u32),
    Dispensed(u32),
    PayoutTimeout(u32),
    CoinsLow,
    FraudAttempt(u32),
    HopperJammed(u32),
    CoinMechJammed,
    CoinMechReturnActive,
    CoinMechError(String),
    DeviceFull,
    SmartEmptying(u32),
    SmartEmptied(u32),
    Error(error::Error),
}

pub trait Device {
    fn init(&mut self, denominations: &Vec<Denomination>, accepted_denominations: &Vec<Denomination>) -> error::Result<()>;
    fn enable(&mut self) -> error::Result<()>;
    fn disable(&mut self) -> error::Result<()>;
    fn poll(&mut self) -> error::Result<Vec<DevicePollResult>>;
}

pub trait PayinDevice {}

pub trait PayoutDevice {
    fn payout_amount(&mut self, amount: u32, country_code: &str, test: bool) -> error::Result<()>;
    fn get_levels(&mut self) -> error::Result<Vec<DenominationCount>>;
    fn set_levels(&mut self) -> error::Result<()>;
    fn empty(&mut self) -> error::Result<()>;
}

pub trait SmartHopper: PayinDevice + PayoutDevice {
    fn smart_empty(&mut self) -> error::Result<()>;
    fn cashbox_payout_operation_data(&mut self) -> error::Result<CashboxPayoutData>;
}
