import package:flutter/material.dart;
import package:flutter_localizations/flutter_localizations.dart;
import package:flutter_secure_storage/flutter_secure_storage.dart;
import package:flutter_riverpod/flutter_riverpod.dart;
import package:fluent_runtime/fluent_runtime.dart;
import package:i18n/i18n.dart;
void main() { runApp(MyApp()); }
class MyApp extends StatelessWidget { @override Widget build(BuildContext context) { return MaterialApp( debugShowCheckedModeBanner: false, localizationsDelegates: [GlobalMaterialLocalizations.delegate, GlobalWidgetsLocalizations.delegate, GlobalCupertinoLocalizations.delegate], home: MyHomePage(), ); } }
class MyHomePage extends StatelessWidget { @override Widget build(BuildContext context) { return Scaffold( appBar: AppBar(title: Text(Aetheris Mobile)), body: Center(child: Text(Welcome to Aetheris Mobile!),), ); } }
