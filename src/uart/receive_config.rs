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
#[doc = "Field `transfer_length` reader - Length of words per UART receive transfer"]
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
#[doc = "Field `transfer_length` writer - Length of words per UART receive transfer"]
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
#[doc = "Field `deglitch_cycle` reader - De-glitch function cycle count"]
pub struct DEGLITCH_CYCLE_R(crate::FieldReader<u8>);
impl DEGLITCH_CYCLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEGLITCH_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEGLITCH_CYCLE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `deglitch_cycle` writer - De-glitch function cycle count"]
pub struct DEGLITCH_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEGLITCH_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `deglitch_enable` reader - Enable receive de-glitch function"]
pub struct DEGLITCH_ENABLE_R(crate::FieldReader<bool>);
impl DEGLITCH_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEGLITCH_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEGLITCH_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `deglitch_enable` writer - Enable receive de-glitch function"]
pub struct DEGLITCH_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEGLITCH_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `word_length` reader - Bit count for each receive data word"]
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
#[doc = "Field `word_length` writer - Bit count for each receive data word"]
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
#[doc = "Field `ir_inverse` reader - Inverse receive signal output in IR mode"]
pub struct IR_INVERSE_R(crate::FieldReader<bool>);
impl IR_INVERSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IR_INVERSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IR_INVERSE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ir_inverse` writer - Inverse receive signal output in IR mode"]
pub struct IR_INVERSE_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_INVERSE_W<'a> {
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
#[doc = "Field `ir_receive` reader - Enable IR receive mode"]
pub struct IR_RECEIVE_R(crate::FieldReader<bool>);
impl IR_RECEIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IR_RECEIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IR_RECEIVE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ir_receive` writer - Enable IR receive mode"]
pub struct IR_RECEIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_RECEIVE_W<'a> {
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
#[doc = "Field `parity_mode` reader - Select receive parity mode if enabled"]
pub struct PARITY_MODE_R(crate::FieldReader<bool>);
impl PARITY_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITY_MODE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `parity_mode` writer - Select receive parity mode if enabled"]
pub struct PARITY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_MODE_W<'a> {
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
#[doc = "Field `parity_enable` reader - Enable receive parity check"]
pub struct PARITY_ENABLE_R(crate::FieldReader<bool>);
impl PARITY_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITY_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `parity_enable` writer - Enable receive parity check"]
pub struct PARITY_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_ENABLE_W<'a> {
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
#[doc = "Field `auto_baudrate` reader - Enable receive auto baudrate detection"]
pub struct AUTO_BAUDRATE_R(crate::FieldReader<bool>);
impl AUTO_BAUDRATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTO_BAUDRATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTO_BAUDRATE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `auto_baudrate` writer - Enable receive auto baudrate detection"]
pub struct AUTO_BAUDRATE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_BAUDRATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `rts_value` reader - Value to override Request-to-Send signal if override is enabled"]
pub struct RTS_VALUE_R(crate::FieldReader<bool>);
impl RTS_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTS_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTS_VALUE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rts_value` writer - Value to override Request-to-Send signal if override is enabled"]
pub struct RTS_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_VALUE_W<'a> {
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
#[doc = "Field `rts_override` reader - Enable manual override of Request-to-Send flow control signal"]
pub struct RTS_OVERRIDE_R(crate::FieldReader<bool>);
impl RTS_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTS_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTS_OVERRIDE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rts_override` writer - Enable manual override of Request-to-Send flow control signal"]
pub struct RTS_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_OVERRIDE_W<'a> {
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
#[doc = "Field `function` reader - Enable receive function"]
pub struct FUNCTION_R(crate::FieldReader<bool>);
impl FUNCTION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FUNCTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUNCTION_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `function` writer - Enable receive function"]
pub struct FUNCTION_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNCTION_W<'a> {
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
    #[doc = "Bits 16:31 - Length of words per UART receive transfer"]
    #[inline(always)]
    pub fn transfer_length(&self) -> TRANSFER_LENGTH_R {
        TRANSFER_LENGTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 12:15 - De-glitch function cycle count"]
    #[inline(always)]
    pub fn deglitch_cycle(&self) -> DEGLITCH_CYCLE_R {
        DEGLITCH_CYCLE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - Enable receive de-glitch function"]
    #[inline(always)]
    pub fn deglitch_enable(&self) -> DEGLITCH_ENABLE_R {
        DEGLITCH_ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Bit count for each receive data word"]
    #[inline(always)]
    pub fn word_length(&self) -> WORD_LENGTH_R {
        WORD_LENGTH_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 7 - Inverse receive signal output in IR mode"]
    #[inline(always)]
    pub fn ir_inverse(&self) -> IR_INVERSE_R {
        IR_INVERSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable IR receive mode"]
    #[inline(always)]
    pub fn ir_receive(&self) -> IR_RECEIVE_R {
        IR_RECEIVE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Select receive parity mode if enabled"]
    #[inline(always)]
    pub fn parity_mode(&self) -> PARITY_MODE_R {
        PARITY_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable receive parity check"]
    #[inline(always)]
    pub fn parity_enable(&self) -> PARITY_ENABLE_R {
        PARITY_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable receive auto baudrate detection"]
    #[inline(always)]
    pub fn auto_baudrate(&self) -> AUTO_BAUDRATE_R {
        AUTO_BAUDRATE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Value to override Request-to-Send signal if override is enabled"]
    #[inline(always)]
    pub fn rts_value(&self) -> RTS_VALUE_R {
        RTS_VALUE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Enable manual override of Request-to-Send flow control signal"]
    #[inline(always)]
    pub fn rts_override(&self) -> RTS_OVERRIDE_R {
        RTS_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Enable receive function"]
    #[inline(always)]
    pub fn function(&self) -> FUNCTION_R {
        FUNCTION_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - Length of words per UART receive transfer"]
    #[inline(always)]
    pub fn transfer_length(&mut self) -> TRANSFER_LENGTH_W {
        TRANSFER_LENGTH_W { w: self }
    }
    #[doc = "Bits 12:15 - De-glitch function cycle count"]
    #[inline(always)]
    pub fn deglitch_cycle(&mut self) -> DEGLITCH_CYCLE_W {
        DEGLITCH_CYCLE_W { w: self }
    }
    #[doc = "Bit 11 - Enable receive de-glitch function"]
    #[inline(always)]
    pub fn deglitch_enable(&mut self) -> DEGLITCH_ENABLE_W {
        DEGLITCH_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:10 - Bit count for each receive data word"]
    #[inline(always)]
    pub fn word_length(&mut self) -> WORD_LENGTH_W {
        WORD_LENGTH_W { w: self }
    }
    #[doc = "Bit 7 - Inverse receive signal output in IR mode"]
    #[inline(always)]
    pub fn ir_inverse(&mut self) -> IR_INVERSE_W {
        IR_INVERSE_W { w: self }
    }
    #[doc = "Bit 6 - Enable IR receive mode"]
    #[inline(always)]
    pub fn ir_receive(&mut self) -> IR_RECEIVE_W {
        IR_RECEIVE_W { w: self }
    }
    #[doc = "Bit 5 - Select receive parity mode if enabled"]
    #[inline(always)]
    pub fn parity_mode(&mut self) -> PARITY_MODE_W {
        PARITY_MODE_W { w: self }
    }
    #[doc = "Bit 4 - Enable receive parity check"]
    #[inline(always)]
    pub fn parity_enable(&mut self) -> PARITY_ENABLE_W {
        PARITY_ENABLE_W { w: self }
    }
    #[doc = "Bit 3 - Enable receive auto baudrate detection"]
    #[inline(always)]
    pub fn auto_baudrate(&mut self) -> AUTO_BAUDRATE_W {
        AUTO_BAUDRATE_W { w: self }
    }
    #[doc = "Bit 2 - Value to override Request-to-Send signal if override is enabled"]
    #[inline(always)]
    pub fn rts_value(&mut self) -> RTS_VALUE_W {
        RTS_VALUE_W { w: self }
    }
    #[doc = "Bit 1 - Enable manual override of Request-to-Send flow control signal"]
    #[inline(always)]
    pub fn rts_override(&mut self) -> RTS_OVERRIDE_W {
        RTS_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 0 - Enable receive function"]
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
}
#[doc = "`reset()` method sets receive_config to value 0x0700"]
impl crate::Resettable for RECEIVE_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0700
    }
}
