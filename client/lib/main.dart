import 'package:flutter/material.dart';
import 'package:flutter/services.dart';   // SystemChrome and DeviceOrientation
import 'package:flutter_web_plugins/url_strategy.dart'; // usePathUrlStrategy

import 'package:app/utils/constants.dart';
import 'package:design_system/design_system.dart';
import 'package:app/app/router.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      routerConfig: router,
    );
  }
}
