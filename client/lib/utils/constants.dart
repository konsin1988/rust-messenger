import 'dart:io';
import 'package:device_info_plus/device_info_plus.dart';

class AppIcons {
  AppIcons._();
  static const String chats = 'assets/images/bottom_navbar/chat_bubble.svg';
  static const String contacts = 'assets/images/bottom_navbar/contacts.svg';
  static const String settings = 'assets/images/bottom_navbar/settings.svg';
  static const List<String> appIcons = [
    chats, contacts, settings, 
  ];
}

//class AppImageIcons {
//  AppImageIcons._();
//  static const String assistant = 'assets/images/app/assistant.jpg';
//  static const String absenceAvatar = 'assets/images/app/absence-avatar-3.jpeg';
//  static const List<String> appImageTitles = [
//    assistant,
//  ];
//}

class AppRoutes {
  AppRoutes._();
  static const String chats = '/chats';
  static const String contacts = '/contacts';
  static const String settings = '/settings';
  static const String profile = '/profile';
  static const List<String> appRoutes = [
    chats,
    contacts,
    settings,
    profile,
  ];
}

class AppLinks {
  static const String baseURL = 
    String.fromEnvironment('BASE_URL', defaultValue: "http://10.100.221.18:8000");
  static const String graphqlURL = '$baseURL/graphql';
  static const String graphqlWS = 
    String.fromEnvironment('BASE_WS_URL', defaultValue: "ws://10.100.221.18:8000/graphql");
  //static const String serverLink = "10.0.2.2:3000";
  //static const String baseQraphqlLink = "http://10.0.2.2:3000/graphql";
  //static const String baseWSQraphqlLink = "ws://10.0.2.2:3000/graphql";  
}

