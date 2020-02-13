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
    final sdk = RustAPI.initSdk('../target/debug/libmorpheus_sdk.so');
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

      final witnessRequest = '{"claim":{"subject":"did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr","content":{"address":"adsf","placeOfBirth":{"country":"sdf","city":"adsf"},"dateOfBirth":"13/02/2002"}},"claimant":"did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr#0","processId":"Digitalize ID card","evidence":{},"nonce":"uOANCT6WfyvEqvZkT8+5WFgubadCdJdiuNjVDpcAdXFzh"}';
      final signedWitnessRequest = sdk.signWitnessRequest(witnessRequest, 'iezbeWGSY2dqcUBqT8K7R14xr');
      print('Signed Witness Request:\n$signedWitnessRequest');

      final witnessStatement = '{"claim":{"subject":"did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr","content":{"address":"Strasse","dateOfBirth":"16/02/2002","placeOfBirth":{"city":"Berlin","country":"Germany"}}},"processId":"cjunI8lB1BEtampkcvotOpF-zr1XmsCRNvntciGl3puOkg","constraints":{"after":"2020-02-13T13:23:56.319668","before":"2021-02-13T00:00:00.000","witness":"did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr#0","authority":"did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr","content":null},"nonce":"abcde"}';
      final signedWitnessStatement = sdk.signWitnessStatement(witnessStatement, 'iezbeWGSY2dqcUBqT8K7R14xr');
      print('Signed Witness Statement:\n$signedWitnessStatement');

      // sdk.fakeLedger();
      sdk.realLedger('http://35.187.56.222:4703');

      // final doc1 = sdk.getDocument(dids[0]);
      // print('first document: \n$doc1');

      final isTombstoned = sdk.isTombstonedAt(dids[0], 126);
      print('tombstoned: ${isTombstoned}');

      final hasRight = sdk.hasRightAt(dids[0], 'iezbeWGSY2dqcUBqT8K7R14xr', 'impersonate', 126);
      print('did has right: ${hasRight}');
    } catch (e) {
      print('Error using SDK: $e');
    } finally {
      sdk.dispose();
    }
  } catch (e) {
    print('Error initializing SDK: $e');
  }
}
