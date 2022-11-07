#[doc = "Register `uart_signal_0` reader"]
pub struct R(crate::R<UART_SIGNAL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_SIGNAL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_SIGNAL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_SIGNAL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_signal_0` writer"]
pub struct W(crate::W<UART_SIGNAL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_SIGNAL_0_SPEC>;
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
impl From<crate::W<UART_SIGNAL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_SIGNAL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `function_0[0-7]` reader - Select peripheral function for UART signal %s"]
pub type FUNCTION_0_R = crate::FieldReader<u8, FUNCTION_A>;
#[doc = "Select peripheral function for UART signal %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FUNCTION_A {
    #[doc = "0: UART0 Request-to-Send flow control"]
    UART0_RTS = 0,
    #[doc = "1: UART0 Clear-to-Send flow control"]
    UART0_CTS = 1,
    #[doc = "2: UART0 transmit data"]
    UART0_TXD = 2,
    #[doc = "3: UART0 receive data"]
    UART0_RXD = 3,
    #[doc = "4: UART1 Request-to-Send flow control"]
    UART1_RTS = 4,
    #[doc = "5: UART1 Clear-to-Send flow control"]
    UART1_CTS = 5,
    #[doc = "6: UART1 transmit data"]
    UART1_TXD = 6,
    #[doc = "7: UART1 receive data"]
    UART1_RXD = 7,
    #[doc = "8: UART2 Request-to-Send flow control"]
    UART2_RTS = 8,
    #[doc = "9: UART2 Clear-to-Send flow control"]
    UART2_CTS = 9,
    #[doc = "10: UART2 transmit data"]
    UART2_TXD = 10,
    #[doc = "11: UART2 receive data"]
    UART2_RXD = 11,
}
impl From<FUNCTION_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNCTION_A) -> Self {
        variant as _
    }
}
impl FUNCTION_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNCTION_A> {
        match self.bits {
            0 => Some(FUNCTION_A::UART0_RTS),
            1 => Some(FUNCTION_A::UART0_CTS),
            2 => Some(FUNCTION_A::UART0_TXD),
            3 => Some(FUNCTION_A::UART0_RXD),
            4 => Some(FUNCTION_A::UART1_RTS),
            5 => Some(FUNCTION_A::UART1_CTS),
            6 => Some(FUNCTION_A::UART1_TXD),
            7 => Some(FUNCTION_A::UART1_RXD),
            8 => Some(FUNCTION_A::UART2_RTS),
            9 => Some(FUNCTION_A::UART2_CTS),
            10 => Some(FUNCTION_A::UART2_TXD),
            11 => Some(FUNCTION_A::UART2_RXD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UART0_RTS`"]
    #[inline(always)]
    pub fn is_uart0_rts(&self) -> bool {
        *self == FUNCTION_A::UART0_RTS
    }
    #[doc = "Checks if the value of the field is `UART0_CTS`"]
    #[inline(always)]
    pub fn is_uart0_cts(&self) -> bool {
        *self == FUNCTION_A::UART0_CTS
    }
    #[doc = "Checks if the value of the field is `UART0_TXD`"]
    #[inline(always)]
    pub fn is_uart0_txd(&self) -> bool {
        *self == FUNCTION_A::UART0_TXD
    }
    #[doc = "Checks if the value of the field is `UART0_RXD`"]
    #[inline(always)]
    pub fn is_uart0_rxd(&self) -> bool {
        *self == FUNCTION_A::UART0_RXD
    }
    #[doc = "Checks if the value of the field is `UART1_RTS`"]
    #[inline(always)]
    pub fn is_uart1_rts(&self) -> bool {
        *self == FUNCTION_A::UART1_RTS
    }
    #[doc = "Checks if the value of the field is `UART1_CTS`"]
    #[inline(always)]
    pub fn is_uart1_cts(&self) -> bool {
        *self == FUNCTION_A::UART1_CTS
    }
    #[doc = "Checks if the value of the field is `UART1_TXD`"]
    #[inline(always)]
    pub fn is_uart1_txd(&self) -> bool {
        *self == FUNCTION_A::UART1_TXD
    }
    #[doc = "Checks if the value of the field is `UART1_RXD`"]
    #[inline(always)]
    pub fn is_uart1_rxd(&self) -> bool {
        *self == FUNCTION_A::UART1_RXD
    }
    #[doc = "Checks if the value of the field is `UART2_RTS`"]
    #[inline(always)]
    pub fn is_uart2_rts(&self) -> bool {
        *self == FUNCTION_A::UART2_RTS
    }
    #[doc = "Checks if the value of the field is `UART2_CTS`"]
    #[inline(always)]
    pub fn is_uart2_cts(&self) -> bool {
        *self == FUNCTION_A::UART2_CTS
    }
    #[doc = "Checks if the value of the field is `UART2_TXD`"]
    #[inline(always)]
    pub fn is_uart2_txd(&self) -> bool {
        *self == FUNCTION_A::UART2_TXD
    }
    #[doc = "Checks if the value of the field is `UART2_RXD`"]
    #[inline(always)]
    pub fn is_uart2_rxd(&self) -> bool {
        *self == FUNCTION_A::UART2_RXD
    }
}
#[doc = "Field `function_0[0-7]` writer - Select peripheral function for UART signal %s"]
pub type FUNCTION_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_SIGNAL_0_SPEC, u8, FUNCTION_A, 8, O>;
impl<'a, const O: u8> FUNCTION_0_W<'a, O> {
    #[doc = "UART0 Request-to-Send flow control"]
    #[inline(always)]
    pub fn uart0_rts(self) -> &'a mut W {
        self.variant(FUNCTION_A::UART0_RTS)
    }
    #[doc = "UART0 Clear-to-Send flow control"]
    #[inline(always)]
    pub fn uart0_cts(self) -> &'a mut W {
        self.variant(FUNCTION_A::UART0_CTS)
    }
    #[doc = "UART0 transmit data"]
    #[inline(always)]
    pub fn uart0_txd(self) -> &'a mut W {
        self.variant(FUNCTION_A::UART0_TXD)
    }
    #[doc = "UART0 receive data"]
    #[inline(always)]
    pub fn uart0_rxd(self) -> &'a mut W {
        self.variant(FUNCTION_A::UART0_RXD)
    }
    #[doc = "UART1 Request-to-Send flow control"]
    #[inline(always)]
    pub fn uart1_rts(self) -> &'a mut W {
        self.variant(FUNCTION_A::UART1_RTS)
    }
    #[doc = "UART1 Clear-to-Send flow control"]
    #[inline(always)]
    pub fn uart1_cts(self) -> &'a mut W {
        self.variant(FUNCTION_A::UART1_CTS)
    }
    #[doc = "UART1 transmit data"]
    #[inline(always)]
    pub fn uart1_txd(self) -> &'a mut W {
        self.variant(FUNCTION_A::UART1_TXD)
    }
    #[doc = "UART1 receive data"]
    #[inline(always)]
    pub fn uart1_rxd(self) -> &'a mut W {
        self.variant(FUNCTION_A::UART1_RXD)
    }
    #[doc = "UART2 Request-to-Send flow control"]
    #[inline(always)]
    pub fn uart2_rts(self) -> &'a mut W {
        self.variant(FUNCTION_A::UART2_RTS)
    }
    #[doc = "UART2 Clear-to-Send flow control"]
    #[inline(always)]
    pub fn uart2_cts(self) -> &'a mut W {
        self.variant(FUNCTION_A::UART2_CTS)
    }
    #[doc = "UART2 transmit data"]
    #[inline(always)]
    pub fn uart2_txd(self) -> &'a mut W {
        self.variant(FUNCTION_A::UART2_TXD)
    }
    #[doc = "UART2 receive data"]
    #[inline(always)]
    pub fn uart2_rxd(self) -> &'a mut W {
        self.variant(FUNCTION_A::UART2_RXD)
    }
}
impl R {
    #[doc = "Select peripheral function for UART signal [0-7]"]
    #[inline(always)]
    pub unsafe fn function_0(&self, n: u8) -> FUNCTION_0_R {
        FUNCTION_0_R::new(((self.bits >> (n * 4)) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Select peripheral function for UART signal 0"]
    #[inline(always)]
    pub fn function_00(&self) -> FUNCTION_0_R {
        FUNCTION_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 4:11 - Select peripheral function for UART signal 1"]
    #[inline(always)]
    pub fn function_01(&self) -> FUNCTION_0_R {
        FUNCTION_0_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Select peripheral function for UART signal 2"]
    #[inline(always)]
    pub fn function_02(&self) -> FUNCTION_0_R {
        FUNCTION_0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 12:19 - Select peripheral function for UART signal 3"]
    #[inline(always)]
    pub fn function_03(&self) -> FUNCTION_0_R {
        FUNCTION_0_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Select peripheral function for UART signal 4"]
    #[inline(always)]
    pub fn function_04(&self) -> FUNCTION_0_R {
        FUNCTION_0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 20:27 - Select peripheral function for UART signal 5"]
    #[inline(always)]
    pub fn function_05(&self) -> FUNCTION_0_R {
        FUNCTION_0_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Select peripheral function for UART signal 6"]
    #[inline(always)]
    pub fn function_06(&self) -> FUNCTION_0_R {
        FUNCTION_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 28:35 - Select peripheral function for UART signal 7"]
    #[inline(always)]
    pub fn function_07(&self) -> FUNCTION_0_R {
        FUNCTION_0_R::new(((self.bits >> 28) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Select peripheral function for UART signal [0-7]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn function_0<const O: u8>(&mut self) -> FUNCTION_0_W<O> {
        FUNCTION_0_W::new(self)
    }
    #[doc = "Bits 0:7 - Select peripheral function for UART signal 0"]
    #[inline(always)]
    #[must_use]
    pub fn function_00(&mut self) -> FUNCTION_0_W<0> {
        FUNCTION_0_W::new(self)
    }
    #[doc = "Bits 4:11 - Select peripheral function for UART signal 1"]
    #[inline(always)]
    #[must_use]
    pub fn function_01(&mut self) -> FUNCTION_0_W<4> {
        FUNCTION_0_W::new(self)
    }
    #[doc = "Bits 8:15 - Select peripheral function for UART signal 2"]
    #[inline(always)]
    #[must_use]
    pub fn function_02(&mut self) -> FUNCTION_0_W<8> {
        FUNCTION_0_W::new(self)
    }
    #[doc = "Bits 12:19 - Select peripheral function for UART signal 3"]
    #[inline(always)]
    #[must_use]
    pub fn function_03(&mut self) -> FUNCTION_0_W<12> {
        FUNCTION_0_W::new(self)
    }
    #[doc = "Bits 16:23 - Select peripheral function for UART signal 4"]
    #[inline(always)]
    #[must_use]
    pub fn function_04(&mut self) -> FUNCTION_0_W<16> {
        FUNCTION_0_W::new(self)
    }
    #[doc = "Bits 20:27 - Select peripheral function for UART signal 5"]
    #[inline(always)]
    #[must_use]
    pub fn function_05(&mut self) -> FUNCTION_0_W<20> {
        FUNCTION_0_W::new(self)
    }
    #[doc = "Bits 24:31 - Select peripheral function for UART signal 6"]
    #[inline(always)]
    #[must_use]
    pub fn function_06(&mut self) -> FUNCTION_0_W<24> {
        FUNCTION_0_W::new(self)
    }
    #[doc = "Bits 28:35 - Select peripheral function for UART signal 7"]
    #[inline(always)]
    #[must_use]
    pub fn function_07(&mut self) -> FUNCTION_0_W<28> {
        FUNCTION_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter signal configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_signal_0](index.html) module"]
pub struct UART_SIGNAL_0_SPEC;
impl crate::RegisterSpec for UART_SIGNAL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_signal_0::R](R) reader structure"]
impl crate::Readable for UART_SIGNAL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_signal_0::W](W) writer structure"]
impl crate::Writable for UART_SIGNAL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uart_signal_0 to value 0"]
impl crate::Resettable for UART_SIGNAL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
