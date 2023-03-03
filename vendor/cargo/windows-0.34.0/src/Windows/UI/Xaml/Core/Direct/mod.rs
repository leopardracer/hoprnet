#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlDirect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlDirect {
    type Vtable = IXamlDirect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ffa1295_add2_590f_a051_70989b866ade);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlDirect_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetXamlDirectObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typeindex: XamlTypeIndex, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetObjectProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetXamlDirectObjectProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetBooleanProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: bool) -> ::windows::core::HRESULT,
    pub SetDoubleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: f64) -> ::windows::core::HRESULT,
    pub SetInt32Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: i32) -> ::windows::core::HRESULT,
    pub SetStringProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDateTimeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDateTimeProperty: usize,
    #[cfg(feature = "Foundation")]
    pub SetPointProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPointProperty: usize,
    #[cfg(feature = "Foundation")]
    pub SetRectProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRectProperty: usize,
    #[cfg(feature = "Foundation")]
    pub SetSizeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSizeProperty: usize,
    #[cfg(feature = "Foundation")]
    pub SetTimeSpanProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTimeSpanProperty: usize,
    pub SetColorProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::Color) -> ::windows::core::HRESULT,
    pub SetCornerRadiusProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::CornerRadius) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDurationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::Duration) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDurationProperty: usize,
    pub SetGridLengthProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::GridLength) -> ::windows::core::HRESULT,
    pub SetThicknessProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::Thickness) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetMatrixProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::Media::Matrix) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetMatrixProperty: usize,
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub SetMatrix3DProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::Media::Media3D::Matrix3D) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Media3D"))]
    SetMatrix3DProperty: usize,
    pub SetEnumProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: u32) -> ::windows::core::HRESULT,
    pub GetObjectProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetXamlDirectObjectProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetBooleanProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetDoubleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut f64) -> ::windows::core::HRESULT,
    pub GetInt32Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GetStringProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDateTimeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDateTimeProperty: usize,
    #[cfg(feature = "Foundation")]
    pub GetPointProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPointProperty: usize,
    #[cfg(feature = "Foundation")]
    pub GetRectProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRectProperty: usize,
    #[cfg(feature = "Foundation")]
    pub GetSizeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSizeProperty: usize,
    #[cfg(feature = "Foundation")]
    pub GetTimeSpanProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTimeSpanProperty: usize,
    pub GetColorProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT,
    pub GetCornerRadiusProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::CornerRadius) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDurationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::Duration) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDurationProperty: usize,
    pub GetGridLengthProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::GridLength) -> ::windows::core::HRESULT,
    pub GetThicknessProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub GetMatrixProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::Media::Matrix) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    GetMatrixProperty: usize,
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub GetMatrix3DProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::Media::Media3D::Matrix3D) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Media3D"))]
    GetMatrix3DProperty: usize,
    pub GetEnumProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ClearProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex) -> ::windows::core::HRESULT,
    pub GetCollectionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub GetXamlDirectObjectFromCollectionAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, index: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddToCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InsertIntoCollectionAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, index: u32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveFromCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub RemoveFromCollectionAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, index: u32) -> ::windows::core::HRESULT,
    pub ClearCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, eventindex: XamlEventIndex, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddEventHandler_HandledEventsToo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, eventindex: XamlEventIndex, handler: *mut ::core::ffi::c_void, handledeventstoo: bool) -> ::windows::core::HRESULT,
    pub RemoveEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, eventindex: XamlEventIndex, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
