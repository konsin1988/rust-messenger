import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:provider/provider.dart';

import 'package:design_system/design_system.dart';

import 'package:app/utils/constants.dart';
import 'package:app/screens/chats/chats_page.dart';
import 'package:app/screens/contacts/contacts_page.dart';
import 'package:app/screens/settings/settings_page.dart';
import 'package:app/screens/profile/profile_page.dart';

import 'package:app/screens/main/main_page.dart';

final _rootNavigatorKey = GlobalKey<NavigatorState>();
final _shellNavigatorKey = GlobalKey<NavigatorState>();

final router = GoRouter(
  initialLocation: '/chats',
  navigatorKey: _rootNavigatorKey,
  routes: [
    StatefulShellRoute.indexedStack(
      builder: (context, state, navigationShell) {
        return MainScreen(navigationShell: navigationShell);
      },
      branches: [
        StatefulShellBranch(
          routes: [GoRoute(path: '/chats', builder: (context, state) => const ChatsPage(title: 'Chats'))],
        ),
        StatefulShellBranch(
          routes: [GoRoute(path: '/contacts', builder: (context, state) => const ContactsPage(title: 'Contacts'))],
        ),
        StatefulShellBranch(
          routes: [GoRoute(path: '/settings', builder: (context, state) => const SettingsPage(title: 'Settings'))],
        ),
        StatefulShellBranch(
          routes: [GoRoute(path: '/profile', builder: (context, state) => const ProfilePage(title: 'Profile'))],
        ),
      ],
    ),
  ],
);
