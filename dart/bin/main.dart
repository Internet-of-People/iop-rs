import 'dart:io';

import 'package:morpheus_dart/rust.dart';

void main(List<String> arguments) {
  /*try {
    const message = 'Unicorn 🦄 loves 💕 dart-C-rust bridging';
    final result = RustAPI.ping(message, 2);
    print('Result: ${result}!');
    final result2 = RustAPI.ping('fail with me!', 1);
    print('Result2: ${result2}!');
  } catch (e) {
    print('Error: $e');
  }*/

  // Running this loop shows that we have some leaks:
  /*for (var i = 0; i < 1000000; i++) {
    final result = RustAPI.ping('this is benchmark run $i', 0);
    if (i % 1000 == 0) {
      print(result);
    }
  }*/

  if (!Platform.isLinux) {
    print('Vault loading is implemented for Linux only yet.');
  }

  try {
    final sdk = RustAPI.initSdk();
    try {
      final vaultPath =
          '${Platform.environment['HOME']}/.config/prometheus/did_vault.dat';
      try {
        sdk.loadVault(vaultPath);
      } catch (e) {
        sdk.createVault(
            'include pear escape sail spy orange cute despair witness trouble sleep torch wire burst unable brass expose fiction drift clock duck oxygen aerobic already',
            vaultPath);
      }

      while (sdk.listDids().length < 2) {
        print('Created ${sdk.createDid()}');
      }

      final dids = sdk.listDids();
      print('Dids: ${dids.join(',')}');
    } catch (e) {
      print('Error using SDK: $e');
    } finally {
      sdk.dispose();
    }
  } catch (e) {
    print('Error initializing SDK: $e');
  }
}
