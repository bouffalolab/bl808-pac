#[doc = "Register `uart_config` reader"]
pub struct R(crate::R<UART_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_config` writer"]
pub struct W(crate::W<UART_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_CONFIG_SPEC>;
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
impl From<crate::W<UART_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clock_divide` reader - Peripheral clock divide factor"]
pub type CLOCK_DIVIDE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clock_divide` writer - Peripheral clock divide factor"]
pub type CLOCK_DIVIDE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `clock_enable` reader - Peripheral level clock gate enable"]
pub type CLOCK_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `clock_enable` writer - Peripheral level clock gate enable"]
pub type CLOCK_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONFIG_SPEC, bool, O>;
#[doc = "Field `hibernate_clock_source` reader - Reads clock source from hibernate registers"]
pub type HIBERNATE_CLOCK_SOURCE_R = crate::BitReader<bool>;
#[doc = "Field `hibernate_clock_source_2` reader - Reads clock source from hibernate registers"]
pub type HIBERNATE_CLOCK_SOURCE_2_R = crate::BitReader<bool>;
#[doc = "Field `uart2_mode` reader - Select interface mode for UART2 peripheral"]
pub type UART2_MODE_R = crate::FieldReader<u8, UART2MODE_A>;
#[doc = "Select interface mode for UART2 peripheral\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UART2MODE_A {
    #[doc = "0: Universal Asynchronous Receiver/Transmitter interface"]
    UART = 0,
    #[doc = "1: ISO 11898 communication protocol interface"]
    ISO11898 = 1,
}
impl From<UART2MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UART2MODE_A) -> Self {
        variant as _
    }
}
impl UART2_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UART2MODE_A> {
        match self.bits {
            0 => Some(UART2MODE_A::UART),
            1 => Some(UART2MODE_A::ISO11898),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UART`"]
    #[inline(always)]
    pub fn is_uart(&self) -> bool {
        *self == UART2MODE_A::UART
    }
    #[doc = "Checks if the value of the field is `ISO11898`"]
    #[inline(always)]
    pub fn is_iso11898(&self) -> bool {
        *self == UART2MODE_A::ISO11898
    }
}
#[doc = "Field `uart2_mode` writer - Select interface mode for UART2 peripheral"]
pub type UART2_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_CONFIG_SPEC, u8, UART2MODE_A, 7, O>;
impl<'a, const O: u8> UART2_MODE_W<'a, O> {
    #[doc = "Universal Asynchronous Receiver/Transmitter interface"]
    #[inline(always)]
    pub fn uart(self) -> &'a mut W {
        self.variant(UART2MODE_A::UART)
    }
    #[doc = "ISO 11898 communication protocol interface"]
    #[inline(always)]
    pub fn iso11898(self) -> &'a mut W {
        self.variant(UART2MODE_A::ISO11898)
    }
}
impl R {
    #[doc = "Bits 0:2 - Peripheral clock divide factor"]
    #[inline(always)]
    pub fn clock_divide(&self) -> CLOCK_DIVIDE_R {
        CLOCK_DIVIDE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Peripheral level clock gate enable"]
    #[inline(always)]
    pub fn clock_enable(&self) -> CLOCK_ENABLE_R {
        CLOCK_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Reads clock source from hibernate registers"]
    #[inline(always)]
    pub fn hibernate_clock_source(&self) -> HIBERNATE_CLOCK_SOURCE_R {
        HIBERNATE_CLOCK_SOURCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 22 - Reads clock source from hibernate registers"]
    #[inline(always)]
    pub fn hibernate_clock_source_2(&self) -> HIBERNATE_CLOCK_SOURCE_2_R {
        HIBERNATE_CLOCK_SOURCE_2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 25:31 - Select interface mode for UART2 peripheral"]
    #[inline(always)]
    pub fn uart2_mode(&self) -> UART2_MODE_R {
        UART2_MODE_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Peripheral clock divide factor"]
    #[inline(always)]
    #[must_use]
    pub fn clock_divide(&mut self) -> CLOCK_DIVIDE_W<0> {
        CLOCK_DIVIDE_W::new(self)
    }
    #[doc = "Bit 4 - Peripheral level clock gate enable"]
    #[inline(always)]
    #[must_use]
    pub fn clock_enable(&mut self) -> CLOCK_ENABLE_W<4> {
        CLOCK_ENABLE_W::new(self)
    }
    #[doc = "Bits 25:31 - Select interface mode for UART2 peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn uart2_mode(&mut self) -> UART2_MODE_W<25> {
        UART2_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_config](index.html) module"]
pub struct UART_CONFIG_SPEC;
impl crate::RegisterSpec for UART_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_config::R](R) reader structure"]
impl crate::Readable for UART_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_config::W](W) writer structure"]
impl crate::Writable for UART_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uart_config to value 0"]
impl crate::Resettable for UART_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
