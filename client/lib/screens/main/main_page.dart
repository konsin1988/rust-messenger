import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

import 'package:design_system/design_system.dart';
import 'package:app/utils/constants.dart';


class MainScreen extends StatelessWidget {
  final StatefulNavigationShell navigationShell;
  const MainScreen({required this.navigationShell, super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Stack(
        children: [
          navigationShell, 
          
          Align(
            alignment: Alignment.bottomCenter,
            child: Padding(
              padding: const EdgeInsets.only(bottom: 18.0), // Floating gap
              child: Container(
                height: 64,
                margin: const EdgeInsets.symmetric(horizontal: 40),
                decoration: BoxDecoration(
                  color: DSColorStyle.dark.value.withOpacity(0.9),
                  borderRadius: BorderRadius.circular(32),
                  boxShadow: [
                    BoxShadow(
                      color: Colors.black.withOpacity(0.3),
                      blurRadius: 10,
                      offset: const Offset(0, 5),
                    ),
                  ],
                ),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                  children: [
                    DSNavItem(index: 0, iconPath: AppIcons.chats, label: 'Chats', navigationShell: navigationShell),
                    DSNavItem(index: 1, iconPath: AppIcons.contacts, label: 'Contacts', navigationShell: navigationShell),
                    DSNavItem(index: 2, iconPath: AppIcons.settings, label: 'Settings', navigationShell: navigationShell),
                    DSNavItem(index: 3, label: 'Profile', navigationShell: navigationShell, isProfile: true),
                  ],
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

