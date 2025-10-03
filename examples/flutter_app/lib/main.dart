import 'package:flutter/material.dart';
import 'package:filefire/filefire.dart';
import 'package:file_picker/file_picker.dart';
import 'package:path_provider/path_provider.dart';
import 'dart:io';

void main() {
  runApp(const FilefireExampleApp());
}

class FilefireExampleApp extends StatelessWidget {
  const FilefireExampleApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'FileFire Example',
      theme: ThemeData(
        primarySwatch: Colors.blue,
        useMaterial3: true,
      ),
      home: const DocumentProcessorPage(),
    );
  }
}

class DocumentProcessorPage extends StatefulWidget {
  const DocumentProcessorPage({super.key});

  @override
  State<DocumentProcessorPage> createState() => _DocumentProcessorPageState();
}

class _DocumentProcessorPageState extends State<DocumentProcessorPage> {
  Document? _currentDocument;
  DocumentMetadata? _metadata;
  List<String> _availablePlugins = [];
  bool _isLoading = false;
  String _statusMessage = '';
  final List<String> _logMessages = [];

  @override
  void initState() {
    super.initState();
    _initializeFileFire();
  }

  Future<void> _initializeFileFire() async {
    setState(() {
      _isLoading = true;
      _statusMessage = 'Initializing FileFire...';
    });

    try {
      await FileFire.initialize();
      final plugins = await FileFire.instance.getAvailablePlugins();

      setState(() {
        _availablePlugins = plugins;
        _statusMessage = 'FileFire initialized successfully';
        _isLoading = false;
      });

      _addLog('FileFire initialized with plugins: ${plugins.join(', ')}');
    } catch (e) {
      setState(() {
        _statusMessage = 'Failed to initialize: $e';
        _isLoading = false;
      });
      _addLog('Initialization error: $e');
    }
  }

  void _addLog(String message) {
    setState(() {
      _logMessages.insert(
          0, '${DateTime.now().toString().substring(11, 19)}: $message');
      if (_logMessages.length > 50) {
        _logMessages.removeLast();
      }
    });
  }

  Future<void> _pickAndOpenDocument() async {
    try {
      final result = await FilePicker.platform.pickFiles(
        type: FileType.custom,
        allowedExtensions: [
          'pdf',
          'docx',
          'xlsx',
          'pptx',
          'jpg',
          'jpeg',
          'png',
          'tiff'
        ],
      );

      if (result != null && result.files.single.path != null) {
        setState(() {
          _isLoading = true;
          _statusMessage = 'Opening document...';
        });

        final filePath = result.files.single.path!;
        final document = await FileFire.open(filePath);

        setState(() {
          _currentDocument = document;
          _metadata = document.metadata;
          _statusMessage = 'Document opened successfully';
          _isLoading = false;
        });

        _addLog('Opened document: ${filePath.split('/').last}');
      }
    } catch (e) {
      setState(() {
        _statusMessage = 'Failed to open document: $e';
        _isLoading = false;
      });
      _addLog('Error opening document: $e');
    }
  }

  Future<void> _addAnnotation() async {
    if (_currentDocument == null) return;

    try {
      setState(() {
        _isLoading = true;
        _statusMessage = 'Adding annotation...';
      });

      final annotationId = await _currentDocument!.annotate(
        page: 1,
        x: 100,
        y: 200,
        width: 200,
        height: 30,
        content: 'Sample annotation from Flutter',
        type: AnnotationType.highlight,
      );

      setState(() {
        _statusMessage = 'Annotation added successfully';
        _isLoading = false;
      });

      _addLog('Added annotation: $annotationId');
    } catch (e) {
      setState(() {
        _statusMessage = 'Failed to add annotation: $e';
        _isLoading = false;
      });
      _addLog('Error adding annotation: $e');
    }
  }

  Future<void> _performOCR() async {
    if (_currentDocument == null) return;

    try {
      setState(() {
        _isLoading = true;
        _statusMessage = 'Performing OCR...';
      });

      final text = await _currentDocument!.performOCR();

      setState(() {
        _statusMessage = 'OCR completed successfully';
        _isLoading = false;
      });

      _addLog('OCR result: ${text.substring(0, text.length.clamp(0, 50))}...');

      // Show OCR result in a dialog
      if (mounted) {
        showDialog(
          context: context,
          builder: (context) => AlertDialog(
            title: const Text('OCR Result'),
            content: SingleChildScrollView(
              child: Text(text),
            ),
            actions: [
              TextButton(
                onPressed: () => Navigator.of(context).pop(),
                child: const Text('OK'),
              ),
            ],
          ),
        );
      }
    } catch (e) {
      setState(() {
        _statusMessage = 'OCR failed: $e';
        _isLoading = false;
      });
      _addLog('OCR error: $e');
    }
  }

