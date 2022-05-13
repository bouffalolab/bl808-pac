#[doc = "Register `auto_baudrate` reader"]
pub struct R(crate::R<AUTO_BAUDRATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTO_BAUDRATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTO_BAUDRATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTO_BAUDRATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `by_five_five` reader - Bit period of auto baudrate detection using codeword 0x55"]
pub struct BY_FIVE_FIVE_R(crate::FieldReader<u16>);
impl BY_FIVE_FIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BY_FIVE_FIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BY_FIVE_FIVE_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `by_start_bit` reader - Bit period of auto baudrate detection using start bit"]
pub struct BY_START_BIT_R(crate::FieldReader<u16>);
impl BY_START_BIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BY_START_BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BY_START_BIT_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:31 - Bit period of auto baudrate detection using codeword 0x55"]
    #[inline(always)]
    pub fn by_five_five(&self) -> BY_FIVE_FIVE_R {
        BY_FIVE_FIVE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Bit period of auto baudrate detection using start bit"]
    #[inline(always)]
    pub fn by_start_bit(&self) -> BY_START_BIT_R {
        BY_START_BIT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Auto baudrate detection register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auto_baudrate](index.html) module"]
pub struct AUTO_BAUDRATE_SPEC;
impl crate::RegisterSpec for AUTO_BAUDRATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auto_baudrate::R](R) reader structure"]
impl crate::Readable for AUTO_BAUDRATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets auto_baudrate to value 0"]
impl crate::Resettable for AUTO_BAUDRATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
