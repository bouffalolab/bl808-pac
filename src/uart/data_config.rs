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
#[doc = "Enable bit inverse in each data word\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `bit_order` reader - Enable bit inverse in each data word"]
pub struct BIT_ORDER_R(crate::FieldReader<bool>);
impl BIT_ORDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIT_ORDER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == BIT_ORDER_A::INVERSE
    }
    #[doc = "Checks if the value of the field is `NO_INVERSE`"]
    #[inline(always)]
    pub fn is_no_inverse(&self) -> bool {
        **self == BIT_ORDER_A::NO_INVERSE
    }
}
impl core::ops::Deref for BIT_ORDER_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bit_order` writer - Enable bit inverse in each data word"]
pub struct BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_ORDER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIT_ORDER_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = "Bit 0 - Enable bit inverse in each data word"]
    #[inline(always)]
    pub fn bit_order(&self) -> BIT_ORDER_R {
        BIT_ORDER_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bit inverse in each data word"]
    #[inline(always)]
    pub fn bit_order(&mut self) -> BIT_ORDER_W {
        BIT_ORDER_W { w: self }
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
}
#[doc = "`reset()` method sets data_config to value 0"]
impl crate::Resettable for DATA_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
