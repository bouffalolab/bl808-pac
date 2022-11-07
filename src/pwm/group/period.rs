#[doc = "Register `period` reader"]
pub struct R(crate::R<PERIOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `period` writer"]
pub struct W(crate::W<PERIOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIOD_SPEC>;
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
impl From<crate::W<PERIOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `interrupt_cycles` reader - If internal counter reaches this cycle count, it interrupts"]
pub type INTERRUPT_CYCLES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `interrupt_cycles` writer - If internal counter reaches this cycle count, it interrupts"]
pub type INTERRUPT_CYCLES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERIOD_SPEC, u16, u16, 16, O>;
#[doc = "Field `repeat_cycles` reader - How many clock cycles a Pulse-Width Modulation cycle includes"]
pub type REPEAT_CYCLES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `repeat_cycles` writer - How many clock cycles a Pulse-Width Modulation cycle includes"]
pub type REPEAT_CYCLES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERIOD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - If internal counter reaches this cycle count, it interrupts"]
    #[inline(always)]
    pub fn interrupt_cycles(&self) -> INTERRUPT_CYCLES_R {
        INTERRUPT_CYCLES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - How many clock cycles a Pulse-Width Modulation cycle includes"]
    #[inline(always)]
    pub fn repeat_cycles(&self) -> REPEAT_CYCLES_R {
        REPEAT_CYCLES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If internal counter reaches this cycle count, it interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_cycles(&mut self) -> INTERRUPT_CYCLES_W<0> {
        INTERRUPT_CYCLES_W::new(self)
    }
    #[doc = "Bits 16:31 - How many clock cycles a Pulse-Width Modulation cycle includes"]
    #[inline(always)]
    #[must_use]
    pub fn repeat_cycles(&mut self) -> REPEAT_CYCLES_W<16> {
        REPEAT_CYCLES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pulse clock period register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [period](index.html) module"]
pub struct PERIOD_SPEC;
impl crate::RegisterSpec for PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [period::R](R) reader structure"]
impl crate::Readable for PERIOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [period::W](W) writer structure"]
impl crate::Writable for PERIOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets period to value 0"]
impl crate::Resettable for PERIOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
