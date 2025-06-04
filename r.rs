#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
pub mod winrt {
    use std::sync::Arc;
    use windows::{
        core::{Result, HSTRING, PWSTR},
        ApplicationModel::Package, Foundation::Uri,
        Management::Deployment::{DeploymentOptions, PackageManager},
        Win32::{
            Foundation::HANDLE,
            Security::{
                Authorization::ConvertSidToStringSidW, GetTokenInformation, TokenUser,
                TOKEN_QUERY, TOKEN_USER,
            },
            System::Threading::{GetCurrentProcess, OpenProcessToken},
        },
    };
    pub mod metadata {
        use std::{fs::File, io::Read, path::Path, sync::Arc};
        use tokio::task::spawn_blocking;
        use windows::ApplicationModel::PackageVersion;
        use windows_core::Result as Win32Result;
        use zip::read::ZipArchive;
        use serde::{Serialize, Deserialize};
        use serde_xml_rs::from_str;
        pub struct Bundle {
            #[serde(rename = "Identity")]
            pub identity: Identity,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Bundle {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Bundle",
                    "identity",
                    &&self.identity,
                )
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Bundle {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Bundle",
                        false as usize + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "Identity",
                        &self.identity,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Bundle {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Identity" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Identity" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Bundle>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Bundle;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Bundle",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                Identity,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Bundle with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Bundle { identity: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Identity> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "Identity",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Identity>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("Identity")?
                                }
                            };
                            _serde::__private::Ok(Bundle { identity: __field0 })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["Identity"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Bundle",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Bundle>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Identity {
            #[serde(rename = "@Name")]
            pub name: String,
            #[serde(rename = "@Version")]
            pub version: String,
            #[serde(rename = "@Publisher")]
            pub publisher: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Identity {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Identity",
                    "name",
                    &self.name,
                    "version",
                    &self.version,
                    "publisher",
                    &&self.publisher,
                )
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Identity {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Identity",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "@Name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "@Version",
                        &self.version,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "@Publisher",
                        &self.publisher,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Identity {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "@Name" => _serde::__private::Ok(__Field::__field0),
                                "@Version" => _serde::__private::Ok(__Field::__field1),
                                "@Publisher" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"@Name" => _serde::__private::Ok(__Field::__field0),
                                b"@Version" => _serde::__private::Ok(__Field::__field1),
                                b"@Publisher" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Identity>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Identity;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Identity",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Identity with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Identity with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Identity with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Identity {
                                name: __field0,
                                version: __field1,
                                publisher: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("@Name"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "@Version",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "@Publisher",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("@Name")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("@Version")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("@Publisher")?
                                }
                            };
                            _serde::__private::Ok(Identity {
                                name: __field0,
                                version: __field1,
                                publisher: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "@Name",
                        "@Version",
                        "@Publisher",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Identity",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Identity>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        use super::MSIXPackageManager;
        #[non_exhaustive]
        pub struct MsixBundle {
            pub path: String,
            pub identity: Identity,
            pub full_name: Option<String>,
            pub manager: Arc<MSIXPackageManager>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MsixBundle {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "MsixBundle",
                    "path",
                    &self.path,
                    "identity",
                    &self.identity,
                    "full_name",
                    &self.full_name,
                    "manager",
                    &&self.manager,
                )
            }
        }
        impl AsRef<str> for MsixBundle {
            fn as_ref(&self) -> &str {
                &self.path
            }
        }
        pub type MsixBundleResult<T> = Result<T, MsixBundleError>;
        pub enum MsixBundleError {
            JoinError(tokio::task::JoinError),
            TokioIO(tokio::io::Error),
            ZipError(zip::result::ZipError),
            Serde(serde_xml_rs::Error),
            Win32(windows_core::Error),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MsixBundleError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    MsixBundleError::JoinError(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "JoinError",
                            &__self_0,
                        )
                    }
                    MsixBundleError::TokioIO(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "TokioIO",
                            &__self_0,
                        )
                    }
                    MsixBundleError::ZipError(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ZipError",
                            &__self_0,
                        )
                    }
                    MsixBundleError::Serde(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Serde",
                            &__self_0,
                        )
                    }
                    MsixBundleError::Win32(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Win32",
                            &__self_0,
                        )
                    }
                }
            }
        }
        impl From<windows_core::Error> for MsixBundleError {
            fn from(value: windows_core::Error) -> Self {
                MsixBundleError::Win32(value)
            }
        }
        impl From<serde_xml_rs::Error> for MsixBundleError {
            fn from(value: serde_xml_rs::Error) -> Self {
                Self::Serde(value)
            }
        }
        impl From<tokio::task::JoinError> for MsixBundleError {
            fn from(value: tokio::task::JoinError) -> Self {
                Self::JoinError(value)
            }
        }
        impl From<zip::result::ZipError> for MsixBundleError {
            fn from(value: zip::result::ZipError) -> Self {
                Self::ZipError(value)
            }
        }
        impl From<tokio::io::Error> for MsixBundleError {
            fn from(value: tokio::io::Error) -> Self {
                Self::TokioIO(value)
            }
        }
        impl MsixBundle {
            pub async fn load<T: AsRef<Path>>(
                path: T,
                manager: &Arc<MSIXPackageManager>,
            ) -> MsixBundleResult<Self> {
                let path = tokio::fs::canonicalize(path).await?;
                let path = path.to_str().unwrap_or("");
                let path = path.get(4..).unwrap_or("");
                let path = path.to_string();
                let manager = manager.clone();
                spawn_blocking(move || {
                        let path = path;
                        let file = File::open(&path)?;
                        let mut archive = ZipArchive::new(file)?;
                        let mut string = String::new();
                        archive
                            .by_name("AppxMetadata/AppxBundleManifest.xml")?
                            .read_to_string(&mut string)?;
                        let bundle: Bundle = from_str(&string)?;
                        let mut bundle = MsixBundle {
                            path,
                            identity: bundle.identity,
                            full_name: None,
                            manager,
                        };
                        bundle.reload_install_status()?;
                        MsixBundleResult::Ok(bundle)
                    })
                    .await?
            }
            pub fn reload_install_status(&mut self) -> MsixBundleResult<()> {
                let identity = &self.identity;
                let info = self
                    .manager
                    .get_intalled_info_sync(&identity.name, &identity.publisher)?;
                let pkg = info
                    .into_iter()
                    .find(|x| {
                        (|| {
                            let author = x.Id()?.Publisher()?;
                            let name = x.Id()?.Name()?;
                            let PackageVersion { Build, Major, Minor, Revision } = x
                                .Id()?
                                .Version()?;
                            let version = ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "{0}.{1}.{2}.{3}",
                                        Major,
                                        Minor,
                                        Build,
                                        Revision,
                                    ),
                                );
                                res
                            });
                            Win32Result::Ok(
                                &identity.name == &name && &identity.publisher == &author
                                    && &identity.version == &version,
                            )
                        })()
                            .unwrap_or(false)
                    });
                if let Some(pkg) = pkg {
                    let name = pkg.Id()?.FullName()?;
                    let name = name.to_string_lossy();
                    self.full_name = Some(name);
                }
                Ok(())
            }
            pub async fn install(&self) -> Win32Result<()> {
                self.manager.install(&self).await
            }
            pub fn is_installed(&mut self) -> MsixBundleResult<bool> {
                self.reload_install_status()?;
                Ok(self.full_name.is_some())
            }
            /// Use [MsixBundle::async_is_installed] instead
            ///
            /// ***SAFETY***
            ///
            /// Await this as soon as you call this function
            ///
            /// THIS FUNCTION USES `UNSAFE` CASTING TO MARK THE VARIABLE AS &'static
            #[deprecated(
                since = "0.1.0",
                note = "This method will not be removed! Use `async_is_installed` instead, as it provides a safe, idiomatic alternative by transferring ownership. This function relies on `unsafe` and strict caller discipline to avoid undefined behavior."
            )]
            pub async unsafe fn async_unsafe_is_installed<'a>(
                &'a mut self,
            ) -> Result<bool, MsixBundleError> {
                let me: &'static mut MsixBundle = unsafe {
                    &mut *(self as *mut _) as &'static mut _
                };
                let is_installed = spawn_blocking(|| { me.is_installed() }).await??;
                Ok(is_installed)
            }
            #[allow(unused_mut)]
            pub async fn async_is_installed(
                mut self,
            ) -> Result<(Self, bool), MsixBundleError> {
                let mut me = self;
                let is_installed = spawn_blocking(move || {
                        let result = me.is_installed()?;
                        MsixBundleResult::Ok((me, result))
                    })
                    .await??;
                Ok(is_installed)
            }
            pub async fn uninstall(&self) -> Win32Result<()> {
                let full_name = self
                    .full_name
                    .as_ref()
                    .map_or_else(|| "", |x| x.as_str());
                self.manager.remove(full_name).await
            }
        }
    }
    pub struct MSIXPackageManager(PackageManager);
    #[automatically_derived]
    impl ::core::fmt::Debug for MSIXPackageManager {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "MSIXPackageManager",
                &&self.0,
            )
        }
    }
    pub fn get_user_sid_string() -> Result<HSTRING> {
        unsafe {
            let mut token_handle: HANDLE = HANDLE::default();
            OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle)?;
            let mut len = 300;
            let mut info = ::alloc::vec::from_elem(0u8, len as usize);
            GetTokenInformation(
                token_handle,
                TokenUser,
                Some(info.as_mut_ptr() as _),
                len,
                &mut len,
            )?;
            let mut sid = PWSTR::default();
            let info = &mut info[0usize..(len as usize)];
            let info = info.as_mut_ptr() as *mut TOKEN_USER;
            let val = (&mut *info).User.Sid;
            ConvertSidToStringSidW(val, &mut sid)?;
            Ok(sid.to_hstring())
        }
    }
    impl MSIXPackageManager {
        pub fn new() -> Result<Arc<Self>> {
            Ok(Arc::new(Self(PackageManager::new()?)))
        }
        pub async fn install<T: AsRef<str>>(&self, path: T) -> Result<()> {
            let path = path.as_ref();
            let path = HSTRING::from(path);
            let uri = Uri::CreateUri(&path)?;
            let prog = self
                .0
                .AddPackageAsync(&uri, None, DeploymentOptions::InstallAllResources)?;
            let result = prog.await?;
            result.ExtendedErrorCode()?.ok()
        }
        pub async fn remove<T: AsRef<str>>(&self, full_name: T) -> Result<()> {
            let full_name = full_name.as_ref();
            let full_name = HSTRING::from(full_name);
            let result = self.0.RemovePackageAsync(&full_name)?.await?;
            result.ExtendedErrorCode()?.ok()
        }
        pub fn get_intalled_info_sync<T: AsRef<str>, E: AsRef<str>>(
            &self,
            app_name: T,
            publisher: E,
        ) -> Result<Vec<Package>> {
            let pkg = self
                .0
                .FindPackagesByUserSecurityIdNamePublisher(
                    &get_user_sid_string()?,
                    &HSTRING::from(app_name.as_ref()),
                    &HSTRING::from(publisher.as_ref()),
                )?;
            Ok(pkg.into_iter().collect::<Vec<_>>())
        }
    }
}
pub mod msi {
    use std::fs;
    use windows::{
        core::w,
        Win32::System::ApplicationInstallationAndServicing::{
            MsiCloseHandle, MsiDatabaseOpenViewW, MsiInstallProductW, MsiOpenDatabaseW,
            MsiQueryProductStateW, MsiRecordGetStringW, MsiViewExecute, MsiViewFetch,
            INSTALLSTATE_ABSENT, INSTALLSTATE_ADVERTISED, INSTALLSTATE_DEFAULT,
            INSTALLSTATE_UNKNOWN, MSIDBOPEN_READONLY, MSIHANDLE,
        },
    };
    use windows_core::{HRESULT, HSTRING, PCWSTR, PWSTR, Result};
    /// This is an MSI Package with a `ProductCode`
    pub struct MsiPackage(String, String);
    pub enum ProductState {
        NotInstalled = -1,
        AdvertisedButNotInstalled = 1,
        InstalledForDifferentUser = 2,
        Installed = 5,
        Unknown,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ProductState {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ProductState::NotInstalled => "NotInstalled",
                    ProductState::AdvertisedButNotInstalled => {
                        "AdvertisedButNotInstalled"
                    }
                    ProductState::InstalledForDifferentUser => {
                        "InstalledForDifferentUser"
                    }
                    ProductState::Installed => "Installed",
                    ProductState::Unknown => "Unknown",
                },
            )
        }
    }
    impl MsiPackage {
        pub unsafe fn new_from_product_code<E: Into<String>, T: Into<String>>(
            path: E,
            product_code: T,
        ) -> Result<MsiPackage> {
            Ok(MsiPackage(path.into(), product_code.into()))
        }
        pub fn product_state(&self) -> ProductState {
            unsafe {
                let data = PCWSTR::from_raw(
                    HSTRING::from(self.1.as_ref() as &str).as_ptr(),
                );
                match MsiQueryProductStateW(data) {
                    INSTALLSTATE_ABSENT => ProductState::InstalledForDifferentUser,
                    INSTALLSTATE_ADVERTISED => ProductState::AdvertisedButNotInstalled,
                    INSTALLSTATE_DEFAULT => ProductState::Installed,
                    INSTALLSTATE_UNKNOWN => ProductState::NotInstalled,
                    _ => ProductState::Unknown,
                }
            }
        }
        pub fn is_installed(&self) -> bool {
            match self.product_state() {
                ProductState::Installed => true,
                _ => false,
            }
        }
        pub fn install(&self) -> Result<()> {
            unsafe {
                let hstring = HSTRING::from(self.0.as_ref() as &str);
                let path = PCWSTR::from_raw(hstring.as_ptr());
                let res = MsiInstallProductW(
                    path,
                    {
                        const INPUT: &[u8] = "ACTION=INSTALL UILevel=3".as_bytes();
                        const OUTPUT_LEN: usize = ::windows_strings::utf16_len(INPUT)
                            + 1;
                        const OUTPUT: &[u16; OUTPUT_LEN] = {
                            let mut buffer = [0; OUTPUT_LEN];
                            let mut input_pos = 0;
                            let mut output_pos = 0;
                            while let Some((mut code_point, new_pos)) = ::windows_strings::decode_utf8_char(
                                INPUT,
                                input_pos,
                            ) {
                                input_pos = new_pos;
                                if code_point <= 0xffff {
                                    buffer[output_pos] = code_point as u16;
                                    output_pos += 1;
                                } else {
                                    code_point -= 0x10000;
                                    buffer[output_pos] = 0xd800 + (code_point >> 10) as u16;
                                    output_pos += 1;
                                    buffer[output_pos] = 0xdc00 + (code_point & 0x3ff) as u16;
                                    output_pos += 1;
                                }
                            }
                            &{ buffer }
                        };
                        ::windows_strings::PCWSTR::from_raw(OUTPUT.as_ptr())
                    },
                );
                HRESULT::from_win32(res).ok()?;
                Ok(())
            }
        }
        pub fn uninstall(&self) -> Result<()> {
            unsafe {
                let hstring = HSTRING::from(self.0.as_ref() as &str);
                let path = PCWSTR::from_raw(hstring.as_ptr());
                let res = MsiInstallProductW(
                    path,
                    {
                        const INPUT: &[u8] = "REMOVE=ALL UILevel=3".as_bytes();
                        const OUTPUT_LEN: usize = ::windows_strings::utf16_len(INPUT)
                            + 1;
                        const OUTPUT: &[u16; OUTPUT_LEN] = {
                            let mut buffer = [0; OUTPUT_LEN];
                            let mut input_pos = 0;
                            let mut output_pos = 0;
                            while let Some((mut code_point, new_pos)) = ::windows_strings::decode_utf8_char(
                                INPUT,
                                input_pos,
                            ) {
                                input_pos = new_pos;
                                if code_point <= 0xffff {
                                    buffer[output_pos] = code_point as u16;
                                    output_pos += 1;
                                } else {
                                    code_point -= 0x10000;
                                    buffer[output_pos] = 0xd800 + (code_point >> 10) as u16;
                                    output_pos += 1;
                                    buffer[output_pos] = 0xdc00 + (code_point & 0x3ff) as u16;
                                    output_pos += 1;
                                }
                            }
                            &{ buffer }
                        };
                        ::windows_strings::PCWSTR::from_raw(OUTPUT.as_ptr())
                    },
                );
                HRESULT::from_win32(res).ok()?;
                Ok(())
            }
        }
        /// Creates a new MsiPackage by opening the specified MSI file,
        /// extracting its ProductCode, and then closing the database.
        ///
        /// # Arguments
        /// * `name` - The relativ/absolute path to the MSI file.
        ///
        /// # Returns
        /// A `Result` containing an `MsiPackage` with the ProductCode on success,
        /// or a `windows_core::Error` on failure.
        pub fn new<T: AsRef<str>>(name: T) -> Result<MsiPackage> {
            let mut hwnd = MSIHANDLE::default();
            Ok(unsafe {
                let name = name.as_ref();
                let fetch = fs::canonicalize(name)?;
                let name = fetch.to_string_lossy();
                let name = name.as_ref().get(4..).unwrap_or(r"\\?\");
                let name_return = name.to_string();
                let name = HSTRING::from(name);
                let name = PCWSTR::from_raw(name.as_ptr());
                let err = MsiOpenDatabaseW(name, MSIDBOPEN_READONLY, &mut hwnd);
                HRESULT::from_win32(err).ok()?;
                let mut view = MSIHANDLE::default();
                HRESULT::from_win32(
                        MsiDatabaseOpenViewW(
                            hwnd,
                            {
                                const INPUT: &[u8] = "SELECT `Value` FROM `Property` WHERE `Property` = 'ProductCode'"
                                    .as_bytes();
                                const OUTPUT_LEN: usize = ::windows_strings::utf16_len(
                                    INPUT,
                                ) + 1;
                                const OUTPUT: &[u16; OUTPUT_LEN] = {
                                    let mut buffer = [0; OUTPUT_LEN];
                                    let mut input_pos = 0;
                                    let mut output_pos = 0;
                                    while let Some((mut code_point, new_pos)) = ::windows_strings::decode_utf8_char(
                                        INPUT,
                                        input_pos,
                                    ) {
                                        input_pos = new_pos;
                                        if code_point <= 0xffff {
                                            buffer[output_pos] = code_point as u16;
                                            output_pos += 1;
                                        } else {
                                            code_point -= 0x10000;
                                            buffer[output_pos] = 0xd800 + (code_point >> 10) as u16;
                                            output_pos += 1;
                                            buffer[output_pos] = 0xdc00 + (code_point & 0x3ff) as u16;
                                            output_pos += 1;
                                        }
                                    }
                                    &{ buffer }
                                };
                                ::windows_strings::PCWSTR::from_raw(OUTPUT.as_ptr())
                            },
                            &mut view,
                        ),
                    )
                    .ok()?;
                HRESULT::from_win32(MsiViewExecute(view, MSIHANDLE::default())).ok()?;
                let mut record = MSIHANDLE::default();
                let phrecord = &mut record as *mut MSIHANDLE;
                HRESULT::from_win32(MsiViewFetch(view, phrecord)).ok()?;
                let mut string = [0u16; 39];
                HRESULT::from_win32(
                        MsiRecordGetStringW(
                            record,
                            1,
                            Some(PWSTR::from_raw(&mut string as *mut _ as _)),
                            Some(&mut 39),
                        ),
                    )
                    .ok()?;
                let data = HSTRING::from_wide(&string).to_string_lossy();
                HRESULT::from_win32(MsiCloseHandle(hwnd)).ok()?;
                MsiPackage(name_return, data)
            })
        }
    }
}
pub mod exe {}
pub mod zip {}
pub mod av {
    use windows::{core::w, Win32::System::Antimalware::{AmsiInitialize, HAMSICONTEXT}};
    pub fn test() {
        let ctx: HAMSICONTEXT = unsafe {
            AmsiInitialize({
                    const INPUT: &[u8] = "Windortent".as_bytes();
                    const OUTPUT_LEN: usize = ::windows_strings::utf16_len(INPUT) + 1;
                    const OUTPUT: &[u16; OUTPUT_LEN] = {
                        let mut buffer = [0; OUTPUT_LEN];
                        let mut input_pos = 0;
                        let mut output_pos = 0;
                        while let Some((mut code_point, new_pos)) = ::windows_strings::decode_utf8_char(
                            INPUT,
                            input_pos,
                        ) {
                            input_pos = new_pos;
                            if code_point <= 0xffff {
                                buffer[output_pos] = code_point as u16;
                                output_pos += 1;
                            } else {
                                code_point -= 0x10000;
                                buffer[output_pos] = 0xd800 + (code_point >> 10) as u16;
                                output_pos += 1;
                                buffer[output_pos] = 0xdc00 + (code_point & 0x3ff) as u16;
                                output_pos += 1;
                            }
                        }
                        &{ buffer }
                    };
                    ::windows_strings::PCWSTR::from_raw(OUTPUT.as_ptr())
                })
                .unwrap()
        };
    }
}
pub use windows;
pub use windows::ApplicationModel::Package;
pub type ApplicationPackage = Package;
