#[doc = "Register `data_config` reader"]
pub struct R(crate::R<DATA_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `data_config` writer"]
pub struct W(crate::W<DATA_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_CONFIG_SPEC>;
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
impl From<crate::W<DATA_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bit_order` reader - Enable bit inverse in each data word"]
pub type BIT_ORDER_R = crate::BitReader<BIT_ORDER_A>;
#[doc = "Enable bit inverse in each data word\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT_ORDER_A {
    #[doc = "1: Each byte is sent out MSB-first"]
    INVERSE = 1,
    #[doc = "0: Each byte is sent out LSB-first"]
    NO_INVERSE = 0,
}
impl From<BIT_ORDER_A> for bool {
    #[inline(always)]
    fn from(variant: BIT_ORDER_A) -> Self {
        variant as u8 != 0
    }
}
impl BIT_ORDER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT_ORDER_A {
        match self.bits {
            true => BIT_ORDER_A::INVERSE,
            false => BIT_ORDER_A::NO_INVERSE,
        }
    }
    #[doc = "Checks if the value of the field is `INVERSE`"]
    #[inline(always)]
    pub fn is_inverse(&self) -> bool {
        *self == BIT_ORDER_A::INVERSE
    }
    #[doc = "Checks if the value of the field is `NO_INVERSE`"]
    #[inline(always)]
    pub fn is_no_inverse(&self) -> bool {
        *self == BIT_ORDER_A::NO_INVERSE
    }
}
#[doc = "Field `bit_order` writer - Enable bit inverse in each data word"]
pub type BIT_ORDER_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_CONFIG_SPEC, BIT_ORDER_A, O>;
impl<'a, const O: u8> BIT_ORDER_W<'a, O> {
    #[doc = "Each byte is sent out MSB-first"]
    #[inline(always)]
    pub fn inverse(self) -> &'a mut W {
        self.variant(BIT_ORDER_A::INVERSE)
    }
    #[doc = "Each byte is sent out LSB-first"]
    #[inline(always)]
    pub fn no_inverse(self) -> &'a mut W {
        self.variant(BIT_ORDER_A::NO_INVERSE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable bit inverse in each data word"]
    #[inline(always)]
    pub fn bit_order(&self) -> BIT_ORDER_R {
        BIT_ORDER_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bit inverse in each data word"]
    #[inline(always)]
    #[must_use]
    pub fn bit_order(&mut self) -> BIT_ORDER_W<0> {
        BIT_ORDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_config](index.html) module"]
pub struct DATA_CONFIG_SPEC;
impl crate::RegisterSpec for DATA_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_config::R](R) reader structure"]
impl crate::Readable for DATA_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_config::W](W) writer structure"]
impl crate::Writable for DATA_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets data_config to value 0"]
impl crate::Resettable for DATA_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
