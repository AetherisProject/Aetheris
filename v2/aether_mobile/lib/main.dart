import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'app.dart';
import 'services/api_service.dart';
import 'services/auth_service.dart';
import 'services/storage_service.dart';
import 'services/chat_service.dart';
import 'services/file_service.dart';
import 'services/research_service.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Initialize services
  final storageService = StorageService();
  await storageService.init();
  
  final authService = AuthService(storageService: storageService);
  await authService.init();
  
  final apiService = ApiService(authService: authService);
  final fileService = FileService(apiService: apiService, storageService: storageService);
  final chatService = ChatService(apiService: apiService, storageService: storageService);
  final researchService = ResearchService(apiService: apiService, storageService: storageService);
  
  runApp(
    MultiProvider(
      providers: [
        ChangeNotifierProvider(create: (_) => authService),
        Provider(create: (_) => apiService),
        Provider(create: (_) => fileService),
        Provider(create: (_) => chatService),
        Provider(create: (_) => researchService),
        Provider(create: (_) => storageService),
      ],
      child: const AetherApp(),
    ),
  );
}