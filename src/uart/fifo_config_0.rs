#[doc = "Register `fifo_config_0` reader"]
pub struct R(crate::R<FIFO_CONFIG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_CONFIG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_CONFIG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_CONFIG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fifo_config_0` writer"]
pub struct W(crate::W<FIFO_CONFIG_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_CONFIG_0_SPEC>;
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
impl From<crate::W<FIFO_CONFIG_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_CONFIG_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive FIFO underflow flag\n\n Can be cleared using `receive_clear`.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE_UNDERFLOW_A {
    #[doc = "0: No FIFO buffer underflow"]
    NOT_UNDERFLOW = 0,
    #[doc = "1: Has FIFO buffer underflow"]
    UNDERFLOW = 1,
}
impl From<RECEIVE_UNDERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE_UNDERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `receive_underflow` reader - Receive FIFO underflow flag\n\n Can be cleared using `receive_clear`."]
pub struct RECEIVE_UNDERFLOW_R(crate::FieldReader<bool>);
impl RECEIVE_UNDERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE_UNDERFLOW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE_UNDERFLOW_A {
        match self.bits {
            false => RECEIVE_UNDERFLOW_A::NOT_UNDERFLOW,
            true => RECEIVE_UNDERFLOW_A::UNDERFLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_UNDERFLOW`"]
    #[inline(always)]
    pub fn is_not_underflow(&self) -> bool {
        **self == RECEIVE_UNDERFLOW_A::NOT_UNDERFLOW
    }
    #[doc = "Checks if the value of the field is `UNDERFLOW`"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        **self == RECEIVE_UNDERFLOW_A::UNDERFLOW
    }
}
impl core::ops::Deref for RECEIVE_UNDERFLOW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive FIFO overflow flag\n\n Can be cleared using `receive_clear`.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE_OVERFLOW_A {
    #[doc = "0: No FIFO buffer overflow"]
    NOT_OVERFLOW = 0,
    #[doc = "1: Has FIFO buffer overflow"]
    OVERFLOW = 1,
}
impl From<RECEIVE_OVERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE_OVERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `receive_overflow` reader - Receive FIFO overflow flag\n\n Can be cleared using `receive_clear`."]
pub struct RECEIVE_OVERFLOW_R(crate::FieldReader<bool>);
impl RECEIVE_OVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE_OVERFLOW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE_OVERFLOW_A {
        match self.bits {
            false => RECEIVE_OVERFLOW_A::NOT_OVERFLOW,
            true => RECEIVE_OVERFLOW_A::OVERFLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OVERFLOW`"]
    #[inline(always)]
    pub fn is_not_overflow(&self) -> bool {
        **self == RECEIVE_OVERFLOW_A::NOT_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        **self == RECEIVE_OVERFLOW_A::OVERFLOW
    }
}
impl core::ops::Deref for RECEIVE_OVERFLOW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit FIFO overflow flag\n\n Can be cleared using `transmit_clear`."]
pub use RECEIVE_OVERFLOW_A as TRANSMIT_OVERFLOW_A;
#[doc = "Field `transmit_overflow` reader - Transmit FIFO overflow flag\n\n Can be cleared using `transmit_clear`."]
pub use RECEIVE_OVERFLOW_R as TRANSMIT_OVERFLOW_R;
#[doc = "Transmit FIFO underflow flag\n\n Can be cleared using `transmit_clear`."]
pub use RECEIVE_UNDERFLOW_A as TRANSMIT_UNDERFLOW_A;
#[doc = "Field `transmit_underflow` reader - Transmit FIFO underflow flag\n\n Can be cleared using `transmit_clear`."]
pub use RECEIVE_UNDERFLOW_R as TRANSMIT_UNDERFLOW_R;
#[doc = "Clears receive FIFO overflow and underflow flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE_CLEAR_AW {
    #[doc = "1: Write 1 to clear fifo flags"]
    CLEAR = 1,
}
impl From<RECEIVE_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `receive_clear` writer - Clears receive FIFO overflow and underflow flags"]
pub struct RECEIVE_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE_CLEAR_AW) -> &'a mut W {
        unsafe { self.bit(variant.into()) }
    }
    #[doc = "Write 1 to clear fifo flags"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RECEIVE_CLEAR_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Clears transmit FIFO overflow and underflow flags"]
pub use RECEIVE_CLEAR_AW as TRANSMIT_CLEAR_AW;
#[doc = "Field `transmit_clear` writer - Clears transmit FIFO overflow and underflow flags"]
pub struct TRANSMIT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSMIT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRANSMIT_CLEAR_AW) -> &'a mut W {
        unsafe { self.bit(variant.into()) }
    }
    #[doc = "Write 1 to clear fifo flags"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TRANSMIT_CLEAR_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Enable signal of receive DMA interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE_DMA_A {
    #[doc = "1: Enable DMA interface"]
    ENABLE = 1,
    #[doc = "0: Disable DMA interface"]
    DISABLE = 0,
}
impl From<RECEIVE_DMA_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE_DMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `receive_dma` reader - Enable signal of receive DMA interface"]
pub struct RECEIVE_DMA_R(crate::FieldReader<bool>);
impl RECEIVE_DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE_DMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE_DMA_A {
        match self.bits {
            true => RECEIVE_DMA_A::ENABLE,
            false => RECEIVE_DMA_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RECEIVE_DMA_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RECEIVE_DMA_A::DISABLE
    }
}
impl core::ops::Deref for RECEIVE_DMA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `receive_dma` writer - Enable signal of receive DMA interface"]
pub struct RECEIVE_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE_DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE_DMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable DMA interface"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RECEIVE_DMA_A::ENABLE)
    }
    #[doc = "Disable DMA interface"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RECEIVE_DMA_A::DISABLE)
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
#[doc = "Enable signal of transmit DMA interface"]
pub use RECEIVE_DMA_A as TRANSMIT_DMA_A;
#[doc = "Field `transmit_dma` reader - Enable signal of transmit DMA interface"]
pub use RECEIVE_DMA_R as TRANSMIT_DMA_R;
#[doc = "Field `transmit_dma` writer - Enable signal of transmit DMA interface"]
pub struct TRANSMIT_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSMIT_DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRANSMIT_DMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable DMA interface"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TRANSMIT_DMA_A::ENABLE)
    }
    #[doc = "Disable DMA interface"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TRANSMIT_DMA_A::DISABLE)
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
    #[doc = "Bit 7 - Receive FIFO underflow flag\n\n Can be cleared using `receive_clear`."]
    #[inline(always)]
    pub fn receive_underflow(&self) -> RECEIVE_UNDERFLOW_R {
        RECEIVE_UNDERFLOW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO overflow flag\n\n Can be cleared using `receive_clear`."]
    #[inline(always)]
    pub fn receive_overflow(&self) -> RECEIVE_OVERFLOW_R {
        RECEIVE_OVERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO underflow flag\n\n Can be cleared using `transmit_clear`."]
    #[inline(always)]
    pub fn transmit_underflow(&self) -> TRANSMIT_UNDERFLOW_R {
        TRANSMIT_UNDERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO overflow flag\n\n Can be cleared using `transmit_clear`."]
    #[inline(always)]
    pub fn transmit_overflow(&self) -> TRANSMIT_OVERFLOW_R {
        TRANSMIT_OVERFLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 1 - Enable signal of receive DMA interface"]
    #[inline(always)]
    pub fn receive_dma(&self) -> RECEIVE_DMA_R {
        RECEIVE_DMA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Enable signal of transmit DMA interface"]
    #[inline(always)]
    pub fn transmit_dma(&self) -> TRANSMIT_DMA_R {
        TRANSMIT_DMA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Clears receive FIFO overflow and underflow flags"]
    #[inline(always)]
    pub fn receive_clear(&mut self) -> RECEIVE_CLEAR_W {
        RECEIVE_CLEAR_W { w: self }
    }
    #[doc = "Bit 2 - Clears transmit FIFO overflow and underflow flags"]
    #[inline(always)]
    pub fn transmit_clear(&mut self) -> TRANSMIT_CLEAR_W {
        TRANSMIT_CLEAR_W { w: self }
    }
    #[doc = "Bit 1 - Enable signal of receive DMA interface"]
    #[inline(always)]
    pub fn receive_dma(&mut self) -> RECEIVE_DMA_W {
        RECEIVE_DMA_W { w: self }
    }
    #[doc = "Bit 0 - Enable signal of transmit DMA interface"]
    #[inline(always)]
    pub fn transmit_dma(&mut self) -> TRANSMIT_DMA_W {
        TRANSMIT_DMA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_config_0](index.html) module"]
pub struct FIFO_CONFIG_0_SPEC;
impl crate::RegisterSpec for FIFO_CONFIG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_config_0::R](R) reader structure"]
impl crate::Readable for FIFO_CONFIG_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_config_0::W](W) writer structure"]
impl crate::Writable for FIFO_CONFIG_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets fifo_config_0 to value 0"]
impl crate::Resettable for FIFO_CONFIG_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
