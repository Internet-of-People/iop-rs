import 'dart:ffi';
import 'package:ffi/ffi.dart';

import './ffi.dart';

// I would love to use reflection to have something like this instead of the 100 lines that follow it.
//
// abstract class MorpheusSdk {
//   Void ping(Pointer<Utf8> message, Int32 delay, Pointer<CallContext> requestId, Pointer callback, Pointer error);
//   Void init_sdk(Pointer<CallContext> requestId, Pointer callback, Pointer error);
//   Void close_sdk(Pointer<Void> sdk);
//   Void load_vault(Pointer<Void> sdk, Pointer<Utf8> path, Pointer<CallContext> requestId, Pointer callback, Pointer error);
//   Void create_vault(Pointer<Void> sdk, Pointer<Utf8> seed, Pointer<Utf8> path, Pointer<CallContext> requestId, Pointer callback, Pointer error);
//   Void fake_ledger(Pointer<Void> sdk, Pointer<CallContext> requestId, Pointer callback, Pointer error);
//   Void real_ledger(Pointer<Void> sdk, Pointer<Utf8> url, Pointer<CallContext> requestId, Pointer callback, Pointer error);
//   Void list_dids(Pointer<Void> sdk, Pointer<CallContext> requestId, Pointer callback, Pointer error);
//   Void create_did(Pointer<Void> sdk, Pointer<CallContext> requestId, Pointer callback, Pointer error);
//   Void get_document(Pointer<Void> sdk, Pointer<Utf8> did, Pointer<CallContext> requestId, Pointer callback, Pointer error);
// }
// final lib = DynamicLibrary.open('../target/debug/libmorpheus_sdk.so');
// final morpheusSdk = NativeLibrary.createProxy<MorpheusSdk>(lib);
// morpheusSdk.ping(...);

typedef NativeFuncPing = Void Function(Pointer<Utf8> message, Int32 delay,
    Pointer<CallContext> requestId, Pointer callback, Pointer error);
typedef DartFuncPing = void Function(Pointer<Utf8> message, int delay,
    Pointer<CallContext> requestId, Pointer callback, Pointer error);

typedef NativeFuncInitSdk = Void Function(
    Pointer<CallContext> requestId, Pointer callback, Pointer error);
typedef DartFuncInitSdk = void Function(
    Pointer<CallContext> requestId, Pointer callback, Pointer error);

typedef NativeFuncCloseSdk = Void Function(Pointer<Void> sdk);
typedef DartFuncCloseSdk = void Function(Pointer<Void> sdk);

typedef NativeFuncLoadVault = Void Function(
  Pointer<Void> sdk,
  Pointer<Utf8> path,
  Pointer<CallContext> requestId,
  Pointer callback,
  Pointer error,
);
typedef DartFuncLoadVault = void Function(Pointer<Void> sdk, Pointer<Utf8> path,
    Pointer<CallContext> requestId, Pointer callback, Pointer error);

typedef NativeFuncCreateVault = Void Function(
  Pointer<Void> sdk,
  Pointer<Utf8> seed,
  Pointer<Utf8> path,
  Pointer<CallContext> requestId,
  Pointer callback,
  Pointer error,
);
typedef DartFuncCreateVault = void Function(
    Pointer<Void> sdk,
    Pointer<Utf8> seed,
    Pointer<Utf8> path,
    Pointer<CallContext> requestId,
    Pointer callback,
    Pointer error);

typedef NativeFuncFakeLedger = Void Function(
  Pointer<Void> sdk,
  Pointer<CallContext> requestId,
  Pointer callback,
  Pointer error,
);
typedef DartFuncFakeLedger = void Function(
    Pointer<Void> sdk,
    Pointer<CallContext> requestId,
    Pointer callback,
    Pointer error);

typedef NativeFuncRealLedger = Void Function(
  Pointer<Void> sdk,
  Pointer<Utf8> url,
  Pointer<CallContext> requestId,
  Pointer callback,
  Pointer error,
);
typedef DartFuncRealLedger = void Function(
    Pointer<Void> sdk,
    Pointer<Utf8> url,
    Pointer<CallContext> requestId,
    Pointer callback,
    Pointer error);

typedef NativeFuncListDids = Void Function(
  Pointer<Void> sdk,
  Pointer<CallContext> requestId,
  Pointer callback,
  Pointer error,
);
typedef DartFuncListDids = void Function(Pointer<Void> sdk,
    Pointer<CallContext> requestId, Pointer callback, Pointer error);

typedef NativeFuncCreateDid = Void Function(
  Pointer<Void> sdk,
  Pointer<CallContext> requestId,
  Pointer callback,
  Pointer error,
);
typedef DartFuncCreateDid = void Function(Pointer<Void> sdk,
    Pointer<CallContext> requestId, Pointer callback, Pointer error);

typedef NativeFuncGetDocument = Void Function(
  Pointer<Void> sdk,
  Pointer<Utf8> did,
  Pointer<CallContext> requestId,
  Pointer callback,
  Pointer error,
);
typedef DartFuncGetDocument = void Function(
    Pointer<Void> sdk,
    Pointer<Utf8> did,
    Pointer<CallContext> requestId,
    Pointer callback,
    Pointer error);

