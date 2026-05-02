library design_system;

import 'package:design_system/style/colors.dart';
import 'package:design_system/style/fonts.dart';
import 'package:flutter/material.dart';

export 'package:design_system/style/colors.dart';
export 'package:design_system/style/fonts.dart';
export 'package:design_system/widgets/widgets.dart';
export 'package:flutter_svg/flutter_svg.dart';

TextStyle textStyle(DSFontStyle fontStyle, DSColorStyle color) {
  return fontStyle.value.copyWith(color: color.value);
}
