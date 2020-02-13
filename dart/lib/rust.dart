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
typedef DartFuncFakeLedger = void Function(Pointer<Void> sdk,
    Pointer<CallContext> requestId, Pointer callback, Pointer error);

typedef NativeFuncRealLedger = Void Function(
  Pointer<Void> sdk,
  Pointer<Utf8> url,
  Pointer<CallContext> requestId,
  Pointer callback,
  Pointer error,
);
typedef DartFuncRealLedger = void Function(Pointer<Void> sdk, Pointer<Utf8> url,
    Pointer<CallContext> requestId, Pointer callback, Pointer error);

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
  Pointer error,
);

typedef NativeFuncSignWitnessRequest = Void Function(
  Pointer<Void> sdk,
  Pointer<Utf8> witnessRequest,
  Pointer<Utf8> auth,
  Pointer<CallContext> requestId,
  Pointer callback,
  Pointer error,
);
typedef DartFuncSignWitnessRequest = void Function(
  Pointer<Void> sdk,
  Pointer<Utf8> witnessRequest,
  Pointer<Utf8> auth,
  Pointer<CallContext> requestId,
  Pointer callback,
  Pointer error,
);

typedef NativeFuncSignWitnessStatement = Void Function(
  Pointer<Void> sdk,
  Pointer<Utf8> witnessStatement,
  Pointer<Utf8> auth,
  Pointer<CallContext> requestId,
  Pointer callback,
  Pointer error,
);
typedef DartFuncSignWitnessStatement = void Function(
  Pointer<Void> sdk,
  Pointer<Utf8> witnessStatement,
  Pointer<Utf8> auth,
  Pointer<CallContext> requestId,
  Pointer callback,
  Pointer error,
);

typedef NativeFuncHasRightAt = Void Function(
  Pointer<Void> sdk,
  Pointer<Utf8> did,
  Pointer<Utf8> auth,
  Pointer<Utf8> right,
  Uint64 height,
  Pointer<CallContext> requestId,
  Pointer callback,
  Pointer error,
);
typedef DartFuncHasRightAt = void Function(
  Pointer<Void> sdk,
  Pointer<Utf8> did,
  Pointer<Utf8> auth,
  Pointer<Utf8> right,
  int height,
  Pointer<CallContext> requestId,
  Pointer callback,
  Pointer error,
);

typedef NativeFuncIsTombstonedAt = Void Function(
  Pointer<Void> sdk,
  Pointer<Utf8> did,
  Uint64 height,
  Pointer<CallContext> requestId,
  Pointer callback,
  Pointer error,
);
typedef DartFuncIsTombstonedAt = void Function(
  Pointer<Void> sdk,
  Pointer<Utf8> did,
  int height,
  Pointer<CallContext> requestId,
  Pointer callback,
  Pointer error,
);


class NativeAPI {
  final DartFuncCloseSdk close_sdk;
  final DartFuncLoadVault load_vault;
  final DartFuncCreateVault create_vault;
  final DartFuncFakeLedger fake_ledger;
  final DartFuncRealLedger real_ledger;
  final DartFuncListDids list_dids;
  final DartFuncCreateDid create_did;
  final DartFuncGetDocument get_document;
  final DartFuncSignWitnessRequest sign_witness_request;
  final DartFuncSignWitnessStatement sign_witness_statement;
  final DartFuncHasRightAt has_right_at;
  final DartFuncIsTombstonedAt is_tombstoned_at;

