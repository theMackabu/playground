macro_rules! common_header {
	 ($(#[$attrs:meta])*($id:ident, $name:expr) => ($item:ty)*) => {
		  $(#[$attrs])*
		  #[derive(Debug, Clone, PartialEq, Eq, ::derive_more::Deref, ::derive_more::DerefMut)]
		  pub struct $id(pub Vec<$item>);

		  impl $crate::http::header::Header for $id {
				#[inline]
				fn name() -> $crate::http::header::HeaderName {
					 $name
				}
		  }

		  impl ::core::fmt::Display for $id {
				#[inline]
				fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					 $crate::http::header::fmt_comma_delimited(f, &self.0[..])
				}
		  }

		  impl $crate::http::header::TryIntoHeaderValue for $id {
				type Error = $crate::http::header::InvalidHeaderValue;

				#[inline]
				fn try_into_value(self) -> Result<$crate::http::header::HeaderValue, Self::Error> {
					 use ::core::fmt::Write;
					 let mut writer = $crate::http::header::Writer::new();
					 let _ = write!(&mut writer, "{}", self);
					 $crate::http::header::HeaderValue::from_maybe_shared(writer.take())
				}
		  }
	 };

	 ($(#[$attrs:meta])*($id:ident, $name:expr) => ($item:ty)+) => {
		  $(#[$attrs])*
		  #[derive(Debug, Clone, PartialEq, Eq, ::derive_more::Deref, ::derive_more::DerefMut)]
		  pub struct $id(pub Vec<$item>);

		  impl $crate::http::header::Header for $id {
				#[inline]
				fn name() -> $crate::http::header::HeaderName {
					 $name
				}
		  }

		  impl ::core::fmt::Display for $id {
				#[inline]
				fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					 $crate::http::header::fmt_comma_delimited(f, &self.0[..])
				}
		  }

		  impl $crate::http::header::TryIntoHeaderValue for $id {
				type Error = $crate::http::header::InvalidHeaderValue;

				#[inline]
				fn try_into_value(self) -> Result<$crate::http::header::HeaderValue, Self::Error> {
					 use ::core::fmt::Write;
					 let mut writer = $crate::http::header::Writer::new();
					 let _ = write!(&mut writer, "{}", self);
					 $crate::http::header::HeaderValue::from_maybe_shared(writer.take())
				}
		  }
	 };

	 ($(#[$attrs:meta])*($id:ident, $name:expr) => [$value:ty]) => {
		  $(#[$attrs])*
		  #[derive(Debug, Clone, PartialEq, Eq, ::derive_more::Deref, ::derive_more::DerefMut)]
		  pub struct $id(pub $value);

		  impl ::core::fmt::Display for $id {
				#[inline]
				fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					 ::core::fmt::Display::fmt(&self.0, f)
				}
		  }
	 };

	 ($(#[$attrs:meta])*($id:ident, $name:expr) => {Any / ($item:ty)+}) => {
		  $(#[$attrs])*
		  #[derive(Clone, Debug, PartialEq, Eq)]
		  pub enum $id {
				Any,
				Items(Vec<$item>),
		  }

		  impl $crate::http::header::Header for $id {
				#[inline]
				fn name() -> $crate::http::header::HeaderName {
					 $name
				}
		  }

		  impl ::core::fmt::Display for $id {
				#[inline]
				fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					 match *self {
						  $id::Any => f.write_str("*"),
						  $id::Items(ref fields) =>
								$crate::http::header::fmt_comma_delimited(f, &fields[..])
					 }
				}
		  }

		  impl $crate::http::header::TryIntoHeaderValue for $id {
				type Error = $crate::http::header::InvalidHeaderValue;

				#[inline]
				fn try_into_value(self) -> Result<$crate::http::header::HeaderValue, Self::Error> {
					 use ::core::fmt::Write;
					 let mut writer = $crate::http::header::Writer::new();
					 let _ = write!(&mut writer, "{}", self);
					 $crate::http::header::HeaderValue::from_maybe_shared(writer.take())
				}
		  }
	 };

	 // optional test module
	 ($(#[$attrs:meta])*($id:ident, $name:expr) => ($item:ty)* $tm:ident{$($tf:item)*}) => {
		  crate::http::header::common_header! {
				$(#[$attrs])*
				($id, $name) => ($item)*
		  }

		  crate::http::header::common_header_test_module! { $id, $tm { $($tf)* }}
	 };
	 ($(#[$attrs:meta])*($id:ident, $n:expr) => ($item:ty)+ $tm:ident{$($tf:item)*}) => {
		  crate::http::header::common_header! {
				$(#[$attrs])*
				($id, $n) => ($item)+
		  }

		  crate::http::header::common_header_test_module! { $id, $tm { $($tf)* }}
	 };
	 ($(#[$attrs:meta])*($id:ident, $name:expr) => [$item:ty] $tm:ident{$($tf:item)*}) => {
		  crate::http::header::common_header! {
				$(#[$attrs])* ($id, $name) => [$item]
		  }

		  crate::http::header::common_header_test_module! { $id, $tm { $($tf)* }}
	 };
	 ($(#[$attrs:meta])*($id:ident, $name:expr) => {Any / ($item:ty)+} $tm:ident{$($tf:item)*}) => {
		  crate::http::header::common_header! {
				$(#[$attrs])*
				($id, $name) => {Any / ($item)+}
		  }

		  crate::http::header::common_header_test_module! { $id, $tm { $($tf)* }}
	 };
}

pub(crate) use common_header;
