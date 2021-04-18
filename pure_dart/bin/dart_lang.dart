import 'dart:async';
import 'dart:io' as io;
import 'dart:io';
import 'dart:isolate';
import 'models/file.dart' as ff;

enum FileType {
  File,
  Dir,
}

class File {
  String name;
  FileType ty;
  int size;

  File(this.name, this.ty, this.size);
}

const win_dir = 'C:\\Users\\lluz\\AppData\\Local\\Yarn\\Cache\\v6';
const wsl_dir = '/mnt/c/Users/lluz/AppData/Local/Yarn/Cache/v6/';
const linux_dir = '/usr/bin';

void list_files_sync(String path) {
  var files_list = List<ff.File>.empty(growable: true);

  final dir = io.Directory(path);

  dir.listSync(recursive: false).forEach((f) {
    var file = ff.File(f.path, 0, ff.FileType.Dir);
    if (f.statSync().type == io.FileSystemEntityType.file) {
      file.size = f.statSync().size;
      file.ty = ff.FileType.File;
    }
    files_list.add(file);
    print('${file.toJson()}');
  });
}

var total_files = 0;
var total_dirs = 0;

void list_files(String path) async {
  // var files_list = List<ff.File>.empty(growable: true);

  final dir = io.Directory(path);

  await dir.list(recursive: false).forEach((f) {
    var file = ff.File(f.path, 0, ff.FileType.Dir);
    if (f.statSync().type == io.FileSystemEntityType.file) {
      file.size = f.statSync().size;
      file.ty = ff.FileType.File;
    }
    // files_list.add(file);
    if (file.ty == ff.FileType.Dir) {
      total_dirs++;
    } else {
      total_files++;
    }
    print('${file.toString()}');
  });

  print('\nFiles: $total_files');
  print('Directories: $total_dirs\n');
  print('Total: ${total_files + total_dirs}\n');
}

void myIsolate(SendPort isolateToMainStream) {
  var mainToIsolateStream = ReceivePort();
  isolateToMainStream.send(mainToIsolateStream.sendPort);

  mainToIsolateStream.listen((path) {
    print('[Secondary thread]\tStarting listing files...');
    list_files_sync(path);
    print('[Secondary thread]\tDone listing files!');
    isolateToMainStream.send('Exiting...');
  });

  // isolateToMainStream.send('This is from myIsolate()');
}

Future<SendPort> initIsolate() async {
  final completer = Completer<SendPort>();
  final isolateToMainStream = ReceivePort();

  isolateToMainStream.listen((data) {
    if (data is SendPort) {
      final mainToIsolateStream = data;
      completer.complete(mainToIsolateStream);
    } else {
      print('[Main Thread]\t$data');
      exit(0);
    }
  });

  await Isolate.spawn(myIsolate, isolateToMainStream.sendPort);
  return completer.future;
}

void with_isolate(String path) async {
  final mainToIsolateStream = await initIsolate();
  mainToIsolateStream.send(path);
}

void main(List<String> arguments) {
  final path = arguments.isNotEmpty ? arguments[0] : linux_dir;
  // with_isolate(path);
  list_files(path);
}
