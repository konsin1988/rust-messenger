import 'package:flutter/material.dart';

enum DSColorStyle {
  pink,
  brown,
  light,
  yellow,
  dark,
  lightpurple,
  green,
  purple,
}

extension DSColorStyleExtension on DSColorStyle {
  Color get value {
    switch (this) {
      case DSColorStyle.pink:
        return const Color(0xFFCA2FBF);
      case DSColorStyle.brown:
        return const Color(0xFFB06E69);
      case DSColorStyle.light:
        return const Color(0xFFDBCCE6);
      case DSColorStyle.yellow:
        return const Color(0xFFD3A464);
      case DSColorStyle.dark:
        return const Color(0xFF261830);
      case DSColorStyle.lightpurple:
        return const Color(0xFFB99BCF);
      case DSColorStyle.green:
        return const Color(0xFF99D56D);
      case DSColorStyle.purple:
        return const Color(0xFF976AB8);
    }
  }
}