  NativeAPI(
    this.close_sdk,
    this.load_vault,
    this.create_vault,
    this.fake_ledger,
    this.real_ledger,
    this.list_dids,
    this.create_did,
    this.get_document,
    this.sign_witness_request,
    this.sign_witness_statement,
    this.has_right_at,
    this.is_tombstoned_at,
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
        _api.load_vault(
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
        _api.create_vault(
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
      _api.fake_ledger(
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
        _api.real_ledger(
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
      _api.list_dids(
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
      _api.create_did(
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
        _api.get_document(
          _sdk,
          nativeDid,
          call.id,
          call.callback,
          call.error,
        );
        return call.result().asString;
      } finally {
        free(nativeDid);
      }
    });
  }

  String signWitnessRequest(String witnessRequest, String authentication) {
    return CallContext.run((call) {
      final nativeWitnessRequest = Utf8.toUtf8(witnessRequest);
      final nativeAuthentication = Utf8.toUtf8(authentication);
      try {
        _api.sign_witness_request(
          _sdk,
          nativeWitnessRequest,
          nativeAuthentication,
          call.id,
          call.callback,
          call.error,
        );
        return call.result().asString;
      } finally {
        free(nativeAuthentication);
        free(nativeWitnessRequest);
      }
    });
  }

  String signWitnessStatement(String witnessStatement, String authentication) {
    return CallContext.run((call) {
      final nativeWitnessStatement = Utf8.toUtf8(witnessStatement);
      final nativeAuthentication = Utf8.toUtf8(authentication);
      try {
        _api.sign_witness_statement(
          _sdk,
          nativeWitnessStatement,
          nativeAuthentication,
          call.id,
          call.callback,
          call.error,
        );
        return call.result().asString;
      } finally {
        free(nativeAuthentication);
        free(nativeWitnessStatement);
      }
    });
  }

  bool hasRightAt(String did, String auth, String right, int height) {
    return CallContext.run((call) {
      final nativeDid = Utf8.toUtf8(did);
      final nativeAuth = Utf8.toUtf8(auth);
      final nativeRight = Utf8.toUtf8(right);
      try {
        _api.has_right_at(
          _sdk,
          nativeDid,
          nativeAuth,
          nativeRight,
          height,
          call.id,
          call.callback,
          call.error,
        );
        return call.result().asBool();
      } finally {
        free(nativeDid);
        free(nativeAuth);
        free(nativeRight);
      }
    });
  }

  bool isTombstonedAt(String did, int height) {
      return CallContext.run((call) {
        final nativeDid = Utf8.toUtf8(did);
        try {
          _api.is_tombstoned_at(
            _sdk,
            nativeDid,
            height,
            call.id,
            call.callback,
            call.error,
          );
          return call.result().asBool();
        } finally {
          free(nativeDid);
        }
      });
    }

  void dispose() {
    _api.close_sdk(_sdk);
  }
}

class RustAPI {
  /*static String ping(String message, int delaySec) {
    return CallContext.run((call) {
      final nativeMessage = Utf8.toUtf8(message);
      try {
        ping(
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
    //final ping = lib.lookupFunction<NativeFuncPing, DartFuncPing>('ping');
    final init_sdk =
        lib.lookupFunction<NativeFuncInitSdk, DartFuncInitSdk>('init_sdk');

    final api = NativeAPI(
      lib.lookupFunction<NativeFuncCloseSdk, DartFuncCloseSdk>('close_sdk'),
      lib.lookupFunction<NativeFuncLoadVault, DartFuncLoadVault>('load_vault'),
      lib.lookupFunction<NativeFuncCreateVault, DartFuncCreateVault>(
          'create_vault'),
      lib.lookupFunction<NativeFuncFakeLedger, DartFuncFakeLedger>(
          'fake_ledger'),
      lib.lookupFunction<NativeFuncRealLedger, DartFuncRealLedger>(
          'real_ledger'),
      lib.lookupFunction<NativeFuncListDids, DartFuncListDids>('list_dids'),
      lib.lookupFunction<NativeFuncCreateDid, DartFuncCreateDid>('create_did'),
      lib.lookupFunction<NativeFuncGetDocument, DartFuncGetDocument>('get_document'),
      lib.lookupFunction<NativeFuncSignWitnessRequest, DartFuncSignWitnessRequest>('sign_witness_request'),
      lib.lookupFunction<NativeFuncSignWitnessStatement, DartFuncSignWitnessStatement>('sign_witness_statement'),
      lib.lookupFunction<NativeFuncHasRightAt, DartFuncHasRightAt>('has_right_at'),
      lib.lookupFunction<NativeFuncIsTombstonedAt, DartFuncIsTombstonedAt>('is_tombstoned_at'),
    );

    return CallContext.run((call) {
      init_sdk(
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
