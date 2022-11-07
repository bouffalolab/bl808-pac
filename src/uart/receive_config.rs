#[doc = "Register `receive_config` reader"]
pub struct R(crate::R<RECEIVE_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECEIVE_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECEIVE_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECEIVE_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `receive_config` writer"]
pub struct W(crate::W<RECEIVE_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECEIVE_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RECEIVE_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECEIVE_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `function` reader - Enable receive function"]
pub type FUNCTION_R = crate::BitReader<FUNCTION_A>;
#[doc = "Enable receive function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUNCTION_A {
    #[doc = "1: Enable UART receive function signal"]
    ENABLE = 1,
    #[doc = "0: Disable UART receive function signal"]
    DISABLE = 0,
}
impl From<FUNCTION_A> for bool {
    #[inline(always)]
    fn from(variant: FUNCTION_A) -> Self {
        variant as u8 != 0
    }
}
impl FUNCTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUNCTION_A {
        match self.bits {
            true => FUNCTION_A::ENABLE,
            false => FUNCTION_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FUNCTION_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FUNCTION_A::DISABLE
    }
}
#[doc = "Field `function` writer - Enable receive function"]
pub type FUNCTION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RECEIVE_CONFIG_SPEC, FUNCTION_A, O>;
impl<'a, const O: u8> FUNCTION_W<'a, O> {
    #[doc = "Enable UART receive function signal"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FUNCTION_A::ENABLE)
    }
    #[doc = "Disable UART receive function signal"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FUNCTION_A::DISABLE)
    }
}
#[doc = "Field `auto_baudrate` reader - Enable receive auto baudrate detection"]
pub type AUTO_BAUDRATE_R = crate::BitReader<AUTO_BAUDRATE_A>;
#[doc = "Enable receive auto baudrate detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTO_BAUDRATE_A {
    #[doc = "1: Enable auto baudrate upon receive"]
    ENABLE = 1,
    #[doc = "0: Disable auto baudrate upon receive"]
    DISABLE = 0,
}
impl From<AUTO_BAUDRATE_A> for bool {
    #[inline(always)]
    fn from(variant: AUTO_BAUDRATE_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTO_BAUDRATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTO_BAUDRATE_A {
        match self.bits {
            true => AUTO_BAUDRATE_A::ENABLE,
            false => AUTO_BAUDRATE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AUTO_BAUDRATE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AUTO_BAUDRATE_A::DISABLE
    }
}
#[doc = "Field `auto_baudrate` writer - Enable receive auto baudrate detection"]
pub type AUTO_BAUDRATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RECEIVE_CONFIG_SPEC, AUTO_BAUDRATE_A, O>;
impl<'a, const O: u8> AUTO_BAUDRATE_W<'a, O> {
    #[doc = "Enable auto baudrate upon receive"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUTO_BAUDRATE_A::ENABLE)
    }
    #[doc = "Disable auto baudrate upon receive"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUTO_BAUDRATE_A::DISABLE)
    }
}
#[doc = "Field `lin_receive` reader - Local Interconnect Network protocol enable"]
pub type LIN_RECEIVE_R = crate::BitReader<LIN_RECEIVE_A>;
#[doc = "Local Interconnect Network protocol enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIN_RECEIVE_A {
    #[doc = "1: Enable Local Interconnect Network protocol"]
    ENABLE = 1,
    #[doc = "0: Disable Local Interconnect Network protocol"]
    DISABLE = 0,
}
impl From<LIN_RECEIVE_A> for bool {
    #[inline(always)]
    fn from(variant: LIN_RECEIVE_A) -> Self {
        variant as u8 != 0
    }
}
impl LIN_RECEIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIN_RECEIVE_A {
        match self.bits {
            true => LIN_RECEIVE_A::ENABLE,
            false => LIN_RECEIVE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LIN_RECEIVE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LIN_RECEIVE_A::DISABLE
    }
}
#[doc = "Field `lin_receive` writer - Local Interconnect Network protocol enable"]
pub type LIN_RECEIVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RECEIVE_CONFIG_SPEC, LIN_RECEIVE_A, O>;
impl<'a, const O: u8> LIN_RECEIVE_W<'a, O> {
    #[doc = "Enable Local Interconnect Network protocol"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LIN_RECEIVE_A::ENABLE)
    }
    #[doc = "Disable Local Interconnect Network protocol"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LIN_RECEIVE_A::DISABLE)
    }
}
#[doc = "Field `parity_enable` reader - Enable receive parity check"]
pub type PARITY_ENABLE_R = crate::BitReader<PARITY_ENABLE_A>;
#[doc = "Enable receive parity check\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARITY_ENABLE_A {
    #[doc = "1: Enable receive parity check"]
    ENABLE = 1,
    #[doc = "0: Disable receive parity check"]
    DISABLE = 0,
}
impl From<PARITY_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PARITY_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl PARITY_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITY_ENABLE_A {
        match self.bits {
            true => PARITY_ENABLE_A::ENABLE,
            false => PARITY_ENABLE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PARITY_ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PARITY_ENABLE_A::DISABLE
    }
}
#[doc = "Field `parity_enable` writer - Enable receive parity check"]
pub type PARITY_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RECEIVE_CONFIG_SPEC, PARITY_ENABLE_A, O>;
impl<'a, const O: u8> PARITY_ENABLE_W<'a, O> {
    #[doc = "Enable receive parity check"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PARITY_ENABLE_A::ENABLE)
    }
    #[doc = "Disable receive parity check"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PARITY_ENABLE_A::DISABLE)
    }
}
#[doc = "Field `parity_mode` reader - Select receive parity mode if enabled"]
pub type PARITY_MODE_R = crate::BitReader<PARITY_MODE_A>;
#[doc = "Select receive parity mode if enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARITY_MODE_A {
    #[doc = "1: Odd parity if `parity_enable` is set"]
    ODD = 1,
    #[doc = "0: Even parity if `parity_enable` is set"]
    EVEN = 0,
}
impl From<PARITY_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: PARITY_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl PARITY_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITY_MODE_A {
        match self.bits {
            true => PARITY_MODE_A::ODD,
            false => PARITY_MODE_A::EVEN,
        }
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PARITY_MODE_A::ODD
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PARITY_MODE_A::EVEN
    }
}
#[doc = "Field `parity_mode` writer - Select receive parity mode if enabled"]
pub type PARITY_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RECEIVE_CONFIG_SPEC, PARITY_MODE_A, O>;
impl<'a, const O: u8> PARITY_MODE_W<'a, O> {
    #[doc = "Odd parity if `parity_enable` is set"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PARITY_MODE_A::ODD)
    }
    #[doc = "Even parity if `parity_enable` is set"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PARITY_MODE_A::EVEN)
    }
}
#[doc = "Field `ir_receive` reader - Enable IR receive mode"]
pub type IR_RECEIVE_R = crate::BitReader<IR_RECEIVE_A>;
#[doc = "Enable IR receive mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IR_RECEIVE_A {
    #[doc = "1: Enable IR receive mode"]
    ENABLE = 1,
    #[doc = "0: Disable IR receive mode"]
    DISABLE = 0,
}
impl From<IR_RECEIVE_A> for bool {
    #[inline(always)]
    fn from(variant: IR_RECEIVE_A) -> Self {
        variant as u8 != 0
    }
}
impl IR_RECEIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IR_RECEIVE_A {
        match self.bits {
            true => IR_RECEIVE_A::ENABLE,
            false => IR_RECEIVE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IR_RECEIVE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IR_RECEIVE_A::DISABLE
    }
}
#[doc = "Field `ir_receive` writer - Enable IR receive mode"]
pub type IR_RECEIVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RECEIVE_CONFIG_SPEC, IR_RECEIVE_A, O>;
impl<'a, const O: u8> IR_RECEIVE_W<'a, O> {
    #[doc = "Enable IR receive mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IR_RECEIVE_A::ENABLE)
    }
    #[doc = "Disable IR receive mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IR_RECEIVE_A::DISABLE)
    }
}
#[doc = "Field `ir_inverse` reader - Inverse receive signal output in IR mode"]
pub type IR_INVERSE_R = crate::BitReader<IR_INVERSE_A>;
#[doc = "Inverse receive signal output in IR mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IR_INVERSE_A {
    #[doc = "1: Inverse receive input in IR mode"]
    INVERSE = 1,
    #[doc = "0: Don't inverse receive input in IR mode"]
    NO_INVERSE = 0,
}
impl From<IR_INVERSE_A> for bool {
    #[inline(always)]
    fn from(variant: IR_INVERSE_A) -> Self {
        variant as u8 != 0
    }
}
impl IR_INVERSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IR_INVERSE_A {
        match self.bits {
            true => IR_INVERSE_A::INVERSE,
            false => IR_INVERSE_A::NO_INVERSE,
        }
    }
    #[doc = "Checks if the value of the field is `INVERSE`"]
    #[inline(always)]
    pub fn is_inverse(&self) -> bool {
        *self == IR_INVERSE_A::INVERSE
    }
    #[doc = "Checks if the value of the field is `NO_INVERSE`"]
    #[inline(always)]
    pub fn is_no_inverse(&self) -> bool {
        *self == IR_INVERSE_A::NO_INVERSE
    }
}
#[doc = "Field `ir_inverse` writer - Inverse receive signal output in IR mode"]
pub type IR_INVERSE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RECEIVE_CONFIG_SPEC, IR_INVERSE_A, O>;
impl<'a, const O: u8> IR_INVERSE_W<'a, O> {
    #[doc = "Inverse receive input in IR mode"]
    #[inline(always)]
    pub fn inverse(self) -> &'a mut W {
        self.variant(IR_INVERSE_A::INVERSE)
    }
    #[doc = "Don't inverse receive input in IR mode"]
    #[inline(always)]
    pub fn no_inverse(self) -> &'a mut W {
        self.variant(IR_INVERSE_A::NO_INVERSE)
    }
}
#[doc = "Field `word_length` reader - Bit count for each receive data word"]
pub type WORD_LENGTH_R = crate::FieldReader<u8, WORD_LENGTH_A>;
#[doc = "Bit count for each receive data word\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WORD_LENGTH_A {
    #[doc = "4: Each word includes 5 bits"]
    FIVE = 4,
    #[doc = "5: Each word includes 6 bits"]
    SIX = 5,
    #[doc = "6: Each word includes 7 bits"]
    SEVEN = 6,
    #[doc = "7: Each word includes 8 bits"]
    EIGHT = 7,
}
impl From<WORD_LENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: WORD_LENGTH_A) -> Self {
        variant as _
    }
}
impl WORD_LENGTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WORD_LENGTH_A> {
        match self.bits {
            4 => Some(WORD_LENGTH_A::FIVE),
            5 => Some(WORD_LENGTH_A::SIX),
            6 => Some(WORD_LENGTH_A::SEVEN),
            7 => Some(WORD_LENGTH_A::EIGHT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FIVE`"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == WORD_LENGTH_A::FIVE
    }
    #[doc = "Checks if the value of the field is `SIX`"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == WORD_LENGTH_A::SIX
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == WORD_LENGTH_A::SEVEN
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == WORD_LENGTH_A::EIGHT
    }
}
#[doc = "Field `word_length` writer - Bit count for each receive data word"]
pub type WORD_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RECEIVE_CONFIG_SPEC, u8, WORD_LENGTH_A, 3, O>;
impl<'a, const O: u8> WORD_LENGTH_W<'a, O> {
    #[doc = "Each word includes 5 bits"]
    #[inline(always)]
    pub fn five(self) -> &'a mut W {
        self.variant(WORD_LENGTH_A::FIVE)
    }
    #[doc = "Each word includes 6 bits"]
    #[inline(always)]
    pub fn six(self) -> &'a mut W {
        self.variant(WORD_LENGTH_A::SIX)
    }
    #[doc = "Each word includes 7 bits"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut W {
        self.variant(WORD_LENGTH_A::SEVEN)
    }
    #[doc = "Each word includes 8 bits"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut W {
        self.variant(WORD_LENGTH_A::EIGHT)
    }
}
#[doc = "Field `deglitch_enable` reader - Enable receive de-glitch function"]
pub type DEGLITCH_ENABLE_R = crate::BitReader<DEGLITCH_ENABLE_A>;
#[doc = "Enable receive de-glitch function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEGLITCH_ENABLE_A {
    #[doc = "1: Enable de-glitch function upon receive"]
    ENABLE = 1,
    #[doc = "0: Disable de-glitch function upon receive"]
    DISABLE = 0,
}
impl From<DEGLITCH_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DEGLITCH_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl DEGLITCH_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEGLITCH_ENABLE_A {
        match self.bits {
            true => DEGLITCH_ENABLE_A::ENABLE,
            false => DEGLITCH_ENABLE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DEGLITCH_ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DEGLITCH_ENABLE_A::DISABLE
    }
}
#[doc = "Field `deglitch_enable` writer - Enable receive de-glitch function"]
pub type DEGLITCH_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RECEIVE_CONFIG_SPEC, DEGLITCH_ENABLE_A, O>;
impl<'a, const O: u8> DEGLITCH_ENABLE_W<'a, O> {
    #[doc = "Enable de-glitch function upon receive"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DEGLITCH_ENABLE_A::ENABLE)
    }
    #[doc = "Disable de-glitch function upon receive"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DEGLITCH_ENABLE_A::DISABLE)
    }
}
#[doc = "Field `deglitch_cycle` reader - De-glitch function cycle count"]
pub type DEGLITCH_CYCLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `deglitch_cycle` writer - De-glitch function cycle count"]
pub type DEGLITCH_CYCLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RECEIVE_CONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `transfer_length` reader - Length of words per UART receive transfer"]
pub type TRANSFER_LENGTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `transfer_length` writer - Length of words per UART receive transfer"]
pub type TRANSFER_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RECEIVE_CONFIG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Enable receive function"]
    #[inline(always)]
    pub fn function(&self) -> FUNCTION_R {
        FUNCTION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable receive auto baudrate detection"]
    #[inline(always)]
    pub fn auto_baudrate(&self) -> AUTO_BAUDRATE_R {
        AUTO_BAUDRATE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Local Interconnect Network protocol enable"]
    #[inline(always)]
    pub fn lin_receive(&self) -> LIN_RECEIVE_R {
        LIN_RECEIVE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable receive parity check"]
    #[inline(always)]
    pub fn parity_enable(&self) -> PARITY_ENABLE_R {
        PARITY_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select receive parity mode if enabled"]
    #[inline(always)]
    pub fn parity_mode(&self) -> PARITY_MODE_R {
        PARITY_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable IR receive mode"]
    #[inline(always)]
    pub fn ir_receive(&self) -> IR_RECEIVE_R {
        IR_RECEIVE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Inverse receive signal output in IR mode"]
    #[inline(always)]
    pub fn ir_inverse(&self) -> IR_INVERSE_R {
        IR_INVERSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Bit count for each receive data word"]
    #[inline(always)]
    pub fn word_length(&self) -> WORD_LENGTH_R {
        WORD_LENGTH_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Enable receive de-glitch function"]
    #[inline(always)]
    pub fn deglitch_enable(&self) -> DEGLITCH_ENABLE_R {
        DEGLITCH_ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - De-glitch function cycle count"]
    #[inline(always)]
    pub fn deglitch_cycle(&self) -> DEGLITCH_CYCLE_R {
        DEGLITCH_CYCLE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Length of words per UART receive transfer"]
    #[inline(always)]
    pub fn transfer_length(&self) -> TRANSFER_LENGTH_R {
        TRANSFER_LENGTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable receive function"]
    #[inline(always)]
    #[must_use]
    pub fn function(&mut self) -> FUNCTION_W<0> {
        FUNCTION_W::new(self)
    }
    #[doc = "Bit 1 - Enable receive auto baudrate detection"]
    #[inline(always)]
    #[must_use]
    pub fn auto_baudrate(&mut self) -> AUTO_BAUDRATE_W<1> {
        AUTO_BAUDRATE_W::new(self)
    }
    #[doc = "Bit 3 - Local Interconnect Network protocol enable"]
    #[inline(always)]
    #[must_use]
    pub fn lin_receive(&mut self) -> LIN_RECEIVE_W<3> {
        LIN_RECEIVE_W::new(self)
    }
    #[doc = "Bit 4 - Enable receive parity check"]
    #[inline(always)]
    #[must_use]
    pub fn parity_enable(&mut self) -> PARITY_ENABLE_W<4> {
        PARITY_ENABLE_W::new(self)
    }
    #[doc = "Bit 5 - Select receive parity mode if enabled"]
    #[inline(always)]
    #[must_use]
    pub fn parity_mode(&mut self) -> PARITY_MODE_W<5> {
        PARITY_MODE_W::new(self)
    }
    #[doc = "Bit 6 - Enable IR receive mode"]
    #[inline(always)]
    #[must_use]
    pub fn ir_receive(&mut self) -> IR_RECEIVE_W<6> {
        IR_RECEIVE_W::new(self)
    }
    #[doc = "Bit 7 - Inverse receive signal output in IR mode"]
    #[inline(always)]
    #[must_use]
    pub fn ir_inverse(&mut self) -> IR_INVERSE_W<7> {
        IR_INVERSE_W::new(self)
    }
    #[doc = "Bits 8:10 - Bit count for each receive data word"]
    #[inline(always)]
    #[must_use]
    pub fn word_length(&mut self) -> WORD_LENGTH_W<8> {
        WORD_LENGTH_W::new(self)
    }
    #[doc = "Bit 11 - Enable receive de-glitch function"]
    #[inline(always)]
    #[must_use]
    pub fn deglitch_enable(&mut self) -> DEGLITCH_ENABLE_W<11> {
        DEGLITCH_ENABLE_W::new(self)
    }
    #[doc = "Bits 12:15 - De-glitch function cycle count"]
    #[inline(always)]
    #[must_use]
    pub fn deglitch_cycle(&mut self) -> DEGLITCH_CYCLE_W<12> {
        DEGLITCH_CYCLE_W::new(self)
    }
    #[doc = "Bits 16:31 - Length of words per UART receive transfer"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_length(&mut self) -> TRANSFER_LENGTH_W<16> {
        TRANSFER_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receive_config](index.html) module"]
pub struct RECEIVE_CONFIG_SPEC;
impl crate::RegisterSpec for RECEIVE_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [receive_config::R](R) reader structure"]
impl crate::Readable for RECEIVE_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [receive_config::W](W) writer structure"]
impl crate::Writable for RECEIVE_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets receive_config to value 0x0700"]
impl crate::Resettable for RECEIVE_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0700;
}
