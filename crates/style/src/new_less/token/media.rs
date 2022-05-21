use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;

///
/// Meida 允许 词根
///
#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenMeidaAllow {
  #[strum(serialize = "(")]
  LeftBrackets,

  #[strum(serialize = ")")]
  RightBrackets,

  #[strum(serialize = ":")]
  Colon,
}

impl EnumExtend for TokenMeidaAllow {}

impl StringInto for TokenMeidaAllow {}

///
/// 媒体类型
///
#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenMediaType {
  #[strum(serialize = "all")]
  All,
  #[strum(serialize = "print")]
  Print,
  #[strum(serialize = "screen")]
  Screen,
  #[strum(serialize = "speech")]
  Speech,
}

impl EnumExtend for TokenMediaType {}

impl StringInto for TokenMediaType {}

///
/// 逻辑操作符
///
#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenMediaLogic {
  #[strum(serialize = "and")]
  And,

  #[strum(serialize = "not")]
  Not,

  #[strum(serialize = "only")]
  Only,
}

impl EnumExtend for TokenMediaLogic {}

impl StringInto for TokenMediaLogic {}

///
/// 媒体功能条件
/// 即 括号里的值
///
#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenMediaFeature {
  #[strum(serialize = "aspect-ratio")]
  AspectRatio,
  //定义输出设备中的页面可见区域宽度与高度的比率
  #[strum(serialize = "color")]
  Color,
  //定义输出设备每一组彩色原件的个数。如果不是彩色设备，则值等于0
  #[strum(serialize = "color-index")]
  ColorIndex,
  //定义在输出设备的彩色查询表中的条目数。如果没有使用彩色查询表，则值等于0
  #[strum(serialize = "device-aspect-ratio")]
  DeviceAspectRatio,
  //定义输出设备的屏幕可见宽度与高度的比率。
  #[strum(serialize = "device-height")]
  DeviceHeight,
  //定义输出设备的屏幕可见高度。
  #[strum(serialize = "device-width")]
  DeviceWidth,
  //定义输出设备的屏幕可见宽度。
  #[strum(serialize = "grid")]
  Grid,
  //用来查询输出设备是否使用栅格或点阵。
  #[strum(serialize = "height")]
  Height,
  //定义输出设备中的页面可见区域高度。
  #[strum(serialize = "max-aspect-ratio")]
  MaxAspectRatio,
  //定义输出设备的屏幕可见宽度与高度的最大比率。
  #[strum(serialize = "max-color")]
  MaxColor,
  //定义输出设备每一组彩色原件的最大个数。
  #[strum(serialize = "max-color-index")]
  MaxColorIndex,
  //定义在输出设备的彩色查询表中的最大条目数。
  #[strum(serialize = "max-device-aspect-ratio")]
  MaxDeviceAspectRatio,
  //定义输出设备的屏幕可见宽度与高度的最大比率。
  #[strum(serialize = "max-device-height")]
  MaxDeviceHeight,
  //定义输出设备的屏幕可见的最大高度。
  #[strum(serialize = "max-device-width")]
  MaxDeviceWidth,
  //定义输出设备的屏幕最大可见宽度。
  #[strum(serialize = "max-height")]
  MaxHeight,
  //定义输出设备中的页面最大可见区域高度。
  #[strum(serialize = "max-monochrome")]
  MaxMonochrome,
  //定义在一个单色框架缓冲区中每像素包含的最大单色原件个数。
  #[strum(serialize = "max-resolution")]
  MaxResolution,
  //定义设备的最大分辨率。
  #[strum(serialize = "max-width")]
  MaxWidth,
  //定义输出设备中的页面最大可见区域宽度。
  #[strum(serialize = "min-aspect-ratio")]
  MinAspectRatio,
  //定义输出设备中的页面可见区域宽度与高度的最小比率。
  #[strum(serialize = "min-color")]
  MinColor,
  //定义输出设备每一组彩色原件的最小个数。
  #[strum(serialize = "min-color-index")]
  MinColorIndex,
  //定义在输出设备的彩色查询表中的最小条目数。
  #[strum(serialize = "min-device-aspect-ratio")]
  MinDeviceAspectRatio,
  //定义输出设备的屏幕可见宽度与高度的最小比率。
  #[strum(serialize = "min-device-width")]
  MinDeviceWidth,
  //定义输出设备的屏幕最小可见宽度。
  #[strum(serialize = "min-device-height")]
  MinDeviceHeight,
  //定义输出设备的屏幕的最小可见高度。
  #[strum(serialize = "min-height")]
  MinHeight,
  //定义输出设备中的页面最小可见区域高度。
  #[strum(serialize = "min-monochrome")]
  MinMonochrome,
  //定义在一个单色框架缓冲区中每像素包含的最小单色原件个数
  #[strum(serialize = "min-resolution")]
  MinResolution,
  //定义设备的最小分辨率。
  #[strum(serialize = "min-width")]
  MinWidth,
  //定义输出设备中的页面最小可见区域宽度。
  #[strum(serialize = "monochrome")]
  Monochrome,
  //定义在一个单色框架缓冲区中每像素包含的单色原件个数。如果不是单色设备，则值等于0
  #[strum(serialize = "orientation")]
  Orientation,
  //定义输出设备中的页面可见区域高度是否大于或等于宽度。
  #[strum(serialize = "resolution")]
  Resolution,
  //定义设备的分辨率。如：96dpi, 300dpi, 118dpcm
  #[strum(serialize = "scan")]
  Scan,
  //定义电视类设备的扫描工序。
  #[strum(serialize = "width")]
  Width, //定义输出设备中的页面可见区域宽度。
}

impl EnumExtend for TokenMediaFeature {}

impl StringInto for TokenMediaFeature {}
