import 'dart:ffi';
import 'package:ffi/ffi.dart';

import './ffi.dart';

// I would love to use reflection to have something like this instead of the 100 lines that follow it.
//
// abstract class MorpheusSdk {
//   Void ping(Pointer<Utf8> message, Int32 delay, Pointer<CallContext> requestId, Pointer callback, Pointer error);
//   Pointer<Void> init_sdk(Pointer<CallContext> requestId, Pointer callback, Pointer error);
//   Void close_sdk(Pointer<Void> sdk);
//   Void load_vault(Pointer<Void> sdk, Pointer<Utf8> path, Pointer<CallContext> requestId, Pointer callback, Pointer error);
//   Void create_vault(Pointer<Void> sdk, Pointer<Utf8> seed, Pointer<Utf8> path, Pointer<CallContext> requestId, Pointer callback, Pointer error);
//   Void list_dids(Pointer<Void> sdk, Pointer<CallContext> requestId, Pointer callback, Pointer error);
//   Void create_did(Pointer<Void> sdk, Pointer<CallContext> requestId, Pointer callback, Pointer error);
// }
// final lib = DynamicLibrary.open('../target/debug/libmorpheus_sdk.so');
// final morpheusSdk = NativeLibrary.createProxy<MorpheusSdk>(lib);
// morpheusSdk.ping(...);

const path = '../target/debug/libmorpheus_sdk.so';
DynamicLibrary lib = DynamicLibrary.open(path);

typedef NativeFuncPing = Void Function(Pointer<Utf8> message, Int32 delay,
    Pointer<CallContext> requestId, Pointer callback, Pointer error);
typedef DartFuncPing = void Function(Pointer<Utf8> message, int delay,
    Pointer<CallContext> requestId, Pointer callback, Pointer error);

typedef NativeFuncInitSdk = Pointer<Void> Function(
    Pointer<CallContext> requestId, Pointer callback, Pointer error);
typedef DartFuncInitSdk = Pointer<Void> Function(
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

final native_ping = lib.lookupFunction<NativeFuncPing, DartFuncPing>('ping');
final native_init_sdk =
    lib.lookupFunction<NativeFuncInitSdk, DartFuncInitSdk>('init_sdk');
final native_close_sdk =
    lib.lookupFunction<NativeFuncCloseSdk, DartFuncCloseSdk>('close_sdk');
final native_load_vault =
    lib.lookupFunction<NativeFuncLoadVault, DartFuncLoadVault>('load_vault');
final native_create_vault = lib
    .lookupFunction<NativeFuncCreateVault, DartFuncCreateVault>('create_vault');
final native_list_dids =
    lib.lookupFunction<NativeFuncListDids, DartFuncListDids>('list_dids');
final native_create_did =
    lib.lookupFunction<NativeFuncCreateDid, DartFuncCreateDid>('create_did');

class RustSdk {
  final Pointer<Void> _sdk;

  RustSdk(this._sdk);

  void loadVault(String path) {
    return CallContext.run((call) {
      final nativePath = Utf8.toUtf8(path);
      try {
        native_load_vault(
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
        native_create_vault(
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

  List<String> listDids() {
    return CallContext.run((call) {
      native_list_dids(
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
      native_create_did(
        _sdk,
        call.id,
        call.callback,
        call.error,
      );
      return call.result().asString;
    });
  }

  void dispose() {
    native_close_sdk(_sdk);
  }
}

class RustAPI {
  static String ping(String message, int delaySec) {
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
  }

  static RustSdk initSdk() {
    return CallContext.run((call) {
      native_init_sdk(
        call.id,
        call.callback,
        call.error,
      );
      return RustSdk(call.result().asPointer());
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
