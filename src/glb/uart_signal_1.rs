#[doc = "Register `uart_signal_1` reader"]
pub struct R(crate::R<UART_SIGNAL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_SIGNAL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_SIGNAL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_SIGNAL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_signal_1` writer"]
pub struct W(crate::W<UART_SIGNAL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_SIGNAL_1_SPEC>;
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
impl From<crate::W<UART_SIGNAL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_SIGNAL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `function_1[0-3]` reader - Select peripheral function for UART signal %s (offset by 8)"]
pub use super::uart_signal_0::FUNCTION_0_R as FUNCTION_1_R;
#[doc = "Select peripheral function for UART signal %s (offset by 8)"]
pub use super::uart_signal_0::FUNCTION_A;
#[doc = "Field `function_1[0-3]` writer - Select peripheral function for UART signal %s (offset by 8)"]
pub type FUNCTION_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_SIGNAL_1_SPEC, u8, FUNCTION_A, 8, O>;
impl R {
    #[doc = "Select peripheral function for UART signal [0-3]
(offset by 8)"]
    #[inline(always)]
    pub unsafe fn function_1(&self, n: u8) -> FUNCTION_1_R {
        FUNCTION_1_R::new(((self.bits >> (n * 4)) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Select peripheral function for UART signal 0 (offset by 8)"]
    #[inline(always)]
    pub fn function_10(&self) -> FUNCTION_1_R {
        FUNCTION_1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 4:11 - Select peripheral function for UART signal 1 (offset by 8)"]
    #[inline(always)]
    pub fn function_11(&self) -> FUNCTION_1_R {
        FUNCTION_1_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Select peripheral function for UART signal 2 (offset by 8)"]
    #[inline(always)]
    pub fn function_12(&self) -> FUNCTION_1_R {
        FUNCTION_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 12:19 - Select peripheral function for UART signal 3 (offset by 8)"]
    #[inline(always)]
    pub fn function_13(&self) -> FUNCTION_1_R {
        FUNCTION_1_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Select peripheral function for UART signal [0-3]
(offset by 8)"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn function_1<const O: u8>(&mut self) -> FUNCTION_1_W<O> {
        FUNCTION_1_W::new(self)
    }
    #[doc = "Bits 0:7 - Select peripheral function for UART signal 0 (offset by 8)"]
    #[inline(always)]
    #[must_use]
    pub fn function_10(&mut self) -> FUNCTION_1_W<0> {
        FUNCTION_1_W::new(self)
    }
    #[doc = "Bits 4:11 - Select peripheral function for UART signal 1 (offset by 8)"]
    #[inline(always)]
    #[must_use]
    pub fn function_11(&mut self) -> FUNCTION_1_W<4> {
        FUNCTION_1_W::new(self)
    }
    #[doc = "Bits 8:15 - Select peripheral function for UART signal 2 (offset by 8)"]
    #[inline(always)]
    #[must_use]
    pub fn function_12(&mut self) -> FUNCTION_1_W<8> {
        FUNCTION_1_W::new(self)
    }
    #[doc = "Bits 12:19 - Select peripheral function for UART signal 3 (offset by 8)"]
    #[inline(always)]
    #[must_use]
    pub fn function_13(&mut self) -> FUNCTION_1_W<12> {
        FUNCTION_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter signal configuration 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_signal_1](index.html) module"]
pub struct UART_SIGNAL_1_SPEC;
impl crate::RegisterSpec for UART_SIGNAL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_signal_1::R](R) reader structure"]
impl crate::Readable for UART_SIGNAL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_signal_1::W](W) writer structure"]
impl crate::Writable for UART_SIGNAL_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uart_signal_1 to value 0"]
impl crate::Resettable for UART_SIGNAL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
