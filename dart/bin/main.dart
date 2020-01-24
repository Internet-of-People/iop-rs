import 'dart:collection';
import 'dart:ffi';
import 'package:ffi/ffi.dart';


const path = '../target/debug/libmorpheus_sdk.so';
DynamicLibrary lib = DynamicLibrary.open(path);

typedef NativeFuncCallback = Void Function(Pointer<Utf8> requestId, Pointer<Void> result);
typedef NativeFuncErrback = Void Function(Pointer<Utf8> requestId, Pointer<Utf8> result);

typedef NativeFuncPing = Void Function(Pointer<Utf8> message, Int32 delay, Pointer<Utf8> requestId, Pointer<NativeFunction<NativeFuncCallback>> callback, Pointer error);
typedef DartFuncPing = void Function(Pointer<Utf8> message, int delay, Pointer<Utf8> requestId, Pointer<NativeFunction<NativeFuncCallback>> callback, Pointer error);

typedef NativeFuncInitSdk = Pointer<Void> Function(Pointer<Utf8> requestId, Pointer callback, Pointer error);
typedef DartFuncInitSdk = Pointer<Void> Function(Pointer<Utf8> requestId, Pointer callback, Pointer error);

typedef NativeFuncCloseSdk = Void Function(Pointer<Void> sdk);
typedef DartFuncCloseSdk = void Function(Pointer<Void> sdk);

final native_ping = lib.lookupFunction<NativeFuncPing, DartFuncPing>('ping');
final native_init_sdk = lib.lookupFunction<NativeFuncInitSdk, DartFuncInitSdk>('init_sdk');
final native_close_sdk = lib.lookupFunction<NativeFuncCloseSdk, DartFuncCloseSdk>('close_sdk');

class RustSdk {
  final Pointer<Void> _sdk;

  RustSdk(this._sdk);
}

class CallContext {
  static int _counter=0;
  static final Map<String, Result> _resultMap = HashMap<String, Result>();

  static void _callback(Pointer<Utf8> requestId, Pointer<Void> result) {
    final id = Utf8.fromUtf8(requestId);

    if(_resultMap.containsKey(id)) {
      throw Exception('$id was already stored as a result');
    }

    _resultMap[id] = SuccessResult(result);
  }

  static void _errback(Pointer<Utf8> requestId, Pointer<Utf8> message) {
    final id = Utf8.fromUtf8(requestId);

    if(_resultMap.containsKey(id)) {
      throw Exception('$id was already stored as a result');
    }

    _resultMap[id] = ErrorResult(Utf8.fromUtf8(message));
  }

  static CallContext get next => CallContext( (++_counter).toString() );

  final String _id;

  CallContext(this._id);

  Pointer<Utf8> get id => Utf8.toUtf8(_id).cast();
  Pointer get callback => Pointer.fromFunction<NativeFuncCallback>(_callback);
  Pointer get error => Pointer.fromFunction<NativeFuncErrback>(_errback);

  Result result() => _resultMap.remove(_id);
}

abstract class Result {
  Pointer<Void> get value;

  String get asString => Utf8.fromUtf8(value.cast());
}

class SuccessResult extends Result {
  final Pointer<Void> _result;

  SuccessResult(this._result);

  @override
  Pointer<Void> get value => _result;
}

class ErrorResult extends Result {
  final String _message;

  ErrorResult(this._message);

  @override
  Pointer<Void> get value => throw _message;
}

class RustAPI {

  static String ping(String message) {
    final call = CallContext.next;
    native_ping(
      Utf8.toUtf8(message).cast(),
      2,
      call.id,
      call.callback,
      call.error,
    );
    return call.result().asString;
  }

  static RustSdk initSdk() {
    final call = CallContext.next;
    native_init_sdk(
      call.id,
      call.callback,
      call.error,
    );
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
  try {
    const message = 'Unicorn 🦄 loves 💕 dart-C-rust bridging';
    final result = RustAPI.ping(message);
    print('Result: ${result}!');
    final result2 = RustAPI.ping('fail with me!');
    print('Result2: ${result2}!');
  } catch (e) {
    print('Error: $e');
  }
}
