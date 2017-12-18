pub use ffi_types::align::*;
pub use ffi_types::dimension::*;
pub use ffi_types::direction::*;
pub use ffi_types::display::*;
pub use ffi_types::edge::*;
pub use ffi_types::flex_direction::*;
pub use ffi_types::justify::*;
pub use ffi_types::log_level::*;
pub use ffi_types::measure_mode::*;
pub use ffi_types::node_ref::*;
pub use ffi_types::node_type::*;
pub use ffi_types::overflow::*;
pub use ffi_types::position_type::*;
pub use ffi_types::print_options::*;
pub use ffi_types::size::*;
pub use ffi_types::style_unit::*;
pub use ffi_types::undefined::*;
pub use ffi_types::wrap::*;

use ordered_float::OrderedFloat;
use std::any::Any;
use std::ops::Deref;

pub type BaselineFunc = Option<extern "C" fn(NodeRef, f32, f32) -> f32>;
pub type MeasureFunc = Option<extern "C" fn(NodeRef, f32, MeasureMode, f32, MeasureMode) -> Size>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FlexStyle {
	AlignContent(Align),
	AlignItems(Align),
	AlignSelf(Align),
	AspectRatio(OrderedFloat<f32>),
	BorderBottom(OrderedFloat<f32>),
	BorderEnd(OrderedFloat<f32>),
	BorderLeft(OrderedFloat<f32>),
	BorderRight(OrderedFloat<f32>),
	BorderStart(OrderedFloat<f32>),
	BorderTop(OrderedFloat<f32>),
	Border(OrderedFloat<f32>),
	Bottom(StyleUnit),
	Display(Display),
	End(StyleUnit),
	Flex(OrderedFloat<f32>),
	FlexBasis(StyleUnit),
	FlexDirection(FlexDirection),
	FlexGrow(OrderedFloat<f32>),
	FlexShrink(OrderedFloat<f32>),
	FlexWrap(Wrap),
	Height(StyleUnit),
	JustifyContent(Justify),
	Left(StyleUnit),
	Margin(StyleUnit),
	MarginBottom(StyleUnit),
	MarginEnd(StyleUnit),
	MarginHorizontal(StyleUnit),
	MarginLeft(StyleUnit),
	MarginRight(StyleUnit),
	MarginStart(StyleUnit),
	MarginTop(StyleUnit),
	MarginVertical(StyleUnit),
	MaxHeight(StyleUnit),
	MaxWidth(StyleUnit),
	MinHeight(StyleUnit),
	MinWidth(StyleUnit),
	Overflow(Overflow),
	Padding(StyleUnit),
	PaddingBottom(StyleUnit),
	PaddingEnd(StyleUnit),
	PaddingHorizontal(StyleUnit),
	PaddingLeft(StyleUnit),
	PaddingRight(StyleUnit),
	PaddingStart(StyleUnit),
	PaddingTop(StyleUnit),
	PaddingVertical(StyleUnit),
	Position(PositionType),
	Right(StyleUnit),
	Start(StyleUnit),
	Top(StyleUnit),
	Width(StyleUnit),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Layout {
	pub left: f32,
	pub right: f32,
	pub top: f32,
	pub bottom: f32,
	pub width: f32,
	pub height: f32,
}

#[derive(Debug)]
pub struct Context(Box<Any>);

impl Context {
	pub fn new<T: Any>(value: T) -> Context {
		Context(Box::new(value))
	}
}

impl Deref for Context {
	type Target = Box<Any>;
	fn deref(&self) -> &Box<Any> {
		&self.0
	}
}

#[macro_export]
macro_rules! unit {
	( $val:tt pt) => (
		$val.point()
	);
	( $val:tt %) => {
		$val.percent()
	};
	( $val:expr) => {
		$val
	};
}

#[macro_export]
macro_rules! flex_style {
	// Manually match on styles which require an OrderedFloat
	// This way the styles like
	//     Flex(1.0)
	// will be converted to:
	//     Flex(1.0.into())
	(AspectRatio($val:expr)) => (
		AspectRatio($val.into())
	);
	(BorderBottom($val:expr)) => (
		BorderBottom($val.into())
	);
	(BorderEnd($val:expr)) => (
		BorderEnd($val.into())
	);
	(BorderLeft($val:expr)) => (
		BorderLeft($val.into())
	);
	(BorderRight($val:expr)) => (
		BorderRight($val.into())
	);
	(BorderStart($val:expr)) => (
		BorderStart($val.into())
	);
	(BorderTop($val:expr)) => (
		BorderTop($val.into())
	);
	(Border($val:expr)) => (
		Border($val.into())
	);
	(Flex($val:expr)) => (
		Flex($val.into())
	);
	(FlexGrow($val:expr)) => (
		FlexGrow($val.into())
	);
	(FlexShrink($val:expr)) => (
		FlexShrink($val.into())
	);
	($s:ident($($unit:tt)*)) => (
		$s(unit!($($unit)*))
	);
}

#[macro_export]
macro_rules! style {
	( $x:expr, $($s:tt($($unit:tt)*)),* ) => {
		$x.apply_styles(&vec!(
			$(
				flex_style!($s(unit!($($unit)*))),
			)*
		))
	};
}

#[macro_export]
macro_rules! make_styles {
	( $($s:tt($($unit:tt)*)),* ) => {
		vec!(
			$(
				flex_style!($s(unit!($($unit)*))),
			)*
		)
	};
}