#[repr(transparent)]
pub struct IXamlDirectObject(::windows::core::IUnknown);
impl IXamlDirectObject {}
impl ::core::convert::From<IXamlDirectObject> for ::windows::core::IUnknown {
    fn from(value: IXamlDirectObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlDirectObject> for ::windows::core::IUnknown {
    fn from(value: &IXamlDirectObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlDirectObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXamlDirectObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXamlDirectObject> for ::windows::core::IInspectable {
    fn from(value: IXamlDirectObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlDirectObject> for ::windows::core::IInspectable {
    fn from(value: &IXamlDirectObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXamlDirectObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXamlDirectObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlDirectObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlDirectObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlDirectObject {}
impl ::core::fmt::Debug for IXamlDirectObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlDirectObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlDirectObject {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{10614a82-cee4-4645-ba25-d071ce778355}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXamlDirectObject {
    type Vtable = IXamlDirectObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10614a82_cee4_4645_ba25_d071ce778355);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlDirectObject_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlDirectStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlDirectStatics {
    type Vtable = IXamlDirectStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x321887cc_14e4_5c6f_878d_fbb604ad7d17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlDirectStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
#[repr(transparent)]
pub struct XamlDirect(::windows::core::IUnknown);
impl XamlDirect {
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn GetObject<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetObject)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn GetXamlDirectObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, object: Param0) -> ::windows::core::Result<IXamlDirectObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetXamlDirectObject)(::core::mem::transmute_copy(this), object.into_param().abi(), &mut result__).from_abi::<IXamlDirectObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn CreateInstance(&self, typeindex: XamlTypeIndex) -> ::windows::core::Result<IXamlDirectObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), typeindex, &mut result__).from_abi::<IXamlDirectObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn SetObjectProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetObjectProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn SetXamlDirectObjectProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetXamlDirectObjectProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn SetBooleanProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBooleanProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn SetDoubleProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDoubleProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn SetInt32Property<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInt32Property)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn SetStringProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStringProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDateTimeProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::DateTime>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDateTimeProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPointProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPointProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRectProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Rect>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRectProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSizeProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Size>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSizeProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTimeSpanProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TimeSpan>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTimeSpanProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn SetColorProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, super::super::super::Color>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetColorProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn SetCornerRadiusProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, super::super::CornerRadius>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCornerRadiusProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDurationProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, super::super::Duration>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDurationProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn SetGridLengthProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, super::super::GridLength>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGridLengthProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn SetThicknessProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, super::super::Thickness>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetThicknessProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetMatrixProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, super::super::Media::Matrix>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMatrixProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`, `\"UI_Xaml_Media_Media3D\"`*"]
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub fn SetMatrix3DProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, super::super::Media::Media3D::Matrix3D>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMatrix3DProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn SetEnumProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEnumProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn GetObjectProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetObjectProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn GetXamlDirectObjectProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<IXamlDirectObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetXamlDirectObjectProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<IXamlDirectObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn GetBooleanProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBooleanProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn GetDoubleProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDoubleProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn GetInt32Property<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetInt32Property)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn GetStringProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStringProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDateTimeProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDateTimeProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPointProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPointProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetRectProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetRectProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSizeProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSizeProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetTimeSpanProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetTimeSpanProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn GetColorProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetColorProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn GetCornerRadiusProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::CornerRadius> {
        let this = self;
        unsafe {
            let mut result__: super::super::CornerRadius = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCornerRadiusProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::CornerRadius>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDurationProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::Duration> {
        let this = self;
        unsafe {
            let mut result__: super::super::Duration = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDurationProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::Duration>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn GetGridLengthProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::GridLength> {
        let this = self;
        unsafe {
            let mut result__: super::super::GridLength = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetGridLengthProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::GridLength>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn GetThicknessProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThicknessProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn GetMatrixProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::Media::Matrix> {
        let this = self;
        unsafe {
            let mut result__: super::super::Media::Matrix = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMatrixProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::Media::Matrix>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`, `\"UI_Xaml_Media_Media3D\"`*"]
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub fn GetMatrix3DProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::Media::Media3D::Matrix3D> {
        let this = self;
        unsafe {
            let mut result__: super::super::Media::Media3D::Matrix3D = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMatrix3DProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::Media::Media3D::Matrix3D>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn GetEnumProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetEnumProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn ClearProperty<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ClearProperty)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn GetCollectionCount<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCollectionCount)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn GetXamlDirectObjectFromCollectionAt<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, index: u32) -> ::windows::core::Result<IXamlDirectObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetXamlDirectObjectFromCollectionAt)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), index, &mut result__).from_abi::<IXamlDirectObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn AddToCollection<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param1: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddToCollection)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn InsertIntoCollectionAt<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, index: u32, value: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).InsertIntoCollectionAt)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn RemoveFromCollection<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param1: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoveFromCollection)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn RemoveFromCollectionAt<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFromCollectionAt)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), index).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn ClearCollection<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ClearCollection)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn AddEventHandler<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xamldirectobject: Param0, eventindex: XamlEventIndex, handler: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddEventHandler)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), eventindex, handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn AddEventHandler_HandledEventsToo<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xamldirectobject: Param0, eventindex: XamlEventIndex, handler: Param2, handledeventstoo: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddEventHandler_HandledEventsToo)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), eventindex, handler.into_param().abi(), handledeventstoo).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn RemoveEventHandler<'a, Param0: ::windows::core::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, xamldirectobject: Param0, eventindex: XamlEventIndex, handler: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEventHandler)(::core::mem::transmute_copy(this), xamldirectobject.into_param().abi(), eventindex, handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<XamlDirect> {
        Self::IXamlDirectStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XamlDirect>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlDirectStatics<R, F: FnOnce(&IXamlDirectStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlDirect, IXamlDirectStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlDirect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlDirect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlDirect {}
impl ::core::fmt::Debug for XamlDirect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlDirect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlDirect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Core.Direct.XamlDirect;{5ffa1295-add2-590f-a051-70989b866ade})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XamlDirect {
    type Vtable = IXamlDirect_Vtbl;
    const IID: ::windows::core::GUID = <IXamlDirect as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlDirect {
    const NAME: &'static str = "Windows.UI.Xaml.Core.Direct.XamlDirect";
}
impl ::core::convert::From<XamlDirect> for ::windows::core::IUnknown {
    fn from(value: XamlDirect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlDirect> for ::windows::core::IUnknown {
    fn from(value: &XamlDirect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlDirect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlDirect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlDirect> for ::windows::core::IInspectable {
    fn from(value: XamlDirect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlDirect> for ::windows::core::IInspectable {
    fn from(value: &XamlDirect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlDirect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlDirect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlDirect {}
unsafe impl ::core::marker::Sync for XamlDirect {}
#[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XamlEventIndex(pub i32);
impl XamlEventIndex {
    pub const FrameworkElement_DataContextChanged: Self = Self(16i32);
    pub const FrameworkElement_SizeChanged: Self = Self(17i32);
    pub const FrameworkElement_LayoutUpdated: Self = Self(18i32);
    pub const UIElement_KeyUp: Self = Self(22i32);
    pub const UIElement_KeyDown: Self = Self(23i32);
    pub const UIElement_GotFocus: Self = Self(24i32);
    pub const UIElement_LostFocus: Self = Self(25i32);
    pub const UIElement_DragStarting: Self = Self(26i32);
    pub const UIElement_DropCompleted: Self = Self(27i32);
    pub const UIElement_CharacterReceived: Self = Self(28i32);
    pub const UIElement_DragEnter: Self = Self(29i32);
    pub const UIElement_DragLeave: Self = Self(30i32);
    pub const UIElement_DragOver: Self = Self(31i32);
    pub const UIElement_Drop: Self = Self(32i32);
    pub const UIElement_PointerPressed: Self = Self(38i32);
    pub const UIElement_PointerMoved: Self = Self(39i32);
    pub const UIElement_PointerReleased: Self = Self(40i32);
    pub const UIElement_PointerEntered: Self = Self(41i32);
    pub const UIElement_PointerExited: Self = Self(42i32);
    pub const UIElement_PointerCaptureLost: Self = Self(43i32);
    pub const UIElement_PointerCanceled: Self = Self(44i32);
    pub const UIElement_PointerWheelChanged: Self = Self(45i32);
    pub const UIElement_Tapped: Self = Self(46i32);
    pub const UIElement_DoubleTapped: Self = Self(47i32);
    pub const UIElement_Holding: Self = Self(48i32);
    pub const UIElement_ContextRequested: Self = Self(49i32);
    pub const UIElement_ContextCanceled: Self = Self(50i32);
    pub const UIElement_RightTapped: Self = Self(51i32);
    pub const UIElement_ManipulationStarting: Self = Self(52i32);
    pub const UIElement_ManipulationInertiaStarting: Self = Self(53i32);
    pub const UIElement_ManipulationStarted: Self = Self(54i32);
    pub const UIElement_ManipulationDelta: Self = Self(55i32);
    pub const UIElement_ManipulationCompleted: Self = Self(56i32);
    pub const UIElement_ProcessKeyboardAccelerators: Self = Self(60i32);
    pub const UIElement_GettingFocus: Self = Self(61i32);
    pub const UIElement_LosingFocus: Self = Self(62i32);
    pub const UIElement_NoFocusCandidateFound: Self = Self(63i32);
    pub const UIElement_PreviewKeyDown: Self = Self(64i32);
    pub const UIElement_PreviewKeyUp: Self = Self(65i32);
    pub const UIElement_BringIntoViewRequested: Self = Self(66i32);
    pub const AppBar_Opening: Self = Self(109i32);
    pub const AppBar_Opened: Self = Self(110i32);
    pub const AppBar_Closing: Self = Self(111i32);
    pub const AppBar_Closed: Self = Self(112i32);
    pub const AutoSuggestBox_SuggestionChosen: Self = Self(113i32);
    pub const AutoSuggestBox_TextChanged: Self = Self(114i32);
    pub const AutoSuggestBox_QuerySubmitted: Self = Self(115i32);
    pub const CalendarDatePicker_CalendarViewDayItemChanging: Self = Self(116i32);
    pub const CalendarDatePicker_DateChanged: Self = Self(117i32);
    pub const CalendarDatePicker_Opened: Self = Self(118i32);
    pub const CalendarDatePicker_Closed: Self = Self(119i32);
    pub const CalendarView_CalendarViewDayItemChanging: Self = Self(120i32);
    pub const CalendarView_SelectedDatesChanged: Self = Self(121i32);
    pub const ComboBox_DropDownClosed: Self = Self(122i32);
    pub const ComboBox_DropDownOpened: Self = Self(123i32);
    pub const CommandBar_DynamicOverflowItemsChanging: Self = Self(124i32);
    pub const ContentDialog_Closing: Self = Self(126i32);
    pub const ContentDialog_Closed: Self = Self(127i32);
    pub const ContentDialog_Opened: Self = Self(128i32);
    pub const ContentDialog_PrimaryButtonClick: Self = Self(129i32);
    pub const ContentDialog_SecondaryButtonClick: Self = Self(130i32);
    pub const ContentDialog_CloseButtonClick: Self = Self(131i32);
    pub const Control_FocusEngaged: Self = Self(132i32);
    pub const Control_FocusDisengaged: Self = Self(133i32);
    pub const DatePicker_DateChanged: Self = Self(135i32);
    pub const Frame_Navigated: Self = Self(136i32);
    pub const Frame_Navigating: Self = Self(137i32);
    pub const Frame_NavigationFailed: Self = Self(138i32);
    pub const Frame_NavigationStopped: Self = Self(139i32);
    pub const Hub_SectionHeaderClick: Self = Self(143i32);
    pub const Hub_SectionsInViewChanged: Self = Self(144i32);
    pub const ItemsPresenter_HorizontalSnapPointsChanged: Self = Self(148i32);
    pub const ItemsPresenter_VerticalSnapPointsChanged: Self = Self(149i32);
    pub const ListViewBase_ItemClick: Self = Self(150i32);
    pub const ListViewBase_DragItemsStarting: Self = Self(151i32);
    pub const ListViewBase_DragItemsCompleted: Self = Self(152i32);
    pub const ListViewBase_ContainerContentChanging: Self = Self(153i32);
    pub const ListViewBase_ChoosingItemContainer: Self = Self(154i32);
    pub const ListViewBase_ChoosingGroupHeaderContainer: Self = Self(155i32);
    pub const MediaTransportControls_ThumbnailRequested: Self = Self(167i32);
    pub const MenuFlyoutItem_Click: Self = Self(168i32);
    pub const RichEditBox_TextChanging: Self = Self(177i32);
    pub const ScrollViewer_ViewChanging: Self = Self(192i32);
    pub const ScrollViewer_ViewChanged: Self = Self(193i32);
    pub const ScrollViewer_DirectManipulationStarted: Self = Self(194i32);
    pub const ScrollViewer_DirectManipulationCompleted: Self = Self(195i32);
    pub const SearchBox_QueryChanged: Self = Self(196i32);
    pub const SearchBox_SuggestionsRequested: Self = Self(197i32);
    pub const SearchBox_QuerySubmitted: Self = Self(198i32);
    pub const SearchBox_ResultSuggestionChosen: Self = Self(199i32);
    pub const SearchBox_PrepareForFocusOnKeyboardInput: Self = Self(200i32);
    pub const SemanticZoom_ViewChangeStarted: Self = Self(201i32);
    pub const SemanticZoom_ViewChangeCompleted: Self = Self(202i32);
    pub const SettingsFlyout_BackClick: Self = Self(203i32);
    pub const StackPanel_HorizontalSnapPointsChanged: Self = Self(208i32);
    pub const StackPanel_VerticalSnapPointsChanged: Self = Self(209i32);
    pub const TimePicker_TimeChanged: Self = Self(227i32);
    pub const ToggleSwitch_Toggled: Self = Self(228i32);
    pub const ToolTip_Closed: Self = Self(229i32);
    pub const ToolTip_Opened: Self = Self(230i32);
    pub const VirtualizingStackPanel_CleanUpVirtualizedItemEvent: Self = Self(231i32);
    pub const WebView_SeparateProcessLost: Self = Self(232i32);
    pub const WebView_LoadCompleted: Self = Self(233i32);
    pub const WebView_ScriptNotify: Self = Self(234i32);
    pub const WebView_NavigationFailed: Self = Self(235i32);
    pub const WebView_NavigationStarting: Self = Self(236i32);
    pub const WebView_ContentLoading: Self = Self(237i32);
    pub const WebView_DOMContentLoaded: Self = Self(238i32);
    pub const WebView_NavigationCompleted: Self = Self(239i32);
    pub const WebView_FrameNavigationStarting: Self = Self(240i32);
    pub const WebView_FrameContentLoading: Self = Self(241i32);
    pub const WebView_FrameDOMContentLoaded: Self = Self(242i32);
    pub const WebView_FrameNavigationCompleted: Self = Self(243i32);
    pub const WebView_LongRunningScriptDetected: Self = Self(244i32);
    pub const WebView_UnsafeContentWarningDisplaying: Self = Self(245i32);
    pub const WebView_UnviewableContentIdentified: Self = Self(246i32);
    pub const WebView_ContainsFullScreenElementChanged: Self = Self(247i32);
    pub const WebView_UnsupportedUriSchemeIdentified: Self = Self(248i32);
    pub const WebView_NewWindowRequested: Self = Self(249i32);
    pub const WebView_PermissionRequested: Self = Self(250i32);
    pub const ButtonBase_Click: Self = Self(256i32);
    pub const CarouselPanel_HorizontalSnapPointsChanged: Self = Self(257i32);
    pub const CarouselPanel_VerticalSnapPointsChanged: Self = Self(258i32);
    pub const OrientedVirtualizingPanel_HorizontalSnapPointsChanged: Self = Self(263i32);
    pub const OrientedVirtualizingPanel_VerticalSnapPointsChanged: Self = Self(264i32);
    pub const RangeBase_ValueChanged: Self = Self(267i32);
    pub const ScrollBar_Scroll: Self = Self(268i32);
    pub const Selector_SelectionChanged: Self = Self(269i32);
    pub const Thumb_DragStarted: Self = Self(270i32);
    pub const Thumb_DragDelta: Self = Self(271i32);
    pub const Thumb_DragCompleted: Self = Self(272i32);
    pub const ToggleButton_Checked: Self = Self(273i32);
    pub const ToggleButton_Unchecked: Self = Self(274i32);
    pub const ToggleButton_Indeterminate: Self = Self(275i32);
    pub const WebView_WebResourceRequested: Self = Self(283i32);
    pub const ScrollViewer_AnchorRequested: Self = Self(291i32);
    pub const DatePicker_SelectedDateChanged: Self = Self(322i32);
    pub const TimePicker_SelectedTimeChanged: Self = Self(323i32);
}
impl ::core::marker::Copy for XamlEventIndex {}
impl ::core::clone::Clone for XamlEventIndex {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XamlEventIndex {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XamlEventIndex {
    type Abi = Self;
}
impl ::core::fmt::Debug for XamlEventIndex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlEventIndex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlEventIndex {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Core.Direct.XamlEventIndex;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XamlPropertyIndex(pub i32);
impl XamlPropertyIndex {
    pub const AutomationProperties_AcceleratorKey: Self = Self(5i32);
    pub const AutomationProperties_AccessibilityView: Self = Self(6i32);
    pub const AutomationProperties_AccessKey: Self = Self(7i32);
    pub const AutomationProperties_AutomationId: Self = Self(8i32);
    pub const AutomationProperties_ControlledPeers: Self = Self(9i32);
    pub const AutomationProperties_HelpText: Self = Self(10i32);
    pub const AutomationProperties_IsRequiredForForm: Self = Self(11i32);
    pub const AutomationProperties_ItemStatus: Self = Self(12i32);
    pub const AutomationProperties_ItemType: Self = Self(13i32);
    pub const AutomationProperties_LabeledBy: Self = Self(14i32);
    pub const AutomationProperties_LiveSetting: Self = Self(15i32);
    pub const AutomationProperties_Name: Self = Self(16i32);
    pub const ToolTipService_Placement: Self = Self(24i32);
    pub const ToolTipService_PlacementTarget: Self = Self(25i32);
    pub const ToolTipService_ToolTip: Self = Self(26i32);
    pub const Typography_AnnotationAlternates: Self = Self(28i32);
    pub const Typography_Capitals: Self = Self(29i32);
    pub const Typography_CapitalSpacing: Self = Self(30i32);
    pub const Typography_CaseSensitiveForms: Self = Self(31i32);
    pub const Typography_ContextualAlternates: Self = Self(32i32);
    pub const Typography_ContextualLigatures: Self = Self(33i32);
    pub const Typography_ContextualSwashes: Self = Self(34i32);
    pub const Typography_DiscretionaryLigatures: Self = Self(35i32);
    pub const Typography_EastAsianExpertForms: Self = Self(36i32);
    pub const Typography_EastAsianLanguage: Self = Self(37i32);
    pub const Typography_EastAsianWidths: Self = Self(38i32);
    pub const Typography_Fraction: Self = Self(39i32);
    pub const Typography_HistoricalForms: Self = Self(40i32);
    pub const Typography_HistoricalLigatures: Self = Self(41i32);
    pub const Typography_Kerning: Self = Self(42i32);
    pub const Typography_MathematicalGreek: Self = Self(43i32);
    pub const Typography_NumeralAlignment: Self = Self(44i32);
    pub const Typography_NumeralStyle: Self = Self(45i32);
    pub const Typography_SlashedZero: Self = Self(46i32);
    pub const Typography_StandardLigatures: Self = Self(47i32);
    pub const Typography_StandardSwashes: Self = Self(48i32);
    pub const Typography_StylisticAlternates: Self = Self(49i32);
    pub const Typography_StylisticSet1: Self = Self(50i32);
    pub const Typography_StylisticSet10: Self = Self(51i32);
    pub const Typography_StylisticSet11: Self = Self(52i32);
    pub const Typography_StylisticSet12: Self = Self(53i32);
    pub const Typography_StylisticSet13: Self = Self(54i32);
    pub const Typography_StylisticSet14: Self = Self(55i32);
    pub const Typography_StylisticSet15: Self = Self(56i32);
    pub const Typography_StylisticSet16: Self = Self(57i32);
    pub const Typography_StylisticSet17: Self = Self(58i32);
    pub const Typography_StylisticSet18: Self = Self(59i32);
    pub const Typography_StylisticSet19: Self = Self(60i32);
    pub const Typography_StylisticSet2: Self = Self(61i32);
    pub const Typography_StylisticSet20: Self = Self(62i32);
    pub const Typography_StylisticSet3: Self = Self(63i32);
    pub const Typography_StylisticSet4: Self = Self(64i32);
    pub const Typography_StylisticSet5: Self = Self(65i32);
    pub const Typography_StylisticSet6: Self = Self(66i32);
    pub const Typography_StylisticSet7: Self = Self(67i32);
    pub const Typography_StylisticSet8: Self = Self(68i32);
    pub const Typography_StylisticSet9: Self = Self(69i32);
    pub const Typography_Variants: Self = Self(70i32);
    pub const AutomationPeer_EventsSource: Self = Self(75i32);
    pub const AutoSuggestBoxSuggestionChosenEventArgs_SelectedItem: Self = Self(76i32);
    pub const AutoSuggestBoxTextChangedEventArgs_Reason: Self = Self(77i32);
    pub const Brush_Opacity: Self = Self(78i32);
    pub const Brush_RelativeTransform: Self = Self(79i32);
    pub const Brush_Transform: Self = Self(80i32);
    pub const CollectionViewSource_IsSourceGrouped: Self = Self(81i32);
    pub const CollectionViewSource_ItemsPath: Self = Self(82i32);
    pub const CollectionViewSource_Source: Self = Self(83i32);
    pub const CollectionViewSource_View: Self = Self(84i32);
    pub const ColorKeyFrame_KeyTime: Self = Self(90i32);
    pub const ColorKeyFrame_Value: Self = Self(91i32);
    pub const ColumnDefinition_ActualWidth: Self = Self(92i32);
    pub const ColumnDefinition_MaxWidth: Self = Self(93i32);
    pub const ColumnDefinition_MinWidth: Self = Self(94i32);
    pub const ColumnDefinition_Width: Self = Self(95i32);
    pub const ComboBoxTemplateSettings_DropDownClosedHeight: Self = Self(96i32);
    pub const ComboBoxTemplateSettings_DropDownOffset: Self = Self(97i32);
    pub const ComboBoxTemplateSettings_DropDownOpenedHeight: Self = Self(98i32);
    pub const ComboBoxTemplateSettings_SelectedItemDirection: Self = Self(99i32);
    pub const DoubleKeyFrame_KeyTime: Self = Self(107i32);
    pub const DoubleKeyFrame_Value: Self = Self(108i32);
    pub const EasingFunctionBase_EasingMode: Self = Self(111i32);
    pub const FlyoutBase_AttachedFlyout: Self = Self(114i32);
    pub const FlyoutBase_Placement: Self = Self(115i32);
    pub const Geometry_Bounds: Self = Self(118i32);
    pub const Geometry_Transform: Self = Self(119i32);
    pub const GradientStop_Color: Self = Self(120i32);
    pub const GradientStop_Offset: Self = Self(121i32);
    pub const GroupStyle_ContainerStyle: Self = Self(124i32);
    pub const GroupStyle_ContainerStyleSelector: Self = Self(125i32);
    pub const GroupStyle_HeaderContainerStyle: Self = Self(126i32);
    pub const GroupStyle_HeaderTemplate: Self = Self(127i32);
    pub const GroupStyle_HeaderTemplateSelector: Self = Self(128i32);
    pub const GroupStyle_HidesIfEmpty: Self = Self(129i32);
    pub const GroupStyle_Panel: Self = Self(130i32);
    pub const InertiaExpansionBehavior_DesiredDeceleration: Self = Self(144i32);
    pub const InertiaExpansionBehavior_DesiredExpansion: Self = Self(145i32);
    pub const InertiaRotationBehavior_DesiredDeceleration: Self = Self(146i32);
    pub const InertiaRotationBehavior_DesiredRotation: Self = Self(147i32);
    pub const InertiaTranslationBehavior_DesiredDeceleration: Self = Self(148i32);
    pub const InertiaTranslationBehavior_DesiredDisplacement: Self = Self(149i32);
    pub const InputScope_Names: Self = Self(150i32);
    pub const InputScopeName_NameValue: Self = Self(151i32);
    pub const KeySpline_ControlPoint1: Self = Self(153i32);
    pub const KeySpline_ControlPoint2: Self = Self(154i32);
    pub const ManipulationPivot_Center: Self = Self(159i32);
    pub const ManipulationPivot_Radius: Self = Self(160i32);
    pub const ObjectKeyFrame_KeyTime: Self = Self(183i32);
    pub const ObjectKeyFrame_Value: Self = Self(184i32);
    pub const PageStackEntry_SourcePageType: Self = Self(185i32);
    pub const PathFigure_IsClosed: Self = Self(192i32);
    pub const PathFigure_IsFilled: Self = Self(193i32);
    pub const PathFigure_Segments: Self = Self(194i32);
    pub const PathFigure_StartPoint: Self = Self(195i32);
    pub const Pointer_IsInContact: Self = Self(199i32);
    pub const Pointer_IsInRange: Self = Self(200i32);
    pub const Pointer_PointerDeviceType: Self = Self(201i32);
    pub const Pointer_PointerId: Self = Self(202i32);
    pub const PointKeyFrame_KeyTime: Self = Self(205i32);
    pub const PointKeyFrame_Value: Self = Self(206i32);
    pub const PrintDocument_DocumentSource: Self = Self(209i32);
    pub const ProgressBarTemplateSettings_ContainerAnimationEndPosition: Self = Self(211i32);
    pub const ProgressBarTemplateSettings_ContainerAnimationStartPosition: Self = Self(212i32);
    pub const ProgressBarTemplateSettings_EllipseAnimationEndPosition: Self = Self(213i32);
    pub const ProgressBarTemplateSettings_EllipseAnimationWellPosition: Self = Self(214i32);
    pub const ProgressBarTemplateSettings_EllipseDiameter: Self = Self(215i32);
    pub const ProgressBarTemplateSettings_EllipseOffset: Self = Self(216i32);
    pub const ProgressBarTemplateSettings_IndicatorLengthDelta: Self = Self(217i32);
    pub const ProgressRingTemplateSettings_EllipseDiameter: Self = Self(218i32);
    pub const ProgressRingTemplateSettings_EllipseOffset: Self = Self(219i32);
    pub const ProgressRingTemplateSettings_MaxSideLength: Self = Self(220i32);
    pub const PropertyPath_Path: Self = Self(221i32);
    pub const RowDefinition_ActualHeight: Self = Self(226i32);
    pub const RowDefinition_Height: Self = Self(227i32);
    pub const RowDefinition_MaxHeight: Self = Self(228i32);
    pub const RowDefinition_MinHeight: Self = Self(229i32);
    pub const SetterBase_IsSealed: Self = Self(233i32);
    pub const SettingsFlyoutTemplateSettings_BorderBrush: Self = Self(234i32);
    pub const SettingsFlyoutTemplateSettings_BorderThickness: Self = Self(235i32);
    pub const SettingsFlyoutTemplateSettings_ContentTransitions: Self = Self(236i32);
    pub const SettingsFlyoutTemplateSettings_HeaderBackground: Self = Self(237i32);
    pub const SettingsFlyoutTemplateSettings_HeaderForeground: Self = Self(238i32);
    pub const SettingsFlyoutTemplateSettings_IconSource: Self = Self(239i32);
    pub const Style_BasedOn: Self = Self(244i32);
    pub const Style_IsSealed: Self = Self(245i32);
    pub const Style_Setters: Self = Self(246i32);
    pub const Style_TargetType: Self = Self(247i32);
    pub const TextElement_CharacterSpacing: Self = Self(249i32);
    pub const TextElement_FontFamily: Self = Self(250i32);
    pub const TextElement_FontSize: Self = Self(251i32);
    pub const TextElement_FontStretch: Self = Self(252i32);
    pub const TextElement_FontStyle: Self = Self(253i32);
    pub const TextElement_FontWeight: Self = Self(254i32);
    pub const TextElement_Foreground: Self = Self(255i32);
    pub const TextElement_IsTextScaleFactorEnabled: Self = Self(256i32);
    pub const TextElement_Language: Self = Self(257i32);
    pub const Timeline_AutoReverse: Self = Self(263i32);
    pub const Timeline_BeginTime: Self = Self(264i32);
    pub const Timeline_Duration: Self = Self(265i32);
    pub const Timeline_FillBehavior: Self = Self(266i32);
    pub const Timeline_RepeatBehavior: Self = Self(267i32);
    pub const Timeline_SpeedRatio: Self = Self(268i32);
    pub const TimelineMarker_Text: Self = Self(269i32);
    pub const TimelineMarker_Time: Self = Self(270i32);
    pub const TimelineMarker_Type: Self = Self(271i32);
    pub const ToggleSwitchTemplateSettings_CurtainCurrentToOffOffset: Self = Self(273i32);
    pub const ToggleSwitchTemplateSettings_CurtainCurrentToOnOffset: Self = Self(274i32);
    pub const ToggleSwitchTemplateSettings_CurtainOffToOnOffset: Self = Self(275i32);
    pub const ToggleSwitchTemplateSettings_CurtainOnToOffOffset: Self = Self(276i32);
    pub const ToggleSwitchTemplateSettings_KnobCurrentToOffOffset: Self = Self(277i32);
    pub const ToggleSwitchTemplateSettings_KnobCurrentToOnOffset: Self = Self(278i32);
    pub const ToggleSwitchTemplateSettings_KnobOffToOnOffset: Self = Self(279i32);
    pub const ToggleSwitchTemplateSettings_KnobOnToOffOffset: Self = Self(280i32);
    pub const ToolTipTemplateSettings_FromHorizontalOffset: Self = Self(281i32);
    pub const ToolTipTemplateSettings_FromVerticalOffset: Self = Self(282i32);
    pub const UIElement_AllowDrop: Self = Self(292i32);
    pub const UIElement_CacheMode: Self = Self(293i32);
    pub const UIElement_Clip: Self = Self(295i32);
    pub const UIElement_CompositeMode: Self = Self(296i32);
    pub const UIElement_IsDoubleTapEnabled: Self = Self(297i32);
    pub const UIElement_IsHitTestVisible: Self = Self(298i32);
    pub const UIElement_IsHoldingEnabled: Self = Self(299i32);
    pub const UIElement_IsRightTapEnabled: Self = Self(300i32);
    pub const UIElement_IsTapEnabled: Self = Self(301i32);
    pub const UIElement_ManipulationMode: Self = Self(302i32);
    pub const UIElement_Opacity: Self = Self(303i32);
    pub const UIElement_PointerCaptures: Self = Self(304i32);
    pub const UIElement_Projection: Self = Self(305i32);
    pub const UIElement_RenderSize: Self = Self(306i32);
    pub const UIElement_RenderTransform: Self = Self(307i32);
    pub const UIElement_RenderTransformOrigin: Self = Self(308i32);
    pub const UIElement_Transitions: Self = Self(309i32);
    pub const UIElement_UseLayoutRounding: Self = Self(311i32);
    pub const UIElement_Visibility: Self = Self(312i32);
    pub const VisualState_Storyboard: Self = Self(322i32);
    pub const VisualStateGroup_States: Self = Self(323i32);
    pub const VisualStateGroup_Transitions: Self = Self(324i32);
    pub const VisualStateManager_CustomVisualStateManager: Self = Self(325i32);
    pub const VisualStateManager_VisualStateGroups: Self = Self(326i32);
    pub const VisualTransition_From: Self = Self(327i32);
    pub const VisualTransition_GeneratedDuration: Self = Self(328i32);
    pub const VisualTransition_GeneratedEasingFunction: Self = Self(329i32);
    pub const VisualTransition_Storyboard: Self = Self(330i32);
    pub const VisualTransition_To: Self = Self(331i32);
    pub const ArcSegment_IsLargeArc: Self = Self(332i32);
    pub const ArcSegment_Point: Self = Self(333i32);
    pub const ArcSegment_RotationAngle: Self = Self(334i32);
    pub const ArcSegment_Size: Self = Self(335i32);
    pub const ArcSegment_SweepDirection: Self = Self(336i32);
    pub const BackEase_Amplitude: Self = Self(337i32);
    pub const BeginStoryboard_Storyboard: Self = Self(338i32);
    pub const BezierSegment_Point1: Self = Self(339i32);
    pub const BezierSegment_Point2: Self = Self(340i32);
    pub const BezierSegment_Point3: Self = Self(341i32);
    pub const BitmapSource_PixelHeight: Self = Self(342i32);
    pub const BitmapSource_PixelWidth: Self = Self(343i32);
    pub const Block_LineHeight: Self = Self(344i32);
    pub const Block_LineStackingStrategy: Self = Self(345i32);
    pub const Block_Margin: Self = Self(346i32);
    pub const Block_TextAlignment: Self = Self(347i32);
    pub const BounceEase_Bounces: Self = Self(348i32);
    pub const BounceEase_Bounciness: Self = Self(349i32);
    pub const ColorAnimation_By: Self = Self(350i32);
    pub const ColorAnimation_EasingFunction: Self = Self(351i32);
    pub const ColorAnimation_EnableDependentAnimation: Self = Self(352i32);
    pub const ColorAnimation_From: Self = Self(353i32);
    pub const ColorAnimation_To: Self = Self(354i32);
    pub const ColorAnimationUsingKeyFrames_EnableDependentAnimation: Self = Self(355i32);
    pub const ColorAnimationUsingKeyFrames_KeyFrames: Self = Self(356i32);
    pub const ContentThemeTransition_HorizontalOffset: Self = Self(357i32);
    pub const ContentThemeTransition_VerticalOffset: Self = Self(358i32);
    pub const ControlTemplate_TargetType: Self = Self(359i32);
    pub const DispatcherTimer_Interval: Self = Self(362i32);
    pub const DoubleAnimation_By: Self = Self(363i32);
    pub const DoubleAnimation_EasingFunction: Self = Self(364i32);
    pub const DoubleAnimation_EnableDependentAnimation: Self = Self(365i32);
    pub const DoubleAnimation_From: Self = Self(366i32);
    pub const DoubleAnimation_To: Self = Self(367i32);
    pub const DoubleAnimationUsingKeyFrames_EnableDependentAnimation: Self = Self(368i32);
    pub const DoubleAnimationUsingKeyFrames_KeyFrames: Self = Self(369i32);
    pub const EasingColorKeyFrame_EasingFunction: Self = Self(372i32);
    pub const EasingDoubleKeyFrame_EasingFunction: Self = Self(373i32);
    pub const EasingPointKeyFrame_EasingFunction: Self = Self(374i32);
    pub const EdgeUIThemeTransition_Edge: Self = Self(375i32);
    pub const ElasticEase_Oscillations: Self = Self(376i32);
    pub const ElasticEase_Springiness: Self = Self(377i32);
    pub const EllipseGeometry_Center: Self = Self(378i32);
    pub const EllipseGeometry_RadiusX: Self = Self(379i32);
    pub const EllipseGeometry_RadiusY: Self = Self(380i32);
    pub const EntranceThemeTransition_FromHorizontalOffset: Self = Self(381i32);
    pub const EntranceThemeTransition_FromVerticalOffset: Self = Self(382i32);
    pub const EntranceThemeTransition_IsStaggeringEnabled: Self = Self(383i32);
    pub const EventTrigger_Actions: Self = Self(384i32);
    pub const EventTrigger_RoutedEvent: Self = Self(385i32);
    pub const ExponentialEase_Exponent: Self = Self(386i32);
    pub const Flyout_Content: Self = Self(387i32);
    pub const Flyout_FlyoutPresenterStyle: Self = Self(388i32);
    pub const FrameworkElement_ActualHeight: Self = Self(389i32);
    pub const FrameworkElement_ActualWidth: Self = Self(390i32);
    pub const FrameworkElement_DataContext: Self = Self(391i32);
    pub const FrameworkElement_FlowDirection: Self = Self(392i32);
    pub const FrameworkElement_Height: Self = Self(393i32);
    pub const FrameworkElement_HorizontalAlignment: Self = Self(394i32);
    pub const FrameworkElement_Language: Self = Self(396i32);
    pub const FrameworkElement_Margin: Self = Self(397i32);
    pub const FrameworkElement_MaxHeight: Self = Self(398i32);
    pub const FrameworkElement_MaxWidth: Self = Self(399i32);
    pub const FrameworkElement_MinHeight: Self = Self(400i32);
    pub const FrameworkElement_MinWidth: Self = Self(401i32);
    pub const FrameworkElement_Parent: Self = Self(402i32);
    pub const FrameworkElement_RequestedTheme: Self = Self(403i32);
    pub const FrameworkElement_Resources: Self = Self(404i32);
    pub const FrameworkElement_Style: Self = Self(405i32);
    pub const FrameworkElement_Tag: Self = Self(406i32);
    pub const FrameworkElement_Triggers: Self = Self(407i32);
    pub const FrameworkElement_VerticalAlignment: Self = Self(408i32);
    pub const FrameworkElement_Width: Self = Self(409i32);
    pub const FrameworkElementAutomationPeer_Owner: Self = Self(410i32);
    pub const GeometryGroup_Children: Self = Self(411i32);
    pub const GeometryGroup_FillRule: Self = Self(412i32);
    pub const GradientBrush_ColorInterpolationMode: Self = Self(413i32);
    pub const GradientBrush_GradientStops: Self = Self(414i32);
    pub const GradientBrush_MappingMode: Self = Self(415i32);
    pub const GradientBrush_SpreadMethod: Self = Self(416i32);
    pub const GridViewItemTemplateSettings_DragItemsCount: Self = Self(417i32);
    pub const ItemAutomationPeer_Item: Self = Self(419i32);
    pub const ItemAutomationPeer_ItemsControlAutomationPeer: Self = Self(420i32);
    pub const LineGeometry_EndPoint: Self = Self(422i32);
    pub const LineGeometry_StartPoint: Self = Self(423i32);
    pub const LineSegment_Point: Self = Self(424i32);
    pub const ListViewItemTemplateSettings_DragItemsCount: Self = Self(425i32);
    pub const Matrix3DProjection_ProjectionMatrix: Self = Self(426i32);
    pub const MenuFlyout_Items: Self = Self(427i32);
    pub const MenuFlyout_MenuFlyoutPresenterStyle: Self = Self(428i32);
    pub const ObjectAnimationUsingKeyFrames_EnableDependentAnimation: Self = Self(429i32);
    pub const ObjectAnimationUsingKeyFrames_KeyFrames: Self = Self(430i32);
    pub const PaneThemeTransition_Edge: Self = Self(431i32);
    pub const PathGeometry_Figures: Self = Self(432i32);
    pub const PathGeometry_FillRule: Self = Self(433i32);
    pub const PlaneProjection_CenterOfRotationX: Self = Self(434i32);
    pub const PlaneProjection_CenterOfRotationY: Self = Self(435i32);
    pub const PlaneProjection_CenterOfRotationZ: Self = Self(436i32);
    pub const PlaneProjection_GlobalOffsetX: Self = Self(437i32);
    pub const PlaneProjection_GlobalOffsetY: Self = Self(438i32);
    pub const PlaneProjection_GlobalOffsetZ: Self = Self(439i32);
    pub const PlaneProjection_LocalOffsetX: Self = Self(440i32);
    pub const PlaneProjection_LocalOffsetY: Self = Self(441i32);
    pub const PlaneProjection_LocalOffsetZ: Self = Self(442i32);
    pub const PlaneProjection_ProjectionMatrix: Self = Self(443i32);
    pub const PlaneProjection_RotationX: Self = Self(444i32);
    pub const PlaneProjection_RotationY: Self = Self(445i32);
    pub const PlaneProjection_RotationZ: Self = Self(446i32);
    pub const PointAnimation_By: Self = Self(447i32);
    pub const PointAnimation_EasingFunction: Self = Self(448i32);
    pub const PointAnimation_EnableDependentAnimation: Self = Self(449i32);
    pub const PointAnimation_From: Self = Self(450i32);
    pub const PointAnimation_To: Self = Self(451i32);
    pub const PointAnimationUsingKeyFrames_EnableDependentAnimation: Self = Self(452i32);
    pub const PointAnimationUsingKeyFrames_KeyFrames: Self = Self(453i32);
    pub const PolyBezierSegment_Points: Self = Self(456i32);
    pub const PolyLineSegment_Points: Self = Self(457i32);
    pub const PolyQuadraticBezierSegment_Points: Self = Self(458i32);
    pub const PopupThemeTransition_FromHorizontalOffset: Self = Self(459i32);
    pub const PopupThemeTransition_FromVerticalOffset: Self = Self(460i32);
    pub const PowerEase_Power: Self = Self(461i32);
    pub const QuadraticBezierSegment_Point1: Self = Self(466i32);
    pub const QuadraticBezierSegment_Point2: Self = Self(467i32);
    pub const RectangleGeometry_Rect: Self = Self(470i32);
    pub const RelativeSource_Mode: Self = Self(471i32);
    pub const RenderTargetBitmap_PixelHeight: Self = Self(472i32);
    pub const RenderTargetBitmap_PixelWidth: Self = Self(473i32);
    pub const Setter_Property: Self = Self(474i32);
    pub const Setter_Value: Self = Self(475i32);
    pub const SolidColorBrush_Color: Self = Self(476i32);
    pub const SplineColorKeyFrame_KeySpline: Self = Self(477i32);
    pub const SplineDoubleKeyFrame_KeySpline: Self = Self(478i32);
    pub const SplinePointKeyFrame_KeySpline: Self = Self(479i32);
    pub const TileBrush_AlignmentX: Self = Self(483i32);
    pub const TileBrush_AlignmentY: Self = Self(484i32);
    pub const TileBrush_Stretch: Self = Self(485i32);
    pub const Binding_Converter: Self = Self(487i32);
    pub const Binding_ConverterLanguage: Self = Self(488i32);
    pub const Binding_ConverterParameter: Self = Self(489i32);
    pub const Binding_ElementName: Self = Self(490i32);
    pub const Binding_FallbackValue: Self = Self(491i32);
    pub const Binding_Mode: Self = Self(492i32);
    pub const Binding_Path: Self = Self(493i32);
    pub const Binding_RelativeSource: Self = Self(494i32);
    pub const Binding_Source: Self = Self(495i32);
    pub const Binding_TargetNullValue: Self = Self(496i32);
    pub const Binding_UpdateSourceTrigger: Self = Self(497i32);
    pub const BitmapImage_CreateOptions: Self = Self(498i32);
    pub const BitmapImage_DecodePixelHeight: Self = Self(499i32);
    pub const BitmapImage_DecodePixelType: Self = Self(500i32);
    pub const BitmapImage_DecodePixelWidth: Self = Self(501i32);
    pub const BitmapImage_UriSource: Self = Self(502i32);
    pub const Border_Background: Self = Self(503i32);
    pub const Border_BorderBrush: Self = Self(504i32);
    pub const Border_BorderThickness: Self = Self(505i32);
    pub const Border_Child: Self = Self(506i32);
    pub const Border_ChildTransitions: Self = Self(507i32);
    pub const Border_CornerRadius: Self = Self(508i32);
    pub const Border_Padding: Self = Self(509i32);
    pub const CaptureElement_Source: Self = Self(510i32);
    pub const CaptureElement_Stretch: Self = Self(511i32);
    pub const CompositeTransform_CenterX: Self = Self(514i32);
    pub const CompositeTransform_CenterY: Self = Self(515i32);
    pub const CompositeTransform_Rotation: Self = Self(516i32);
    pub const CompositeTransform_ScaleX: Self = Self(517i32);
    pub const CompositeTransform_ScaleY: Self = Self(518i32);
    pub const CompositeTransform_SkewX: Self = Self(519i32);
    pub const CompositeTransform_SkewY: Self = Self(520i32);
    pub const CompositeTransform_TranslateX: Self = Self(521i32);
    pub const CompositeTransform_TranslateY: Self = Self(522i32);
    pub const ContentPresenter_CharacterSpacing: Self = Self(523i32);
    pub const ContentPresenter_Content: Self = Self(524i32);
    pub const ContentPresenter_ContentTemplate: Self = Self(525i32);
    pub const ContentPresenter_ContentTemplateSelector: Self = Self(526i32);
    pub const ContentPresenter_ContentTransitions: Self = Self(527i32);
    pub const ContentPresenter_FontFamily: Self = Self(528i32);
    pub const ContentPresenter_FontSize: Self = Self(529i32);
    pub const ContentPresenter_FontStretch: Self = Self(530i32);
    pub const ContentPresenter_FontStyle: Self = Self(531i32);
    pub const ContentPresenter_FontWeight: Self = Self(532i32);
    pub const ContentPresenter_Foreground: Self = Self(533i32);
    pub const ContentPresenter_IsTextScaleFactorEnabled: Self = Self(534i32);
    pub const ContentPresenter_LineStackingStrategy: Self = Self(535i32);
    pub const ContentPresenter_MaxLines: Self = Self(536i32);
    pub const ContentPresenter_OpticalMarginAlignment: Self = Self(537i32);
    pub const ContentPresenter_TextLineBounds: Self = Self(539i32);
    pub const ContentPresenter_TextWrapping: Self = Self(540i32);
    pub const Control_Background: Self = Self(541i32);
    pub const Control_BorderBrush: Self = Self(542i32);
    pub const Control_BorderThickness: Self = Self(543i32);
    pub const Control_CharacterSpacing: Self = Self(544i32);
    pub const Control_FocusState: Self = Self(546i32);
    pub const Control_FontFamily: Self = Self(547i32);
    pub const Control_FontSize: Self = Self(548i32);
    pub const Control_FontStretch: Self = Self(549i32);
    pub const Control_FontStyle: Self = Self(550i32);
    pub const Control_FontWeight: Self = Self(551i32);
    pub const Control_Foreground: Self = Self(552i32);
    pub const Control_HorizontalContentAlignment: Self = Self(553i32);
    pub const Control_IsEnabled: Self = Self(554i32);
    pub const Control_IsTabStop: Self = Self(555i32);
    pub const Control_IsTextScaleFactorEnabled: Self = Self(556i32);
    pub const Control_Padding: Self = Self(557i32);
    pub const Control_TabIndex: Self = Self(558i32);
    pub const Control_TabNavigation: Self = Self(559i32);
    pub const Control_Template: Self = Self(560i32);
    pub const Control_VerticalContentAlignment: Self = Self(561i32);
    pub const DragItemThemeAnimation_TargetName: Self = Self(565i32);
    pub const DragOverThemeAnimation_Direction: Self = Self(566i32);
    pub const DragOverThemeAnimation_TargetName: Self = Self(567i32);
    pub const DragOverThemeAnimation_ToOffset: Self = Self(568i32);
    pub const DropTargetItemThemeAnimation_TargetName: Self = Self(569i32);
    pub const FadeInThemeAnimation_TargetName: Self = Self(570i32);
    pub const FadeOutThemeAnimation_TargetName: Self = Self(571i32);
    pub const Glyphs_Fill: Self = Self(574i32);
    pub const Glyphs_FontRenderingEmSize: Self = Self(575i32);
    pub const Glyphs_FontUri: Self = Self(576i32);
    pub const Glyphs_Indices: Self = Self(577i32);
    pub const Glyphs_OriginX: Self = Self(578i32);
    pub const Glyphs_OriginY: Self = Self(579i32);
    pub const Glyphs_StyleSimulations: Self = Self(580i32);
    pub const Glyphs_UnicodeString: Self = Self(581i32);
    pub const IconElement_Foreground: Self = Self(584i32);
    pub const Image_NineGrid: Self = Self(586i32);
    pub const Image_PlayToSource: Self = Self(587i32);
    pub const Image_Source: Self = Self(588i32);
    pub const Image_Stretch: Self = Self(589i32);
    pub const ImageBrush_ImageSource: Self = Self(591i32);
    pub const InlineUIContainer_Child: Self = Self(592i32);
    pub const ItemsPresenter_Footer: Self = Self(594i32);
    pub const ItemsPresenter_FooterTemplate: Self = Self(595i32);
    pub const ItemsPresenter_FooterTransitions: Self = Self(596i32);
    pub const ItemsPresenter_Header: Self = Self(597i32);
    pub const ItemsPresenter_HeaderTemplate: Self = Self(598i32);
    pub const ItemsPresenter_HeaderTransitions: Self = Self(599i32);
    pub const ItemsPresenter_Padding: Self = Self(601i32);
    pub const LinearGradientBrush_EndPoint: Self = Self(602i32);
    pub const LinearGradientBrush_StartPoint: Self = Self(603i32);
    pub const MatrixTransform_Matrix: Self = Self(604i32);
    pub const MediaElement_ActualStereo3DVideoPackingMode: Self = Self(605i32);
    pub const MediaElement_AreTransportControlsEnabled: Self = Self(606i32);
    pub const MediaElement_AspectRatioHeight: Self = Self(607i32);
    pub const MediaElement_AspectRatioWidth: Self = Self(608i32);
    pub const MediaElement_AudioCategory: Self = Self(609i32);
    pub const MediaElement_AudioDeviceType: Self = Self(610i32);
    pub const MediaElement_AudioStreamCount: Self = Self(611i32);
    pub const MediaElement_AudioStreamIndex: Self = Self(612i32);
    pub const MediaElement_AutoPlay: Self = Self(613i32);
    pub const MediaElement_Balance: Self = Self(614i32);
    pub const MediaElement_BufferingProgress: Self = Self(615i32);
    pub const MediaElement_CanPause: Self = Self(616i32);
    pub const MediaElement_CanSeek: Self = Self(617i32);
    pub const MediaElement_CurrentState: Self = Self(618i32);
    pub const MediaElement_DefaultPlaybackRate: Self = Self(619i32);
    pub const MediaElement_DownloadProgress: Self = Self(620i32);
    pub const MediaElement_DownloadProgressOffset: Self = Self(621i32);
    pub const MediaElement_IsAudioOnly: Self = Self(623i32);
    pub const MediaElement_IsFullWindow: Self = Self(624i32);
    pub const MediaElement_IsLooping: Self = Self(625i32);
    pub const MediaElement_IsMuted: Self = Self(626i32);
    pub const MediaElement_IsStereo3DVideo: Self = Self(627i32);
    pub const MediaElement_Markers: Self = Self(628i32);
    pub const MediaElement_NaturalDuration: Self = Self(629i32);
    pub const MediaElement_NaturalVideoHeight: Self = Self(630i32);
    pub const MediaElement_NaturalVideoWidth: Self = Self(631i32);
    pub const MediaElement_PlaybackRate: Self = Self(632i32);
    pub const MediaElement_PlayToPreferredSourceUri: Self = Self(633i32);
    pub const MediaElement_PlayToSource: Self = Self(634i32);
    pub const MediaElement_Position: Self = Self(635i32);
    pub const MediaElement_PosterSource: Self = Self(636i32);
    pub const MediaElement_ProtectionManager: Self = Self(637i32);
    pub const MediaElement_RealTimePlayback: Self = Self(638i32);
    pub const MediaElement_Source: Self = Self(639i32);
    pub const MediaElement_Stereo3DVideoPackingMode: Self = Self(640i32);
    pub const MediaElement_Stereo3DVideoRenderMode: Self = Self(641i32);
    pub const MediaElement_Stretch: Self = Self(642i32);
    pub const MediaElement_TransportControls: Self = Self(643i32);
    pub const MediaElement_Volume: Self = Self(644i32);
    pub const Panel_Background: Self = Self(647i32);
    pub const Panel_Children: Self = Self(648i32);
    pub const Panel_ChildrenTransitions: Self = Self(649i32);
    pub const Panel_IsItemsHost: Self = Self(651i32);
    pub const Paragraph_Inlines: Self = Self(652i32);
    pub const Paragraph_TextIndent: Self = Self(653i32);
    pub const PointerDownThemeAnimation_TargetName: Self = Self(660i32);
    pub const PointerUpThemeAnimation_TargetName: Self = Self(662i32);
    pub const PopInThemeAnimation_FromHorizontalOffset: Self = Self(664i32);
    pub const PopInThemeAnimation_FromVerticalOffset: Self = Self(665i32);
    pub const PopInThemeAnimation_TargetName: Self = Self(666i32);
    pub const PopOutThemeAnimation_TargetName: Self = Self(667i32);
    pub const Popup_Child: Self = Self(668i32);
    pub const Popup_ChildTransitions: Self = Self(669i32);
    pub const Popup_HorizontalOffset: Self = Self(670i32);
    pub const Popup_IsLightDismissEnabled: Self = Self(673i32);
    pub const Popup_IsOpen: Self = Self(674i32);
    pub const Popup_VerticalOffset: Self = Self(676i32);
    pub const RepositionThemeAnimation_FromHorizontalOffset: Self = Self(683i32);
    pub const RepositionThemeAnimation_FromVerticalOffset: Self = Self(684i32);
    pub const RepositionThemeAnimation_TargetName: Self = Self(685i32);
    pub const ResourceDictionary_MergedDictionaries: Self = Self(687i32);
    pub const ResourceDictionary_Source: Self = Self(688i32);
    pub const ResourceDictionary_ThemeDictionaries: Self = Self(689i32);
    pub const RichTextBlock_Blocks: Self = Self(691i32);
    pub const RichTextBlock_CharacterSpacing: Self = Self(692i32);
    pub const RichTextBlock_FontFamily: Self = Self(693i32);
    pub const RichTextBlock_FontSize: Self = Self(694i32);
    pub const RichTextBlock_FontStretch: Self = Self(695i32);
    pub const RichTextBlock_FontStyle: Self = Self(696i32);
    pub const RichTextBlock_FontWeight: Self = Self(697i32);
    pub const RichTextBlock_Foreground: Self = Self(698i32);
    pub const RichTextBlock_HasOverflowContent: Self = Self(699i32);
    pub const RichTextBlock_IsColorFontEnabled: Self = Self(700i32);
    pub const RichTextBlock_IsTextScaleFactorEnabled: Self = Self(701i32);
    pub const RichTextBlock_IsTextSelectionEnabled: Self = Self(702i32);
    pub const RichTextBlock_LineHeight: Self = Self(703i32);
    pub const RichTextBlock_LineStackingStrategy: Self = Self(704i32);
    pub const RichTextBlock_MaxLines: Self = Self(705i32);
    pub const RichTextBlock_OpticalMarginAlignment: Self = Self(706i32);
    pub const RichTextBlock_OverflowContentTarget: Self = Self(707i32);
    pub const RichTextBlock_Padding: Self = Self(708i32);
    pub const RichTextBlock_SelectedText: Self = Self(709i32);
    pub const RichTextBlock_SelectionHighlightColor: Self = Self(710i32);
    pub const RichTextBlock_TextAlignment: Self = Self(711i32);
    pub const RichTextBlock_TextIndent: Self = Self(712i32);
    pub const RichTextBlock_TextLineBounds: Self = Self(713i32);
    pub const RichTextBlock_TextReadingOrder: Self = Self(714i32);
    pub const RichTextBlock_TextTrimming: Self = Self(715i32);
    pub const RichTextBlock_TextWrapping: Self = Self(716i32);
    pub const RichTextBlockOverflow_HasOverflowContent: Self = Self(717i32);
    pub const RichTextBlockOverflow_MaxLines: Self = Self(718i32);
    pub const RichTextBlockOverflow_OverflowContentTarget: Self = Self(719i32);
    pub const RichTextBlockOverflow_Padding: Self = Self(720i32);
    pub const RotateTransform_Angle: Self = Self(721i32);
    pub const RotateTransform_CenterX: Self = Self(722i32);
    pub const RotateTransform_CenterY: Self = Self(723i32);
    pub const Run_FlowDirection: Self = Self(725i32);
    pub const Run_Text: Self = Self(726i32);
    pub const ScaleTransform_CenterX: Self = Self(727i32);
    pub const ScaleTransform_CenterY: Self = Self(728i32);
    pub const ScaleTransform_ScaleX: Self = Self(729i32);
    pub const ScaleTransform_ScaleY: Self = Self(730i32);
    pub const SetterBaseCollection_IsSealed: Self = Self(732i32);
    pub const Shape_Fill: Self = Self(733i32);
    pub const Shape_GeometryTransform: Self = Self(734i32);
    pub const Shape_Stretch: Self = Self(735i32);
    pub const Shape_Stroke: Self = Self(736i32);
    pub const Shape_StrokeDashArray: Self = Self(737i32);
    pub const Shape_StrokeDashCap: Self = Self(738i32);
    pub const Shape_StrokeDashOffset: Self = Self(739i32);
    pub const Shape_StrokeEndLineCap: Self = Self(740i32);
    pub const Shape_StrokeLineJoin: Self = Self(741i32);
    pub const Shape_StrokeMiterLimit: Self = Self(742i32);
    pub const Shape_StrokeStartLineCap: Self = Self(743i32);
    pub const Shape_StrokeThickness: Self = Self(744i32);
    pub const SkewTransform_AngleX: Self = Self(745i32);
    pub const SkewTransform_AngleY: Self = Self(746i32);
    pub const SkewTransform_CenterX: Self = Self(747i32);
    pub const SkewTransform_CenterY: Self = Self(748i32);
    pub const Span_Inlines: Self = Self(749i32);
    pub const SplitCloseThemeAnimation_ClosedLength: Self = Self(750i32);
    pub const SplitCloseThemeAnimation_ClosedTarget: Self = Self(751i32);
    pub const SplitCloseThemeAnimation_ClosedTargetName: Self = Self(752i32);
    pub const SplitCloseThemeAnimation_ContentTarget: Self = Self(753i32);
    pub const SplitCloseThemeAnimation_ContentTargetName: Self = Self(754i32);
    pub const SplitCloseThemeAnimation_ContentTranslationDirection: Self = Self(755i32);
    pub const SplitCloseThemeAnimation_ContentTranslationOffset: Self = Self(756i32);
    pub const SplitCloseThemeAnimation_OffsetFromCenter: Self = Self(757i32);
    pub const SplitCloseThemeAnimation_OpenedLength: Self = Self(758i32);
    pub const SplitCloseThemeAnimation_OpenedTarget: Self = Self(759i32);
    pub const SplitCloseThemeAnimation_OpenedTargetName: Self = Self(760i32);
    pub const SplitOpenThemeAnimation_ClosedLength: Self = Self(761i32);
    pub const SplitOpenThemeAnimation_ClosedTarget: Self = Self(762i32);
    pub const SplitOpenThemeAnimation_ClosedTargetName: Self = Self(763i32);
    pub const SplitOpenThemeAnimation_ContentTarget: Self = Self(764i32);
    pub const SplitOpenThemeAnimation_ContentTargetName: Self = Self(765i32);
    pub const SplitOpenThemeAnimation_ContentTranslationDirection: Self = Self(766i32);
    pub const SplitOpenThemeAnimation_ContentTranslationOffset: Self = Self(767i32);
    pub const SplitOpenThemeAnimation_OffsetFromCenter: Self = Self(768i32);
    pub const SplitOpenThemeAnimation_OpenedLength: Self = Self(769i32);
    pub const SplitOpenThemeAnimation_OpenedTarget: Self = Self(770i32);
    pub const SplitOpenThemeAnimation_OpenedTargetName: Self = Self(771i32);
    pub const Storyboard_Children: Self = Self(772i32);
    pub const Storyboard_TargetName: Self = Self(774i32);
    pub const Storyboard_TargetProperty: Self = Self(775i32);
    pub const SwipeBackThemeAnimation_FromHorizontalOffset: Self = Self(776i32);
    pub const SwipeBackThemeAnimation_FromVerticalOffset: Self = Self(777i32);
    pub const SwipeBackThemeAnimation_TargetName: Self = Self(778i32);
    pub const SwipeHintThemeAnimation_TargetName: Self = Self(779i32);
    pub const SwipeHintThemeAnimation_ToHorizontalOffset: Self = Self(780i32);
    pub const SwipeHintThemeAnimation_ToVerticalOffset: Self = Self(781i32);
    pub const TextBlock_CharacterSpacing: Self = Self(782i32);
    pub const TextBlock_FontFamily: Self = Self(783i32);
    pub const TextBlock_FontSize: Self = Self(784i32);
    pub const TextBlock_FontStretch: Self = Self(785i32);
    pub const TextBlock_FontStyle: Self = Self(786i32);
    pub const TextBlock_FontWeight: Self = Self(787i32);
    pub const TextBlock_Foreground: Self = Self(788i32);
    pub const TextBlock_Inlines: Self = Self(789i32);
    pub const TextBlock_IsColorFontEnabled: Self = Self(790i32);
    pub const TextBlock_IsTextScaleFactorEnabled: Self = Self(791i32);
    pub const TextBlock_IsTextSelectionEnabled: Self = Self(792i32);
    pub const TextBlock_LineHeight: Self = Self(793i32);
    pub const TextBlock_LineStackingStrategy: Self = Self(794i32);
    pub const TextBlock_MaxLines: Self = Self(795i32);
    pub const TextBlock_OpticalMarginAlignment: Self = Self(796i32);
    pub const TextBlock_Padding: Self = Self(797i32);
    pub const TextBlock_SelectedText: Self = Self(798i32);
    pub const TextBlock_SelectionHighlightColor: Self = Self(799i32);
    pub const TextBlock_Text: Self = Self(800i32);
    pub const TextBlock_TextAlignment: Self = Self(801i32);
    pub const TextBlock_TextDecorations: Self = Self(802i32);
    pub const TextBlock_TextLineBounds: Self = Self(803i32);
    pub const TextBlock_TextReadingOrder: Self = Self(804i32);
    pub const TextBlock_TextTrimming: Self = Self(805i32);
    pub const TextBlock_TextWrapping: Self = Self(806i32);
    pub const TransformGroup_Children: Self = Self(811i32);
    pub const TransformGroup_Value: Self = Self(812i32);
    pub const TranslateTransform_X: Self = Self(814i32);
    pub const TranslateTransform_Y: Self = Self(815i32);
    pub const Viewbox_Child: Self = Self(819i32);
    pub const Viewbox_Stretch: Self = Self(820i32);
    pub const Viewbox_StretchDirection: Self = Self(821i32);
    pub const WebViewBrush_SourceName: Self = Self(825i32);
    pub const AppBarSeparator_IsCompact: Self = Self(826i32);
    pub const BitmapIcon_UriSource: Self = Self(827i32);
    pub const Canvas_Left: Self = Self(828i32);
    pub const Canvas_Top: Self = Self(829i32);
    pub const Canvas_ZIndex: Self = Self(830i32);
    pub const ContentControl_Content: Self = Self(832i32);
    pub const ContentControl_ContentTemplate: Self = Self(833i32);
    pub const ContentControl_ContentTemplateSelector: Self = Self(834i32);
    pub const ContentControl_ContentTransitions: Self = Self(835i32);
    pub const DatePicker_CalendarIdentifier: Self = Self(837i32);
    pub const DatePicker_Date: Self = Self(838i32);
    pub const DatePicker_DayFormat: Self = Self(839i32);
    pub const DatePicker_DayVisible: Self = Self(840i32);
    pub const DatePicker_Header: Self = Self(841i32);
    pub const DatePicker_HeaderTemplate: Self = Self(842i32);
    pub const DatePicker_MaxYear: Self = Self(843i32);
    pub const DatePicker_MinYear: Self = Self(844i32);
    pub const DatePicker_MonthFormat: Self = Self(845i32);
    pub const DatePicker_MonthVisible: Self = Self(846i32);
    pub const DatePicker_Orientation: Self = Self(847i32);
    pub const DatePicker_YearFormat: Self = Self(848i32);
    pub const DatePicker_YearVisible: Self = Self(849i32);
    pub const FontIcon_FontFamily: Self = Self(851i32);
    pub const FontIcon_FontSize: Self = Self(852i32);
    pub const FontIcon_FontStyle: Self = Self(853i32);
    pub const FontIcon_FontWeight: Self = Self(854i32);
    pub const FontIcon_Glyph: Self = Self(855i32);
    pub const FontIcon_IsTextScaleFactorEnabled: Self = Self(856i32);
    pub const Grid_Column: Self = Self(857i32);
    pub const Grid_ColumnDefinitions: Self = Self(858i32);
    pub const Grid_ColumnSpan: Self = Self(859i32);
    pub const Grid_Row: Self = Self(860i32);
    pub const Grid_RowDefinitions: Self = Self(861i32);
    pub const Grid_RowSpan: Self = Self(862i32);
    pub const Hub_DefaultSectionIndex: Self = Self(863i32);
    pub const Hub_Header: Self = Self(864i32);
    pub const Hub_HeaderTemplate: Self = Self(865i32);
    pub const Hub_IsActiveView: Self = Self(866i32);
    pub const Hub_IsZoomedInView: Self = Self(867i32);
    pub const Hub_Orientation: Self = Self(868i32);
    pub const Hub_SectionHeaders: Self = Self(869i32);
    pub const Hub_Sections: Self = Self(870i32);
    pub const Hub_SectionsInView: Self = Self(871i32);
    pub const Hub_SemanticZoomOwner: Self = Self(872i32);
    pub const HubSection_ContentTemplate: Self = Self(873i32);
    pub const HubSection_Header: Self = Self(874i32);
    pub const HubSection_HeaderTemplate: Self = Self(875i32);
    pub const HubSection_IsHeaderInteractive: Self = Self(876i32);
    pub const Hyperlink_NavigateUri: Self = Self(877i32);
    pub const ItemsControl_DisplayMemberPath: Self = Self(879i32);
    pub const ItemsControl_GroupStyle: Self = Self(880i32);
    pub const ItemsControl_GroupStyleSelector: Self = Self(881i32);
    pub const ItemsControl_IsGrouping: Self = Self(882i32);
    pub const ItemsControl_ItemContainerStyle: Self = Self(884i32);
    pub const ItemsControl_ItemContainerStyleSelector: Self = Self(885i32);
    pub const ItemsControl_ItemContainerTransitions: Self = Self(886i32);
    pub const ItemsControl_Items: Self = Self(887i32);
    pub const ItemsControl_ItemsPanel: Self = Self(889i32);
    pub const ItemsControl_ItemsSource: Self = Self(890i32);
    pub const ItemsControl_ItemTemplate: Self = Self(891i32);
    pub const ItemsControl_ItemTemplateSelector: Self = Self(892i32);
    pub const Line_X1: Self = Self(893i32);
    pub const Line_X2: Self = Self(894i32);
    pub const Line_Y1: Self = Self(895i32);
    pub const Line_Y2: Self = Self(896i32);
    pub const MediaTransportControls_IsFastForwardButtonVisible: Self = Self(898i32);
    pub const MediaTransportControls_IsFastRewindButtonVisible: Self = Self(900i32);
    pub const MediaTransportControls_IsFullWindowButtonVisible: Self = Self(902i32);
    pub const MediaTransportControls_IsPlaybackRateButtonVisible: Self = Self(904i32);
    pub const MediaTransportControls_IsSeekBarVisible: Self = Self(905i32);
    pub const MediaTransportControls_IsStopButtonVisible: Self = Self(908i32);
    pub const MediaTransportControls_IsVolumeButtonVisible: Self = Self(910i32);
    pub const MediaTransportControls_IsZoomButtonVisible: Self = Self(912i32);
    pub const PasswordBox_Header: Self = Self(913i32);
    pub const PasswordBox_HeaderTemplate: Self = Self(914i32);
    pub const PasswordBox_IsPasswordRevealButtonEnabled: Self = Self(915i32);
    pub const PasswordBox_MaxLength: Self = Self(916i32);
    pub const PasswordBox_Password: Self = Self(917i32);
    pub const PasswordBox_PasswordChar: Self = Self(918i32);
    pub const PasswordBox_PlaceholderText: Self = Self(919i32);
    pub const PasswordBox_PreventKeyboardDisplayOnProgrammaticFocus: Self = Self(920i32);
    pub const PasswordBox_SelectionHighlightColor: Self = Self(921i32);
    pub const Path_Data: Self = Self(922i32);
    pub const PathIcon_Data: Self = Self(923i32);
    pub const Polygon_FillRule: Self = Self(924i32);
    pub const Polygon_Points: Self = Self(925i32);
    pub const Polyline_FillRule: Self = Self(926i32);
    pub const Polyline_Points: Self = Self(927i32);
    pub const ProgressRing_IsActive: Self = Self(928i32);
    pub const ProgressRing_TemplateSettings: Self = Self(929i32);
    pub const RangeBase_LargeChange: Self = Self(930i32);
    pub const RangeBase_Maximum: Self = Self(931i32);
    pub const RangeBase_Minimum: Self = Self(932i32);
    pub const RangeBase_SmallChange: Self = Self(933i32);
    pub const RangeBase_Value: Self = Self(934i32);
    pub const Rectangle_RadiusX: Self = Self(935i32);
    pub const Rectangle_RadiusY: Self = Self(936i32);
    pub const RichEditBox_AcceptsReturn: Self = Self(937i32);
    pub const RichEditBox_Header: Self = Self(938i32);
    pub const RichEditBox_HeaderTemplate: Self = Self(939i32);
    pub const RichEditBox_InputScope: Self = Self(940i32);
    pub const RichEditBox_IsColorFontEnabled: Self = Self(941i32);
    pub const RichEditBox_IsReadOnly: Self = Self(942i32);
    pub const RichEditBox_IsSpellCheckEnabled: Self = Self(943i32);
    pub const RichEditBox_IsTextPredictionEnabled: Self = Self(944i32);
    pub const RichEditBox_PlaceholderText: Self = Self(945i32);
    pub const RichEditBox_PreventKeyboardDisplayOnProgrammaticFocus: Self = Self(946i32);
    pub const RichEditBox_SelectionHighlightColor: Self = Self(947i32);
    pub const RichEditBox_TextAlignment: Self = Self(948i32);
    pub const RichEditBox_TextWrapping: Self = Self(949i32);
    pub const SearchBox_ChooseSuggestionOnEnter: Self = Self(950i32);
    pub const SearchBox_FocusOnKeyboardInput: Self = Self(951i32);
    pub const SearchBox_PlaceholderText: Self = Self(952i32);
    pub const SearchBox_QueryText: Self = Self(953i32);
    pub const SearchBox_SearchHistoryContext: Self = Self(954i32);
    pub const SearchBox_SearchHistoryEnabled: Self = Self(955i32);
    pub const SemanticZoom_CanChangeViews: Self = Self(956i32);
    pub const SemanticZoom_IsZoomedInViewActive: Self = Self(957i32);
    pub const SemanticZoom_IsZoomOutButtonEnabled: Self = Self(958i32);
    pub const SemanticZoom_ZoomedInView: Self = Self(959i32);
    pub const SemanticZoom_ZoomedOutView: Self = Self(960i32);
    pub const StackPanel_AreScrollSnapPointsRegular: Self = Self(961i32);
    pub const StackPanel_Orientation: Self = Self(962i32);
    pub const SymbolIcon_Symbol: Self = Self(963i32);
    pub const TextBox_AcceptsReturn: Self = Self(964i32);
    pub const TextBox_Header: Self = Self(965i32);
    pub const TextBox_HeaderTemplate: Self = Self(966i32);
    pub const TextBox_InputScope: Self = Self(967i32);
    pub const TextBox_IsColorFontEnabled: Self = Self(968i32);
    pub const TextBox_IsReadOnly: Self = Self(971i32);
    pub const TextBox_IsSpellCheckEnabled: Self = Self(972i32);
    pub const TextBox_IsTextPredictionEnabled: Self = Self(973i32);
    pub const TextBox_MaxLength: Self = Self(974i32);
    pub const TextBox_PlaceholderText: Self = Self(975i32);
    pub const TextBox_PreventKeyboardDisplayOnProgrammaticFocus: Self = Self(976i32);
    pub const TextBox_SelectedText: Self = Self(977i32);
    pub const TextBox_SelectionHighlightColor: Self = Self(978i32);
    pub const TextBox_SelectionLength: Self = Self(979i32);
    pub const TextBox_SelectionStart: Self = Self(980i32);
    pub const TextBox_Text: Self = Self(981i32);
    pub const TextBox_TextAlignment: Self = Self(982i32);
    pub const TextBox_TextWrapping: Self = Self(983i32);
    pub const Thumb_IsDragging: Self = Self(984i32);
    pub const TickBar_Fill: Self = Self(985i32);
    pub const TimePicker_ClockIdentifier: Self = Self(986i32);
    pub const TimePicker_Header: Self = Self(987i32);
    pub const TimePicker_HeaderTemplate: Self = Self(988i32);
    pub const TimePicker_MinuteIncrement: Self = Self(989i32);
    pub const TimePicker_Time: Self = Self(990i32);
    pub const ToggleSwitch_Header: Self = Self(991i32);
    pub const ToggleSwitch_HeaderTemplate: Self = Self(992i32);
    pub const ToggleSwitch_IsOn: Self = Self(993i32);
    pub const ToggleSwitch_OffContent: Self = Self(994i32);
    pub const ToggleSwitch_OffContentTemplate: Self = Self(995i32);
    pub const ToggleSwitch_OnContent: Self = Self(996i32);
    pub const ToggleSwitch_OnContentTemplate: Self = Self(997i32);
    pub const ToggleSwitch_TemplateSettings: Self = Self(998i32);
    pub const UserControl_Content: Self = Self(999i32);
    pub const VariableSizedWrapGrid_ColumnSpan: Self = Self(1000i32);
    pub const VariableSizedWrapGrid_HorizontalChildrenAlignment: Self = Self(1001i32);
    pub const VariableSizedWrapGrid_ItemHeight: Self = Self(1002i32);
    pub const VariableSizedWrapGrid_ItemWidth: Self = Self(1003i32);
    pub const VariableSizedWrapGrid_MaximumRowsOrColumns: Self = Self(1004i32);
    pub const VariableSizedWrapGrid_Orientation: Self = Self(1005i32);
    pub const VariableSizedWrapGrid_RowSpan: Self = Self(1006i32);
    pub const VariableSizedWrapGrid_VerticalChildrenAlignment: Self = Self(1007i32);
    pub const WebView_AllowedScriptNotifyUris: Self = Self(1008i32);
    pub const WebView_CanGoBack: Self = Self(1009i32);
    pub const WebView_CanGoForward: Self = Self(1010i32);
    pub const WebView_ContainsFullScreenElement: Self = Self(1011i32);
    pub const WebView_DataTransferPackage: Self = Self(1012i32);
    pub const WebView_DefaultBackgroundColor: Self = Self(1013i32);
    pub const WebView_DocumentTitle: Self = Self(1014i32);
    pub const WebView_Source: Self = Self(1015i32);
    pub const AppBar_ClosedDisplayMode: Self = Self(1016i32);
    pub const AppBar_IsOpen: Self = Self(1017i32);
    pub const AppBar_IsSticky: Self = Self(1018i32);
    pub const AutoSuggestBox_AutoMaximizeSuggestionArea: Self = Self(1019i32);
    pub const AutoSuggestBox_Header: Self = Self(1020i32);
    pub const AutoSuggestBox_IsSuggestionListOpen: Self = Self(1021i32);
    pub const AutoSuggestBox_MaxSuggestionListHeight: Self = Self(1022i32);
    pub const AutoSuggestBox_PlaceholderText: Self = Self(1023i32);
    pub const AutoSuggestBox_Text: Self = Self(1024i32);
    pub const AutoSuggestBox_TextBoxStyle: Self = Self(1025i32);
    pub const AutoSuggestBox_TextMemberPath: Self = Self(1026i32);
    pub const AutoSuggestBox_UpdateTextOnSelect: Self = Self(1027i32);
    pub const ButtonBase_ClickMode: Self = Self(1029i32);
    pub const ButtonBase_Command: Self = Self(1030i32);
    pub const ButtonBase_CommandParameter: Self = Self(1031i32);
    pub const ButtonBase_IsPointerOver: Self = Self(1032i32);
    pub const ButtonBase_IsPressed: Self = Self(1033i32);
    pub const ContentDialog_FullSizeDesired: Self = Self(1034i32);
    pub const ContentDialog_IsPrimaryButtonEnabled: Self = Self(1035i32);
    pub const ContentDialog_IsSecondaryButtonEnabled: Self = Self(1036i32);
    pub const ContentDialog_PrimaryButtonCommand: Self = Self(1037i32);
    pub const ContentDialog_PrimaryButtonCommandParameter: Self = Self(1038i32);
    pub const ContentDialog_PrimaryButtonText: Self = Self(1039i32);
    pub const ContentDialog_SecondaryButtonCommand: Self = Self(1040i32);
    pub const ContentDialog_SecondaryButtonCommandParameter: Self = Self(1041i32);
    pub const ContentDialog_SecondaryButtonText: Self = Self(1042i32);
    pub const ContentDialog_Title: Self = Self(1043i32);
    pub const ContentDialog_TitleTemplate: Self = Self(1044i32);
    pub const Frame_BackStack: Self = Self(1045i32);
    pub const Frame_BackStackDepth: Self = Self(1046i32);
    pub const Frame_CacheSize: Self = Self(1047i32);
    pub const Frame_CanGoBack: Self = Self(1048i32);
    pub const Frame_CanGoForward: Self = Self(1049i32);
    pub const Frame_CurrentSourcePageType: Self = Self(1050i32);
    pub const Frame_ForwardStack: Self = Self(1051i32);
    pub const Frame_SourcePageType: Self = Self(1052i32);
    pub const GridViewItemPresenter_CheckBrush: Self = Self(1053i32);
    pub const GridViewItemPresenter_CheckHintBrush: Self = Self(1054i32);
    pub const GridViewItemPresenter_CheckSelectingBrush: Self = Self(1055i32);
    pub const GridViewItemPresenter_ContentMargin: Self = Self(1056i32);
    pub const GridViewItemPresenter_DisabledOpacity: Self = Self(1057i32);
    pub const GridViewItemPresenter_DragBackground: Self = Self(1058i32);
    pub const GridViewItemPresenter_DragForeground: Self = Self(1059i32);
    pub const GridViewItemPresenter_DragOpacity: Self = Self(1060i32);
    pub const GridViewItemPresenter_FocusBorderBrush: Self = Self(1061i32);
    pub const GridViewItemPresenter_GridViewItemPresenterHorizontalContentAlignment: Self = Self(1062i32);
    pub const GridViewItemPresenter_GridViewItemPresenterPadding: Self = Self(1063i32);
    pub const GridViewItemPresenter_PlaceholderBackground: Self = Self(1064i32);
    pub const GridViewItemPresenter_PointerOverBackground: Self = Self(1065i32);
    pub const GridViewItemPresenter_PointerOverBackgroundMargin: Self = Self(1066i32);
    pub const GridViewItemPresenter_ReorderHintOffset: Self = Self(1067i32);
    pub const GridViewItemPresenter_SelectedBackground: Self = Self(1068i32);
    pub const GridViewItemPresenter_SelectedBorderThickness: Self = Self(1069i32);
    pub const GridViewItemPresenter_SelectedForeground: Self = Self(1070i32);
    pub const GridViewItemPresenter_SelectedPointerOverBackground: Self = Self(1071i32);
    pub const GridViewItemPresenter_SelectedPointerOverBorderBrush: Self = Self(1072i32);
    pub const GridViewItemPresenter_SelectionCheckMarkVisualEnabled: Self = Self(1073i32);
    pub const GridViewItemPresenter_GridViewItemPresenterVerticalContentAlignment: Self = Self(1074i32);
    pub const ItemsStackPanel_CacheLength: Self = Self(1076i32);
    pub const ItemsStackPanel_GroupHeaderPlacement: Self = Self(1077i32);
    pub const ItemsStackPanel_GroupPadding: Self = Self(1078i32);
    pub const ItemsStackPanel_ItemsUpdatingScrollMode: Self = Self(1079i32);
    pub const ItemsStackPanel_Orientation: Self = Self(1080i32);
    pub const ItemsWrapGrid_CacheLength: Self = Self(1081i32);
    pub const ItemsWrapGrid_GroupHeaderPlacement: Self = Self(1082i32);
    pub const ItemsWrapGrid_GroupPadding: Self = Self(1083i32);
    pub const ItemsWrapGrid_ItemHeight: Self = Self(1084i32);
    pub const ItemsWrapGrid_ItemWidth: Self = Self(1085i32);
    pub const ItemsWrapGrid_MaximumRowsOrColumns: Self = Self(1086i32);
    pub const ItemsWrapGrid_Orientation: Self = Self(1087i32);
    pub const ListViewItemPresenter_CheckBrush: Self = Self(1088i32);
    pub const ListViewItemPresenter_CheckHintBrush: Self = Self(1089i32);
    pub const ListViewItemPresenter_CheckSelectingBrush: Self = Self(1090i32);
    pub const ListViewItemPresenter_ContentMargin: Self = Self(1091i32);
    pub const ListViewItemPresenter_DisabledOpacity: Self = Self(1092i32);
    pub const ListViewItemPresenter_DragBackground: Self = Self(1093i32);
    pub const ListViewItemPresenter_DragForeground: Self = Self(1094i32);
    pub const ListViewItemPresenter_DragOpacity: Self = Self(1095i32);
    pub const ListViewItemPresenter_FocusBorderBrush: Self = Self(1096i32);
    pub const ListViewItemPresenter_ListViewItemPresenterHorizontalContentAlignment: Self = Self(1097i32);
    pub const ListViewItemPresenter_ListViewItemPresenterPadding: Self = Self(1098i32);
    pub const ListViewItemPresenter_PlaceholderBackground: Self = Self(1099i32);
    pub const ListViewItemPresenter_PointerOverBackground: Self = Self(1100i32);
    pub const ListViewItemPresenter_PointerOverBackgroundMargin: Self = Self(1101i32);
    pub const ListViewItemPresenter_ReorderHintOffset: Self = Self(1102i32);
    pub const ListViewItemPresenter_SelectedBackground: Self = Self(1103i32);
    pub const ListViewItemPresenter_SelectedBorderThickness: Self = Self(1104i32);
    pub const ListViewItemPresenter_SelectedForeground: Self = Self(1105i32);
    pub const ListViewItemPresenter_SelectedPointerOverBackground: Self = Self(1106i32);
    pub const ListViewItemPresenter_SelectedPointerOverBorderBrush: Self = Self(1107i32);
    pub const ListViewItemPresenter_SelectionCheckMarkVisualEnabled: Self = Self(1108i32);
    pub const ListViewItemPresenter_ListViewItemPresenterVerticalContentAlignment: Self = Self(1109i32);
    pub const MenuFlyoutItem_Command: Self = Self(1110i32);
    pub const MenuFlyoutItem_CommandParameter: Self = Self(1111i32);
    pub const MenuFlyoutItem_Text: Self = Self(1112i32);
    pub const Page_BottomAppBar: Self = Self(1114i32);
    pub const Page_Frame: Self = Self(1115i32);
    pub const Page_NavigationCacheMode: Self = Self(1116i32);
    pub const Page_TopAppBar: Self = Self(1117i32);
    pub const ProgressBar_IsIndeterminate: Self = Self(1118i32);
    pub const ProgressBar_ShowError: Self = Self(1119i32);
    pub const ProgressBar_ShowPaused: Self = Self(1120i32);
    pub const ProgressBar_TemplateSettings: Self = Self(1121i32);
    pub const ScrollBar_IndicatorMode: Self = Self(1122i32);
    pub const ScrollBar_Orientation: Self = Self(1123i32);
    pub const ScrollBar_ViewportSize: Self = Self(1124i32);
    pub const Selector_IsSynchronizedWithCurrentItem: Self = Self(1126i32);
    pub const Selector_SelectedIndex: Self = Self(1127i32);
    pub const Selector_SelectedItem: Self = Self(1128i32);
    pub const Selector_SelectedValue: Self = Self(1129i32);
    pub const Selector_SelectedValuePath: Self = Self(1130i32);
    pub const SelectorItem_IsSelected: Self = Self(1131i32);
    pub const SettingsFlyout_HeaderBackground: Self = Self(1132i32);
    pub const SettingsFlyout_HeaderForeground: Self = Self(1133i32);
    pub const SettingsFlyout_IconSource: Self = Self(1134i32);
    pub const SettingsFlyout_TemplateSettings: Self = Self(1135i32);
    pub const SettingsFlyout_Title: Self = Self(1136i32);
    pub const Slider_Header: Self = Self(1137i32);
    pub const Slider_HeaderTemplate: Self = Self(1138i32);
    pub const Slider_IntermediateValue: Self = Self(1139i32);
    pub const Slider_IsDirectionReversed: Self = Self(1140i32);
    pub const Slider_IsThumbToolTipEnabled: Self = Self(1141i32);
    pub const Slider_Orientation: Self = Self(1142i32);
    pub const Slider_SnapsTo: Self = Self(1143i32);
    pub const Slider_StepFrequency: Self = Self(1144i32);
    pub const Slider_ThumbToolTipValueConverter: Self = Self(1145i32);
    pub const Slider_TickFrequency: Self = Self(1146i32);
    pub const Slider_TickPlacement: Self = Self(1147i32);
    pub const SwapChainPanel_CompositionScaleX: Self = Self(1148i32);
    pub const SwapChainPanel_CompositionScaleY: Self = Self(1149i32);
    pub const ToolTip_HorizontalOffset: Self = Self(1150i32);
    pub const ToolTip_IsOpen: Self = Self(1151i32);
    pub const ToolTip_Placement: Self = Self(1152i32);
    pub const ToolTip_PlacementTarget: Self = Self(1153i32);
    pub const ToolTip_TemplateSettings: Self = Self(1154i32);
    pub const ToolTip_VerticalOffset: Self = Self(1155i32);
    pub const Button_Flyout: Self = Self(1156i32);
    pub const ComboBox_Header: Self = Self(1157i32);
    pub const ComboBox_HeaderTemplate: Self = Self(1158i32);
    pub const ComboBox_IsDropDownOpen: Self = Self(1159i32);
    pub const ComboBox_IsEditable: Self = Self(1160i32);
    pub const ComboBox_IsSelectionBoxHighlighted: Self = Self(1161i32);
    pub const ComboBox_MaxDropDownHeight: Self = Self(1162i32);
    pub const ComboBox_PlaceholderText: Self = Self(1163i32);
    pub const ComboBox_SelectionBoxItem: Self = Self(1164i32);
    pub const ComboBox_SelectionBoxItemTemplate: Self = Self(1165i32);
    pub const ComboBox_TemplateSettings: Self = Self(1166i32);
    pub const CommandBar_PrimaryCommands: Self = Self(1167i32);
    pub const CommandBar_SecondaryCommands: Self = Self(1168i32);
    pub const FlipView_UseTouchAnimationsForAllNavigation: Self = Self(1169i32);
    pub const HyperlinkButton_NavigateUri: Self = Self(1170i32);
    pub const ListBox_SelectedItems: Self = Self(1171i32);
    pub const ListBox_SelectionMode: Self = Self(1172i32);
    pub const ListViewBase_CanDragItems: Self = Self(1173i32);
    pub const ListViewBase_CanReorderItems: Self = Self(1174i32);
    pub const ListViewBase_DataFetchSize: Self = Self(1175i32);
    pub const ListViewBase_Footer: Self = Self(1176i32);
    pub const ListViewBase_FooterTemplate: Self = Self(1177i32);
    pub const ListViewBase_FooterTransitions: Self = Self(1178i32);
    pub const ListViewBase_Header: Self = Self(1179i32);
    pub const ListViewBase_HeaderTemplate: Self = Self(1180i32);
    pub const ListViewBase_HeaderTransitions: Self = Self(1181i32);
    pub const ListViewBase_IncrementalLoadingThreshold: Self = Self(1182i32);
    pub const ListViewBase_IncrementalLoadingTrigger: Self = Self(1183i32);
    pub const ListViewBase_IsActiveView: Self = Self(1184i32);
    pub const ListViewBase_IsItemClickEnabled: Self = Self(1185i32);
    pub const ListViewBase_IsSwipeEnabled: Self = Self(1186i32);
    pub const ListViewBase_IsZoomedInView: Self = Self(1187i32);
    pub const ListViewBase_ReorderMode: Self = Self(1188i32);
    pub const ListViewBase_SelectedItems: Self = Self(1189i32);
    pub const ListViewBase_SelectionMode: Self = Self(1190i32);
    pub const ListViewBase_SemanticZoomOwner: Self = Self(1191i32);
    pub const ListViewBase_ShowsScrollingPlaceholders: Self = Self(1192i32);
    pub const RepeatButton_Delay: Self = Self(1193i32);
    pub const RepeatButton_Interval: Self = Self(1194i32);
    pub const ScrollViewer_BringIntoViewOnFocusChange: Self = Self(1195i32);
    pub const ScrollViewer_ComputedHorizontalScrollBarVisibility: Self = Self(1196i32);
    pub const ScrollViewer_ComputedVerticalScrollBarVisibility: Self = Self(1197i32);
    pub const ScrollViewer_ExtentHeight: Self = Self(1198i32);
    pub const ScrollViewer_ExtentWidth: Self = Self(1199i32);
    pub const ScrollViewer_HorizontalOffset: Self = Self(1200i32);
    pub const ScrollViewer_HorizontalScrollBarVisibility: Self = Self(1201i32);
    pub const ScrollViewer_HorizontalScrollMode: Self = Self(1202i32);
    pub const ScrollViewer_HorizontalSnapPointsAlignment: Self = Self(1203i32);
    pub const ScrollViewer_HorizontalSnapPointsType: Self = Self(1204i32);
    pub const ScrollViewer_IsDeferredScrollingEnabled: Self = Self(1205i32);
    pub const ScrollViewer_IsHorizontalRailEnabled: Self = Self(1206i32);
    pub const ScrollViewer_IsHorizontalScrollChainingEnabled: Self = Self(1207i32);
    pub const ScrollViewer_IsScrollInertiaEnabled: Self = Self(1208i32);
    pub const ScrollViewer_IsVerticalRailEnabled: Self = Self(1209i32);
    pub const ScrollViewer_IsVerticalScrollChainingEnabled: Self = Self(1210i32);
    pub const ScrollViewer_IsZoomChainingEnabled: Self = Self(1211i32);
    pub const ScrollViewer_IsZoomInertiaEnabled: Self = Self(1212i32);
    pub const ScrollViewer_LeftHeader: Self = Self(1213i32);
    pub const ScrollViewer_MaxZoomFactor: Self = Self(1214i32);
    pub const ScrollViewer_MinZoomFactor: Self = Self(1215i32);
    pub const ScrollViewer_ScrollableHeight: Self = Self(1216i32);
    pub const ScrollViewer_ScrollableWidth: Self = Self(1217i32);
    pub const ScrollViewer_TopHeader: Self = Self(1218i32);
    pub const ScrollViewer_TopLeftHeader: Self = Self(1219i32);
    pub const ScrollViewer_VerticalOffset: Self = Self(1220i32);
    pub const ScrollViewer_VerticalScrollBarVisibility: Self = Self(1221i32);
    pub const ScrollViewer_VerticalScrollMode: Self = Self(1222i32);
    pub const ScrollViewer_VerticalSnapPointsAlignment: Self = Self(1223i32);
    pub const ScrollViewer_VerticalSnapPointsType: Self = Self(1224i32);
    pub const ScrollViewer_ViewportHeight: Self = Self(1225i32);
    pub const ScrollViewer_ViewportWidth: Self = Self(1226i32);
    pub const ScrollViewer_ZoomFactor: Self = Self(1227i32);
    pub const ScrollViewer_ZoomMode: Self = Self(1228i32);
    pub const ScrollViewer_ZoomSnapPoints: Self = Self(1229i32);
    pub const ScrollViewer_ZoomSnapPointsType: Self = Self(1230i32);
    pub const ToggleButton_IsChecked: Self = Self(1231i32);
    pub const ToggleButton_IsThreeState: Self = Self(1232i32);
    pub const ToggleMenuFlyoutItem_IsChecked: Self = Self(1233i32);
    pub const VirtualizingStackPanel_AreScrollSnapPointsRegular: Self = Self(1234i32);
    pub const VirtualizingStackPanel_IsVirtualizing: Self = Self(1236i32);
    pub const VirtualizingStackPanel_Orientation: Self = Self(1237i32);
    pub const VirtualizingStackPanel_VirtualizationMode: Self = Self(1238i32);
    pub const WrapGrid_HorizontalChildrenAlignment: Self = Self(1239i32);
    pub const WrapGrid_ItemHeight: Self = Self(1240i32);
    pub const WrapGrid_ItemWidth: Self = Self(1241i32);
    pub const WrapGrid_MaximumRowsOrColumns: Self = Self(1242i32);
    pub const WrapGrid_Orientation: Self = Self(1243i32);
    pub const WrapGrid_VerticalChildrenAlignment: Self = Self(1244i32);
    pub const AppBarButton_Icon: Self = Self(1245i32);
    pub const AppBarButton_IsCompact: Self = Self(1246i32);
    pub const AppBarButton_Label: Self = Self(1247i32);
    pub const AppBarToggleButton_Icon: Self = Self(1248i32);
    pub const AppBarToggleButton_IsCompact: Self = Self(1249i32);
    pub const AppBarToggleButton_Label: Self = Self(1250i32);
    pub const GridViewItem_TemplateSettings: Self = Self(1251i32);
    pub const ListViewItem_TemplateSettings: Self = Self(1252i32);
    pub const RadioButton_GroupName: Self = Self(1253i32);
    pub const Glyphs_ColorFontPaletteIndex: Self = Self(1267i32);
    pub const Glyphs_IsColorFontEnabled: Self = Self(1268i32);
    pub const CalendarViewTemplateSettings_HasMoreContentAfter: Self = Self(1274i32);
    pub const CalendarViewTemplateSettings_HasMoreContentBefore: Self = Self(1275i32);
    pub const CalendarViewTemplateSettings_HasMoreViews: Self = Self(1276i32);
    pub const CalendarViewTemplateSettings_HeaderText: Self = Self(1277i32);
    pub const CalendarViewTemplateSettings_WeekDay1: Self = Self(1280i32);
    pub const CalendarViewTemplateSettings_WeekDay2: Self = Self(1281i32);
    pub const CalendarViewTemplateSettings_WeekDay3: Self = Self(1282i32);
    pub const CalendarViewTemplateSettings_WeekDay4: Self = Self(1283i32);
    pub const CalendarViewTemplateSettings_WeekDay5: Self = Self(1284i32);
    pub const CalendarViewTemplateSettings_WeekDay6: Self = Self(1285i32);
    pub const CalendarViewTemplateSettings_WeekDay7: Self = Self(1286i32);
    pub const CalendarView_CalendarIdentifier: Self = Self(1291i32);
    pub const CalendarView_DayOfWeekFormat: Self = Self(1299i32);
    pub const CalendarView_DisplayMode: Self = Self(1302i32);
    pub const CalendarView_FirstDayOfWeek: Self = Self(1303i32);
    pub const CalendarView_IsOutOfScopeEnabled: Self = Self(1317i32);
    pub const CalendarView_IsTodayHighlighted: Self = Self(1318i32);
    pub const CalendarView_MaxDate: Self = Self(1320i32);
    pub const CalendarView_MinDate: Self = Self(1321i32);
    pub const CalendarView_NumberOfWeeksInView: Self = Self(1327i32);
    pub const CalendarView_SelectedDates: Self = Self(1333i32);
    pub const CalendarView_SelectionMode: Self = Self(1335i32);
    pub const CalendarView_TemplateSettings: Self = Self(1336i32);
    pub const CalendarViewDayItem_Date: Self = Self(1339i32);
    pub const CalendarViewDayItem_IsBlackout: Self = Self(1340i32);
    pub const MediaTransportControls_IsFastForwardEnabled: Self = Self(1382i32);
    pub const MediaTransportControls_IsFastRewindEnabled: Self = Self(1383i32);
    pub const MediaTransportControls_IsFullWindowEnabled: Self = Self(1384i32);
    pub const MediaTransportControls_IsPlaybackRateEnabled: Self = Self(1385i32);
    pub const MediaTransportControls_IsSeekEnabled: Self = Self(1386i32);
    pub const MediaTransportControls_IsStopEnabled: Self = Self(1387i32);
    pub const MediaTransportControls_IsVolumeEnabled: Self = Self(1388i32);
    pub const MediaTransportControls_IsZoomEnabled: Self = Self(1389i32);
    pub const ContentPresenter_LineHeight: Self = Self(1425i32);
    pub const CalendarViewTemplateSettings_MinViewWidth: Self = Self(1435i32);
    pub const ListViewBase_SelectedRanges: Self = Self(1459i32);
    pub const SplitViewTemplateSettings_CompactPaneGridLength: Self = Self(1462i32);
    pub const SplitViewTemplateSettings_NegativeOpenPaneLength: Self = Self(1463i32);
    pub const SplitViewTemplateSettings_NegativeOpenPaneLengthMinusCompactLength: Self = Self(1464i32);
    pub const SplitViewTemplateSettings_OpenPaneGridLength: Self = Self(1465i32);
    pub const SplitViewTemplateSettings_OpenPaneLengthMinusCompactLength: Self = Self(1466i32);
    pub const SplitView_CompactPaneLength: Self = Self(1467i32);
    pub const SplitView_Content: Self = Self(1468i32);
    pub const SplitView_DisplayMode: Self = Self(1469i32);
    pub const SplitView_IsPaneOpen: Self = Self(1470i32);
    pub const SplitView_OpenPaneLength: Self = Self(1471i32);
    pub const SplitView_Pane: Self = Self(1472i32);
    pub const SplitView_PanePlacement: Self = Self(1473i32);
    pub const SplitView_TemplateSettings: Self = Self(1474i32);
    pub const UIElement_Transform3D: Self = Self(1475i32);
    pub const CompositeTransform3D_CenterX: Self = Self(1476i32);
    pub const CompositeTransform3D_CenterY: Self = Self(1478i32);
    pub const CompositeTransform3D_CenterZ: Self = Self(1480i32);
    pub const CompositeTransform3D_RotationX: Self = Self(1482i32);
    pub const CompositeTransform3D_RotationY: Self = Self(1484i32);
    pub const CompositeTransform3D_RotationZ: Self = Self(1486i32);
    pub const CompositeTransform3D_ScaleX: Self = Self(1488i32);
    pub const CompositeTransform3D_ScaleY: Self = Self(1490i32);
    pub const CompositeTransform3D_ScaleZ: Self = Self(1492i32);
    pub const CompositeTransform3D_TranslateX: Self = Self(1494i32);
    pub const CompositeTransform3D_TranslateY: Self = Self(1496i32);
    pub const CompositeTransform3D_TranslateZ: Self = Self(1498i32);
    pub const PerspectiveTransform3D_Depth: Self = Self(1500i32);
    pub const PerspectiveTransform3D_OffsetX: Self = Self(1501i32);
    pub const PerspectiveTransform3D_OffsetY: Self = Self(1502i32);
    pub const RelativePanel_Above: Self = Self(1508i32);
    pub const RelativePanel_AlignBottomWith: Self = Self(1509i32);
    pub const RelativePanel_AlignLeftWith: Self = Self(1510i32);
    pub const RelativePanel_AlignRightWith: Self = Self(1515i32);
    pub const RelativePanel_AlignTopWith: Self = Self(1516i32);
    pub const RelativePanel_Below: Self = Self(1517i32);
    pub const RelativePanel_LeftOf: Self = Self(1520i32);
    pub const RelativePanel_RightOf: Self = Self(1521i32);
    pub const SplitViewTemplateSettings_OpenPaneLength: Self = Self(1524i32);
    pub const PasswordBox_PasswordRevealMode: Self = Self(1527i32);
    pub const SplitView_PaneBackground: Self = Self(1528i32);
    pub const ItemsStackPanel_AreStickyGroupHeadersEnabled: Self = Self(1529i32);
    pub const ItemsWrapGrid_AreStickyGroupHeadersEnabled: Self = Self(1530i32);
    pub const MenuFlyoutSubItem_Items: Self = Self(1531i32);
    pub const MenuFlyoutSubItem_Text: Self = Self(1532i32);
    pub const UIElement_CanDrag: Self = Self(1534i32);
    pub const DataTemplate_ExtensionInstance: Self = Self(1535i32);
    pub const RelativePanel_AlignHorizontalCenterWith: Self = Self(1552i32);
    pub const RelativePanel_AlignVerticalCenterWith: Self = Self(1553i32);
    pub const TargetPropertyPath_Path: Self = Self(1555i32);
    pub const TargetPropertyPath_Target: Self = Self(1556i32);
    pub const VisualState_Setters: Self = Self(1558i32);
    pub const VisualState_StateTriggers: Self = Self(1559i32);
    pub const AdaptiveTrigger_MinWindowHeight: Self = Self(1560i32);
    pub const AdaptiveTrigger_MinWindowWidth: Self = Self(1561i32);
    pub const Setter_Target: Self = Self(1562i32);
    pub const CalendarView_BlackoutForeground: Self = Self(1565i32);
    pub const CalendarView_CalendarItemBackground: Self = Self(1566i32);
    pub const CalendarView_CalendarItemBorderBrush: Self = Self(1567i32);
    pub const CalendarView_CalendarItemBorderThickness: Self = Self(1568i32);
    pub const CalendarView_CalendarItemForeground: Self = Self(1569i32);
    pub const CalendarView_CalendarViewDayItemStyle: Self = Self(1570i32);
    pub const CalendarView_DayItemFontFamily: Self = Self(1571i32);
    pub const CalendarView_DayItemFontSize: Self = Self(1572i32);
    pub const CalendarView_DayItemFontStyle: Self = Self(1573i32);
    pub const CalendarView_DayItemFontWeight: Self = Self(1574i32);
    pub const CalendarView_FirstOfMonthLabelFontFamily: Self = Self(1575i32);
    pub const CalendarView_FirstOfMonthLabelFontSize: Self = Self(1576i32);
    pub const CalendarView_FirstOfMonthLabelFontStyle: Self = Self(1577i32);
    pub const CalendarView_FirstOfMonthLabelFontWeight: Self = Self(1578i32);
    pub const CalendarView_FirstOfYearDecadeLabelFontFamily: Self = Self(1579i32);
    pub const CalendarView_FirstOfYearDecadeLabelFontSize: Self = Self(1580i32);
    pub const CalendarView_FirstOfYearDecadeLabelFontStyle: Self = Self(1581i32);
    pub const CalendarView_FirstOfYearDecadeLabelFontWeight: Self = Self(1582i32);
    pub const CalendarView_FocusBorderBrush: Self = Self(1583i32);
    pub const CalendarView_HorizontalDayItemAlignment: Self = Self(1584i32);
    pub const CalendarView_HorizontalFirstOfMonthLabelAlignment: Self = Self(1585i32);
    pub const CalendarView_HoverBorderBrush: Self = Self(1586i32);
    pub const CalendarView_MonthYearItemFontFamily: Self = Self(1588i32);
    pub const CalendarView_MonthYearItemFontSize: Self = Self(1589i32);
    pub const CalendarView_MonthYearItemFontStyle: Self = Self(1590i32);
    pub const CalendarView_MonthYearItemFontWeight: Self = Self(1591i32);
    pub const CalendarView_OutOfScopeBackground: Self = Self(1592i32);
    pub const CalendarView_OutOfScopeForeground: Self = Self(1593i32);
    pub const CalendarView_PressedBorderBrush: Self = Self(1594i32);
    pub const CalendarView_PressedForeground: Self = Self(1595i32);
    pub const CalendarView_SelectedBorderBrush: Self = Self(1596i32);
    pub const CalendarView_SelectedForeground: Self = Self(1597i32);
    pub const CalendarView_SelectedHoverBorderBrush: Self = Self(1598i32);
    pub const CalendarView_SelectedPressedBorderBrush: Self = Self(1599i32);
    pub const CalendarView_TodayFontWeight: Self = Self(1600i32);
    pub const CalendarView_TodayForeground: Self = Self(1601i32);
    pub const CalendarView_VerticalDayItemAlignment: Self = Self(1602i32);
    pub const CalendarView_VerticalFirstOfMonthLabelAlignment: Self = Self(1603i32);
    pub const MediaTransportControls_IsCompact: Self = Self(1605i32);
    pub const RelativePanel_AlignBottomWithPanel: Self = Self(1606i32);
    pub const RelativePanel_AlignHorizontalCenterWithPanel: Self = Self(1607i32);
    pub const RelativePanel_AlignLeftWithPanel: Self = Self(1608i32);
    pub const RelativePanel_AlignRightWithPanel: Self = Self(1609i32);
    pub const RelativePanel_AlignTopWithPanel: Self = Self(1610i32);
    pub const RelativePanel_AlignVerticalCenterWithPanel: Self = Self(1611i32);
    pub const ListViewBase_IsMultiSelectCheckBoxEnabled: Self = Self(1612i32);
    pub const AutomationProperties_Level: Self = Self(1614i32);
    pub const AutomationProperties_PositionInSet: Self = Self(1615i32);
    pub const AutomationProperties_SizeOfSet: Self = Self(1616i32);
    pub const ListViewItemPresenter_CheckBoxBrush: Self = Self(1617i32);
    pub const ListViewItemPresenter_CheckMode: Self = Self(1618i32);
    pub const ListViewItemPresenter_PressedBackground: Self = Self(1620i32);
    pub const ListViewItemPresenter_SelectedPressedBackground: Self = Self(1621i32);
    pub const Control_IsTemplateFocusTarget: Self = Self(1623i32);
    pub const Control_UseSystemFocusVisuals: Self = Self(1624i32);
    pub const ListViewItemPresenter_FocusSecondaryBorderBrush: Self = Self(1628i32);
    pub const ListViewItemPresenter_PointerOverForeground: Self = Self(1630i32);
    pub const FontIcon_MirroredWhenRightToLeft: Self = Self(1631i32);
    pub const CalendarViewTemplateSettings_CenterX: Self = Self(1632i32);
    pub const CalendarViewTemplateSettings_CenterY: Self = Self(1633i32);
    pub const CalendarViewTemplateSettings_ClipRect: Self = Self(1634i32);
    pub const PasswordBox_TextReadingOrder: Self = Self(1650i32);
    pub const RichEditBox_TextReadingOrder: Self = Self(1651i32);
    pub const TextBox_TextReadingOrder: Self = Self(1652i32);
    pub const WebView_ExecutionMode: Self = Self(1653i32);
    pub const WebView_DeferredPermissionRequests: Self = Self(1655i32);
    pub const WebView_Settings: Self = Self(1656i32);
    pub const RichEditBox_DesiredCandidateWindowAlignment: Self = Self(1660i32);
    pub const TextBox_DesiredCandidateWindowAlignment: Self = Self(1662i32);
    pub const CalendarDatePicker_CalendarIdentifier: Self = Self(1663i32);
    pub const CalendarDatePicker_CalendarViewStyle: Self = Self(1664i32);
    pub const CalendarDatePicker_Date: Self = Self(1665i32);
    pub const CalendarDatePicker_DateFormat: Self = Self(1666i32);
    pub const CalendarDatePicker_DayOfWeekFormat: Self = Self(1667i32);
    pub const CalendarDatePicker_DisplayMode: Self = Self(1668i32);
    pub const CalendarDatePicker_FirstDayOfWeek: Self = Self(1669i32);
    pub const CalendarDatePicker_Header: Self = Self(1670i32);
    pub const CalendarDatePicker_HeaderTemplate: Self = Self(1671i32);
    pub const CalendarDatePicker_IsCalendarOpen: Self = Self(1672i32);
    pub const CalendarDatePicker_IsGroupLabelVisible: Self = Self(1673i32);
    pub const CalendarDatePicker_IsOutOfScopeEnabled: Self = Self(1674i32);
    pub const CalendarDatePicker_IsTodayHighlighted: Self = Self(1675i32);
    pub const CalendarDatePicker_MaxDate: Self = Self(1676i32);
    pub const CalendarDatePicker_MinDate: Self = Self(1677i32);
    pub const CalendarDatePicker_PlaceholderText: Self = Self(1678i32);
    pub const CalendarView_IsGroupLabelVisible: Self = Self(1679i32);
    pub const ContentPresenter_Background: Self = Self(1680i32);
    pub const ContentPresenter_BorderBrush: Self = Self(1681i32);
    pub const ContentPresenter_BorderThickness: Self = Self(1682i32);
    pub const ContentPresenter_CornerRadius: Self = Self(1683i32);
    pub const ContentPresenter_Padding: Self = Self(1684i32);
    pub const Grid_BorderBrush: Self = Self(1685i32);
    pub const Grid_BorderThickness: Self = Self(1686i32);
    pub const Grid_CornerRadius: Self = Self(1687i32);
    pub const Grid_Padding: Self = Self(1688i32);
    pub const RelativePanel_BorderBrush: Self = Self(1689i32);
    pub const RelativePanel_BorderThickness: Self = Self(1690i32);
    pub const RelativePanel_CornerRadius: Self = Self(1691i32);
    pub const RelativePanel_Padding: Self = Self(1692i32);
    pub const StackPanel_BorderBrush: Self = Self(1693i32);
    pub const StackPanel_BorderThickness: Self = Self(1694i32);
    pub const StackPanel_CornerRadius: Self = Self(1695i32);
    pub const StackPanel_Padding: Self = Self(1696i32);
    pub const PasswordBox_InputScope: Self = Self(1697i32);
    pub const MediaTransportControlsHelper_DropoutOrder: Self = Self(1698i32);
    pub const AutoSuggestBoxQuerySubmittedEventArgs_ChosenSuggestion: Self = Self(1699i32);
    pub const AutoSuggestBoxQuerySubmittedEventArgs_QueryText: Self = Self(1700i32);
    pub const AutoSuggestBox_QueryIcon: Self = Self(1701i32);
    pub const StateTrigger_IsActive: Self = Self(1702i32);
    pub const ContentPresenter_HorizontalContentAlignment: Self = Self(1703i32);
    pub const ContentPresenter_VerticalContentAlignment: Self = Self(1704i32);
    pub const AppBarTemplateSettings_ClipRect: Self = Self(1705i32);
    pub const AppBarTemplateSettings_CompactRootMargin: Self = Self(1706i32);
    pub const AppBarTemplateSettings_CompactVerticalDelta: Self = Self(1707i32);
    pub const AppBarTemplateSettings_HiddenRootMargin: Self = Self(1708i32);
    pub const AppBarTemplateSettings_HiddenVerticalDelta: Self = Self(1709i32);
    pub const AppBarTemplateSettings_MinimalRootMargin: Self = Self(1710i32);
    pub const AppBarTemplateSettings_MinimalVerticalDelta: Self = Self(1711i32);
    pub const CommandBarTemplateSettings_ContentHeight: Self = Self(1712i32);
    pub const CommandBarTemplateSettings_NegativeOverflowContentHeight: Self = Self(1713i32);
    pub const CommandBarTemplateSettings_OverflowContentClipRect: Self = Self(1714i32);
    pub const CommandBarTemplateSettings_OverflowContentHeight: Self = Self(1715i32);
    pub const CommandBarTemplateSettings_OverflowContentHorizontalOffset: Self = Self(1716i32);
    pub const CommandBarTemplateSettings_OverflowContentMaxHeight: Self = Self(1717i32);
    pub const CommandBarTemplateSettings_OverflowContentMinWidth: Self = Self(1718i32);
    pub const AppBar_TemplateSettings: Self = Self(1719i32);
    pub const CommandBar_CommandBarOverflowPresenterStyle: Self = Self(1720i32);
    pub const CommandBar_CommandBarTemplateSettings: Self = Self(1721i32);
    pub const DrillInThemeAnimation_EntranceTarget: Self = Self(1722i32);
    pub const DrillInThemeAnimation_EntranceTargetName: Self = Self(1723i32);
    pub const DrillInThemeAnimation_ExitTarget: Self = Self(1724i32);
    pub const DrillInThemeAnimation_ExitTargetName: Self = Self(1725i32);
    pub const DrillOutThemeAnimation_EntranceTarget: Self = Self(1726i32);
    pub const DrillOutThemeAnimation_EntranceTargetName: Self = Self(1727i32);
    pub const DrillOutThemeAnimation_ExitTarget: Self = Self(1728i32);
    pub const DrillOutThemeAnimation_ExitTargetName: Self = Self(1729i32);
    pub const XamlBindingHelper_DataTemplateComponent: Self = Self(1730i32);
    pub const AutomationProperties_Annotations: Self = Self(1732i32);
    pub const AutomationAnnotation_Element: Self = Self(1733i32);
    pub const AutomationAnnotation_Type: Self = Self(1734i32);
    pub const AutomationPeerAnnotation_Peer: Self = Self(1735i32);
    pub const AutomationPeerAnnotation_Type: Self = Self(1736i32);
    pub const Hyperlink_UnderlineStyle: Self = Self(1741i32);
    pub const CalendarView_DisabledForeground: Self = Self(1742i32);
    pub const CalendarView_TodayBackground: Self = Self(1743i32);
    pub const CalendarView_TodayBlackoutBackground: Self = Self(1744i32);
    pub const CalendarView_TodaySelectedInnerBorderBrush: Self = Self(1747i32);
    pub const Control_IsFocusEngaged: Self = Self(1749i32);
    pub const Control_IsFocusEngagementEnabled: Self = Self(1752i32);
    pub const RichEditBox_ClipboardCopyFormat: Self = Self(1754i32);
    pub const CommandBarTemplateSettings_OverflowContentMaxWidth: Self = Self(1757i32);
    pub const ComboBoxTemplateSettings_DropDownContentMinWidth: Self = Self(1758i32);
    pub const MenuFlyoutPresenterTemplateSettings_FlyoutContentMinWidth: Self = Self(1762i32);
    pub const MenuFlyoutPresenter_TemplateSettings: Self = Self(1763i32);
    pub const AutomationProperties_LandmarkType: Self = Self(1766i32);
    pub const AutomationProperties_LocalizedLandmarkType: Self = Self(1767i32);
    pub const RepositionThemeTransition_IsStaggeringEnabled: Self = Self(1769i32);
    pub const ListBox_SingleSelectionFollowsFocus: Self = Self(1770i32);
    pub const ListViewBase_SingleSelectionFollowsFocus: Self = Self(1771i32);
    pub const BitmapImage_AutoPlay: Self = Self(1773i32);
    pub const BitmapImage_IsAnimatedBitmap: Self = Self(1774i32);
    pub const BitmapImage_IsPlaying: Self = Self(1775i32);
    pub const AutomationProperties_FullDescription: Self = Self(1776i32);
    pub const AutomationProperties_IsDataValidForForm: Self = Self(1777i32);
    pub const AutomationProperties_IsPeripheral: Self = Self(1778i32);
    pub const AutomationProperties_LocalizedControlType: Self = Self(1779i32);
    pub const FlyoutBase_AllowFocusOnInteraction: Self = Self(1780i32);
    pub const TextElement_AllowFocusOnInteraction: Self = Self(1781i32);
    pub const FrameworkElement_AllowFocusOnInteraction: Self = Self(1782i32);
    pub const Control_RequiresPointer: Self = Self(1783i32);
    pub const UIElement_ContextFlyout: Self = Self(1785i32);
    pub const TextElement_AccessKey: Self = Self(1786i32);
    pub const UIElement_AccessKeyScopeOwner: Self = Self(1787i32);
    pub const UIElement_IsAccessKeyScope: Self = Self(1788i32);
    pub const AutomationProperties_DescribedBy: Self = Self(1790i32);
    pub const UIElement_AccessKey: Self = Self(1803i32);
    pub const Control_XYFocusDown: Self = Self(1804i32);
    pub const Control_XYFocusLeft: Self = Self(1805i32);
    pub const Control_XYFocusRight: Self = Self(1806i32);
    pub const Control_XYFocusUp: Self = Self(1807i32);
    pub const Hyperlink_XYFocusDown: Self = Self(1808i32);
    pub const Hyperlink_XYFocusLeft: Self = Self(1809i32);
    pub const Hyperlink_XYFocusRight: Self = Self(1810i32);
    pub const Hyperlink_XYFocusUp: Self = Self(1811i32);
    pub const WebView_XYFocusDown: Self = Self(1812i32);
    pub const WebView_XYFocusLeft: Self = Self(1813i32);
    pub const WebView_XYFocusRight: Self = Self(1814i32);
    pub const WebView_XYFocusUp: Self = Self(1815i32);
    pub const CommandBarTemplateSettings_EffectiveOverflowButtonVisibility: Self = Self(1816i32);
    pub const AppBarSeparator_IsInOverflow: Self = Self(1817i32);
    pub const CommandBar_DefaultLabelPosition: Self = Self(1818i32);
    pub const CommandBar_IsDynamicOverflowEnabled: Self = Self(1819i32);
    pub const CommandBar_OverflowButtonVisibility: Self = Self(1820i32);
    pub const AppBarButton_IsInOverflow: Self = Self(1821i32);
    pub const AppBarButton_LabelPosition: Self = Self(1822i32);
    pub const AppBarToggleButton_IsInOverflow: Self = Self(1823i32);
    pub const AppBarToggleButton_LabelPosition: Self = Self(1824i32);
    pub const FlyoutBase_LightDismissOverlayMode: Self = Self(1825i32);
    pub const Popup_LightDismissOverlayMode: Self = Self(1827i32);
    pub const CalendarDatePicker_LightDismissOverlayMode: Self = Self(1829i32);
    pub const DatePicker_LightDismissOverlayMode: Self = Self(1830i32);
    pub const SplitView_LightDismissOverlayMode: Self = Self(1831i32);
    pub const TimePicker_LightDismissOverlayMode: Self = Self(1832i32);
    pub const AppBar_LightDismissOverlayMode: Self = Self(1833i32);
    pub const AutoSuggestBox_LightDismissOverlayMode: Self = Self(1834i32);
    pub const ComboBox_LightDismissOverlayMode: Self = Self(1835i32);
    pub const AppBarSeparator_DynamicOverflowOrder: Self = Self(1836i32);
    pub const AppBarButton_DynamicOverflowOrder: Self = Self(1837i32);
    pub const AppBarToggleButton_DynamicOverflowOrder: Self = Self(1838i32);
    pub const FrameworkElement_FocusVisualMargin: Self = Self(1839i32);
    pub const FrameworkElement_FocusVisualPrimaryBrush: Self = Self(1840i32);
    pub const FrameworkElement_FocusVisualPrimaryThickness: Self = Self(1841i32);
    pub const FrameworkElement_FocusVisualSecondaryBrush: Self = Self(1842i32);
    pub const FrameworkElement_FocusVisualSecondaryThickness: Self = Self(1843i32);
    pub const FlyoutBase_AllowFocusWhenDisabled: Self = Self(1846i32);
    pub const FrameworkElement_AllowFocusWhenDisabled: Self = Self(1847i32);
    pub const ComboBox_IsTextSearchEnabled: Self = Self(1848i32);
    pub const TextElement_ExitDisplayModeOnAccessKeyInvoked: Self = Self(1849i32);
    pub const UIElement_ExitDisplayModeOnAccessKeyInvoked: Self = Self(1850i32);
    pub const MediaPlayerPresenter_IsFullWindow: Self = Self(1851i32);
    pub const MediaPlayerPresenter_MediaPlayer: Self = Self(1852i32);
    pub const MediaPlayerPresenter_Stretch: Self = Self(1853i32);
    pub const MediaPlayerElement_AreTransportControlsEnabled: Self = Self(1854i32);
    pub const MediaPlayerElement_AutoPlay: Self = Self(1855i32);
    pub const MediaPlayerElement_IsFullWindow: Self = Self(1856i32);
    pub const MediaPlayerElement_MediaPlayer: Self = Self(1857i32);
    pub const MediaPlayerElement_PosterSource: Self = Self(1858i32);
    pub const MediaPlayerElement_Source: Self = Self(1859i32);
    pub const MediaPlayerElement_Stretch: Self = Self(1860i32);
    pub const MediaPlayerElement_TransportControls: Self = Self(1861i32);
    pub const MediaTransportControls_FastPlayFallbackBehaviour: Self = Self(1862i32);
    pub const MediaTransportControls_IsNextTrackButtonVisible: Self = Self(1863i32);
    pub const MediaTransportControls_IsPreviousTrackButtonVisible: Self = Self(1864i32);
    pub const MediaTransportControls_IsSkipBackwardButtonVisible: Self = Self(1865i32);
    pub const MediaTransportControls_IsSkipBackwardEnabled: Self = Self(1866i32);
    pub const MediaTransportControls_IsSkipForwardButtonVisible: Self = Self(1867i32);
    pub const MediaTransportControls_IsSkipForwardEnabled: Self = Self(1868i32);
    pub const FlyoutBase_ElementSoundMode: Self = Self(1869i32);
    pub const Control_ElementSoundMode: Self = Self(1870i32);
    pub const Hyperlink_ElementSoundMode: Self = Self(1871i32);
    pub const AutomationProperties_FlowsFrom: Self = Self(1876i32);
    pub const AutomationProperties_FlowsTo: Self = Self(1877i32);
    pub const TextElement_TextDecorations: Self = Self(1879i32);
    pub const RichTextBlock_TextDecorations: Self = Self(1881i32);
    pub const Control_DefaultStyleResourceUri: Self = Self(1882i32);
    pub const ContentDialog_PrimaryButtonStyle: Self = Self(1884i32);
    pub const ContentDialog_SecondaryButtonStyle: Self = Self(1885i32);
    pub const TextElement_KeyTipHorizontalOffset: Self = Self(1890i32);
    pub const TextElement_KeyTipPlacementMode: Self = Self(1891i32);
    pub const TextElement_KeyTipVerticalOffset: Self = Self(1892i32);
    pub const UIElement_KeyTipHorizontalOffset: Self = Self(1893i32);
    pub const UIElement_KeyTipPlacementMode: Self = Self(1894i32);
    pub const UIElement_KeyTipVerticalOffset: Self = Self(1895i32);
    pub const FlyoutBase_OverlayInputPassThroughElement: Self = Self(1896i32);
    pub const UIElement_XYFocusKeyboardNavigation: Self = Self(1897i32);
    pub const AutomationProperties_Culture: Self = Self(1898i32);
    pub const UIElement_XYFocusDownNavigationStrategy: Self = Self(1918i32);
    pub const UIElement_XYFocusLeftNavigationStrategy: Self = Self(1919i32);
    pub const UIElement_XYFocusRightNavigationStrategy: Self = Self(1920i32);
    pub const UIElement_XYFocusUpNavigationStrategy: Self = Self(1921i32);
    pub const Hyperlink_XYFocusDownNavigationStrategy: Self = Self(1922i32);
    pub const Hyperlink_XYFocusLeftNavigationStrategy: Self = Self(1923i32);
    pub const Hyperlink_XYFocusRightNavigationStrategy: Self = Self(1924i32);
    pub const Hyperlink_XYFocusUpNavigationStrategy: Self = Self(1925i32);
    pub const TextElement_AccessKeyScopeOwner: Self = Self(1926i32);
    pub const TextElement_IsAccessKeyScope: Self = Self(1927i32);
    pub const Hyperlink_FocusState: Self = Self(1934i32);
    pub const ContentDialog_CloseButtonCommand: Self = Self(1936i32);
    pub const ContentDialog_CloseButtonCommandParameter: Self = Self(1937i32);
    pub const ContentDialog_CloseButtonStyle: Self = Self(1938i32);
    pub const ContentDialog_CloseButtonText: Self = Self(1939i32);
    pub const ContentDialog_DefaultButton: Self = Self(1940i32);
    pub const RichEditBox_SelectionHighlightColorWhenNotFocused: Self = Self(1941i32);
    pub const TextBox_SelectionHighlightColorWhenNotFocused: Self = Self(1942i32);
    pub const SvgImageSource_RasterizePixelHeight: Self = Self(1948i32);
    pub const SvgImageSource_RasterizePixelWidth: Self = Self(1949i32);
    pub const SvgImageSource_UriSource: Self = Self(1950i32);
    pub const LoadedImageSurface_DecodedPhysicalSize: Self = Self(1955i32);
    pub const LoadedImageSurface_DecodedSize: Self = Self(1956i32);
    pub const LoadedImageSurface_NaturalSize: Self = Self(1957i32);
    pub const ComboBox_SelectionChangedTrigger: Self = Self(1958i32);
    pub const XamlCompositionBrushBase_FallbackColor: Self = Self(1960i32);
    pub const UIElement_Lights: Self = Self(1962i32);
    pub const MenuFlyoutItem_Icon: Self = Self(1963i32);
    pub const MenuFlyoutSubItem_Icon: Self = Self(1964i32);
    pub const BitmapIcon_ShowAsMonochrome: Self = Self(1965i32);
    pub const UIElement_HighContrastAdjustment: Self = Self(1967i32);
    pub const RichEditBox_MaxLength: Self = Self(1968i32);
    pub const UIElement_TabFocusNavigation: Self = Self(1969i32);
    pub const Control_IsTemplateKeyTipTarget: Self = Self(1970i32);
    pub const Hyperlink_IsTabStop: Self = Self(1972i32);
    pub const Hyperlink_TabIndex: Self = Self(1973i32);
    pub const MediaTransportControls_IsRepeatButtonVisible: Self = Self(1974i32);
    pub const MediaTransportControls_IsRepeatEnabled: Self = Self(1975i32);
    pub const MediaTransportControls_ShowAndHideAutomatically: Self = Self(1976i32);
    pub const RichEditBox_DisabledFormattingAccelerators: Self = Self(1977i32);
    pub const RichEditBox_CharacterCasing: Self = Self(1978i32);
    pub const TextBox_CharacterCasing: Self = Self(1979i32);
    pub const RichTextBlock_IsTextTrimmed: Self = Self(1980i32);
    pub const RichTextBlockOverflow_IsTextTrimmed: Self = Self(1981i32);
    pub const TextBlock_IsTextTrimmed: Self = Self(1982i32);
    pub const TextHighlighter_Background: Self = Self(1985i32);
    pub const TextHighlighter_Foreground: Self = Self(1986i32);
    pub const TextHighlighter_Ranges: Self = Self(1987i32);
    pub const RichTextBlock_TextHighlighters: Self = Self(1988i32);
    pub const TextBlock_TextHighlighters: Self = Self(1989i32);
    pub const FrameworkElement_ActualTheme: Self = Self(1992i32);
    pub const Grid_ColumnSpacing: Self = Self(1993i32);
    pub const Grid_RowSpacing: Self = Self(1994i32);
    pub const StackPanel_Spacing: Self = Self(1995i32);
    pub const Block_HorizontalTextAlignment: Self = Self(1996i32);
    pub const RichTextBlock_HorizontalTextAlignment: Self = Self(1997i32);
    pub const TextBlock_HorizontalTextAlignment: Self = Self(1998i32);
    pub const RichEditBox_HorizontalTextAlignment: Self = Self(1999i32);
    pub const TextBox_HorizontalTextAlignment: Self = Self(2000i32);
    pub const TextBox_PlaceholderForeground: Self = Self(2001i32);
    pub const ComboBox_PlaceholderForeground: Self = Self(2002i32);
    pub const KeyboardAccelerator_IsEnabled: Self = Self(2003i32);
    pub const KeyboardAccelerator_Key: Self = Self(2004i32);
    pub const KeyboardAccelerator_Modifiers: Self = Self(2005i32);
    pub const KeyboardAccelerator_ScopeOwner: Self = Self(2006i32);
    pub const UIElement_KeyboardAccelerators: Self = Self(2007i32);
    pub const ListViewItemPresenter_RevealBackground: Self = Self(2009i32);
    pub const ListViewItemPresenter_RevealBackgroundShowsAboveContent: Self = Self(2010i32);
    pub const ListViewItemPresenter_RevealBorderBrush: Self = Self(2011i32);
    pub const ListViewItemPresenter_RevealBorderThickness: Self = Self(2012i32);
    pub const UIElement_KeyTipTarget: Self = Self(2014i32);
    pub const AppBarButtonTemplateSettings_KeyboardAcceleratorTextMinWidth: Self = Self(2015i32);
    pub const AppBarToggleButtonTemplateSettings_KeyboardAcceleratorTextMinWidth: Self = Self(2016i32);
    pub const MenuFlyoutItemTemplateSettings_KeyboardAcceleratorTextMinWidth: Self = Self(2017i32);
    pub const MenuFlyoutItem_TemplateSettings: Self = Self(2019i32);
    pub const AppBarButton_TemplateSettings: Self = Self(2021i32);
    pub const AppBarToggleButton_TemplateSettings: Self = Self(2023i32);
    pub const UIElement_KeyboardAcceleratorPlacementMode: Self = Self(2028i32);
    pub const MediaTransportControls_IsCompactOverlayButtonVisible: Self = Self(2032i32);
    pub const MediaTransportControls_IsCompactOverlayEnabled: Self = Self(2033i32);
    pub const UIElement_KeyboardAcceleratorPlacementTarget: Self = Self(2061i32);
    pub const UIElement_CenterPoint: Self = Self(2062i32);
    pub const UIElement_Rotation: Self = Self(2063i32);
    pub const UIElement_RotationAxis: Self = Self(2064i32);
    pub const UIElement_Scale: Self = Self(2065i32);
    pub const UIElement_TransformMatrix: Self = Self(2066i32);
    pub const UIElement_Translation: Self = Self(2067i32);
    pub const TextBox_HandwritingView: Self = Self(2068i32);
    pub const AutomationProperties_HeadingLevel: Self = Self(2069i32);
    pub const TextBox_IsHandwritingViewEnabled: Self = Self(2076i32);
    pub const RichEditBox_ContentLinkProviders: Self = Self(2078i32);
    pub const RichEditBox_ContentLinkBackgroundColor: Self = Self(2079i32);
    pub const RichEditBox_ContentLinkForegroundColor: Self = Self(2080i32);
    pub const HandwritingView_AreCandidatesEnabled: Self = Self(2081i32);
    pub const HandwritingView_IsOpen: Self = Self(2082i32);
    pub const HandwritingView_PlacementTarget: Self = Self(2084i32);
    pub const HandwritingView_PlacementAlignment: Self = Self(2085i32);
    pub const RichEditBox_HandwritingView: Self = Self(2086i32);
    pub const RichEditBox_IsHandwritingViewEnabled: Self = Self(2087i32);
    pub const MenuFlyoutItem_KeyboardAcceleratorTextOverride: Self = Self(2090i32);
    pub const AppBarButton_KeyboardAcceleratorTextOverride: Self = Self(2091i32);
    pub const AppBarToggleButton_KeyboardAcceleratorTextOverride: Self = Self(2092i32);
    pub const ContentLink_Background: Self = Self(2093i32);
    pub const ContentLink_Cursor: Self = Self(2094i32);
    pub const ContentLink_ElementSoundMode: Self = Self(2095i32);
    pub const ContentLink_FocusState: Self = Self(2096i32);
    pub const ContentLink_IsTabStop: Self = Self(2097i32);
    pub const ContentLink_TabIndex: Self = Self(2098i32);
    pub const ContentLink_XYFocusDown: Self = Self(2099i32);
    pub const ContentLink_XYFocusDownNavigationStrategy: Self = Self(2100i32);
    pub const ContentLink_XYFocusLeft: Self = Self(2101i32);
    pub const ContentLink_XYFocusLeftNavigationStrategy: Self = Self(2102i32);
    pub const ContentLink_XYFocusRight: Self = Self(2103i32);
    pub const ContentLink_XYFocusRightNavigationStrategy: Self = Self(2104i32);
    pub const ContentLink_XYFocusUp: Self = Self(2105i32);
    pub const ContentLink_XYFocusUpNavigationStrategy: Self = Self(2106i32);
    pub const IconSource_Foreground: Self = Self(2112i32);
    pub const BitmapIconSource_ShowAsMonochrome: Self = Self(2113i32);
    pub const BitmapIconSource_UriSource: Self = Self(2114i32);
    pub const FontIconSource_FontFamily: Self = Self(2115i32);
    pub const FontIconSource_FontSize: Self = Self(2116i32);
    pub const FontIconSource_FontStyle: Self = Self(2117i32);
    pub const FontIconSource_FontWeight: Self = Self(2118i32);
    pub const FontIconSource_Glyph: Self = Self(2119i32);
    pub const FontIconSource_IsTextScaleFactorEnabled: Self = Self(2120i32);
    pub const FontIconSource_MirroredWhenRightToLeft: Self = Self(2121i32);
    pub const PathIconSource_Data: Self = Self(2122i32);
    pub const SymbolIconSource_Symbol: Self = Self(2123i32);
    pub const UIElement_Shadow: Self = Self(2130i32);
    pub const IconSourceElement_IconSource: Self = Self(2131i32);
    pub const PasswordBox_CanPasteClipboardContent: Self = Self(2137i32);
    pub const TextBox_CanPasteClipboardContent: Self = Self(2138i32);
    pub const TextBox_CanRedo: Self = Self(2139i32);
    pub const TextBox_CanUndo: Self = Self(2140i32);
    pub const FlyoutBase_ShowMode: Self = Self(2141i32);
    pub const FlyoutBase_Target: Self = Self(2142i32);
    pub const Control_CornerRadius: Self = Self(2143i32);
    pub const AutomationProperties_IsDialog: Self = Self(2149i32);
    pub const AppBarElementContainer_DynamicOverflowOrder: Self = Self(2150i32);
    pub const AppBarElementContainer_IsCompact: Self = Self(2151i32);
    pub const AppBarElementContainer_IsInOverflow: Self = Self(2152i32);
    pub const ScrollContentPresenter_CanContentRenderOutsideBounds: Self = Self(2157i32);
    pub const ScrollViewer_CanContentRenderOutsideBounds: Self = Self(2158i32);
    pub const RichEditBox_SelectionFlyout: Self = Self(2159i32);
    pub const TextBox_SelectionFlyout: Self = Self(2160i32);
    pub const Border_BackgroundSizing: Self = Self(2161i32);
    pub const ContentPresenter_BackgroundSizing: Self = Self(2162i32);
    pub const Control_BackgroundSizing: Self = Self(2163i32);
    pub const Grid_BackgroundSizing: Self = Self(2164i32);
    pub const RelativePanel_BackgroundSizing: Self = Self(2165i32);
    pub const StackPanel_BackgroundSizing: Self = Self(2166i32);
    pub const ScrollViewer_HorizontalAnchorRatio: Self = Self(2170i32);
    pub const ScrollViewer_VerticalAnchorRatio: Self = Self(2171i32);
    pub const ComboBox_Text: Self = Self(2208i32);
    pub const TextBox_Description: Self = Self(2217i32);
    pub const ToolTip_PlacementRect: Self = Self(2218i32);
    pub const RichTextBlock_SelectionFlyout: Self = Self(2219i32);
    pub const TextBlock_SelectionFlyout: Self = Self(2220i32);
    pub const PasswordBox_SelectionFlyout: Self = Self(2221i32);
    pub const Border_BackgroundTransition: Self = Self(2222i32);
    pub const ContentPresenter_BackgroundTransition: Self = Self(2223i32);
    pub const Panel_BackgroundTransition: Self = Self(2224i32);
    pub const ColorPaletteResources_Accent: Self = Self(2227i32);
    pub const ColorPaletteResources_AltHigh: Self = Self(2228i32);
    pub const ColorPaletteResources_AltLow: Self = Self(2229i32);
    pub const ColorPaletteResources_AltMedium: Self = Self(2230i32);
    pub const ColorPaletteResources_AltMediumHigh: Self = Self(2231i32);
    pub const ColorPaletteResources_AltMediumLow: Self = Self(2232i32);
    pub const ColorPaletteResources_BaseHigh: Self = Self(2233i32);
    pub const ColorPaletteResources_BaseLow: Self = Self(2234i32);
    pub const ColorPaletteResources_BaseMedium: Self = Self(2235i32);
    pub const ColorPaletteResources_BaseMediumHigh: Self = Self(2236i32);
    pub const ColorPaletteResources_BaseMediumLow: Self = Self(2237i32);
    pub const ColorPaletteResources_ChromeAltLow: Self = Self(2238i32);
    pub const ColorPaletteResources_ChromeBlackHigh: Self = Self(2239i32);
    pub const ColorPaletteResources_ChromeBlackLow: Self = Self(2240i32);
    pub const ColorPaletteResources_ChromeBlackMedium: Self = Self(2241i32);
    pub const ColorPaletteResources_ChromeBlackMediumLow: Self = Self(2242i32);
    pub const ColorPaletteResources_ChromeDisabledHigh: Self = Self(2243i32);
    pub const ColorPaletteResources_ChromeDisabledLow: Self = Self(2244i32);
    pub const ColorPaletteResources_ChromeGray: Self = Self(2245i32);
    pub const ColorPaletteResources_ChromeHigh: Self = Self(2246i32);
    pub const ColorPaletteResources_ChromeLow: Self = Self(2247i32);
    pub const ColorPaletteResources_ChromeMedium: Self = Self(2248i32);
    pub const ColorPaletteResources_ChromeMediumLow: Self = Self(2249i32);
    pub const ColorPaletteResources_ChromeWhite: Self = Self(2250i32);
    pub const ColorPaletteResources_ErrorText: Self = Self(2252i32);
    pub const ColorPaletteResources_ListLow: Self = Self(2253i32);
    pub const ColorPaletteResources_ListMedium: Self = Self(2254i32);
    pub const UIElement_TranslationTransition: Self = Self(2255i32);
    pub const UIElement_OpacityTransition: Self = Self(2256i32);
    pub const UIElement_RotationTransition: Self = Self(2257i32);
    pub const UIElement_ScaleTransition: Self = Self(2258i32);
    pub const BrushTransition_Duration: Self = Self(2261i32);
    pub const ScalarTransition_Duration: Self = Self(2262i32);
    pub const Vector3Transition_Duration: Self = Self(2263i32);
    pub const Vector3Transition_Components: Self = Self(2266i32);
    pub const FlyoutBase_IsOpen: Self = Self(2267i32);
    pub const StandardUICommand_Kind: Self = Self(2275i32);
    pub const UIElement_CanBeScrollAnchor: Self = Self(2276i32);
    pub const ThemeShadow_Receivers: Self = Self(2279i32);
    pub const ScrollContentPresenter_SizesContentToTemplatedParent: Self = Self(2280i32);
    pub const ComboBox_TextBoxStyle: Self = Self(2281i32);
    pub const Frame_IsNavigationStackEnabled: Self = Self(2282i32);
    pub const RichEditBox_ProofingMenuFlyout: Self = Self(2283i32);
    pub const TextBox_ProofingMenuFlyout: Self = Self(2284i32);
    pub const ScrollViewer_ReduceViewportForCoreInputViewOcclusions: Self = Self(2295i32);
    pub const FlyoutBase_AreOpenCloseAnimationsEnabled: Self = Self(2296i32);
    pub const FlyoutBase_InputDevicePrefersPrimaryCommands: Self = Self(2297i32);
    pub const CalendarDatePicker_Description: Self = Self(2300i32);
    pub const PasswordBox_Description: Self = Self(2308i32);
    pub const RichEditBox_Description: Self = Self(2316i32);
    pub const AutoSuggestBox_Description: Self = Self(2331i32);
    pub const ComboBox_Description: Self = Self(2339i32);
    pub const XamlUICommand_AccessKey: Self = Self(2347i32);
    pub const XamlUICommand_Command: Self = Self(2348i32);
    pub const XamlUICommand_Description: Self = Self(2349i32);
    pub const XamlUICommand_IconSource: Self = Self(2350i32);
    pub const XamlUICommand_KeyboardAccelerators: Self = Self(2351i32);
    pub const XamlUICommand_Label: Self = Self(2352i32);
    pub const DatePicker_SelectedDate: Self = Self(2355i32);
    pub const TimePicker_SelectedTime: Self = Self(2356i32);
    pub const AppBarTemplateSettings_NegativeCompactVerticalDelta: Self = Self(2367i32);
    pub const AppBarTemplateSettings_NegativeHiddenVerticalDelta: Self = Self(2368i32);
    pub const AppBarTemplateSettings_NegativeMinimalVerticalDelta: Self = Self(2369i32);
    pub const FlyoutBase_ShouldConstrainToRootBounds: Self = Self(2378i32);
    pub const Popup_ShouldConstrainToRootBounds: Self = Self(2379i32);
    pub const FlyoutPresenter_IsDefaultShadowEnabled: Self = Self(2380i32);
    pub const MenuFlyoutPresenter_IsDefaultShadowEnabled: Self = Self(2381i32);
    pub const UIElement_ActualOffset: Self = Self(2382i32);
    pub const UIElement_ActualSize: Self = Self(2383i32);
    pub const CommandBarTemplateSettings_OverflowContentCompactYTranslation: Self = Self(2384i32);
    pub const CommandBarTemplateSettings_OverflowContentHiddenYTranslation: Self = Self(2385i32);
    pub const CommandBarTemplateSettings_OverflowContentMinimalYTranslation: Self = Self(2386i32);
    pub const HandwritingView_IsCommandBarOpen: Self = Self(2395i32);
    pub const HandwritingView_IsSwitchToKeyboardEnabled: Self = Self(2396i32);
    pub const ListViewItemPresenter_SelectionIndicatorVisualEnabled: Self = Self(2399i32);
    pub const ListViewItemPresenter_SelectionIndicatorBrush: Self = Self(2400i32);
    pub const ListViewItemPresenter_SelectionIndicatorMode: Self = Self(2401i32);
    pub const ListViewItemPresenter_SelectionIndicatorPointerOverBrush: Self = Self(2402i32);
    pub const ListViewItemPresenter_SelectionIndicatorPressedBrush: Self = Self(2403i32);
    pub const ListViewItemPresenter_SelectedBorderBrush: Self = Self(2410i32);
    pub const ListViewItemPresenter_SelectedInnerBorderBrush: Self = Self(2411i32);
    pub const ListViewItemPresenter_CheckBoxCornerRadius: Self = Self(2412i32);
    pub const ListViewItemPresenter_SelectionIndicatorCornerRadius: Self = Self(2413i32);
    pub const ListViewItemPresenter_SelectedDisabledBorderBrush: Self = Self(2414i32);
    pub const ListViewItemPresenter_SelectedPressedBorderBrush: Self = Self(2415i32);
    pub const ListViewItemPresenter_SelectedDisabledBackground: Self = Self(2416i32);
    pub const ListViewItemPresenter_PointerOverBorderBrush: Self = Self(2417i32);
    pub const ListViewItemPresenter_CheckBoxPointerOverBrush: Self = Self(2418i32);
    pub const ListViewItemPresenter_CheckBoxPressedBrush: Self = Self(2419i32);
    pub const ListViewItemPresenter_CheckDisabledBrush: Self = Self(2420i32);
    pub const ListViewItemPresenter_CheckPressedBrush: Self = Self(2421i32);
    pub const ListViewItemPresenter_CheckBoxBorderBrush: Self = Self(2422i32);
    pub const ListViewItemPresenter_CheckBoxDisabledBorderBrush: Self = Self(2423i32);
    pub const ListViewItemPresenter_CheckBoxPressedBorderBrush: Self = Self(2424i32);
    pub const ListViewItemPresenter_CheckBoxDisabledBrush: Self = Self(2425i32);
    pub const ListViewItemPresenter_CheckBoxSelectedBrush: Self = Self(2426i32);
    pub const ListViewItemPresenter_CheckBoxSelectedDisabledBrush: Self = Self(2427i32);
    pub const ListViewItemPresenter_CheckBoxSelectedPointerOverBrush: Self = Self(2428i32);
    pub const ListViewItemPresenter_CheckBoxSelectedPressedBrush: Self = Self(2429i32);
    pub const ListViewItemPresenter_CheckBoxPointerOverBorderBrush: Self = Self(2430i32);
    pub const ListViewItemPresenter_SelectionIndicatorDisabledBrush: Self = Self(2431i32);
    pub const CalendarView_BlackoutBackground: Self = Self(2432i32);
    pub const CalendarView_BlackoutStrikethroughBrush: Self = Self(2433i32);
    pub const CalendarView_CalendarItemCornerRadius: Self = Self(2434i32);
    pub const CalendarView_CalendarItemDisabledBackground: Self = Self(2435i32);
    pub const CalendarView_CalendarItemHoverBackground: Self = Self(2436i32);
    pub const CalendarView_CalendarItemPressedBackground: Self = Self(2437i32);
    pub const CalendarView_DayItemMargin: Self = Self(2438i32);
    pub const CalendarView_FirstOfMonthLabelMargin: Self = Self(2439i32);
    pub const CalendarView_FirstOfYearDecadeLabelMargin: Self = Self(2440i32);
    pub const CalendarView_MonthYearItemMargin: Self = Self(2441i32);
    pub const CalendarView_OutOfScopeHoverForeground: Self = Self(2442i32);
    pub const CalendarView_OutOfScopePressedForeground: Self = Self(2443i32);
    pub const CalendarView_SelectedDisabledBorderBrush: Self = Self(2444i32);
    pub const CalendarView_SelectedDisabledForeground: Self = Self(2445i32);
    pub const CalendarView_SelectedHoverForeground: Self = Self(2446i32);
    pub const CalendarView_SelectedPressedForeground: Self = Self(2447i32);
    pub const CalendarView_TodayBlackoutForeground: Self = Self(2448i32);
    pub const CalendarView_TodayDisabledBackground: Self = Self(2449i32);
    pub const CalendarView_TodayHoverBackground: Self = Self(2450i32);
    pub const CalendarView_TodayPressedBackground: Self = Self(2451i32);
    pub const Popup_ActualPlacement: Self = Self(2452i32);
    pub const Popup_DesiredPlacement: Self = Self(2453i32);
    pub const Popup_PlacementTarget: Self = Self(2454i32);
    pub const AutomationProperties_AutomationControlType: Self = Self(2455i32);
}
impl ::core::marker::Copy for XamlPropertyIndex {}
impl ::core::clone::Clone for XamlPropertyIndex {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XamlPropertyIndex {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XamlPropertyIndex {
    type Abi = Self;
}
impl ::core::fmt::Debug for XamlPropertyIndex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlPropertyIndex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlPropertyIndex {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Core.Direct.XamlPropertyIndex;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Core_Direct\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XamlTypeIndex(pub i32);
impl XamlTypeIndex {
    pub const AutoSuggestBoxSuggestionChosenEventArgs: Self = Self(34i32);
    pub const AutoSuggestBoxTextChangedEventArgs: Self = Self(35i32);
    pub const CollectionViewSource: Self = Self(41i32);
    pub const ColumnDefinition: Self = Self(44i32);
    pub const GradientStop: Self = Self(64i32);
    pub const InputScope: Self = Self(74i32);
    pub const InputScopeName: Self = Self(75i32);
    pub const KeySpline: Self = Self(78i32);
    pub const PathFigure: Self = Self(93i32);
    pub const PrintDocument: Self = Self(100i32);
    pub const RowDefinition: Self = Self(106i32);
    pub const Style: Self = Self(114i32);
    pub const TimelineMarker: Self = Self(126i32);
    pub const VisualState: Self = Self(137i32);
    pub const VisualStateGroup: Self = Self(138i32);
    pub const VisualStateManager: Self = Self(139i32);
    pub const VisualTransition: Self = Self(140i32);
    pub const AddDeleteThemeTransition: Self = Self(177i32);
    pub const ArcSegment: Self = Self(178i32);
    pub const BackEase: Self = Self(179i32);
    pub const BeginStoryboard: Self = Self(180i32);
    pub const BezierSegment: Self = Self(181i32);
    pub const BindingBase: Self = Self(182i32);
    pub const BitmapCache: Self = Self(183i32);
    pub const BounceEase: Self = Self(186i32);
    pub const CircleEase: Self = Self(187i32);
    pub const ColorAnimation: Self = Self(188i32);
    pub const ColorAnimationUsingKeyFrames: Self = Self(189i32);
    pub const ContentThemeTransition: Self = Self(190i32);
    pub const ControlTemplate: Self = Self(191i32);
    pub const CubicEase: Self = Self(192i32);
    pub const DataTemplate: Self = Self(194i32);
    pub const DiscreteColorKeyFrame: Self = Self(195i32);
    pub const DiscreteDoubleKeyFrame: Self = Self(196i32);
    pub const DiscreteObjectKeyFrame: Self = Self(197i32);
    pub const DiscretePointKeyFrame: Self = Self(198i32);
    pub const DoubleAnimation: Self = Self(200i32);
    pub const DoubleAnimationUsingKeyFrames: Self = Self(201i32);
    pub const EasingColorKeyFrame: Self = Self(204i32);
    pub const EasingDoubleKeyFrame: Self = Self(205i32);
    pub const EasingPointKeyFrame: Self = Self(206i32);
    pub const EdgeUIThemeTransition: Self = Self(207i32);
    pub const ElasticEase: Self = Self(208i32);
    pub const EllipseGeometry: Self = Self(209i32);
    pub const EntranceThemeTransition: Self = Self(210i32);
    pub const EventTrigger: Self = Self(211i32);
    pub const ExponentialEase: Self = Self(212i32);
    pub const Flyout: Self = Self(213i32);
    pub const GeometryGroup: Self = Self(216i32);
    pub const ItemsPanelTemplate: Self = Self(227i32);
    pub const LinearColorKeyFrame: Self = Self(230i32);
    pub const LinearDoubleKeyFrame: Self = Self(231i32);
    pub const LinearPointKeyFrame: Self = Self(232i32);
    pub const LineGeometry: Self = Self(233i32);
    pub const LineSegment: Self = Self(234i32);
    pub const Matrix3DProjection: Self = Self(236i32);
    pub const MenuFlyout: Self = Self(238i32);
    pub const ObjectAnimationUsingKeyFrames: Self = Self(240i32);
    pub const PaneThemeTransition: Self = Self(241i32);
    pub const PathGeometry: Self = Self(243i32);
    pub const PlaneProjection: Self = Self(244i32);
    pub const PointAnimation: Self = Self(245i32);
    pub const PointAnimationUsingKeyFrames: Self = Self(246i32);
    pub const PolyBezierSegment: Self = Self(248i32);
    pub const PolyLineSegment: Self = Self(249i32);
    pub const PolyQuadraticBezierSegment: Self = Self(250i32);
    pub const PopupThemeTransition: Self = Self(251i32);
    pub const PowerEase: Self = Self(252i32);
    pub const QuadraticBezierSegment: Self = Self(254i32);
    pub const QuadraticEase: Self = Self(255i32);
    pub const QuarticEase: Self = Self(256i32);
    pub const QuinticEase: Self = Self(257i32);
    pub const RectangleGeometry: Self = Self(258i32);
    pub const RelativeSource: Self = Self(259i32);
    pub const RenderTargetBitmap: Self = Self(260i32);
    pub const ReorderThemeTransition: Self = Self(261i32);
    pub const RepositionThemeTransition: Self = Self(262i32);
    pub const Setter: Self = Self(263i32);
    pub const SineEase: Self = Self(264i32);
    pub const SolidColorBrush: Self = Self(265i32);
    pub const SplineColorKeyFrame: Self = Self(266i32);
    pub const SplineDoubleKeyFrame: Self = Self(267i32);
    pub const SplinePointKeyFrame: Self = Self(268i32);
    pub const BitmapImage: Self = Self(285i32);
    pub const Border: Self = Self(286i32);
    pub const CaptureElement: Self = Self(288i32);
    pub const CompositeTransform: Self = Self(295i32);
    pub const ContentPresenter: Self = Self(296i32);
    pub const DragItemThemeAnimation: Self = Self(302i32);
    pub const DragOverThemeAnimation: Self = Self(303i32);
    pub const DropTargetItemThemeAnimation: Self = Self(304i32);
    pub const FadeInThemeAnimation: Self = Self(306i32);
    pub const FadeOutThemeAnimation: Self = Self(307i32);
    pub const Glyphs: Self = Self(312i32);
    pub const Image: Self = Self(326i32);
    pub const ImageBrush: Self = Self(328i32);
    pub const InlineUIContainer: Self = Self(329i32);
    pub const ItemsPresenter: Self = Self(332i32);
    pub const LinearGradientBrush: Self = Self(334i32);
    pub const LineBreak: Self = Self(335i32);
    pub const MatrixTransform: Self = Self(340i32);
    pub const MediaElement: Self = Self(342i32);
    pub const Paragraph: Self = Self(349i32);
    pub const PointerDownThemeAnimation: Self = Self(357i32);
    pub const PointerUpThemeAnimation: Self = Self(359i32);
    pub const PopInThemeAnimation: Self = Self(361i32);
    pub const PopOutThemeAnimation: Self = Self(362i32);
    pub const Popup: Self = Self(363i32);
    pub const RepositionThemeAnimation: Self = Self(370i32);
    pub const ResourceDictionary: Self = Self(371i32);
    pub const RichTextBlock: Self = Self(374i32);
    pub const RichTextBlockOverflow: Self = Self(376i32);
    pub const RotateTransform: Self = Self(378i32);
    pub const Run: Self = Self(380i32);
    pub const ScaleTransform: Self = Self(381i32);
    pub const SkewTransform: Self = Self(389i32);
    pub const Span: Self = Self(390i32);
    pub const SplitCloseThemeAnimation: Self = Self(391i32);
    pub const SplitOpenThemeAnimation: Self = Self(392i32);
    pub const Storyboard: Self = Self(393i32);
    pub const SwipeBackThemeAnimation: Self = Self(394i32);
    pub const SwipeHintThemeAnimation: Self = Self(395i32);
    pub const TextBlock: Self = Self(396i32);
    pub const TransformGroup: Self = Self(411i32);
    pub const TranslateTransform: Self = Self(413i32);
    pub const Viewbox: Self = Self(417i32);
    pub const WebViewBrush: Self = Self(423i32);
    pub const AppBarSeparator: Self = Self(427i32);
    pub const BitmapIcon: Self = Self(429i32);
    pub const Bold: Self = Self(430i32);
    pub const Canvas: Self = Self(432i32);
    pub const ContentControl: Self = Self(435i32);
    pub const DatePicker: Self = Self(436i32);
    pub const DependencyObjectCollection: Self = Self(437i32);
    pub const Ellipse: Self = Self(438i32);
    pub const FontIcon: Self = Self(440i32);
    pub const Grid: Self = Self(442i32);
    pub const Hub: Self = Self(445i32);
    pub const HubSection: Self = Self(446i32);
    pub const Hyperlink: Self = Self(447i32);
    pub const Italic: Self = Self(449i32);
    pub const ItemsControl: Self = Self(451i32);
    pub const Line: Self = Self(452i32);
    pub const MediaTransportControls: Self = Self(458i32);
    pub const PasswordBox: Self = Self(462i32);
    pub const Path: Self = Self(463i32);
    pub const PathIcon: Self = Self(464i32);
    pub const Polygon: Self = Self(465i32);
    pub const Polyline: Self = Self(466i32);
    pub const ProgressRing: Self = Self(468i32);
    pub const Rectangle: Self = Self(470i32);
    pub const RichEditBox: Self = Self(473i32);
    pub const ScrollContentPresenter: Self = Self(476i32);
    pub const SearchBox: Self = Self(477i32);
    pub const SemanticZoom: Self = Self(479i32);
    pub const StackPanel: Self = Self(481i32);
    pub const SymbolIcon: Self = Self(482i32);
    pub const TextBox: Self = Self(483i32);
    pub const Thumb: Self = Self(485i32);
    pub const TickBar: Self = Self(486i32);
    pub const TimePicker: Self = Self(487i32);
    pub const ToggleSwitch: Self = Self(489i32);
    pub const Underline: Self = Self(490i32);
    pub const UserControl: Self = Self(491i32);
    pub const VariableSizedWrapGrid: Self = Self(492i32);
    pub const WebView: Self = Self(494i32);
    pub const AppBar: Self = Self(495i32);
    pub const AutoSuggestBox: Self = Self(499i32);
    pub const CarouselPanel: Self = Self(502i32);
    pub const ContentDialog: Self = Self(506i32);
    pub const FlyoutPresenter: Self = Self(508i32);
    pub const Frame: Self = Self(509i32);
    pub const GridViewItemPresenter: Self = Self(511i32);
    pub const GroupItem: Self = Self(512i32);
    pub const ItemsStackPanel: Self = Self(514i32);
    pub const ItemsWrapGrid: Self = Self(515i32);
    pub const ListViewItemPresenter: Self = Self(520i32);
    pub const MenuFlyoutItem: Self = Self(521i32);
    pub const MenuFlyoutPresenter: Self = Self(522i32);
    pub const MenuFlyoutSeparator: Self = Self(523i32);
    pub const Page: Self = Self(525i32);
    pub const ProgressBar: Self = Self(528i32);
    pub const ScrollBar: Self = Self(530i32);
    pub const SettingsFlyout: Self = Self(533i32);
    pub const Slider: Self = Self(534i32);
    pub const SwapChainBackgroundPanel: Self = Self(535i32);
    pub const SwapChainPanel: Self = Self(536i32);
    pub const ToolTip: Self = Self(538i32);
    pub const Button: Self = Self(540i32);
    pub const ComboBoxItem: Self = Self(541i32);
    pub const CommandBar: Self = Self(542i32);
    pub const FlipViewItem: Self = Self(543i32);
    pub const GridViewHeaderItem: Self = Self(545i32);
    pub const HyperlinkButton: Self = Self(546i32);
    pub const ListBoxItem: Self = Self(547i32);
    pub const ListViewHeaderItem: Self = Self(550i32);
    pub const RepeatButton: Self = Self(551i32);
    pub const ScrollViewer: Self = Self(552i32);
    pub const ToggleButton: Self = Self(553i32);
    pub const ToggleMenuFlyoutItem: Self = Self(554i32);
    pub const VirtualizingStackPanel: Self = Self(555i32);
    pub const WrapGrid: Self = Self(556i32);
    pub const AppBarButton: Self = Self(557i32);
    pub const AppBarToggleButton: Self = Self(558i32);
    pub const CheckBox: Self = Self(559i32);
    pub const GridViewItem: Self = Self(560i32);
    pub const ListViewItem: Self = Self(561i32);
    pub const RadioButton: Self = Self(562i32);
    pub const Binding: Self = Self(564i32);
    pub const ComboBox: Self = Self(566i32);
    pub const FlipView: Self = Self(567i32);
    pub const ListBox: Self = Self(568i32);
    pub const GridView: Self = Self(570i32);
    pub const ListView: Self = Self(571i32);
    pub const CalendarView: Self = Self(707i32);
    pub const CalendarViewDayItem: Self = Self(709i32);
    pub const CalendarPanel: Self = Self(723i32);
    pub const SplitView: Self = Self(728i32);
    pub const CompositeTransform3D: Self = Self(732i32);
    pub const PerspectiveTransform3D: Self = Self(733i32);
    pub const RelativePanel: Self = Self(744i32);
    pub const InkCanvas: Self = Self(748i32);
    pub const MenuFlyoutSubItem: Self = Self(749i32);
    pub const AdaptiveTrigger: Self = Self(757i32);
    pub const SoftwareBitmapSource: Self = Self(761i32);
    pub const StateTrigger: Self = Self(767i32);
    pub const CalendarDatePicker: Self = Self(774i32);
    pub const AutoSuggestBoxQuerySubmittedEventArgs: Self = Self(778i32);
    pub const CommandBarOverflowPresenter: Self = Self(781i32);
    pub const DrillInThemeAnimation: Self = Self(782i32);
    pub const DrillOutThemeAnimation: Self = Self(783i32);
    pub const AutomationAnnotation: Self = Self(789i32);
    pub const AutomationPeerAnnotation: Self = Self(790i32);
    pub const MediaPlayerPresenter: Self = Self(828i32);
    pub const MediaPlayerElement: Self = Self(829i32);
    pub const XamlLight: Self = Self(855i32);
    pub const SvgImageSource: Self = Self(860i32);
    pub const KeyboardAccelerator: Self = Self(897i32);
    pub const HandwritingView: Self = Self(920i32);
    pub const ContentLink: Self = Self(925i32);
    pub const BitmapIconSource: Self = Self(929i32);
    pub const FontIconSource: Self = Self(930i32);
    pub const PathIconSource: Self = Self(931i32);
    pub const SymbolIconSource: Self = Self(933i32);
    pub const IconSourceElement: Self = Self(939i32);
    pub const AppBarElementContainer: Self = Self(945i32);
    pub const ColorPaletteResources: Self = Self(952i32);
    pub const StandardUICommand: Self = Self(961i32);
    pub const ThemeShadow: Self = Self(964i32);
    pub const XamlUICommand: Self = Self(969i32);
}
impl ::core::marker::Copy for XamlTypeIndex {}
impl ::core::clone::Clone for XamlTypeIndex {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XamlTypeIndex {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XamlTypeIndex {
    type Abi = Self;
}
impl ::core::fmt::Debug for XamlTypeIndex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlTypeIndex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlTypeIndex {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Core.Direct.XamlTypeIndex;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");