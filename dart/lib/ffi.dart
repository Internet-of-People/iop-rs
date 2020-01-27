import 'dart:ffi';
import 'dart:typed_data';
import 'package:ffi/ffi.dart';

typedef NativeFuncCallback = Void Function(
    Pointer<CallContext> requestId, Pointer result);
typedef NativeFuncErrback = Void Function(
    Pointer<CallContext> requestId, Pointer<Utf8> result);

class CallContext extends Struct {
  static void _callback(Pointer<CallContext> requestId, Pointer result) {
    final ctx = requestId.ref;
    ctx._complete(Result.success(result));
  }

  static void _errback(Pointer<CallContext> requestId, Pointer<Utf8> message) {
    final ctx = requestId.ref;
    ctx._complete(Result.error(message));
  }

  factory CallContext._next() {
    final r = allocate<CallContext>();
    return r.ref.._result = nullptr;
  }

  static R run<R>(R Function(CallContext call) action) {
    final call = CallContext._next();
    try {
      return action(call);
    } finally {
      call.dispose();
    }
  }

  Pointer<Result> _result;

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
    return _error == nullptr ? _success : throw Utf8.fromUtf8(_error);
  }

  String get asString {
    final result = Utf8.fromUtf8(_value.cast());
    return result;
  }

  /// Taking ownership of the pointer from the result
  Pointer<T> asPointer<T extends NativeType>() {
    final result = _value;
    _success = nullptr;
    return result.cast();
  }

  List<T> asList<T>() {
    // return _value;
    throw UnimplementedError('asList');
  }

  void get asVoid => _value;

  int get asInteger => _value.address;

  void dispose() {
    if (_success != nullptr) {
      free(_success);
    }
    if (_error != nullptr) {
      free(_error);
    }
    free(addressOf);
  }
}
