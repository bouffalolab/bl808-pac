#[doc = "Register `transmit_config` reader"]
pub struct R(crate::R<TRANSMIT_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSMIT_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSMIT_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSMIT_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `transmit_config` writer"]
pub struct W(crate::W<TRANSMIT_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSMIT_CONFIG_SPEC>;
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
impl From<crate::W<TRANSMIT_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSMIT_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `transfer_length` reader - Length of words per UART transmit transfer\n\n This field is ignored when `freerun` mode is enabled."]
pub struct TRANSFER_LENGTH_R(crate::FieldReader<u16>);
impl TRANSFER_LENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TRANSFER_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANSFER_LENGTH_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `transfer_length` writer - Length of words per UART transmit transfer\n\n This field is ignored when `freerun` mode is enabled."]
pub struct TRANSFER_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSFER_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `stop_bits` reader - Number of stop bits"]
pub struct STOP_BITS_R(crate::FieldReader<u8>);
impl STOP_BITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STOP_BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_BITS_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `stop_bits` writer - Number of stop bits"]
pub struct STOP_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `word_length` reader - Bit count for each transmit data word"]
pub struct WORD_LENGTH_R(crate::FieldReader<u8>);
impl WORD_LENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WORD_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WORD_LENGTH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `word_length` writer - Bit count for each transmit data word"]
pub struct WORD_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WORD_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 8)) | ((value as u32 & 7) << 8);
        self.w
    }
}
#[doc = "Inverse transmit signal output in IR mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IR_INVERSE_A {
    #[doc = "1: Inverse transmit input in IR mode"]
    INVERSE = 1,
    #[doc = "0: Don't inverse transmit input in IR mode"]
    NO_INVERSE = 0,
}
impl From<IR_INVERSE_A> for bool {
    #[inline(always)]
    fn from(variant: IR_INVERSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ir_inverse` reader - Inverse transmit signal output in IR mode"]
pub struct IR_INVERSE_R(crate::FieldReader<bool>);
impl IR_INVERSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IR_INVERSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == IR_INVERSE_A::INVERSE
    }
    #[doc = "Checks if the value of the field is `NO_INVERSE`"]
    #[inline(always)]
    pub fn is_no_inverse(&self) -> bool {
        **self == IR_INVERSE_A::NO_INVERSE
    }
}
impl core::ops::Deref for IR_INVERSE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ir_inverse` writer - Inverse transmit signal output in IR mode"]
pub struct IR_INVERSE_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_INVERSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IR_INVERSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Inverse transmit input in IR mode"]
    #[inline(always)]
    pub fn inverse(self) -> &'a mut W {
        self.variant(IR_INVERSE_A::INVERSE)
    }
    #[doc = "Don't inverse transmit input in IR mode"]
    #[inline(always)]
    pub fn no_inverse(self) -> &'a mut W {
        self.variant(IR_INVERSE_A::NO_INVERSE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Enable IR transmit mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IR_TRANSMIT_A {
    #[doc = "1: Enable IR transmit mode"]
    ENABLE = 1,
    #[doc = "0: Disable IR transmit mode"]
    DISABLE = 0,
}
impl From<IR_TRANSMIT_A> for bool {
    #[inline(always)]
    fn from(variant: IR_TRANSMIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ir_transmit` reader - Enable IR transmit mode"]
pub struct IR_TRANSMIT_R(crate::FieldReader<bool>);
impl IR_TRANSMIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IR_TRANSMIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IR_TRANSMIT_A {
        match self.bits {
            true => IR_TRANSMIT_A::ENABLE,
            false => IR_TRANSMIT_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IR_TRANSMIT_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IR_TRANSMIT_A::DISABLE
    }
}
impl core::ops::Deref for IR_TRANSMIT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ir_transmit` writer - Enable IR transmit mode"]
pub struct IR_TRANSMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_TRANSMIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IR_TRANSMIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable IR transmit mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IR_TRANSMIT_A::ENABLE)
    }
    #[doc = "Disable IR transmit mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IR_TRANSMIT_A::DISABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Select transmit parity mode if enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `parity_mode` reader - Select transmit parity mode if enabled"]
pub struct PARITY_MODE_R(crate::FieldReader<bool>);
impl PARITY_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PARITY_MODE_A::ODD
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        **self == PARITY_MODE_A::EVEN
    }
}
impl core::ops::Deref for PARITY_MODE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `parity_mode` writer - Select transmit parity mode if enabled"]
pub struct PARITY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARITY_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Enable transmit parity check\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `parity_enable` reader - Enable transmit parity check"]
pub struct PARITY_ENABLE_R(crate::FieldReader<bool>);
impl PARITY_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PARITY_ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PARITY_ENABLE_A::DISABLE
    }
}
impl core::ops::Deref for PARITY_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `parity_enable` writer - Enable transmit parity check"]
pub struct PARITY_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARITY_ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Enable freerun mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREERUN_A {
    #[doc = "1: Enable freerun mode"]
    ENABLE = 1,
    #[doc = "0: Disable freerun mode"]
    DISABLE = 0,
}
impl From<FREERUN_A> for bool {
    #[inline(always)]
    fn from(variant: FREERUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `freerun` reader - Enable freerun mode"]
pub struct FREERUN_R(crate::FieldReader<bool>);
impl FREERUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FREERUN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREERUN_A {
        match self.bits {
            true => FREERUN_A::ENABLE,
            false => FREERUN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FREERUN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FREERUN_A::DISABLE
    }
}
impl core::ops::Deref for FREERUN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `freerun` writer - Enable freerun mode"]
pub struct FREERUN_W<'a> {
    w: &'a mut W,
}
impl<'a> FREERUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREERUN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable freerun mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FREERUN_A::ENABLE)
    }
    #[doc = "Disable freerun mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FREERUN_A::DISABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Enable Clear-to-Send flow control signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_A {
    #[doc = "1: Enable Clear-to-Send flow control signal"]
    ENABLE = 1,
    #[doc = "0: Disable Clear-to-Send flow control signal"]
    DISABLE = 0,
}
impl From<CTS_A> for bool {
    #[inline(always)]
    fn from(variant: CTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cts` reader - Enable Clear-to-Send flow control signal"]
pub struct CTS_R(crate::FieldReader<bool>);
impl CTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTS_A {
        match self.bits {
            true => CTS_A::ENABLE,
            false => CTS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CTS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CTS_A::DISABLE
    }
}
impl core::ops::Deref for CTS_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cts` writer - Enable Clear-to-Send flow control signal"]
pub struct CTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable Clear-to-Send flow control signal"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTS_A::ENABLE)
    }
    #[doc = "Disable Clear-to-Send flow control signal"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTS_A::DISABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Enable transmit function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `function` reader - Enable transmit function"]
pub struct FUNCTION_R(crate::FieldReader<bool>);
impl FUNCTION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FUNCTION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == FUNCTION_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FUNCTION_A::DISABLE
    }
}
impl core::ops::Deref for FUNCTION_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `function` writer - Enable transmit function"]
pub struct FUNCTION_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNCTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNCTION_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Length of words per UART transmit transfer\n\n This field is ignored when `freerun` mode is enabled."]
    #[inline(always)]
    pub fn transfer_length(&self) -> TRANSFER_LENGTH_R {
        TRANSFER_LENGTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 12:13 - Number of stop bits"]
    #[inline(always)]
    pub fn stop_bits(&self) -> STOP_BITS_R {
        STOP_BITS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Bit count for each transmit data word"]
    #[inline(always)]
    pub fn word_length(&self) -> WORD_LENGTH_R {
        WORD_LENGTH_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 7 - Inverse transmit signal output in IR mode"]
    #[inline(always)]
    pub fn ir_inverse(&self) -> IR_INVERSE_R {
        IR_INVERSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable IR transmit mode"]
    #[inline(always)]
    pub fn ir_transmit(&self) -> IR_TRANSMIT_R {
        IR_TRANSMIT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Select transmit parity mode if enabled"]
    #[inline(always)]
    pub fn parity_mode(&self) -> PARITY_MODE_R {
        PARITY_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable transmit parity check"]
    #[inline(always)]
    pub fn parity_enable(&self) -> PARITY_ENABLE_R {
        PARITY_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable freerun mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Clear-to-Send flow control signal"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Enable transmit function"]
    #[inline(always)]
    pub fn function(&self) -> FUNCTION_R {
        FUNCTION_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - Length of words per UART transmit transfer\n\n This field is ignored when `freerun` mode is enabled."]
    #[inline(always)]
    pub fn transfer_length(&mut self) -> TRANSFER_LENGTH_W {
        TRANSFER_LENGTH_W { w: self }
    }
    #[doc = "Bits 12:13 - Number of stop bits"]
    #[inline(always)]
    pub fn stop_bits(&mut self) -> STOP_BITS_W {
        STOP_BITS_W { w: self }
    }
    #[doc = "Bits 8:10 - Bit count for each transmit data word"]
    #[inline(always)]
    pub fn word_length(&mut self) -> WORD_LENGTH_W {
        WORD_LENGTH_W { w: self }
    }
    #[doc = "Bit 7 - Inverse transmit signal output in IR mode"]
    #[inline(always)]
    pub fn ir_inverse(&mut self) -> IR_INVERSE_W {
        IR_INVERSE_W { w: self }
    }
    #[doc = "Bit 6 - Enable IR transmit mode"]
    #[inline(always)]
    pub fn ir_transmit(&mut self) -> IR_TRANSMIT_W {
        IR_TRANSMIT_W { w: self }
    }
    #[doc = "Bit 5 - Select transmit parity mode if enabled"]
    #[inline(always)]
    pub fn parity_mode(&mut self) -> PARITY_MODE_W {
        PARITY_MODE_W { w: self }
    }
    #[doc = "Bit 4 - Enable transmit parity check"]
    #[inline(always)]
    pub fn parity_enable(&mut self) -> PARITY_ENABLE_W {
        PARITY_ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Enable freerun mode"]
    #[inline(always)]
    pub fn freerun(&mut self) -> FREERUN_W {
        FREERUN_W { w: self }
    }
    #[doc = "Bit 1 - Enable Clear-to-Send flow control signal"]
    #[inline(always)]
    pub fn cts(&mut self) -> CTS_W {
        CTS_W { w: self }
    }
    #[doc = "Bit 0 - Enable transmit function"]
    #[inline(always)]
    pub fn function(&mut self) -> FUNCTION_W {
        FUNCTION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transmit_config](index.html) module"]
pub struct TRANSMIT_CONFIG_SPEC;
impl crate::RegisterSpec for TRANSMIT_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transmit_config::R](R) reader structure"]
impl crate::Readable for TRANSMIT_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [transmit_config::W](W) writer structure"]
impl crate::Writable for TRANSMIT_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets transmit_config to value 0x1700"]
impl crate::Resettable for TRANSMIT_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1700
    }
}
