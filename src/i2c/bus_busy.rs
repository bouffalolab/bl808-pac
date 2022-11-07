#[doc = "Register `bus_busy` reader"]
pub struct R(crate::R<BUS_BUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_BUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_BUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_BUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bus_busy` writer"]
pub struct W(crate::W<BUS_BUSY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUS_BUSY_SPEC>;
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
impl From<crate::W<BUS_BUSY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUS_BUSY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `busy` reader - Indicator to I2C bus busy signal"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Indicator to I2C bus busy signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "1: Bus is busy"]
    BUSY = 1,
    #[doc = "0: Bus is not busy"]
    IDLE = 0,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            true => BUSY_A::BUSY,
            false => BUSY_A::IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
}
#[doc = "Force clear I2C bus busy state\n\n Not for normal use; only use when I2C bus hangs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_CLEAR_AW {
    #[doc = "1: Write 1 to force clear busy flag"]
    CLEAR = 1,
}
impl From<FORCE_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: FORCE_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `force_clear` writer - Force clear I2C bus busy state\n\n Not for normal use; only use when I2C bus hangs"]
pub type FORCE_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BUS_BUSY_SPEC, FORCE_CLEAR_AW, O>;
impl<'a, const O: u8> FORCE_CLEAR_W<'a, O> {
    #[doc = "Write 1 to force clear busy flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FORCE_CLEAR_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Indicator to I2C bus busy signal"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Force clear I2C bus busy state\n\n Not for normal use; only use when I2C bus hangs"]
    #[inline(always)]
    #[must_use]
    pub fn force_clear(&mut self) -> FORCE_CLEAR_W<1> {
        FORCE_CLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus busy state indicator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_busy](index.html) module"]
pub struct BUS_BUSY_SPEC;
impl crate::RegisterSpec for BUS_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_busy::R](R) reader structure"]
impl crate::Readable for BUS_BUSY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bus_busy::W](W) writer structure"]
impl crate::Writable for BUS_BUSY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bus_busy to value 0"]
impl crate::Resettable for BUS_BUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
