// Real Puter.js integration — https://docs.puter.com/AI/chat/
// Puter.js is loaded from the CDN at runtime; AI calls use the "user pays"
// model (no API keys). All models exposed by Puter are usable via
// listModels() + chat(stream: true).

const PUTER_SCRIPT_URL = 'https://js.puter.com/v2/';
const SCRIPT_LOAD_TIMEOUT_MS = 20_000;

export interface PuterModel {
  id: string;
  provider: string;
  name?: string;
  aliases?: string[];
  context?: number;
  max_tokens?: number;
  cost?: {
    currency: string;
    tokens: number;
    input: number;
    output: number;
  };
}

export interface PuterChatMessage {
  role: 'system' | 'user' | 'assistant';
  content: string;
}

export interface PuterUser {
  username?: string;
  email?: string;
}

// Raw Puter.js globals — typed narrowly; the CDN bundle ships no .d.ts.
interface PuterJs {
  ai: {
    chat: (
      messages: string | PuterChatMessage[],
      options?: { model?: string; stream?: boolean; [k: string]: unknown },
    ) => Promise<AsyncIterable<unknown> | unknown>;
    listModels: (provider?: string) => Promise<unknown>;
  };
  auth: {
    signIn: () => Promise<unknown>;
    signOut: () => void;
    isSignedIn: () => boolean;
    getUser: () => Promise<PuterUser>;
  };
}

declare global {
  interface Window {
    puter?: PuterJs;
  }
}

let loadPromise: Promise<PuterJs> | null = null;

/** Loads the Puter.js CDN bundle once; resolves the global `puter` object. */
export function loadPuter(): Promise<PuterJs> {
  if (window.puter) return Promise.resolve(window.puter);
  if (loadPromise) return loadPromise;

  loadPromise = new Promise<PuterJs>((resolve, reject) => {
    const existing = document.querySelector<HTMLScriptElement>(
      `script[src^="${PUTER_SCRIPT_URL}"]`,
    );
    const script = existing ?? document.createElement('script');
    const timer = window.setTimeout(
      () => reject(new Error('Puter.js failed to load (timeout)')),
      SCRIPT_LOAD_TIMEOUT_MS,
    );

    const done = () => {
      window.clearTimeout(timer);
      if (window.puter) resolve(window.puter);
      else reject(new Error('Puter.js loaded but did not initialize'));
    };
    script.addEventListener('load', done, { once: true });
    script.addEventListener('error', () => {
      window.clearTimeout(timer);
      loadPromise = null;
      reject(new Error('Puter.js failed to load (network error)'));
    }, { once: true });

    if (!existing) {
      script.src = PUTER_SCRIPT_URL;
      document.head.appendChild(script);
    } else if (window.puter) {
      done();
    }
  });

  return loadPromise;
}

/** All models Puter currently exposes, sorted by provider then name. */
export async function listModels(): Promise<PuterModel[]> {
  const puter = await loadPuter();
  const raw = (await puter.ai.listModels()) as unknown;
  if (!Array.isArray(raw)) {
    throw new Error('Unexpected listModels() response shape');
  }
  const models: PuterModel[] = [];
  for (const entry of raw) {
    if (entry && typeof entry === 'object' && 'id' in entry
        && typeof entry.id === 'string') {
      const provider = 'provider' in entry && typeof entry.provider === 'string'
        ? entry.provider
        : 'unknown';
      models.push({ ...entry, id: entry.id, provider });
    }
  }
  models.sort((a, b) =>
    a.provider.localeCompare(b.provider) || (a.name ?? a.id).localeCompare(b.name ?? b.id));
  return models;
}

// Raw streaming shapes seen from Puter: string, { text }, or
// [{ type: 'text', text }, ...] after normalization.
export function extractDelta(chunk: unknown): string {
  if (typeof chunk === 'string') return chunk;
  if (chunk === null || typeof chunk !== 'object') return '';
  if ('message' in chunk) return extractText(chunk.message);
  if ('content' in chunk) return extractText(chunk.content);
  if ('text' in chunk) return extractText(chunk.text);
  return '';
}

function extractText(content: unknown): string {
  if (typeof content === 'string') return content;
  if (content && typeof content === 'object') {
    if ('text' in content && typeof content.text === 'string') return content.text;
    if (Array.isArray(content)) {
      return content.map((part) => extractText(part)).join('');
    }
  }
  return '';
}

/**
 * Streams a chat completion. Calls onDelta for each text chunk and resolves
 * with the full response text.
 */
export async function chatStream(
  messages: PuterChatMessage[],
  model: string,
  onDelta: (text: string) => void,
): Promise<string> {
  const puter = await loadPuter();
  const response = await puter.ai.chat(messages, { model, stream: true });

  let full = '';
  const iterable = response as AsyncIterable<unknown> | null;
  if (iterable && typeof iterable[Symbol.asyncIterator] === 'function') {
    for await (const chunk of iterable) {
      const delta = extractDelta(chunk);
      if (delta) {
        full += delta;
        onDelta(delta);
      }
    }
  } else {
    // Non-streaming fallback (some models may not support streaming)
    const text = extractDelta(response);
    if (text) {
      full = text;
      onDelta(text);
    }
  }
  return full;
}

/** True when the visitor already has a Puter session. */
export async function isSignedIn(): Promise<boolean> {
  const puter = await loadPuter();
  try {
    return puter.auth.isSignedIn();
  } catch {
    return false;
  }
}

/** Opens the Puter sign-in flow (popup). */
export async function signIn(): Promise<PuterUser> {
  const puter = await loadPuter();
  await puter.auth.signIn();
  return puter.auth.getUser();
}

export async function getUser(): Promise<PuterUser | null> {
  const puter = await loadPuter();
  try {
    return await puter.auth.getUser();
  } catch {
    return null;
  }
}
