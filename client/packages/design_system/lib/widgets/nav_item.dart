import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:go_router/go_router.dart';
import 'package:design_system/design_system.dart';

class DSNavItem extends StatelessWidget {
  final int index;
  final String? iconPath;
  final String? imageUrl;
  final bool? isProfile;
  final String label;
  final StatefulNavigationShell navigationShell;

  const DSNavItem({
    super.key,
    required this.index,
    this.iconPath,
    this.imageUrl,
    this.isProfile = false,
    required this.label,
    required this.navigationShell,
  });

  @override
  Widget build(BuildContext context) {
    final isSelected = navigationShell.currentIndex == index;
    final color = isSelected ? DSColorStyle.green.value : Colors.grey;

    return InkWell(
      onTap: () => navigationShell.goBranch(index),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            width: 30,
            height: 30,
            padding: const EdgeInsets.all(1),
            decoration: BoxDecoration(
              shape: BoxShape.circle,
              border: Border.all(
                color: isSelected && isProfile! ? DSColorStyle.green.value : Colors.transparent,
                width: 1.5,
              ),
            ),
            child: _buildIcon(color),
          ),
          const SizedBox(height: 1),
          Text(label, style: TextStyle(
            color: color, 
            fontSize: 12,
            fontWeight: isSelected ? FontWeight.bold : FontWeight.normal, 
          )),
        ],
      ),
    );
  }

  Widget _buildIcon(Color color) {
    if (isProfile! && imageUrl != null && imageUrl!.isNotEmpty) {
      return CircleAvatar(
        backgroundImage: NetworkImage(imageUrl!),
      );
    }
    
    if (isProfile!) {
      return CircleAvatar(
        backgroundColor: Colors.grey.shade300,
        child: Icon(Icons.person, size: 20, color: Colors.grey.shade600),
      );
    }

    return SvgPicture.asset(
      iconPath!,
      colorFilter: ColorFilter.mode(color, BlendMode.srcIn),
    );
  }
}


