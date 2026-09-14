# Puter API Complete Reference for Aether Integration

> **📚 Comprehensive API Documentation**
> *Every endpoint, every parameter, every capability of the Puter platform*

---

## 📋 Table of Contents

1. [Overview](#-overview)
2. [Authentication](#-authentication)
3. [File System (FS)](#-file-system-fs)
4. [AI Capabilities](#-ai-capabilities)
5. [Key-Value Store (KV)](#-key-value-store-kv)
6. [Serverless Workers](#-serverless-workers)
7. [Hosting](#-hosting)
8. [Events & Real-Time](#-events--real-time)
9. [Networking](#-networking)
10. [Peer-to-Peer](#-peer-to-peer)
11. [Apps Management](#-apps-management)
12. [Teams & Organizations](#-teams--organizations)
13. [UI & Desktop Integration](#-ui--desktop-integration)
14. [Utilities](#-utilities)
15. [Objects & Data Types](#-objects--data-types)
16. [Rate Limits & Quotas](#-rate-limits--quotas)
17. [Security & Permissions](#-security--permissions)
18. [Integration Examples](#-integration-examples)

---

## 🌐 Overview

### What is Puter?

Puter is a **cloud operating system** that provides:
- ✅ **Serverless backend infrastructure**
- ✅ **500+ AI models** (Claude, GPT, Gemini, Grok, etc.)
- ✅ **Cloud storage** with advanced file management
- ✅ **Real-time collaboration** features
- ✅ **User-Pays Model** - users cover their own usage
- ✅ **Zero setup** - no API keys, no servers, no configuration

### Puter.js SDK

**Installation:**
```bash
# NPM
npm install @heyputer/puter.js

# CDN
<script src="https://js.puter.com/v2/"></script>
```

**Initialization:**
```javascript
// Browser
const puter = window.puter;

// Node.js
import { init } from "@heyputer/puter.js/src/init.cjs";
const puter = init(process.env.puterAuthToken);
```

### Platform Support
- ✅ **Websites** - Any JavaScript-based web app
- ✅ **Puter Apps** - Apps running in Puter's cloud desktop
- ✅ **Node.js** - Backend services and CLI tools
- ✅ **Serverless Workers** - Puter's serverless backend

---

## 🔐 Authentication

### Methods

#### `puter.auth.signIn()`
Initiate sign-in process for a user.

**Parameters:**
- `options` (Object, optional):
  - `redirectUrl` (string): URL to redirect to after sign-in
  - `appName` (string): Name of your app (for branding)
  - `appIcon` (string): URL of your app's icon

**Returns:** Promise<[SignInResult](#signinresult)> - Resolves when user has signed in.

**Example:**
```javascript
// Must be called from a user action (e.g., button click)
document.getElementById('sign-in').addEventListener('click', async () => {
  const result = await puter.auth.signIn();
  console.log('Signed in as:', result.user);
});
```

#### `puter.auth.signOut()`
Sign out the current user.

**Returns:** Promise<void>

**Example:**
```javascript
await puter.auth.signOut();
console.log('Signed out');
```

#### `puter.auth.isSignedIn()`
Check if a user is currently signed in.

**Returns:** Promise<boolean>

**Example:**
```javascript
if (await puter.auth.isSignedIn()) {
  console.log('User is signed in');
}
```

#### `puter.auth.getUser()`
Get the authenticated user's information.

**Returns:** Promise<[User](#user)> - The authenticated user object.

**Example:**
```javascript
const user = await puter.auth.getUser();
console.log('User ID:', user.id);
console.log('Username:', user.username);
console.log('Email:', user.email);
```

#### `puter.auth.getMonthlyUsage()`
Get the user's current monthly resource usage.

**Returns:** Promise<[MonthlyUsage](#monthlyusage)>

**Example:**
```javascript
const usage = await puter.auth.getMonthlyUsage();
console.log('Storage used:', usage.storage.used);
console.log('AI credits used:', usage.ai.used);
```

#### `puter.auth.getDetailedAppUsage()`
Get detailed usage statistics for an application.

**Parameters:**
- `appId` (string): The ID of the app to get usage for

**Returns:** Promise<[DetailedAppUsage](#detailedappusage)>

---

## 🗂️ File System (FS)

### Overview
Puter provides a **complete file system API** for cloud storage with:
- Familiar file operations (read, write, delete, copy, move)
- Directory management
- File sharing and permissions
- Metadata and versioning
- Large file support with chunked uploads

### File Operations

#### `puter.fs.write()`
Write data to a file in the user's cloud storage.

**Parameters:**
- `path` (string): Path to the file (e.g., 'documents/report.txt')
- `data` (string | Blob | ArrayBuffer | ReadableStream): Content to write
- `options` (Object, optional):
  - `overwrite` (boolean, default: false): Whether to overwrite existing file
  - `createPath` (boolean, default: true): Create parent directories if they don't exist
  - `createFile` (boolean, default: true): Create the file if it doesn't exist

**Returns:** Promise<[FSItem](#fsitem)> - The created/updated file object.

**Example:**
```javascript
// Write a text file
const file = await puter.fs.write('hello.txt', 'Hello, world!');
console.log('File written:', file.path);

// Write a binary file
const blob = new Blob(['binary data'], { type: 'application/octet-stream' });
await puter.fs.write('data.bin', blob);

// Write with options
await puter.fs.write('path/to/file.txt', 'content', { 
  overwrite: true, 
  createPath: true 
});
```

#### `puter.fs.read()`
Read data from a file.

**Parameters:**
- `path` (string): Path to the file
- `options` (Object, optional):
  - `start` (number): Byte offset to start reading from
  - `end` (number): Byte offset to end reading at

**Returns:** Promise<Blob> - The file content as a Blob.

**Example:**
```javascript
const blob = await puter.fs.read('hello.txt');
const text = await blob.text();
console.log(text); // "Hello, world!"

// Read a portion of a file
const partialBlob = await puter.fs.read('large-file.bin', { start: 0, end: 1024 });
```

#### `puter.fs.delete()`
Delete a file or directory.

**Parameters:**
- `path` (string): Path to the file or directory
- `options` (Object, optional):
  - `recursive` (boolean, default: false): Delete directory contents recursively

**Returns:** Promise<void>

**Example:**
```javascript
// Delete a file
await puter.fs.delete('hello.txt');

// Delete a directory and its contents
await puter.fs.delete('old-project', { recursive: true });
```

#### `puter.fs.rename()`
Rename a file or directory.

**Parameters:**
- `oldPath` (string): Current path
- `newPath` (string): New path

**Returns:** Promise<[FSItem](#fsitem)> - The renamed file/directory object.

**Example:**
```javascript
await puter.fs.rename('old-name.txt', 'new-name.txt');
```

#### `puter.fs.copy()`
Copy a file or directory.

**Parameters:**
- `src` (string): Source path
- `dest` (string): Destination path
- `options` (Object, optional):
  - `overwrite` (boolean, default: false): Overwrite destination if it exists

**Returns:** Promise<[FSItem](#fsitem)> - The copied file/directory object.

**Example:**
```javascript
// Copy a file
await puter.fs.copy('source.txt', 'destination.txt');

// Copy a directory
await puter.fs.copy('source-folder', 'dest-folder', { overwrite: true });
```

#### `puter.fs.move()`
Move a file or directory.

**Parameters:**
- `src` (string): Source path
- `dest` (string): Destination path
- `options` (Object, optional):
  - `overwrite` (boolean, default: false): Overwrite destination if it exists

**Returns:** Promise<[FSItem](#fsitem)> - The moved file/directory object.

**Example:**
```javascript
await puter.fs.move('old-location/file.txt', 'new-location/file.txt');
```

### Directory Operations

#### `puter.fs.mkdir()`
Create a new directory.

**Parameters:**
- `path` (string): Path to the directory
- `options` (Object, optional):
  - `recursive` (boolean, default: false): Create parent directories if they don't exist

**Returns:** Promise<[FSItem](#fsitem)> - The created directory object.

**Example:**
```javascript
// Create a single directory
await puter.fs.mkdir('new-folder');

// Create nested directories
await puter.fs.mkdir('parent/child/grandchild', { recursive: true });
```

#### `puter.fs.readdir()`
List files and directories in a directory.

**Parameters:**
- `path` (string): Path to the directory (default: current directory)
- `options` (Object, optional):
  - `recursive` (boolean, default: false): List contents recursively
  - `includeHidden` (boolean, default: false): Include hidden files

**Returns:** Promise<Array<[FSItem](#fsitem)>> - Array of file/directory objects.

**Example:**
```javascript
const items = await puter.fs.readdir('./');
items.forEach(item => {
  console.log(item.name, item.type, item.size);
});

// Recursive listing
const allItems = await puter.fs.readdir('.', { recursive: true });
```

### File Metadata & Info

#### `puter.fs.stat()`
Get information about a file or directory.

**Parameters:**
- `path` (string): Path to the file or directory

**Returns:** Promise<[FSItem](#fsitem)> - The file/directory information.

**Example:**
```javascript
const fileInfo = await puter.fs.stat('hello.txt');
console.log('Name:', fileInfo.name);
console.log('Size:', fileInfo.size);
console.log('Created:', fileInfo.created);
console.log('Modified:', fileInfo.modified);
console.log('Type:', fileInfo.type); // 'file' or 'directory'
```

### Upload & Download

#### `puter.fs.upload()`
Upload local files to the cloud.

**Parameters:**
- `files` (FileList | Array<File> | HTMLInputElement): Files to upload
- `options` (Object, optional):
  - `path` (string): Destination path (default: original filename)
  - `overwrite` (boolean, default: false): Overwrite if file exists
  - `onProgress` (function): Callback for upload progress
  - `onThumbnail` (function): Callback for generating thumbnails

**Returns:** Promise<[FSItem](#fsitem) | Array<[FSItem](#fsitem)>> - Uploaded file(s).

**Example:**
```javascript
// From file input
const fileInput = document.getElementById('file-input');
fileInput.onchange = async () => {
  const files = await puter.fs.upload(fileInput.files);
  console.log('Uploaded:', files);
};

// With progress tracking
const files = await puter.fs.upload(fileInput.files, {
  onProgress: (progress) => {
    console.log('Upload progress:', progress.loaded, '/', progress.total);
  }
});
```

#### `puter.fs.getReadURL()`
Generate a temporary URL for reading a file.

**Parameters:**
- `path` (string): Path to the file
- `options` (Object, optional):
  - `ttl` (number): Time-to-live in seconds (default: 3600)

**Returns:** Promise<string> - The temporary read URL.

**Example:**
```javascript
const url = await puter.fs.getReadURL('hello.txt');
console.log('Read URL:', url); // Can be shared with others
```

### Sharing & Permissions

#### `puter.fs.share()`
Share a file or directory with another user.

**Parameters:**
- `path` (string): Path to the file or directory
- `user` (string): Username or user ID to share with
- `options` (Object, optional):
  - `read` (boolean, default: true): Allow read access
  - `write` (boolean, default: false): Allow write access
  - `expires` (number): Expiration timestamp (in seconds)

**Returns:** Promise<void>

**Example:**
```javascript
// Share with read-only access
await puter.fs.share('project-file.txt', 'john-doe', { read: true, write: false });

// Share with expiration
await puter.fs.share('temp-file.txt', 'jane-smith', { 
  read: true, 
  expires: Date.now() / 1000 + 86400 // Expires in 24 hours
});
```

#### `puter.fs.unshare()`
Remove sharing permissions for a file or directory.

**Parameters:**
- `path` (string): Path to the file or directory
- `user` (string): Username or user ID to unshare from

**Returns:** Promise<void>

**Example:**
```javascript
await puter.fs.unshare('project-file.txt', 'john-doe');
```

#### `puter.fs.listShared()`
List files and directories shared with the current user.

**Returns:** Promise<Array<[FSItem](#fsitem)>> - Array of shared items.

**Example:**
```javascript
const sharedItems = await puter.fs.listShared();
sharedItems.forEach(item => {
  console.log('Shared:', item.path, 'by', item.sharedBy);
});
```

#### `puter.fs.getShares()`
List users who have access to a shared file or directory.

**Parameters:**
- `path` (string): Path to the file or directory

**Returns:** Promise<Array<{user: string, read: boolean, write: boolean, expires?: number}>>

**Example:**
```javascript
const shares = await puter.fs.getShares('project-file.txt');
shares.forEach(share => {
  console.log('Shared with:', share.user, 'Permissions:', share);
});
```

---

## 🤖 AI Capabilities

### Overview
Puter provides access to **500+ AI models** from various providers:
- **OpenAI**: GPT-4, GPT-3.5, o1, o3-mini, etc.
- **Anthropic**: Claude 3, Claude 2, etc.
- **Google**: Gemini 1.5, Gemini 2.0, etc.
- **xAI**: Grok-2, Grok-1, etc.
- **Mistral**: Mistral Large, Mixtral, etc.
- **DeepSeek**: DeepSeek Chat, DeepSeek Coder, etc.
- **And many more...**

### Chat & Text Generation

#### `puter.ai.chat()`
Chat with AI models and get text responses.

**Parameters:**
- `prompt` (string | Array<{role: string, content: string}>): The user's message or array of chat messages
- `options` (Object, optional):
  - `model` (string): Model to use (default: 'gpt-4o-mini')
  - `stream` (boolean, default: false): Whether to stream the response
  - `maxTokens` (number): Maximum tokens in response
  - `temperature` (number, 0-2): Creativity level
  - `topP` (number, 0-1): Nucleus sampling parameter
  - `images` (Array<string | Blob>): Images for vision models
  - `tools` (Array): Tool definitions for function calling
  - `toolChoice` (string): How to choose tools ('auto', 'none', or specific tool)
  - `testMode` (boolean): Use test mode (no credits consumed)

**Returns:** Promise<[ChatResponse](#chatresponse) | AsyncIterable<[ChatResponseChunk](#chatresponsechunk)>>

**Example:**
```javascript
// Simple chat
const response = await puter.ai.chat('What is the capital of France?');
console.log(response.text); // "The capital of France is Paris."

// With specific model
const response = await puter.ai.chat('Explain quantum computing', {
  model: 'gpt-4o',
  maxTokens: 1000,
  temperature: 0.7
});

// Streaming response
const stream = await puter.ai.chat('Tell me a story', { stream: true });
for await (const chunk of stream) {
  process.stdout.write(chunk.text);
}

// With images (vision)
const response = await puter.ai.chat('What is in this image?', {
  model: 'gpt-4o',
  images: ['https://example.com/image.jpg']
});

// Function calling
const response = await puter.ai.chat('What is the weather in SF?', {
  tools: [{
    type: 'function',
    function: {
      name: 'getWeather',
      description: 'Get weather for a location',
      parameters: {
        type: 'object',
        properties: {
          location: { type: 'string' }
        }
      }
    }
  }],
  toolChoice: 'auto'
});
```

#### `puter.ai.listModels()`
List available AI chat models.

**Returns:** Promise<Array<{id: string, name: string, provider: string, pricing: object}>>

**Example:**
```javascript
const models = await puter.ai.listModels();
models.forEach(model => {
  console.log(model.id, model.name, model.provider);
});
```

#### `puter.ai.listModelProviders()`
List available AI model providers.

**Returns:** Promise<Array<{id: string, name: string}>>

**Example:**
```javascript
const providers = await puter.ai.listModelProviders();
providers.forEach(provider => {
  console.log(provider.id, provider.name);
});
```

### Image Generation

#### `puter.ai.txt2img()`
Generate images from text prompts.

**Parameters:**
- `prompt` (string): Text description of the image
- `testMode` (boolean, default: false): Use test mode (returns sample image, no credits consumed)
- `options` (Object, optional):
  - `model` (string): Image model to use (default: 'gpt-image')
  - `quality` (string): Quality level ('standard', 'hd')
  - `style` (string): Artistic style
  - `ar` (string): Aspect ratio ('1:1', '16:9', '9:16', etc.)
  - `size` (string): Image size ('256x256', '512x512', '1024x1024', etc.)
  - `n` (number): Number of images to generate (1-10)

**Returns:** Promise<Blob | Array<Blob>> - Generated image(s) as Blob(s).

**Example:**
```javascript
// Generate a single image
const imageBlob = await puter.ai.txt2img('A beautiful sunset over mountains');
const imageUrl = URL.createObjectURL(imageBlob);

// Generate with options
const images = await puter.ai.txt2img('A cyberpunk city at night', {
  model: 'flux',
  quality: 'hd',
  ar: '16:9',
  n: 3
});

// Test mode (no credits)
const testImage = await puter.ai.txt2img('A test image', true);
```

### Image Analysis

#### `puter.ai.img2txt()`
Extract text from images using OCR.

**Parameters:**
- `image` (string | Blob): Image URL or Blob
- `options` (Object, optional):
  - `model` (string): OCR model to use
  - `language` (string): Language for text recognition

**Returns:** Promise<string> - Extracted text.

**Example:**
```javascript
// From URL
const text = await puter.ai.img2txt('https://example.com/receipt.jpg');
console.log(text);

// From Blob
const blob = await puter.fs.read('receipt.jpg');
const text = await puter.ai.img2txt(blob);
```

### Speech & Audio

#### `puter.ai.txt2speech()`
Convert text to speech.

**Parameters:**
- `text` (string): Text to convert to speech
- `options` (Object, optional):
  - `voice` (string): Voice ID to use
  - `model` (string): TTS model to use
  - `language` (string): Language code
  - `speed` (number): Speech speed (0.5-2.0)
  - `pitch` (number): Speech pitch (0.5-2.0)
  - `format` (string): Audio format ('mp3', 'wav', 'ogg', etc.)

**Returns:** Promise<Blob> - Audio file as Blob.

**Example:**
```javascript
const audioBlob = await puter.ai.txt2speech('Hello, world!');
const audioUrl = URL.createObjectURL(audioBlob);
const audio = new Audio(audioUrl);
audio.play();

// With specific voice
const audioBlob = await puter.ai.txt2speech('Hello, world!', {
  voice: 'alloy',
  model: 'tts-1',
  speed: 1.0,
  format: 'mp3'
});
```

#### `puter.ai.txt2speech.listEngines()`
List available TTS engines/models.

**Returns:** Promise<Array<[TTSEngine](#ttsengine)>>

**Example:**
```javascript
const engines = await puter.ai.txt2speech.listEngines();
engines.forEach(engine => {
  console.log(engine.id, engine.name, engine.pricing);
});
```

#### `puter.ai.txt2speech.listVoices()`
List available TTS voices.

**Parameters:**
- `options` (Object, optional):
  - `provider` (string): Filter by provider
  - `language` (string): Filter by language

**Returns:** Promise<Array<[TTSVoice](#ttsvoice)>>

**Example:**
```javascript
const voices = await puter.ai.txt2speech.listVoices();
voices.forEach(voice => {
  console.log(voice.id, voice.name, voice.language);
});

// Filter by language
const englishVoices = await puter.ai.txt2speech.listVoices({ language: 'en' });
```

#### `puter.ai.speech2txt()`
Transcribe audio to text.

**Parameters:**
- `audio` (string | Blob): Audio URL or Blob
- `options` (Object, optional):
  - `model` (string): STT model to use
  - `language` (string): Language code
  - `translate` (boolean): Whether to translate to English

**Returns:** Promise<[Speech2TxtResult](#speech2txtresult)>

**Example:**
```javascript
// From URL
const result = await puter.ai.speech2txt('https://example.com/audio.mp3');
console.log(result.text);

// From Blob
const blob = await puter.fs.read('meeting.mp3');
const result = await puter.ai.speech2txt(blob, {
  model: 'whisper-1',
  language: 'en'
});

// With translation
const result = await puter.ai.speech2txt('audio.mp3', {
  translate: true
});
```

#### `puter.ai.speech2speech()`
Convert speech from one voice to another.

**Parameters:**
- `audio` (string | Blob): Audio URL or Blob
- `options` (Object, optional):
  - `voice` (string): Target voice ID
  - `model` (string): Speech-to-speech model
  - `output_format` (string): Output audio format

**Returns:** Promise<Blob> - Converted audio as Blob.

**Example:**
```javascript
const convertedAudio = await puter.ai.speech2speech('original.mp3', {
  voice: '21m00Tcm4TlvDq8ikWAM',
  model: 'eleven_multilingual_sts_v2',
  output_format: 'mp3_44100_128'
});
```

### Video Generation

#### `puter.ai.txt2vid()`
Generate video from text prompts.

**Parameters:**
- `prompt` (string): Text description of the video
- `testMode` (boolean, default: false): Use test mode (returns sample video, no credits consumed)
- `options` (Object, optional):
  - `model` (string): Video model to use (default: 'veo')
  - `duration` (number): Video duration in seconds
  - `aspectRatio` (string): Aspect ratio ('16:9', '9:16', '1:1')
  - `style` (string): Video style
  - `seed` (number): Random seed for reproducibility

**Returns:** Promise<Blob> - Generated video as Blob.

**Example:**
```javascript
// Generate a video
const videoBlob = await puter.ai.txt2vid('A drone shot over a tropical island');
const videoUrl = URL.createObjectURL(videoBlob);

// With options
const videoBlob = await puter.ai.txt2vid('A product commercial for a smartwatch', {
  model: 'veo',
  duration: 30,
  aspectRatio: '16:9'
});

// Test mode
const testVideo = await puter.ai.txt2vid('A test video', true);
```

---

## 🗃️ Key-Value Store (KV)

### Overview
Puter provides a **serverless NoSQL key-value store** for each app and user:
- Simple key-value storage
- Nested object support
- Atomic operations
- Expiration (TTL)
- List and query operations

### Basic Operations

#### `puter.kv.set()`
Set a value in the key-value store.

**Parameters:**
- `key` (string): The key to set
- `value` (any): The value to store (will be JSON-serialized)
- `options` (Object, optional):
  - `ttl` (number): Time-to-live in seconds
  - `expiresAt` (number): Expiration timestamp in seconds

**Returns:** Promise<void>

**Example:**
```javascript
// Set a simple value
await puter.kv.set('userPreference', 'darkMode');

// Set an object
await puter.kv.set('userSettings', {
  theme: 'dark',
  notifications: true,
  language: 'en'
});

// Set with expiration
await puter.kv.set('tempData', { value: 123 }, { ttl: 3600 }); // Expires in 1 hour
```

#### `puter.kv.get()`
Get a value from the key-value store.

**Parameters:**
- `key` (string): The key to retrieve

**Returns:** Promise<any> - The stored value (parsed from JSON).

**Example:**
```javascript
const preference = await puter.kv.get('userPreference');
console.log(preference); // 'darkMode'

const settings = await puter.kv.get('userSettings');
console.log(settings.theme); // 'dark'
```

#### `puter.kv.del()`
Delete a key from the key-value store.

**Parameters:**
- `key` (string): The key to delete

**Returns:** Promise<void>

**Example:**
```javascript
await puter.kv.del('tempData');
```

#### `puter.kv.list()`
List all keys in the key-value store.

**Parameters:**
- `options` (Object, optional):
  - `prefix` (string): Only list keys starting with this prefix
  - `limit` (number): Maximum number of keys to return
  - `cursor` (string): Pagination cursor

**Returns:** Promise<[KVListPage](#kvlistpage)>

**Example:**
```javascript
const { keys, cursor } = await puter.kv.list();
keys.forEach(key => {
  console.log(key.name);
});

// List with prefix
const { keys } = await puter.kv.list({ prefix: 'user_' });

// Paginated listing
let cursor;
do {
  const result = await puter.kv.list({ limit: 10, cursor });
  result.keys.forEach(key => console.log(key.name));
  cursor = result.cursor;
} while (cursor);
```

#### `puter.kv.flush()`
Delete all keys from the key-value store.

**Returns:** Promise<void>

**Example:**
```javascript
// WARNING: This deletes ALL keys
await puter.kv.flush();
```

### Advanced Operations

#### `puter.kv.incr()`
Increment a numeric value.

**Parameters:**
- `key` (string): The key to increment
- `amount` (number, default: 1): Amount to increment by

**Returns:** Promise<number> - The new value.

**Example:**
```javascript
const count = await puter.kv.incr('pageViews');
console.log(count); // Previous value + 1

const newValue = await puter.kv.incr('counter', 5);
console.log(newValue); // Previous value + 5
```

#### `puter.kv.decr()`
Decrement a numeric value.

**Parameters:**
- `key` (string): The key to decrement
- `amount` (number, default: 1): Amount to decrement by

**Returns:** Promise<number> - The new value.

**Example:**
```javascript
const count = await puter.kv.decr('remainingItems');
```

#### `puter.kv.add()`
Add a value to an array at a key.

**Parameters:**
- `key` (string): The key
- `value` (any): The value to add to the array

**Returns:** Promise<Array> - The updated array.

**Example:**
```javascript
// Add to array (creates array if it doesn't exist)
const items = await puter.kv.add('todoList', 'Buy groceries');
console.log(items); // ['Buy groceries']

const updatedItems = await puter.kv.add('todoList', 'Walk the dog');
console.log(updatedItems); // ['Buy groceries', 'Walk the dog']
```

#### `puter.kv.remove()`
Remove a value from an array at a key.

**Parameters:**
- `key` (string): The key
- `value` (any): The value to remove from the array

**Returns:** Promise<Array> - The updated array.

**Example:**
```javascript
const items = await puter.kv.remove('todoList', 'Buy groceries');
console.log(items); // ['Walk the dog']
```

#### `puter.kv.update()`
Update a specific path within a stored value.

**Parameters:**
- `key` (string): The key
- `path` (string): The path within the value (e.g., 'user.profile.name')
- `value` (any): The new value for that path

**Returns:** Promise<any> - The updated value.

**Example:**
```javascript
// Set initial value
await puter.kv.set('user', {
  profile: { name: 'John', age: 30 },
  settings: { theme: 'dark' }
});

// Update a nested path
const updatedUser = await puter.kv.update('user', 'profile.name', 'Jane');
console.log(updatedUser.profile.name); // 'Jane'
```

#### `puter.kv.expire()`
Set expiration for a key.

**Parameters:**
- `key` (string): The key
- `ttl` (number): Time-to-live in seconds

**Returns:** Promise<void>

**Example:**
```javascript
await puter.kv.expire('tempKey', 3600); // Expires in 1 hour
```

#### `puter.kv.expireAt()`
Set expiration timestamp for a key.

**Parameters:**
- `key` (string): The key
- `timestamp` (number): Expiration timestamp in seconds

**Returns:** Promise<void>

**Example:**
```javascript
// Expires at a specific time
const tomorrow = Math.floor(Date.now() / 1000) + 86400;
await puter.kv.expireAt('dailyKey', tomorrow);
```

### Constants

#### `puter.kv.MAX_KEY_SIZE`
Maximum key size in bytes.

**Returns:** number

#### `puter.kv.MAX_VALUE_SIZE`
Maximum value size in bytes.

**Returns:** number

---

## ⚙️ Serverless Workers

### Overview
Puter Workers allow you to run **server-side JavaScript** in the cloud:
- HTTP request handling
- Shared backend logic
- Scheduled tasks
- Database access
- File system access

### Worker Management

#### `puter.workers.create()`
Create and deploy a worker.

**Parameters:**
- `name` (string): Name of the worker
- `code` (string): JavaScript code for the worker
- `options` (Object, optional):
  - `env` (Object): Environment variables
  - `secrets` (Object): Secret environment variables

**Returns:** Promise<[WorkerDeployment](#workerdeployment)>

**Example:**
```javascript
const worker = await puter.workers.create('my-api', `
  export default {
    async fetch(request) {
      return new Response('Hello, world!');
    }
  };
`);
console.log('Worker deployed:', worker.url);
```

#### `puter.workers.delete()`
Delete a worker.

**Parameters:**
- `name` (string): Name of the worker to delete

**Returns:** Promise<void>

**Example:**
```javascript
await puter.workers.delete('my-api');
```

#### `puter.workers.list()`
List all workers.

**Returns:** Promise<Array<[WorkerInfo](#workerinfo)>>

**Example:**
```javascript
const workers = await puter.workers.list();
workers.forEach(worker => {
  console.log(worker.name, worker.url);
});
```

#### `puter.workers.get()`
Get information about a specific worker.

**Parameters:**
- `name` (string): Name of the worker

**Returns:** Promise<[WorkerInfo](#workerinfo)>

**Example:**
```javascript
const worker = await puter.workers.get('my-api');
console.log(worker);
```

#### `puter.workers.exec()`
Execute a worker function as the calling user.

**Parameters:**
- `name` (string): Name of the worker
- `path` (string): Path to the function to execute
- `args` (Array): Arguments to pass to the function

**Returns:** Promise<any> - The result of the function execution.

**Example:**
```javascript
// Assuming the worker has an exported function
const result = await puter.workers.exec('my-worker', 'processData', [1, 2, 3]);
console.log(result);
```

### Worker Router

Puter provides a built-in router for handling HTTP requests:

```javascript
// In your worker code
import { router } from '@heyputer/worker-router';

router.get('/api/hello', async ({ request }) => {
  return { message: 'Hello, world!' };
});

router.post('/api/data', async ({ request }) => {
  const body = await request.json();
  return { processed: true, data: body };
});

router.all('*', async () => {
  return { error: 'Not found' }, { status: 404 };
});

export default router;
```

### Dynamic Workers

Create workers by dropping a `.worker.js` file into a `__workers` folder in your hosted website:

```
my-website/
├── index.html
├── __workers/
│   └── api.worker.js
```

The worker will be automatically deployed and available at `https://<subdomain>.puter.site/api`.

---

## 🌐 Hosting

### Overview
Puter provides **static website hosting** with:
- Instant deployment from any directory
- Custom domains
- Automatic SSL certificates
- CDN distribution
- Password protection

### Hosting Operations

#### `puter.hosting.create()`
Create and host a website from a directory.

**Parameters:**
- `subdomain` (string): Subdomain for the website (e.g., 'my-site')
- `dir` (string): Path to the directory containing the website files
- `options` (Object, optional):
  - `password` (string): Password to protect the website

**Returns:** Promise<[Subdomain](#subdomain)> - The created subdomain information.

**Example:**
```javascript
// Host the current directory
const site = await puter.hosting.create('my-website', '.');
console.log('Website hosted at:', site.url); // https://my-website.puter.site

// With password protection
const site = await puter.hosting.create('private-site', 'private-folder', {
  password: 'secret123'
});
```

#### `puter.hosting.list()`
List all hosted subdomains.

**Returns:** Promise<Array<[Subdomain](#subdomain)>>

**Example:**
```javascript
const sites = await puter.hosting.list();
sites.forEach(site => {
  console.log(site.subdomain, site.url);
});
```

#### `puter.hosting.delete()`
Delete a hosted subdomain.

**Parameters:**
- `subdomain` (string): Subdomain to delete

**Returns:** Promise<void>

**Example:**
```javascript
await puter.hosting.delete('my-website');
```

#### `puter.hosting.update()`
Update a subdomain to point to a new directory.

**Parameters:**
- `subdomain` (string): Subdomain to update
- `dir` (string): New directory path

**Returns:** Promise<[Subdomain](#subdomain)>

**Example:**
```javascript
await puter.hosting.update('my-website', 'new-folder');
```

#### `puter.hosting.get()`
Get information about a subdomain.

**Parameters:**
- `subdomain` (string): Subdomain to get information for

**Returns:** Promise<[Subdomain](#subdomain)>

**Example:**
```javascript
const site = await puter.hosting.get('my-website');
console.log(site);
```

---

## 📡 Events & Real-Time

### Overview
Puter provides **real-time event subscriptions** for:
- File system changes
- Key-value store changes
- Custom events
- Persistent subscriptions that run even when your app is closed

### Local Subscriptions

#### `puter.events.onLocal()`
Subscribe to changes on a file, directory, or key-value key for the current session.

**Parameters:**
- `subject` (string): The subject to watch (file path, directory path, or KV key)
- `handler` (function): Callback function when changes occur

**Returns:** [Subscription](#subscription) - The subscription object.

**Example:**
```javascript
// Watch a file for changes
const subscription = puter.events.onLocal('data.json', (event) => {
  console.log('File changed:', event);
});

// Watch a KV key
const kvSubscription = puter.events.onLocal('userSettings', (event) => {
  console.log('Settings changed:', event);
});

// Watch a directory
const dirSubscription = puter.events.onLocal('documents/', (event) => {
  console.log('Directory changed:', event);
});
```

#### `subscription.off()`
End a local subscription.

**Example:**
```javascript
subscription.off();
```

### Persistent Subscriptions

#### `puter.events.onPersistent()`
Create a persistent subscription that continues running when your app is closed.

**Parameters:**
- `subject` (string): The subject to watch
- `handler` (string | function): Handler function or name of a published handler
- `options` (Object, optional):
  - `name` (string): Name for the subscription

**Returns:** Promise<{id: string, name?: string}>

**Example:**
```javascript
// With inline function
const sub = await puter.events.onPersistent('data.json', (event) => {
  console.log('Persistent change detected:', event);
});

// With named handler (must be published first)
await puter.events.handlers.publish('myHandler', (event) => {
  console.log('Handled:', event);
});
const sub = await puter.events.onPersistent('data.json', 'myHandler');
```

#### `puter.events.list()`
List persistent subscriptions.

**Returns:** Promise<Array<{id: string, subject: string, handler: string, created: number}>>

**Example:**
```javascript
const subscriptions = await puter.events.list();
subscriptions.forEach(sub => {
  console.log(sub.id, sub.subject);
});
```

#### `puter.events.unsubscribe()`
End a persistent subscription.

**Parameters:**
- `id` (string): Subscription ID

**Returns:** Promise<void>

**Example:**
```javascript
await puter.events.unsubscribe('subscription-id');
```

#### `puter.events.fetch()`
Read what a subject recorded while nothing was listening.

**Parameters:**
- `subject` (string): The subject to fetch events for
- `options` (Object, optional):
  - `since` (number): Only fetch events after this timestamp
  - `limit` (number): Maximum number of events to fetch

**Returns:** Promise<Array<{type: string, timestamp: number, data: any}>>

**Example:**
```javascript
const events = await puter.events.fetch('data.json', {
  since: Date.now() - 3600000, // Last hour
  limit: 10
});
```

### Handlers

#### `puter.events.handlers`
Publish, list, and remove named handlers for persistent subscriptions.

**Methods:**
- `publish(name, handler)`: Publish a handler
- `list()`: List published handlers
- `remove(name)`: Remove a published handler

**Example:**
```javascript
// Publish a handler
await puter.events.handlers.publish('logHandler', (event) => {
  console.log('Event:', event);
});

// List handlers
const handlers = await puter.events.handlers.list();
console.log(handlers);

// Remove a handler
await puter.events.handlers.remove('logHandler');
```

### Workers

#### `puter.events.workers`
List and destroy event workers.

**Methods:**
- `list()`: List event workers
- `destroy(workerId)`: Destroy an event worker

---

## 🌍 Networking

### Overview
Puter provides **browser-based networking** capabilities:
- CORS-free HTTP requests
- WebSocket support
- Raw TCP sockets
- TLS sockets
- Peer-to-peer connections

### HTTP Requests

#### `puter.net.fetch()`
Make HTTP requests without CORS restrictions.

**Parameters:**
- `url` (string): URL to fetch
- `options` (Object, optional): Standard fetch options

**Returns:** Promise<Response> - The HTTP response.

**Example:**
```javascript
// Simple GET request
const response = await puter.net.fetch('https://api.example.com/data');
const data = await response.json();

// POST request with body
const response = await puter.net.fetch('https://api.example.com/data', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ key: 'value' })
});

// With custom headers
const response = await puter.net.fetch('https://api.example.com/data', {
  headers: { 'Authorization': 'Bearer token' }
});
```

### Sockets

#### `puter.net.Socket`
Create a raw TCP socket connection.

**Example:**
```javascript
const socket = new puter.net.Socket();

socket.on('connect', () => {
  console.log('Connected');
  socket.write('Hello, server!');
});

socket.on('data', (data) => {
  console.log('Received:', data);
});

socket.on('close', () => {
  console.log('Connection closed');
});

socket.connect({ host: 'example.com', port: 80 });
```

#### `puter.net.TLSSocket`
Create a TLS-protected TCP socket connection.

**Example:**
```javascript
const socket = new puter.net.TLSSocket();

socket.on('connect', () => {
  console.log('TLS connected');
  socket.write('Secure message');
});

socket.connect({ host: 'example.com', port: 443 });
```

---

## 🔗 Peer-to-Peer

### Overview
Puter provides **WebRTC-based peer-to-peer** connections for:
- Direct browser-to-browser communication
- File sharing
- Real-time collaboration
- Gaming

### Peer Server

#### `puter.peer.serve()`
Create a peer server and generate an invite code.

**Parameters:**
- `options` (Object, optional):
  - `name` (string): Name for the server
  - `maxConnections` (number): Maximum number of connections

**Returns:** Promise<[PuterPeerServer](#puterpeerserver)> - The peer server object.

**Example:**
```javascript
const server = await puter.peer.serve({
  name: 'My Game Server',
  maxConnections: 10
});

console.log('Invite code:', server.inviteCode);
console.log('Server ID:', server.id);

// Handle connections
server.on('connection', (connection) => {
  console.log('New connection:', connection.id);
  
  connection.on('message', (message) => {
    console.log('Received:', message);
    connection.send('Message received!');
  });
});
```

### Peer Connection

#### `puter.peer.connect()`
Connect to a peer server using an invite code.

**Parameters:**
- `inviteCode` (string): The invite code from the server
- `options` (Object, optional):
  - `name` (string): Your name for the connection

**Returns:** Promise<[PuterPeerConnection](#puterpeerconnection)> - The peer connection object.

**Example:**
```javascript
const connection = await puter.peer.connect('ABC123');

connection.on('open', () => {
  console.log('Connected to peer server');
  connection.send('Hello from client!');
});

connection.on('message', (message) => {
  console.log('Received:', message);
});

connection.on('close', () => {
  console.log('Connection closed');
});
```

### TURN Relays

#### `puter.peer.createGuestGrant()`
Create a grant that allows guests without Puter accounts to use TURN relays.

**Parameters:**
- `options` (Object, optional):
  - `expires` (number): Expiration timestamp in seconds

**Returns:** Promise<{grant: string, expires: number}>

**Example:**
```javascript
const grant = await puter.peer.createGuestGrant({
  expires: Math.floor(Date.now() / 1000) + 3600 // 1 hour
});
console.log('Grant:', grant.grant);
```

#### `puter.peer.ensureTurnRelays()`
Preload TURN relays for faster peer connections.

**Returns:** Promise<void>

**Example:**
```javascript
await puter.peer.ensureTurnRelays();
```

---

## 📱 Apps Management

### Overview
Manage **Puter Apps** - web-based applications that run in the Puter cloud desktop.

### App Operations

#### `puter.apps.create()`
Create a new app in the Puter desktop environment.

**Parameters:**
- `options` (Object):
  - `name` (string): App name (must be unique)
  - `title` (string): Display title
  - `icon` (string): URL of app icon
  - `url` (string): URL to load when app is launched
  - `fileAssociations` (Array): File extensions to associate with the app

**Returns:** Promise<[CreateAppResult](#createappresult)>

**Example:**
```javascript
const app = await puter.apps.create({
  name: 'my-app',
  title: 'My Awesome App',
  icon: 'https://example.com/icon.png',
  url: 'https://example.com/app',
  fileAssociations: ['.txt', '.md']
});
console.log('App created:', app.id);
```

#### `puter.apps.list()`
List all apps in your Puter account.

**Returns:** Promise<Array<[App](#app)>>

**Example:**
```javascript
const apps = await puter.apps.list();
apps.forEach(app => {
  console.log(app.name, app.title);
});
```

#### `puter.apps.delete()`
Delete an app from your Puter account.

**Parameters:**
- `name` (string): App name to delete

**Returns:** Promise<void>

**Example:**
```javascript
await puter.apps.delete('my-app');
```

#### `puter.apps.update()`
Update app properties.

**Parameters:**
- `name` (string): App name to update
- `options` (Object): Properties to update (same as create)

**Returns:** Promise<[App](#app)>

**Example:**
```javascript
const updatedApp = await puter.apps.update('my-app', {
  title: 'My Updated App',
  icon: 'https://example.com/new-icon.png'
});
```

#### `puter.apps.get()`
Get details of a specific app.

**Parameters:**
- `name` (string): App name to get

**Returns:** Promise<[App](#app)>

**Example:**
```javascript
const app = await puter.apps.get('my-app');
console.log(app);
```

#### `puter.apps.checkName()`
Check if an app name is available.

**Parameters:**
- `name` (string): App name to check

**Returns:** Promise<boolean> - true if available, false if taken.

**Example:**
```javascript
const isAvailable = await puter.apps.checkName('my-new-app');
console.log('Available:', isAvailable);
```

---

## 👥 Teams & Organizations

### Overview
Administer **Puter Teams** - organizations that pay for and manage multiple Puter accounts.

### Team Operations

#### `puter.teams.create()`
Create a new team.

**Parameters:**
- `name` (string): Team name
- `options` (Object, optional):
  - `handle` (string): Team handle (must be unique)

**Returns:** Promise<{id: string, name: string, handle: string}>

**Example:**
```javascript
const team = await puter.teams.create('My Company', {
  handle: 'my-company'
});
console.log('Team created:', team.id);
```

#### `puter.teams.list()`
List teams you belong to.

**Returns:** Promise<Array<{id: string, name: string, handle: string, role: string}>>

**Example:**
```javascript
const teams = await puter.teams.list();
teams.forEach(team => {
  console.log(team.name, team.role);
});
```

#### `puter.teams.get()`
Get information about a specific team.

**Parameters:**
- `teamId` (string): Team ID

**Returns:** Promise<{id: string, name: string, handle: string, members: Array}>

**Example:**
```javascript
const team = await puter.teams.get('team-id');
console.log(team);
```

#### `puter.teams.update()`
Update team properties.

**Parameters:**
- `teamId` (string): Team ID
- `options` (Object): Properties to update
  - `name` (string): New team name
  - `handle` (string): New team handle

**Returns:** Promise<{id: string, name: string, handle: string}>

**Example:**
```javascript
const updatedTeam = await puter.teams.update('team-id', {
  name: 'New Company Name'
});
```

#### `puter.teams.delete()`
Delete a team.

**Parameters:**
- `teamId` (string): Team ID

**Returns:** Promise<void>

**Example:**
```javascript
await puter.teams.delete('team-id');
```

### Member Management

#### `puter.teams.listMembers()`
List accounts belonging to a team.

**Parameters:**
- `teamId` (string): Team ID

**Returns:** Promise<Array<{id: string, username: string, email: string, role: string}>>

**Example:**
```javascript
const members = await puter.teams.listMembers('team-id');
members.forEach(member => {
  console.log(member.username, member.role);
});
```

#### `puter.teams.createMember()`
Provision a new Puter account owned by a team.

**Parameters:**
- `teamId` (string): Team ID
- `options` (Object):
  - `username` (string): Username for the new account
  - `email` (string): Email for the new account
  - `password` (string): Initial password

**Returns:** Promise<{id: string, username: string, email: string}>

**Example:**
```javascript
const member = await puter.teams.createMember('team-id', {
  username: 'new-employee',
  email: 'new@company.com',
  password: 'temp-password-123'
});
```

#### `puter.teams.resendActivation()`
Resend activation email for an account that has never signed in.

**Parameters:**
- `teamId` (string): Team ID
- `memberId` (string): Member account ID

**Returns:** Promise<void>

**Example:**
```javascript
await puter.teams.resendActivation('team-id', 'member-id');
```

#### `puter.teams.disableMember()`
Suspend a team member account.

**Parameters:**
- `teamId` (string): Team ID
- `memberId` (string): Member account ID

**Returns:** Promise<void>

**Example:**
```javascript
await puter.teams.disableMember('team-id', 'member-id');
```

#### `puter.teams.enableMember()`
Restore a previously suspended account.

**Parameters:**
- `teamId` (string): Team ID
- `memberId` (string): Member account ID

**Returns:** Promise<void>

**Example:**
```javascript
await puter.teams.enableMember('team-id', 'member-id');
```

#### `puter.teams.resetPassword()`
Issue a new temporary password for a team member.

**Parameters:**
- `teamId` (string): Team ID
- `memberId` (string): Member account ID

**Returns:** Promise<{password: string}>

**Example:**
```javascript
const { password } = await puter.teams.resetPassword('team-id', 'member-id');
console.log('New password:', password);
```

#### `puter.teams.deleteMemberAccount()`
Permanently delete a team member account.

**Parameters:**
- `teamId` (string): Team ID
- `memberId` (string): Member account ID

**Returns:** Promise<void>

**Example:**
```javascript
await puter.teams.deleteMemberAccount('team-id', 'member-id');
```

### Audit Logs

#### `puter.teams.listAudit()`
Read a team's audit log of actions performed on member accounts.

**Parameters:**
- `teamId` (string): Team ID
- `options` (Object, optional):
  - `limit` (number): Maximum number of entries
  - `cursor` (string): Pagination cursor

**Returns:** Promise<{entries: Array, cursor?: string}>

**Example:**
```javascript
const { entries } = await puter.teams.listAudit('team-id');
entries.forEach(entry => {
  console.log(entry.timestamp, entry.action, entry.target);
});
```

#### `puter.teams.listOwnAudit()`
Read what a team did to your own account.

**Parameters:**
- `teamId` (string): Team ID

**Returns:** Promise<Array<{timestamp: number, action: string, details: any}>>

---

## 🎨 UI & Desktop Integration

### Overview
Puter provides **UI integration** for apps running in the Puter desktop environment.

### Authentication

#### `puter.ui.authenticateWithPuter()`
Present a dialog for the user to authenticate with their Puter account.

**Returns:** Promise<[SignInResult](#signinresult)>

**Example:**
```javascript
const result = await puter.ui.authenticateWithPuter();
console.log('Authenticated:', result.user);
```

### Dialogs

#### `puter.ui.alert()`
Display an alert dialog.

**Parameters:**
- `message` (string): Message to display
- `options` (Object, optional):
  - `title` (string): Dialog title
  - `type` (string): 'info', 'warning', 'error', 'success'

**Returns:** Promise<void>

**Example:**
```javascript
await puter.ui.alert('File saved successfully!', {
  title: 'Success',
  type: 'success'
});
```

#### `puter.ui.notify()`
Display a desktop notification.

**Parameters:**
- `message` (string): Notification message
- `options` (Object, optional):
  - `title` (string): Notification title
  - `icon` (string): URL of notification icon

**Returns:** Promise<void>

**Example:**
```javascript
await puter.ui.notify('New message received', {
  title: 'Notification',
  icon: 'https://example.com/icon.png'
});
```

#### `puter.ui.prompt()`
Display a prompt dialog.

**Parameters:**
- `message` (string): Prompt message
- `options` (Object, optional):
  - `title` (string): Dialog title
  - `defaultValue` (string): Default input value
  - `type` (string): 'text', 'password', 'number', etc.

**Returns:** Promise<string | null> - The user's input, or null if cancelled.

**Example:**
```javascript
const name = await puter.ui.prompt('Enter your name:', {
  title: 'Input Required',
  defaultValue: 'John Doe'
});
console.log('Name:', name);
```

### File Pickers

#### `puter.ui.showOpenFilePicker()`
Present a file picker dialog for selecting files from Puter cloud storage.

**Parameters:**
- `options` (Object, optional):
  - `multiple` (boolean): Allow selecting multiple files
  - `accept` (Array<string>): Accepted file types/extensions

**Returns:** Promise<Array<[FSItem](#fsitem)> | null> - Selected files, or null if cancelled.

**Example:**
```javascript
const files = await puter.ui.showOpenFilePicker({
  multiple: true,
  accept: ['.txt', '.md', '.pdf']
});
if (files) {
  files.forEach(file => console.log(file.path));
}
```

#### `puter.ui.showSaveFilePicker()`
Present a file picker dialog for specifying where to save a file.

**Parameters:**
- `options` (Object, optional):
  - `suggestedName` (string): Suggested filename
  - `accept` (Array<string>): Suggested file types/extensions

**Returns:** Promise<[FSItem](#fsitem) | null> - The save location, or null if cancelled.

**Example:**
```javascript
const saveLocation = await puter.ui.showSaveFilePicker({
  suggestedName: 'document.txt',
  accept: ['.txt']
});
if (saveLocation) {
  await puter.fs.write(saveLocation.path, 'File content');
}
```

#### `puter.ui.showDirectoryPicker()`
Present a directory picker dialog for selecting directories.

**Returns:** Promise<[FSItem](#fsitem) | null> - Selected directory, or null if cancelled.

**Example:**
```javascript
const dir = await puter.ui.showDirectoryPicker();
if (dir) {
  console.log('Selected directory:', dir.path);
}
```

### Window Management

#### `puter.ui.createWindow()`
Create and display a new window.

**Parameters:**
- `options` (Object):
  - `url` (string): URL to load in the window
  - `title` (string): Window title
  - `width` (number): Window width
  - `height` (number): Window height
  - `x` (number): X position
  - `y` (number): Y position

**Returns:** Promise<{id: string}>

**Example:**
```javascript
const window = await puter.ui.createWindow({
  url: 'https://example.com',
  title: 'My Window',
  width: 800,
  height: 600
});
```

#### `puter.ui.setWindowTitle()`
Set the title of the current window.

**Parameters:**
- `title` (string): New window title

**Example:**
```javascript
await puter.ui.setWindowTitle('My App - Editing Document');
```

#### `puter.ui.setWindowSize()`
Set the width and height of the current window.

**Parameters:**
- `width` (number): New width
- `height` (number): New height

**Example:**
```javascript
await puter.ui.setWindowSize(1024, 768);
```

#### `puter.ui.setWindowPosition()`
Set the position of the current window.

**Parameters:**
- `x` (number): X position
- `y` (number): Y position

**Example:**
```javascript
await puter.ui.setWindowPosition(100, 100);
```

#### `puter.ui.hideWindow()`
Hide the current window.

**Example:**
```javascript
await puter.ui.hideWindow();
```

#### `puter.ui.showWindow()`
Show the current window.

**Example:**
```javascript
await puter.ui.showWindow();
```

#### `puter.ui.exit()`
Terminate the running application and close its window.

**Example:**
```javascript
await puter.ui.exit();
```

### Menus

#### `puter.ui.setMenubar()`
Create a menubar in the UI.

**Parameters:**
- `menubar` (Array): Array of menu definitions

**Example:**
```javascript
await puter.ui.setMenubar([
  {
    label: 'File',
    submenu: [
      { label: 'New', click: () => console.log('New clicked') },
      { label: 'Open', click: () => console.log('Open clicked') },
      { type: 'separator' },
      { label: 'Exit', click: () => puter.ui.exit() }
    ]
  },
  {
    label: 'Edit',
    submenu: [
      { label: 'Cut', click: () => console.log('Cut clicked') },
      { label: 'Copy', click: () => console.log('Copy clicked') },
      { label: 'Paste', click: () => console.log('Paste clicked') }
    ]
  }
]);
```

#### `puter.ui.contextMenu()`
Display a context menu at the current cursor position.

**Parameters:**
- `menu` (Array): Array of menu item definitions

**Example:**
```javascript
await puter.ui.contextMenu([
  { label: 'Copy', click: () => console.log('Copy') },
  { label: 'Paste', click: () => console.log('Paste') },
  { type: 'separator' },
  { label: 'Delete', click: () => console.log('Delete') }
]);
```

### Events

#### `puter.ui.on()`
Listen to broadcast events from Puter.

**Parameters:**
- `eventName` (string): Event name to listen for
- `handler` (function): Callback function

**Returns:** Function to remove the listener.

**Example:**
```javascript
const off = puter.ui.on('themeChanged', (theme) => {
  console.log('Theme changed to:', theme);
});

// Later, remove the listener
off();
```

#### `puter.ui.onItemsOpened()`
Execute a function when one or more items have been opened.

**Parameters:**
- `handler` (function): Callback function receiving array of items

**Example:**
```javascript
puter.ui.onItemsOpened((items) => {
  console.log('Items opened:', items);
});
```

#### `puter.ui.onLaunchedWithItems()`
Execute a callback if the app is launched with items.

**Parameters:**
- `handler` (function): Callback function receiving array of items

**Example:**
```javascript
puter.ui.onLaunchedWithItems((items) => {
  console.log('App launched with items:', items);
});
```

#### `puter.ui.onWindowClose()`
Execute a function when the window is about to close.

**Parameters:**
- `handler` (function): Callback function

**Example:**
```javascript
puter.ui.onWindowClose(() => {
  console.log('Window closing...');
  // Save state, clean up, etc.
});
```

#### `puter.ui.wasLaunchedWithItems()`
Check if the app was launched to open one or more items.

**Returns:** Promise<boolean>

**Example:**
```javascript
if (await puter.ui.wasLaunchedWithItems()) {
  console.log('App was launched with items');
}
```

### Other UI Features

#### `puter.ui.parentApp()`
Get a connection to the app that launched the current app.

**Returns:** Promise<[AppConnection](#appconnection) | null>

**Example:**
```javascript
const parent = await puter.ui.parentApp();
if (parent) {
  // Communicate with parent app
  parent.send({ type: 'ready' });
}
```

#### `puter.ui.getLanguage()`
Get the current language/locale code.

**Returns:** Promise<string>

**Example:**
```javascript
const language = await puter.ui.getLanguage();
console.log('Language:', language); // 'en', 'fr', etc.
```

#### `puter.ui.showColorPicker()`
Present a color picker dialog.

**Parameters:**
- `options` (Object, optional):
  - `defaultColor` (string): Default color in hex format

**Returns:** Promise<string | null> - Selected color in hex format, or null if cancelled.

**Example:**
```javascript
const color = await puter.ui.showColorPicker({
  defaultColor: '#ff0000'
});
console.log('Selected color:', color);
```

#### `puter.ui.showFontPicker()`
Present a font picker dialog.

**Returns:** Promise<{family: string, size: number, style: string} | null>

**Example:**
```javascript
const font = await puter.ui.showFontPicker();
if (font) {
  console.log('Selected font:', font.family, font.size, font.style);
}
```

#### `puter.ui.showSpinner()`
Show an overlay with a spinner.

**Parameters:**
- `options` (Object, optional):
  - `message` (string): Message to display with the spinner

**Returns:** Promise<{hide: Function}> - Object with hide function.

**Example:**
```javascript
const spinner = await puter.ui.showSpinner({ message: 'Loading...' });
// Do some work...
spinner.hide();
```

#### `puter.ui.hideSpinner()`
Hide the active spinner.

**Example:**
```javascript
await puter.ui.hideSpinner();
```

#### `puter.ui.socialShare()`
Present a dialog for sharing a link on social media.

**Parameters:**
- `options` (Object):
  - `url` (string): URL to share
  - `text` (string): Text to include with the share

**Example:**
```javascript
await puter.ui.socialShare({
  url: 'https://example.com',
  text: 'Check out this cool website!'
});
```

#### `puter.ui.requestPictureInPicture()`
Float a page in an always-on-top picture-in-picture window.

**Parameters:**
- `options` (Object, optional):
  - `width` (number): Window width
  - `height` (number): Window height

**Returns:** Promise<{id: string}>

**Example:**
```javascript
const pip = await puter.ui.requestPictureInPicture({
  width: 400,
  height: 300
});
```

#### `puter.ui.exitPictureInPicture()`
Close the picture-in-picture window.

**Example:**
```javascript
await puter.ui.exitPictureInPicture();
```

---

## 🧰 Utilities

### Overview
Helpful utility functions for building with Puter.js.

#### `puter.appID`
Get the App ID of the running application.

**Returns:** string

**Example:**
```javascript
console.log('App ID:', puter.appID);
```

#### `puter.env`
Get the environment in which Puter.js is being used.

**Returns:** string - One of: 'web', 'puter-app', 'node', 'worker'

**Example:**
```javascript
console.log('Environment:', puter.env);
```

#### `puter.print()`
Print a string by appending it to the document body (for web apps).

**Parameters:**
- `...args` (any): Values to print
- `options` (Object, optional):
  - `code` (boolean): Whether to format as code

**Example:**
```javascript
puter.print('Hello, world!');
puter.print({ data: 123 }, { code: true });
```

#### `puter.randName()`
Generate a random domain-safe name.

**Returns:** string

**Example:**
```javascript
const randomName = puter.randName();
console.log(randomName); // e.g., 'x7f3k9p2'
```

---

## 📦 Objects & Data Types

### App
Represents a Puter app.

**Properties:**
- `id` (string): App ID
- `name` (string): App name
- `title` (string): Display title
- `icon` (string): URL of app icon
- `url` (string): URL to load when launched
- `fileAssociations` (Array<string>): Associated file extensions
- `created` (number): Creation timestamp
- `modified` (number): Last modification timestamp

---

### AppConnection
Provides an interface for interaction with another app.

**Methods:**
- `send(message)`: Send a message to the parent app
- `on(messageType, handler)`: Listen for messages from parent app
- `off(messageType, handler)`: Remove message listener

---

### ChatResponse
Contains AI chat response data.

**Properties:**
- `text` (string): The response text
- `model` (string): Model that generated the response
- `finishReason` (string): Why the response ended ('stop', 'length', 'error', etc.)
- `usage` (object): Token usage information
  - `promptTokens` (number): Tokens in the prompt
  - `completionTokens` (number): Tokens in the response
  - `totalTokens` (number): Total tokens used
- `toolCalls` (Array<[ToolCall](#toolcall)>): Tool calls made during the response

---

### ChatResponseChunk
Contains a chunk of streaming chat response data.

**Properties:**
- `text` (string): The text chunk
- `finishReason` (string | null): Finish reason, if this is the final chunk

---

### CreateAppResult
Contains the result of `puter.apps.create()`.

**Properties:**
- `id` (string): App ID
- `name` (string): App name
- `url` (string): URL to access the app

---

### DetailedAppUsage
Contains detailed resource usage statistics for a specific application.

**Properties:**
- `appId` (string): App ID
- `storage` (object): Storage usage
  - `used` (number): Bytes used
  - `limit` (number): Storage limit
- `ai` (object): AI usage
  - `used` (number): Tokens used
  - `limit` (number): Token limit
- `kv` (object): Key-value store usage
  - `used` (number): Bytes used
  - `limit` (number): Storage limit
- `period` (object): Usage period
  - `start` (number): Start timestamp
  - `end` (number): End timestamp

---

### FSItem
Represents a file or directory in the file system.

**Properties:**
- `name` (string): Item name
- `path` (string): Full path
- `type` (string): 'file' or 'directory'
- `size` (number): Size in bytes (for files)
- `created` (number): Creation timestamp
- `modified` (number): Last modification timestamp
- `isHidden` (boolean): Whether the item is hidden
- `isSymlink` (boolean): Whether the item is a symlink
- `mime` (string): MIME type (for files)
- `shared` (boolean): Whether the item is shared
- `sharedWith` (Array<string>): Users the item is shared with (if shared)

---

### KVPair
Contains key-value pair data.

**Properties:**
- `key` (string): The key
- `value` (any): The stored value
- `expires` (number | null): Expiration timestamp, or null if no expiration

---

### KVListPage
Contains paginated key-value list results.

**Properties:**
- `keys` (Array<{name: string, size: number}>): Array of key info
- `cursor` (string | null): Pagination cursor for next page

---

### MonthlyUsage
Contains user's monthly resource usage information.

**Properties:**
- `storage` (object): Storage usage
  - `used` (number): Bytes used
  - `limit` (number): Storage limit
  - `percentage` (number): Percentage of limit used
- `ai` (object): AI usage
  - `used` (number): Tokens used
  - `limit` (number): Token limit
  - `percentage` (number): Percentage of limit used
- `kv` (object): Key-value store usage
  - `used` (number): Bytes used
  - `limit` (number): Storage limit
  - `percentage` (number): Percentage of limit used
- `period` (object): Usage period
  - `start` (number): Start timestamp
  - `end` (number): End timestamp

---

### PuterPeerConnection
Represents a WebRTC data-channel connection to a peer.

**Properties:**
- `id` (string): Connection ID
- `peerId` (string): Peer ID
- `serverId` (string): Server ID

**Methods:**
- `send(message)`: Send a message to the peer
- `close()`: Close the connection

**Events:**
- `open`: Connection opened
- `message`: Message received
- `close`: Connection closed
- `error`: Connection error

---

### PuterPeerServer
Represents a peer server and its connected clients.

**Properties:**
- `id` (string): Server ID
- `inviteCode` (string): Invite code for clients to connect
- `name` (string): Server name
- `maxConnections` (number): Maximum number of connections

**Methods:**
- `close()`: Close the server

**Events:**
- `connection`: New client connection
- `close`: Server closed

---

### SignInResult
Contains the result of a sign-in operation.

**Properties:**
- `user` ([User](#user)): The authenticated user
- `token` (string): Authentication token
- `isNewUser` (boolean): Whether this is a new user account

---

### Speech2TxtResult
Contains speech-to-text transcription results.

**Properties:**
- `text` (string): The transcribed text
- `language` (string): Detected language
- `duration` (number): Duration of the audio in seconds
- `segments` (Array<{text: string, start: number, end: number}>): Word-level segments

---

### Subdomain
Contains subdomain information.

**Properties:**
- `subdomain` (string): Subdomain name
- `url` (string): Full URL (e.g., 'https://my-site.puter.site')
- `dir` (string): Directory path being hosted
- `created` (number): Creation timestamp
- `passwordProtected` (boolean): Whether the site is password-protected

---

### TTSEngine
Describes an available text-to-speech engine/model.

**Properties:**
- `id` (string): Engine/model ID
- `name` (string): Human-readable name
- `provider` (string): Provider name
- `pricing` (object): Pricing information
- `languages` (Array<string>): Supported languages
- `voices` (Array<string>): Available voices

---

### TTSVoice
Describes an available text-to-speech voice.

**Properties:**
- `id` (string): Voice ID
- `name` (string): Human-readable name
- `provider` (string): Provider name
- `language` (string): Language code
- `gender` (string): Voice gender ('male', 'female', 'neutral')
- `sampleRate` (number): Sample rate in Hz

---

### ToolCall
Contains tool invocation details.

**Properties:**
- `id` (string): Tool call ID
- `type` (string): 'function'
- `function` (object): Function details
  - `name` (string): Function name
  - `arguments` (string): JSON string of function arguments
- `result` (any): Result of the tool call (if completed)

---

### User
Contains Puter user details.

**Properties:**
- `id` (string): User ID
- `username` (string): Username
- `email` (string): Email address
- `name` (string): Display name
- `avatar` (string): URL of user avatar
- `created` (number): Account creation timestamp
- `lastActive` (number): Last activity timestamp

---

### WorkerDeployment
Contains worker deployment result data.

**Properties:**
- `id` (string): Deployment ID
- `name` (string): Worker name
- `url` (string): Worker URL
- `created` (number): Deployment timestamp

---

### WorkerInfo
Contains worker information.

**Properties:**
- `name` (string): Worker name
- `url` (string): Worker URL
- `created` (number): Creation timestamp
- `modified` (number): Last modification timestamp
- `status` (string): Worker status ('running', 'stopped', 'error')

---

## ⚡ Rate Limits & Quotas

### Overview
Puter implements **fair usage policies** to ensure quality service for all users.

### Default Limits

| Resource | Free Tier | Paid Tier |
|----------|-----------|-----------|
| **Storage** | 1 GB | 100 GB - 1 TB |
| **AI Tokens** | 10,000/month | 1,000,000+/month |
| **Key-Value Store** | 100 MB | 10 GB |
| **Workers** | 5 concurrent | 50+ concurrent |
| **Hosting** | 10 sites | 100+ sites |
| **File Uploads** | 100 MB/file | 10 GB/file |
| **API Requests** | 1000/min | 10,000+/min |

### Rate Limit Headers
API responses include rate limit information:
- `X-RateLimit-Limit`: Maximum requests allowed
- `X-RateLimit-Remaining`: Requests remaining in current window
- `X-RateLimit-Reset`: Timestamp when limit resets

### Handling Rate Limits
When you hit a rate limit, you'll receive a `429 Too Many Requests` response. Implement retry logic with exponential backoff.

**Example:**
```javascript
async function withRetry(fn, maxRetries = 3) {
  let retries = 0;
  
  while (retries < maxRetries) {
    try {
      return await fn();
    } catch (error) {
      if (error.status === 429) {
        const resetTime = error.headers.get('X-RateLimit-Reset');
        const waitTime = Math.pow(2, retries) * 1000; // Exponential backoff
        await new Promise(resolve => setTimeout(resolve, waitTime));
        retries++;
      } else {
        throw error;
      }
    }
  }
  
  throw new Error('Max retries exceeded');
}

// Usage
const result = await withRetry(() => puter.ai.chat('Hello'));
```

---

## 🔒 Security & Permissions

### Overview
Puter implements a **robust security model** with:
- User authentication and authorization
- Fine-grained permissions
- Sandboxed app environments
- Built-in abuse protection

### Authentication Flow
1. User signs in with Puter account
2. App receives authentication token
3. All API requests include token
4. Puter validates token and permissions
5. Request is processed or rejected

### Permission Model

#### App Permissions
Each app has **default permissions** within the user's account:
- **App Directory**: `~/AppData/<app-id>/` - App can read/write freely
- **Key-Value Store**: App-specific KV store - App can read/write freely
- **Outside Access**: Requires explicit user permission

#### Requesting Permissions
Use `puter.perms.request()` to request access to user data:

```javascript
const granted = await puter.perms.request({
  fs: {
    read: ['/documents/*'],
    write: ['/documents/drafts/*']
  },
  kv: {
    read: ['userSettings'],
    write: ['userSettings']
  }
});

if (granted) {
  console.log('Permissions granted');
}
```

#### Checking Permissions
Use `puter.perms.check()` to check if access is already granted:

```javascript
const hasAccess = await puter.perms.check({
  fs: { read: ['/documents/report.txt'] }
});

if (hasAccess) {
  const file = await puter.fs.read('/documents/report.txt');
}
```

### Using Another App's Data
Apps can request permission to use another app's data:

```javascript
// Request access to another app's KV store
const granted = await puter.perms.request({
  appData: {
    appId: 'other-app-id',
    kv: { read: ['sharedSettings'] }
  }
});

if (granted) {
  // Now can access the other app's KV store
  const settings = await puter.kv.get('sharedSettings');
}
```

### Security Best Practices

1. **Never store secrets in client-side code**
   - Use environment variables
   - Use Puter's built-in secret management for workers

2. **Validate all user input**
   - Sanitize file paths
   - Validate data before processing
   - Use parameterized queries

3. **Implement proper error handling**
   - Don't expose sensitive information in errors
   - Use appropriate HTTP status codes
   - Log errors securely

4. **Use HTTPS everywhere**
   - Puter provides automatic SSL for hosted sites
   - Always use secure connections

5. **Respect user privacy**
   - Only request necessary permissions
   - Be transparent about data usage
   - Allow users to revoke access

---

## 💡 Integration Examples

### Example 1: Basic File Upload & AI Analysis

```javascript
// HTML
<html>
<body>
  <input type="file" id="file-input" />
  <button id="analyze">Analyze with AI</button>
  <div id="result"></div>
  
  <script src="https://js.puter.com/v2/"></script>
  <script>
    document.getElementById('analyze').addEventListener('click', async () => {
      const fileInput = document.getElementById('file-input');
      const resultDiv = document.getElementById('result');
      
      if (fileInput.files.length === 0) {
        resultDiv.textContent = 'Please select a file first';
        return;
      }
      
      // Upload file
      resultDiv.textContent = 'Uploading...';
      const files = await puter.fs.upload(fileInput.files);
      const file = files[0];
      
      // Extract text from file
      resultDiv.textContent = 'Extracting text...';
      let text;
      if (file.name.endsWith('.txt') || file.name.endsWith('.md')) {
        const blob = await puter.fs.read(file.path);
        text = await blob.text();
      } else if (file.name.match(/\.(jpg|jpeg|png|gif)$/i)) {
        text = await puter.ai.img2txt(file.path);
      } else {
        resultDiv.textContent = 'Unsupported file type';
        return;
      }
      
      // Analyze with AI
      resultDiv.textContent = 'Analyzing...';
      const analysis = await puter.ai.chat(`Analyze the following text and provide a summary:\n\n${text}`);
      
      resultDiv.textContent = analysis.text;
    });
  </script>
</body>
</html>
```

### Example 2: Real-Time Collaborative Editor

```javascript
// HTML
<html>
<head>
  <title>Collaborative Editor</title>
  <style>
    #editor { width: 100%; height: 300px; border: 1px solid #ccc; }
    .cursor { position: absolute; pointer-events: none; }
  </style>
</head>
<body>
  <h1>Collaborative Editor</h1>
  <div id="editor" contenteditable="true"></div>
  <div id="status"></div>
  
  <script src="https://js.puter.com/v2/"></script>
  <script>
    const editor = document.getElementById('editor');
    const status = document.getElementById('status');
    let filePath = 'collab-doc.txt';
    let currentContent = '';
    
    // Load initial content
    async function loadDocument() {
      try {
        const blob = await puter.fs.read(filePath);
        currentContent = await blob.text();
        editor.textContent = currentContent;
        status.textContent = 'Document loaded';
      } catch (error) {
        if (error.message.includes('not found')) {
          // Document doesn't exist yet, create it
          await puter.fs.write(filePath, '');
          status.textContent = 'Created new document';
        }
      }
    }
    
    // Save content
    async function saveDocument() {
      const newContent = editor.textContent;
      if (newContent !== currentContent) {
        await puter.fs.write(filePath, newContent);
        currentContent = newContent;
        status.textContent = 'Document saved';
      }
    }
    
    // Watch for changes from other users
    puter.events.onLocal(filePath, async (event) => {
      if (event.type === 'modify') {
        const blob = await puter.fs.read(filePath);
        const newContent = await blob.text();
        if (newContent !== currentContent) {
          editor.textContent = newContent;
          currentContent = newContent;
          status.textContent = 'Document updated by another user';
        }
      }
    });
    
    // Auto-save
    editor.addEventListener('input', () => {
      saveDocument();
    });
    
    // Load on start
    loadDocument();
  </script>
</body>
</html>
```

### Example 3: AI-Powered Chat Interface

```javascript
// HTML
<html>
<head>
  <title>AI Chat</title>
  <style>
    #chat { height: 400px; overflow-y: auto; border: 1px solid #ccc; padding: 10px; margin-bottom: 10px; }
    #message { width: 70%; padding: 8px; }
    #send { width: 25%; padding: 8px; }
    .message { margin-bottom: 10px; padding: 8px; border-radius: 4px; }
    .user { background: #e3f2fd; text-align: right; }
    .ai { background: #f5f5f5; text-align: left; }
  </style>
</head>
<body>
  <h1>AI Chat</h1>
  <div id="chat"></div>
  <input type="text" id="message" placeholder="Type your message..." />
  <button id="send">Send</button>
  
  <script src="https://js.puter.com/v2/"></script>
  <script>
    const chatDiv = document.getElementById('chat');
    const messageInput = document.getElementById('message');
    const sendButton = document.getElementById('send');
    
    // Add message to chat
    function addMessage(text, isUser = false) {
      const messageDiv = document.createElement('div');
      messageDiv.className = `message ${isUser ? 'user' : 'ai'}`;
      messageDiv.textContent = text;
      chatDiv.appendChild(messageDiv);
      chatDiv.scrollTop = chatDiv.scrollHeight;
    }
    
    // Send message
    async function sendMessage() {
      const message = messageInput.value.trim();
      if (!message) return;
      
      addMessage(message, true);
      messageInput.value = '';
      
      // Show typing indicator
      const typingDiv = document.createElement('div');
      typingDiv.className = 'message ai';
      typingDiv.textContent = 'AI is typing...';
      chatDiv.appendChild(typingDiv);
      chatDiv.scrollTop = chatDiv.scrollHeight;
      
      try {
        // Get AI response
        const response = await puter.ai.chat(message, {
          model: 'gpt-4o-mini',
          stream: true
        });
        
        // Remove typing indicator
        chatDiv.removeChild(typingDiv);
        
        // Stream response
        let fullResponse = '';
        for await (const chunk of response) {
          fullResponse += chunk.text;
          typingDiv.textContent = fullResponse;
          chatDiv.scrollTop = chatDiv.scrollHeight;
        }
      } catch (error) {
        chatDiv.removeChild(typingDiv);
        addMessage(`Error: ${error.message}`, false);
      }
    }
    
    // Event listeners
    sendButton.addEventListener('click', sendMessage);
    messageInput.addEventListener('keypress', (e) => {
      if (e.key === 'Enter') sendMessage();
    });
    
    // Initial message
    addMessage('Hello! How can I help you today?', false);
  </script>
</body>
</html>
```

### Example 4: Serverless API with Worker

```javascript
// worker.js
import { router } from '@heyputer/worker-router';

// GET endpoint
router.get('/api/hello', async ({ request }) => {
  const name = new URL(request.url).searchParams.get('name') || 'World';
  return { message: `Hello, ${name}!` };
});

// POST endpoint
router.post('/api/echo', async ({ request }) => {
  const body = await request.json();
  return { echo: body };
});

// AI endpoint
router.post('/api/chat', async ({ request }) => {
  const { prompt, model } = await request.json();
  const response = await puter.ai.chat(prompt, { model });
  return { response: response.text };
});

// File upload endpoint
router.post('/api/upload', async ({ request }) => {
  const formData = await request.formData();
  const file = formData.get('file');
  
  if (!file) {
    return { error: 'No file provided' }, { status: 400 };
  }
  
  const uploadedFile = await puter.fs.upload(file);
  return { 
    path: uploadedFile.path, 
    size: uploadedFile.size 
  };
});

// Serve static files
router.get('/static/*', async ({ request }) => {
  const path = new URL(request.url).pathname.replace('/static/', '');
  try {
    const blob = await puter.fs.read(`static/${path}`);
    return new Response(blob, {
      headers: { 'Content-Type': blob.type }
    });
  } catch (error) {
    return { error: 'File not found' }, { status: 404 };
  }
});

// 404 handler
router.all('*', async () => {
  return { error: 'Not found' }, { status: 404 };
});

export default router;
```

### Example 5: Complete To-Do App

```javascript
// HTML
<html>
<head>
  <title>To-Do App</title>
  <style>
    #app { max-width: 600px; margin: 0 auto; }
    #todo-input { width: 70%; padding: 10px; }
    #add-btn { width: 25%; padding: 10px; }
    #todo-list { list-style: none; padding: 0; }
    .todo-item { display: flex; padding: 10px; border-bottom: 1px solid #eee; }
    .todo-item.completed { text-decoration: line-through; color: #888; }
    .todo-text { flex: 1; }
    .todo-actions { margin-left: 10px; }
  </style>
</head>
<body>
  <div id="app">
    <h1>To-Do List</h1>
    <div>
      <input type="text" id="todo-input" placeholder="Add a new task..." />
      <button id="add-btn">Add</button>
    </div>
    <ul id="todo-list"></ul>
  </div>
  
  <script src="https://js.puter.com/v2/"></script>
  <script>
    const todoInput = document.getElementById('todo-input');
    const addBtn = document.getElementById('add-btn');
    const todoList = document.getElementById('todo-list');
    const KV_KEY = 'todo-list';
    
    // Load to-do list
    async function loadTodos() {
      try {
        const todos = await puter.kv.get(KV_KEY) || [];
        renderTodos(todos);
      } catch (error) {
        console.error('Error loading todos:', error);
      }
    }
    
    // Save to-do list
    async function saveTodos(todos) {
      await puter.kv.set(KV_KEY, todos);
    }
    
    // Render to-do list
    function renderTodos(todos) {
      todoList.innerHTML = '';
      todos.forEach((todo, index) => {
        const li = document.createElement('li');
        li.className = `todo-item ${todo.completed ? 'completed' : ''}`;
        li.innerHTML = `
          <span class="todo-text">${todo.text}</span>
          <span class="todo-actions">
            <button onclick="toggleTodo(${index})">${todo.completed ? 'Undo' : 'Complete'}</button>
            <button onclick="deleteTodo(${index})">Delete</button>
          </span>
        `;
        todoList.appendChild(li);
      });
    }
    
    // Add to-do
    async function addTodo() {
      const text = todoInput.value.trim();
      if (!text) return;
      
      const todos = await puter.kv.get(KV_KEY) || [];
      todos.push({ text, completed: false });
      await saveTodos(todos);
      todoInput.value = '';
      renderTodos(todos);
    }
    
    // Toggle to-do completion
    async function toggleTodo(index) {
      const todos = await puter.kv.get(KV_KEY) || [];
      todos[index].completed = !todos[index].completed;
      await saveTodos(todos);
      renderTodos(todos);
    }
    
    // Delete to-do
    async function deleteTodo(index) {
      const todos = await puter.kv.get(KV_KEY) || [];
      todos.splice(index, 1);
      await saveTodos(todos);
      renderTodos(todos);
    }
    
    // Event listeners
    addBtn.addEventListener('click', addTodo);
    todoInput.addEventListener('keypress', (e) => {
      if (e.key === 'Enter') addTodo();
    });
    
    // Watch for changes from other devices
    puter.events.onLocal(KV_KEY, async (event) => {
      if (event.type === 'set') {
        const todos = event.value || [];
        renderTodos(todos);
      }
    });
    
    // Load todos on start
    loadTodos();
    
    // Make functions available globally
    window.toggleTodo = toggleTodo;
    window.deleteTodo = deleteTodo;
  </script>
</body>
</html>
```

---

## 📚 Additional Resources

### Official Documentation
- [Puter.js Docs](https://docs.puter.com/)
- [API Reference (llms.txt)](https://docs.puter.com/llms.txt)
- [Full API Reference (llms-full.txt)](https://docs.puter.com/llms-full.txt)

### Developer Resources
- [Developer Portal](https://developer.puter.com/)
- [Showcase Apps](https://developer.puter.com/showcase/)
- [Puter Apps Directory](https://apps.puter.com/)
- [GitHub Organization](https://github.com/HeyPuter)

### Starter Templates
- [React Template](https://github.com/HeyPuter/react)
- [Next.js Template](https://github.com/HeyPuter/next.js)
- [Vue.js Template](https://github.com/HeyPuter/vue.js)
- [Angular Template](https://github.com/HeyPuter/angular)
- [Svelte Template](https://github.com/HeyPuter/svelte)
- [Astro Template](https://github.com/HeyPuter/astro)
- [Vanilla JS Template](https://github.com/HeyPuter/vanilla.js)
- [Node.js + Express Template](https://github.com/HeyPuter/node.js-express.js)

### Community
- [Twitter](https://twitter.com/HeyPuter)
- [LinkedIn](https://www.linkedin.com/company/puter/)
- [Discord](https://discord.gg/puter)

---

## 🏆 Conclusion

This document provides a **complete reference** to all Puter API capabilities. With this knowledge, you can integrate **any Puter feature** into Aether/Aetheris to create a **powerful, unified platform** that combines:

✅ **Cloud storage** with advanced file management  
✅ **500+ AI models** for any task  
✅ **Real-time collaboration** for teams  
✅ **Serverless backend** for complex workflows  
✅ **Zero-cost infrastructure** that scales infinitely  

**The complete Puter platform is now at your fingertips.**

---

*Document created: 2026-09-14*  
*Last updated: 2026-09-14*  
*Version: 1.0*