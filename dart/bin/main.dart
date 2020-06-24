import 'dart:io' show Platform, File;

import 'package:morpheus_sdk/crypto.dart';

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

  print('Initializing SDK');
  final sdk = CryptoAPI.create('../target/debug/libmorpheus_core.so');
  try {
    final nonce = sdk.generateNonce();
    print('Generated nonce: $nonce');

    final seedPhrase = sdk.bip39GeneratePhrase('en');
    print('Generated seed phrase: $seedPhrase');

    sdk.bip39ValidatePhrase('en', seedPhrase);
    try {
      sdk.bip39ValidatePhrase('en', seedPhrase + "x");
    } catch (e) {
      print('Validation throws for invalid phrase');
    }

    final words = sdk.bip39ListWords('en', 'woo');
    print('Matching Bip39 words: $words');

    final contentId = 'cjuzC-XxgzNMwYXtw8aMIAeS2Xjlw1hlSNKTvVtUwPuyYo';
    final maskedContentId = sdk.maskJson('"${contentId}"', '.');
    print('Masking string is idempotent: ${contentId == maskedContentId}');

    final json = File('bin/witnessStatement.json').readAsStringSync();
    final maskedJson = sdk.maskJson(json, '.claim.content.dateOfBirth');
    print('Masked Json:\n$maskedJson');

    final word25 = "";
    final unlockPassword = "testing";
    var vault = null;
//    try {
//      vault = sdk.loadVault(vaultPath);
//    } catch (e) {
        print('Creating vault...');
        vault = sdk.createVault(
          'include pear escape sail spy orange cute despair witness trouble sleep torch wire burst unable brass expose fiction drift clock duck oxygen aerobic already',
          word25, unlockPassword);
//    }

    final vaultPath =
        '${Platform.environment['HOME']}/.config/prometheus/did_vault.dat';

    print('Vault dirty flag: ${sdk.vaultIsDirty(vault)}');
    var vaultJson = sdk.vaultToJson(vault);
    print('Serialized Vault: $vaultJson');
    print('Vault dirty flag: ${sdk.vaultIsDirty(vault)}');

//    while (sdk.listDids().length < 2) {
//      print('Created ${sdk.createDid()}');
//    }
//
//    final dids = sdk.listDids();
//    print('Dids: ${dids.join(',')}');
//
//    final witnessRequest = File('bin/witnessRequest.json').readAsStringSync();
//    final witnessRequestId = sdk.maskJson(witnessRequest, '.');
//    final signedWitnessRequest = sdk.signWitnessRequest(witnessRequest, 'iezbeWGSY2dqcUBqT8K7R14xr');
//    print('Signed Witness Request:\n$signedWitnessRequest');
//    print('Witness Request Content ID: ${witnessRequestId}');
//
//    final witnessStatement = File('bin/witnessStatement.json').readAsStringSync();
//    final signedWitnessStatement = sdk.signWitnessStatement(witnessStatement, 'iezbeWGSY2dqcUBqT8K7R14xr');
//    print('Signed Witness Statement:\n${signedWitnessStatement.toJson()}');
//
//    final claimPresentation = File('bin/claimPresentation.json').readAsStringSync();
//    final signedClaimPresentation = sdk.signClaimPresentation(claimPresentation, 'iezbeWGSY2dqcUBqT8K7R14xr');
//    print('Signed Claim Presentation:\n${signedClaimPresentation.toJson()}');

    print('Closing vault');
    sdk.freeVault(vault);
  } finally {
    sdk.dispose();
  }
}
