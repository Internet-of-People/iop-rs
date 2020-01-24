import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';

const path = '../target/debug/libmorpheus_sdk.so';
DynamicLibrary lib = DynamicLibrary.open(path);

typedef NativeFuncCallback = Void Function(
    Pointer<CallContext> requestId, Pointer result);
typedef NativeFuncErrback = Void Function(
    Pointer<CallContext> requestId, Pointer<Utf8> result);

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
typedef DartFuncCreateVault = void Function(Pointer<Void> sdk, Pointer<Utf8> seed, Pointer<Utf8> path,
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

final native_ping = lib.lookupFunction<NativeFuncPing, DartFuncPing>('ping');
final native_init_sdk =
    lib.lookupFunction<NativeFuncInitSdk, DartFuncInitSdk>('init_sdk');
final native_close_sdk =
    lib.lookupFunction<NativeFuncCloseSdk, DartFuncCloseSdk>('close_sdk');
final native_load_vault =
    lib.lookupFunction<NativeFuncLoadVault, DartFuncLoadVault>('load_vault');
final native_create_vault =
    lib.lookupFunction<NativeFuncCreateVault, DartFuncCreateVault>('create_vault');
final native_list_dids =
    lib.lookupFunction<NativeFuncListDids, DartFuncListDids>('list_dids');
final native_create_did =
    lib.lookupFunction<NativeFuncCreateDid, DartFuncCreateDid>('create_did');

class CallContext extends Struct {
  static void _callback(Pointer<CallContext> requestId, Pointer result) {
    final ctx = requestId.ref;
    ctx._complete(Result.success(result));
  }

  static void _errback(Pointer<CallContext> requestId, Pointer<Utf8> message) {
    final ctx = requestId.ref;
    ctx._complete(Result.error(message));
  }

  static CallContext next() {
    final r = allocate<CallContext>();
    return r.ref.._result = nullptr;
  }

  Pointer<Result> _result;

  CallContext._();

  Pointer<CallContext> get id => addressOf;
  Pointer get callback => Pointer.fromFunction<NativeFuncCallback>(_callback);
  Pointer get error => Pointer.fromFunction<NativeFuncErrback>(_errback);

  Result result() => _result.ref;

  void dispose() {
    if (_result != nullptr) {
      _result.ref.dispose();
    }
    free(id);
  }

  void _complete(Result r) {
    _result = r.addressOf;
  }
}

class Result extends Struct {
  Pointer _success;
  Pointer<Utf8> _error;

  static Result success(Pointer v) {
    final r = allocate<Result>();
    return r.ref
      .._success = v
      .._error = nullptr;
  }

  static Result error(Pointer<Utf8> v) {
    final r = allocate<Result>();
    return r.ref
      .._success = nullptr
      .._error = v;
  }

  Pointer get _value {
    return (_error == nullptr) ? _success : throw Utf8.fromUtf8(_error);
  }

  String get asString => Utf8.fromUtf8(_value.cast());
  Pointer<T> asPointer<T extends NativeType>() => _value.cast();
  void get asVoid => _value;
  int get asInteger => _value.address;

  void dispose() {
    // if (_success != nullptr) {
    //   free(_success);
    // }
    if (_error != nullptr) {
      free(_error);
    }
    free(addressOf);
  }
}

class RustSdk {
  final Pointer<Void> _sdk;

  RustSdk(this._sdk);

  void loadVault(String path) {
    final call = CallContext.next();
    try {
      native_load_vault(
        _sdk,
        Utf8.toUtf8(path),
        call.id,
        call.callback,
        call.error,
      );
      return call.result().asVoid;
    } finally {
      call.dispose();
    }
  }

  void createVault(String seed, String path) {
    final call = CallContext.next();
    try {
      native_create_vault(
        _sdk,
        Utf8.toUtf8(seed),
        Utf8.toUtf8(path),
        call.id,
        call.callback,
        call.error,
      );
      return call.result().asVoid;
    } finally {
      call.dispose();
    }
  }

  List<String> listDids() {
    final call = CallContext.next();
    try {
      native_list_dids(
        _sdk,
        call.id,
        call.callback,
        call.error,
      );
      final count = call.result().asInteger;
      return List.filled(count, 'placeholder');
    } finally {
      call.dispose();
    }
  }

  String createDid() {
    final call = CallContext.next();
    try {
      native_create_did(
        _sdk,
        call.id,
        call.callback,
        call.error,
      );
      return call.result().asString;
    } finally {
      call.dispose();
    }
  }

  void dispose() {
    native_close_sdk(_sdk);
  }
}

class RustAPI {
  static String ping(String message, int delaySec) {
    final call = CallContext.next();
    try {
      native_ping(
        Utf8.toUtf8(message).cast(),
        delaySec,
        call.id,
        call.callback,
        call.error,
      );
      return call.result().asString;
    } finally {
      call.dispose();
    }
  }

  static RustSdk initSdk() {
    final call = CallContext.next();
    try {
      native_init_sdk(
        call.id,
        call.callback,
        call.error,
      );
      return RustSdk(call.result().asPointer());
    } finally {
      call.dispose();
    }
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
      final vaultPath = '${Platform.environment['HOME']}/.config/prometheus/did_vault.dat';
      try {
        sdk.loadVault(vaultPath);
      } catch (e) {
        sdk.createVault('include pear escape sail spy orange cute despair witness trouble sleep torch wire burst unable brass expose fiction drift clock duck oxygen aerobic already', vaultPath);
      }

      while (sdk.listDids().length < 2) {
        print('Created ${ sdk.createDid() }');
      }

      final dids = sdk.listDids();
      print('Dids: ${ dids.join(',') }');
    } catch (e) {
      print('Error using SDK: $e');
    } finally {
      sdk.dispose();
    }
  } catch (e) {
    print('Error initializing SDK: $e');
  }
}
