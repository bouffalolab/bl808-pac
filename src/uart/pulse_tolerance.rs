#[doc = "Register `pulse_tolerance` reader"]
pub struct R(crate::R<PULSE_TOLERANCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULSE_TOLERANCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULSE_TOLERANCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULSE_TOLERANCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pulse_tolerance` writer"]
pub struct W(crate::W<PULSE_TOLERANCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PULSE_TOLERANCE_SPEC>;
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
impl From<crate::W<PULSE_TOLERANCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PULSE_TOLERANCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `by_five_five` reader - Pulse width tolerance of auto baudrate detection using codeword 0x55"]
pub type BY_FIVE_FIVE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `by_five_five` writer - Pulse width tolerance of auto baudrate detection using codeword 0x55"]
pub type BY_FIVE_FIVE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PULSE_TOLERANCE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Pulse width tolerance of auto baudrate detection using codeword 0x55"]
    #[inline(always)]
    pub fn by_five_five(&self) -> BY_FIVE_FIVE_R {
        BY_FIVE_FIVE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pulse width tolerance of auto baudrate detection using codeword 0x55"]
    #[inline(always)]
    #[must_use]
    pub fn by_five_five(&mut self) -> BY_FIVE_FIVE_W<0> {
        BY_FIVE_FIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pulse width tolerance for auto baudrate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse_tolerance](index.html) module"]
pub struct PULSE_TOLERANCE_SPEC;
impl crate::RegisterSpec for PULSE_TOLERANCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pulse_tolerance::R](R) reader structure"]
impl crate::Readable for PULSE_TOLERANCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pulse_tolerance::W](W) writer structure"]
impl crate::Writable for PULSE_TOLERANCE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pulse_tolerance to value 0x03"]
impl crate::Resettable for PULSE_TOLERANCE_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
