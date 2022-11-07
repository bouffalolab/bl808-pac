#[doc = "Register `bit_period` reader"]
pub struct R(crate::R<BIT_PERIOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIT_PERIOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIT_PERIOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIT_PERIOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bit_period` writer"]
pub struct W(crate::W<BIT_PERIOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIT_PERIOD_SPEC>;
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
impl From<crate::W<BIT_PERIOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIT_PERIOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `transmit` reader - Period of each transmit bit\n\n Add 1 to this value and divide by clock to get transmit baudrate."]
pub type TRANSMIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `transmit` writer - Period of each transmit bit\n\n Add 1 to this value and divide by clock to get transmit baudrate."]
pub type TRANSMIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BIT_PERIOD_SPEC, u16, u16, 16, O>;
#[doc = "Field `receive` reader - Period of each receive bit\n\n Add 1 to this value and divide by clock to get receive baudrate."]
pub type RECEIVE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `receive` writer - Period of each receive bit\n\n Add 1 to this value and divide by clock to get receive baudrate."]
pub type RECEIVE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BIT_PERIOD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Period of each transmit bit\n\n Add 1 to this value and divide by clock to get transmit baudrate."]
    #[inline(always)]
    pub fn transmit(&self) -> TRANSMIT_R {
        TRANSMIT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Period of each receive bit\n\n Add 1 to this value and divide by clock to get receive baudrate."]
    #[inline(always)]
    pub fn receive(&self) -> RECEIVE_R {
        RECEIVE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Period of each transmit bit\n\n Add 1 to this value and divide by clock to get transmit baudrate."]
    #[inline(always)]
    #[must_use]
    pub fn transmit(&mut self) -> TRANSMIT_W<0> {
        TRANSMIT_W::new(self)
    }
    #[doc = "Bits 16:31 - Period of each receive bit\n\n Add 1 to this value and divide by clock to get receive baudrate."]
    #[inline(always)]
    #[must_use]
    pub fn receive(&mut self) -> RECEIVE_W<16> {
        RECEIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit period control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bit_period](index.html) module"]
pub struct BIT_PERIOD_SPEC;
impl crate::RegisterSpec for BIT_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bit_period::R](R) reader structure"]
impl crate::Readable for BIT_PERIOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bit_period::W](W) writer structure"]
impl crate::Writable for BIT_PERIOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bit_period to value 0x00ff_00ff"]
impl crate::Resettable for BIT_PERIOD_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_00ff;
}