  Future<void> _addWatermark() async {
    if (_currentDocument == null) return;

    try {
      setState(() {
        _isLoading = true;
        _statusMessage = 'Adding watermark...';
      });

      await _currentDocument!.addWatermark(
        text: 'CONFIDENTIAL',
        opacity: 0.3,
        position: 'center',
      );

      setState(() {
        _statusMessage = 'Watermark added successfully';
        _isLoading = false;
      });

      _addLog('Added watermark: CONFIDENTIAL');
    } catch (e) {
      setState(() {
        _statusMessage = 'Failed to add watermark: $e';
        _isLoading = false;
      });
      _addLog('Error adding watermark: $e');
    }
  }

  Future<void> _saveDocument() async {
    if (_currentDocument == null) return;

    try {
      setState(() {
        _isLoading = true;
        _statusMessage = 'Saving document...';
      });

      final directory = await getApplicationDocumentsDirectory();
      final timestamp = DateTime.now().millisecondsSinceEpoch;
      final outputPath = '${directory.path}/filefire_output_$timestamp.pdf';

      await _currentDocument!.save(outputPath);

      setState(() {
        _statusMessage = 'Document saved successfully';
        _isLoading = false;
      });

      _addLog('Saved document to: $outputPath');
    } catch (e) {
      setState(() {
        _statusMessage = 'Failed to save document: $e';
        _isLoading = false;
      });
      _addLog('Error saving document: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('FileFire Example'),
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            // Status section
            Card(
              child: Padding(
                padding: const EdgeInsets.all(16.0),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'Status',
                      style: Theme.of(context).textTheme.titleMedium,
                    ),
                    const SizedBox(height: 8),
                    Text(_statusMessage),
                    if (_isLoading) ...[
                      const SizedBox(height: 8),
                      const LinearProgressIndicator(),
                    ],
                  ],
                ),
              ),
            ),

            const SizedBox(height: 16),

            // Document info section
            if (_metadata != null) ...[
              Card(
                child: Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        'Document Information',
                        style: Theme.of(context).textTheme.titleMedium,
                      ),
                      const SizedBox(height: 8),
                      Text('Title: ${_metadata!.title ?? 'Unknown'}'),
                      Text('Author: ${_metadata!.author ?? 'Unknown'}'),
                      Text('Pages: ${_metadata!.pageCount}'),
                      Text(
                          'Size: ${(_metadata!.fileSize / 1024).toStringAsFixed(1)} KB'),
                      Text('Type: ${_metadata!.mimeType}'),
                    ],
                  ),
                ),
              ),
              const SizedBox(height: 16),
            ],

            // Action buttons
            Wrap(
              spacing: 8,
              runSpacing: 8,
              children: [
                ElevatedButton(
                  onPressed: _isLoading ? null : _pickAndOpenDocument,
                  child: const Text('Open Document'),
                ),
                ElevatedButton(
                  onPressed: _currentDocument == null || _isLoading
                      ? null
                      : _addAnnotation,
                  child: const Text('Add Annotation'),
                ),
                ElevatedButton(
                  onPressed: _currentDocument == null || _isLoading
                      ? null
                      : _performOCR,
                  child: const Text('Perform OCR'),
                ),
                ElevatedButton(
                  onPressed: _currentDocument == null || _isLoading
                      ? null
                      : _addWatermark,
                  child: const Text('Add Watermark'),
                ),
                ElevatedButton(
                  onPressed: _currentDocument == null || _isLoading
                      ? null
                      : _saveDocument,
                  child: const Text('Save Document'),
                ),
              ],
            ),

            const SizedBox(height: 16),

            // Plugins info
            Card(
              child: Padding(
                padding: const EdgeInsets.all(16.0),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'Available Plugins',
                      style: Theme.of(context).textTheme.titleMedium,
                    ),
                    const SizedBox(height: 8),
                    if (_availablePlugins.isEmpty)
                      const Text('No plugins loaded')
                    else
                      Wrap(
                        spacing: 8,
                        children: _availablePlugins
                            .map((plugin) => Chip(label: Text(plugin)))
                            .toList(),
                      ),
                  ],
                ),
              ),
            ),

            const SizedBox(height: 16),

            // Log section
            Expanded(
              child: Card(
                child: Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        'Activity Log',
                        style: Theme.of(context).textTheme.titleMedium,
                      ),
                      const SizedBox(height: 8),
                      Expanded(
                        child: Container(
                          width: double.infinity,
                          decoration: BoxDecoration(
                            border: Border.all(color: Colors.grey.shade300),
                            borderRadius: BorderRadius.circular(4),
                          ),
                          child: _logMessages.isEmpty
                              ? const Padding(
                                  padding: EdgeInsets.all(8.0),
                                  child: Text('No activity yet...'),
                                )
                              : ListView.builder(
                                  itemCount: _logMessages.length,
                                  itemBuilder: (context, index) {
                                    return Padding(
                                      padding: const EdgeInsets.symmetric(
                                        horizontal: 8.0,
                                        vertical: 4.0,
                                      ),
                                      child: Text(
                                        _logMessages[index],
                                        style: Theme.of(context)
                                            .textTheme
                                            .bodySmall,
                                      ),
                                    );
                                  },
                                ),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  @override
  void dispose() {
    FileFire.cleanup();
    super.dispose();
  }
}