class NativeAPI {
  final DartFuncCloseSdk native_close_sdk;
  final DartFuncLoadVault native_load_vault;
  final DartFuncCreateVault native_create_vault;
  final DartFuncFakeLedger native_fake_ledger;
  final DartFuncRealLedger native_real_ledger;
  final DartFuncListDids native_list_dids;
  final DartFuncCreateDid native_create_did;
  final DartFuncGetDocument native_get_document;

  NativeAPI(
    this.native_close_sdk,
    this.native_load_vault,
    this.native_create_vault,
    this.native_fake_ledger,
    this.native_real_ledger,
    this.native_list_dids,
    this.native_create_did,
    this.native_get_document,
  );
}

class RustSdk {
  final Pointer<Void> _sdk;
  final NativeAPI _api;

  RustSdk(this._sdk, this._api);

  void loadVault(String path) {
    return CallContext.run((call) {
      final nativePath = Utf8.toUtf8(path);
      try {
        _api.native_load_vault(
          _sdk,
          nativePath,
          call.id,
          call.callback,
          call.error,
        );
        return call.result().asVoid;
      } finally {
        free(nativePath);
      }
    });
  }

  void createVault(String seed, String path) {
    return CallContext.run((call) {
      final nativeSeed = Utf8.toUtf8(seed);
      final nativePath = Utf8.toUtf8(path);
      try {
        _api.native_create_vault(
          _sdk,
          nativeSeed,
          nativePath,
          call.id,
          call.callback,
          call.error,
        );
        return call.result().asVoid;
      } finally {
        free(nativePath);
        free(nativeSeed);
      }
    });
  }

  void fakeLedger() {
    return CallContext.run((call) {
      _api.native_fake_ledger(
        _sdk,
        call.id,
        call.callback,
        call.error,
      );
      return call.result().asVoid;
    });
  }

  void realLedger(String url) {
    return CallContext.run((call) {
      final nativeUrl = Utf8.toUtf8(url);
      try {
        _api.native_real_ledger(
          _sdk,
          nativeUrl,
          call.id,
          call.callback,
          call.error,
        );
        return call.result().asVoid;
      } finally {
        free(nativeUrl);
      }
    });
  }

  List<String> listDids() {
    return CallContext.run((call) {
      _api.native_list_dids(
        _sdk,
        call.id,
        call.callback,
        call.error,
      );
      return call.result().asStringList();
    });
  }

  String createDid() {
    return CallContext.run((call) {
      _api.native_create_did(
        _sdk,
        call.id,
        call.callback,
        call.error,
      );
      return call.result().asString;
    });
  }

  String getDocument(String did) {
    return CallContext.run((call) {
      final nativeDid = Utf8.toUtf8(did);
      try {
        _api.native_get_document(
          _sdk,
          nativeDid,
          call.id,
          call.callback,
          call.error,
        );
        return call.result().asString;
      } finally {}
    });
  }

  void dispose() {
    _api.native_close_sdk(_sdk);
  }
}

class RustAPI {
  /*static String ping(String message, int delaySec) {
    return CallContext.run((call) {
      final nativeMessage = Utf8.toUtf8(message);
      try {
        native_ping(
          nativeMessage,
          delaySec,
          call.id,
          call.callback,
          call.error,
        );
        return call.result().asString;
      } finally {
        free(nativeMessage);
      }
    });
  }*/

  static RustSdk initSdk(String libPath) {
    final lib = DynamicLibrary.open(libPath);
    //final native_ping = lib.lookupFunction<NativeFuncPing, DartFuncPing>('ping');
    final native_init_sdk = lib.lookupFunction<NativeFuncInitSdk, DartFuncInitSdk>('init_sdk');

    final api = NativeAPI(
      lib.lookupFunction<NativeFuncCloseSdk, DartFuncCloseSdk>('close_sdk'),
      lib.lookupFunction<NativeFuncLoadVault, DartFuncLoadVault>('load_vault'),
      lib.lookupFunction<NativeFuncCreateVault, DartFuncCreateVault>('create_vault'),
      lib.lookupFunction<NativeFuncFakeLedger, DartFuncFakeLedger>('fake_ledger'),
      lib.lookupFunction<NativeFuncRealLedger, DartFuncRealLedger>('real_ledger'),
      lib.lookupFunction<NativeFuncListDids, DartFuncListDids>('list_dids'),
      lib.lookupFunction<NativeFuncCreateDid, DartFuncCreateDid>('create_did'),
      lib.lookupFunction<NativeFuncGetDocument, DartFuncGetDocument>('get_document'),
    );

    return CallContext.run((call) {
      native_init_sdk(
        call.id,
        call.callback,
        call.error,
      );
      return RustSdk(call.result().asPointer(), api);
    });
  }

  // static List<String> listDids(_){
  //   final id = _getNextId;
  //   sleep(Duration(seconds: 2));
  //   _resultMap[id] = [Utf8.toUtf8('did:morpheus:ezFoo1'), Utf8.toUtf8('did:morpheus:ezFoo2')];
  //   return (_resultMap.remove(id) as List<Pointer<Utf8>>)
  //       .map((did)=>Utf8.fromUtf8(did))
  //       .toList();
  // }
}
