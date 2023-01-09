#[doc = "Register `SDATAH4` reader"]
pub struct R(crate::R<SDATAH4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDATAH4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDATAH4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDATAH4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDATAH4` writer"]
pub struct W(crate::W<SDATAH4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDATAH4_SPEC>;
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
impl From<crate::W<SDATAH4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDATAH4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDATA` reader - Segments Data"]
pub type SDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SDATA` writer - Segments Data"]
pub type SDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDATAH4_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Segments Data"]
    #[inline(always)]
    pub fn sdata(&self) -> SDATA_R {
        SDATA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Segments Data"]
    #[inline(always)]
    #[must_use]
    pub fn sdata(&mut self) -> SDATA_W<0> {
        SDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Segments Data High for COM4 Line\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdatah4](index.html) module"]
pub struct SDATAH4_SPEC;
impl crate::RegisterSpec for SDATAH4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdatah4::R](R) reader structure"]
impl crate::Readable for SDATAH4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdatah4::W](W) writer structure"]
impl crate::Writable for SDATAH4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDATAH4 to value 0"]
impl crate::Resettable for SDATAH4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
