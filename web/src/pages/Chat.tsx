import React, { useState, useEffect, useRef } from 'react';
import { colors, spacing, typography, borderRadius, shadow } from '../components/AetherisComponents';

const MODELS = [
  { id: 'puter/claude-fable-5', label: 'Fable 5' },
  { id: 'puter/claude-fable-5.1', label: 'Fable 5.1' },
  { id: 'puter/opus-5', label: 'Opus 5' },
  { id: 'puter/sonnet-5', label: 'Sonnet 5' },
];

interface ChatMessage {
  role: 'user' | 'assistant';
  content: string;
  timestamp: number;
  model?: string;
}

interface Conversation {
  id: string;
  messages: ChatMessage[];
}

export default function ChatPage() {
  const [selectedModel, setSelectedModel] = useState(MODELS[1].id);
  const [prompt, setPrompt] = useState('');
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [activeConversationId, setActiveConversationId] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [serverOnline, setServerOnline] = useState<boolean | null>(null);
  const [error, setError] = useState<string | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Get active conversation
  const activeConversation = conversations.find(c => c.id === activeConversationId);
  const messages = activeConversation?.messages || [];

  // Check server status
  useEffect(() => {
    checkServerStatus();
    const interval = setInterval(checkServerStatus, 30000);
    return () => clearInterval(interval);
  }, []);

  const checkServerStatus = async () => {
    try {
      const response = await fetch('http://127.0.0.1:8008/models');
      setServerOnline(response.ok);
    } catch {
      setServerOnline(false);
    }
  };

  // Create new conversation
  const createConversation = () => {
    const newConversation: Conversation = {
      id: `conv_${Date.now()}`,
      messages: [],
    };
    setConversations([...conversations, newConversation]);
    setActiveConversationId(newConversation.id);
  };

  // Send chat message
  const sendMessage = async () => {
    if (!prompt.trim() || !selectedModel || !serverOnline) return;

    setIsLoading(true);
    setError(null);

    // Add user message immediately
    const userMessage: ChatMessage = {
      role: 'user',
      content: prompt.trim(),
      timestamp: Date.now(),
      model: selectedModel,
    };

    if (!activeConversationId) {
      createConversation();
    }

    // Update conversations
    setConversations(prev => {
      const updated = [...prev];
      const activeIdx = updated.findIndex(c => c.id === activeConversationId);
      if (activeIdx >= 0) {
        updated[activeIdx] = {
          ...updated[activeIdx],
          messages: [...updated[activeIdx].messages, userMessage],
        };
      }
      return updated;
    });
    setPrompt('');

    try {
      const response = await fetch('http://127.0.0.1:8008/chat/completions', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          model: selectedModel,
          messages: [{ role: 'user', content: prompt.trim() }],
        }),
      });

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }

      const data = await response.json();
      const assistantMessage: ChatMessage = {
        role: 'assistant',
        content: data.choices?.[0]?.text || 'No response text',
        timestamp: Date.now(),
        model: selectedModel,
      };

      setConversations(prev => {
        const updated = [...prev];
        const activeIdx = updated.findIndex(c => c.id === activeConversationId);
        if (activeIdx >= 0) {
          updated[activeIdx] = {
            ...updated[activeIdx],
            messages: [...updated[activeIdx].messages, assistantMessage],
          };
        }
        return updated;
      });
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
      // Add error message to conversation
      const errorMessage: ChatMessage = {
        role: 'assistant',
        content: `Error: ${err instanceof Error ? err.message : 'Unknown error'}`,
        timestamp: Date.now(),
        model: selectedModel,
      };
      setConversations(prev => {
        const updated = [...prev];
        const activeIdx = updated.findIndex(c => c.id === activeConversationId);
        if (activeIdx >= 0) {
          updated[activeIdx] = {
            ...updated[activeIdx],
            messages: [...updated[activeIdx].messages, errorMessage],
          };
        }
        return updated;
      });
    } finally {
      setIsLoading(false);
    }
  };

  // Scroll to bottom of messages
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  // Handle key press
  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  };

  // Get friendly model name
  const getModelLabel = (modelId: string) => {
    const model = MODELS.find(m => m.id === modelId);
    return model?.label || modelId;
  };

  return (
    <div style={{ 
      background: colors.bg, 
      color: colors.textPrimary, 
      fontFamily: typography.family, 
      minHeight: '100vh', 
      display: 'flex', 
      flexDirection: 'column' 
    }}>
      {/* Header */}
      <header style={{ 
        padding: spacing.large, 
        borderBottom: `1px solid ${colors.border}`,
        display: 'flex', 
        justifyContent: 'space-between', 
        alignItems: 'center'
      }}>
        <div>
          <h1 style={{ 
            fontSize: 24, 
            fontWeight: 700, 
            color: colors.primary,
            margin: 0 
          }}>
            Aetheris Chat
          </h1>
          <p style={{ 
            color: colors.textSecondary, 
            fontSize: 14,
            margin: `${spacing.xsmall}px 0 0 0`
          }}>
            Puter Models via Local LLM
          </p>
        </div>
        <div style={{ display: 'flex', gap: spacing.small, alignItems: 'center' }}>
          <span style={{ 
            fontSize: 12, 
            color: serverOnline ? colors.success : serverOnline === null ? colors.warning : colors.error
          }}>
            {serverOnline ? 'Online' : serverOnline === null ? 'Checking...' : 'Offline'}
          </span>
          <select
            value={selectedModel}
            onChange={(e) => setSelectedModel(e.target.value)}
            style={{ 
              padding: `${spacing.xsmall}px ${spacing.small}px`, 
              background: colors.surface, 
              color: colors.textPrimary,
              border: `1px solid ${colors.border}`,
              borderRadius: borderRadius.small,
              fontSize: 14,
              cursor: 'pointer'
            }}
          >
            {MODELS.map(model => (
              <option key={model.id} value={model.id}>
                {model.label}
              </option>
            ))}
          </select>
        </div>
      </header>

      {/* Main chat area */}
      <main style={{ 
        flex: 1, 
        display: 'flex', 
        overflow: 'hidden',
        padding: spacing.medium
      }}>
        {/* Sidebar - conversation list */}
        <aside style={{ 
          width: 250, 
          borderRight: `1px solid ${colors.border}`,
          paddingRight: spacing.medium,
          overflowY: 'auto'
        }}>
          <button
            onClick={createConversation}
            style={{ 
              width: '100%', 
              padding: `${spacing.small}px ${spacing.medium}px`, 
              background: colors.primary, 
              color: colors.bg,
              border: 'none',
              borderRadius: borderRadius.small,
              fontSize: 14,
              fontWeight: 600,
              cursor: 'pointer',
              marginBottom: spacing.medium
            }}
          >
            + New Chat
          </button>
          
          <div style={{ fontSize: 12, color: colors.textSecondary, marginBottom: spacing.small }}>
            Conversations
          </div>
          
          <div style={{ display: 'flex', flexDirection: 'column', gap: spacing.xsmall }}>
            {conversations.map(conversation => (
              <button
                key={conversation.id}
                onClick={() => setActiveConversationId(conversation.id)}
                style={{ 
                  padding: `${spacing.xsmall}px ${spacing.small}px`, 
                  background: conversation.id === activeConversationId ? colors.surfaceHover : 'transparent',
                  color: colors.textPrimary,
                  border: 'none',
                  borderRadius: borderRadius.small,
                  fontSize: 13,
                  cursor: 'pointer',
                  textAlign: 'left',
                  whiteSpace: 'nowrap',
                  overflow: 'hidden',
                  textOverflow: 'ellipsis'
                }}
              >
                {conversation.messages.length > 0 
                  ? conversation.messages[0].content.slice(0, 30)
                  : 'New conversation'}
              </button>
            ))}
          </div>
        </aside>

        {/* Chat messages */}
        <section style={{ 
          flex: 1, 
          display: 'flex', 
          flexDirection: 'column',
          overflow: 'hidden'
        }}>
          <div style={{ 
            flex: 1, 
            overflowY: 'auto',
            padding: spacing.medium,
            display: 'flex', 
            flexDirection: 'column',
            gap: spacing.medium
          }}>
            {messages.length === 0 ? (
              <div style={{ 
                textAlign: 'center', 
                color: colors.textSecondary,
                padding: spacing.xlarge
              }}>
                {activeConversationId ? 'Start a conversation' : 'Select or create a conversation'}
              </div>
            ) : (
              messages.map((message, index) => (
                <div
                  key={index}
                  style={{ 
                    display: 'flex', 
                    gap: spacing.small,
                    alignItems: 'flex-start'
                  }}
                >
                  <div style={{ 
                    width: 32, 
                    height: 32, 
                    borderRadius: '50%',
                    background: message.role === 'user' ? colors.primary : colors.surface,
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    flexShrink: 0,
                    fontSize: 12,
                    color: message.role === 'user' ? colors.bg : colors.textPrimary
                  }}>
                    {message.role === 'user' ? 'U' : 'A'}
                  </div>
                  <div style={{ 
                    maxWidth: '80%',
                    background: message.role === 'user' ? colors.surface : colors.surfaceHover,
                    padding: `${spacing.small}px ${spacing.medium}px`,
                    borderRadius: borderRadius.medium,
                    color: colors.textPrimary,
                    lineHeight: 1.5,
                    whiteSpace: 'pre-wrap',
                    wordWrap: 'break-word'
                  }}>
                    {message.model && message.role === 'assistant' && (
                      <div style={{ 
                        fontSize: 11, 
                        color: colors.textSecondary,
                        marginBottom: spacing.xsmall
                      }}>
                        {getModelLabel(message.model)}
                      </div>
                    )}
                    {message.content}
                    <div style={{ 
                      fontSize: 10, 
                      color: colors.textDisabled,
                      marginTop: spacing.xsmall
                    }}>
                      {new Date(message.timestamp).toLocaleTimeString()}
                    </div>
                  </div>
                </div>
              ))
            )}
            {isLoading && (
              <div style={{ 
                display: 'flex', 
                gap: spacing.small,
                alignItems: 'flex-start'
              }}>
                <div style={{ 
                  width: 32, 
                  height: 32, 
                  borderRadius: '50%',
                  background: colors.surface,
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  flexShrink: 0
                }}>
                  <div className="loading" style={{ 
                    width: 16, 
                    height: 16, 
                    border: '2px solid #666',
                    borderTopColor: 'transparent',
                    borderRadius: '50%',
                    animation: 'spin 1s linear infinite'
                  }} />
                </div>
                <div style={{ 
                  padding: `${spacing.small}px ${spacing.medium}px`,
                  background: colors.surfaceHover,
                  borderRadius: borderRadius.medium,
                  color: colors.textPrimary
                }}>
                  Thinking...
                </div>
              </div>
            )}
            <div ref={messagesEndRef} />
          </div>

          {/* Error display */}
          {error && (
            <div style={{ 
              padding: spacing.medium,
              background: colors.errorBg,
              color: colors.error,
              borderRadius: borderRadius.small,
              fontSize: 13,
              margin: `0 ${spacing.medium}px ${spacing.medium}px`
            }}>
              {error}
            </div>
          )}

          {/* Input area */}
          <div style={{ 
            display: 'flex', 
            gap: spacing.small,
            padding: spacing.medium,
            borderTop: `1px solid ${colors.border}`
          }}>
            <input
              type="text"
              value={prompt}
              onChange={(e) => setPrompt(e.target.value)}
              onKeyPress={handleKeyPress}
              placeholder={serverOnline ? "Type your message..." : "Server offline - cannot send messages"}
              disabled={!serverOnline}
              style={{ 
                flex: 1, 
                padding: `${spacing.small}px ${spacing.medium}px`, 
                background: colors.surface, 
                color: colors.textPrimary,
                border: `1px solid ${colors.border}`,
                borderRadius: borderRadius.medium,
                fontSize: 14,
                fontFamily: typography.family
              }}
            />
            <button
              onClick={sendMessage}
              disabled={!prompt.trim() || !serverOnline || isLoading}
              style={{ 
                padding: `${spacing.small}px ${spacing.large}px`, 
                background: colors.primary, 
                color: colors.bg,
                border: 'none',
                borderRadius: borderRadius.medium,
                fontSize: 14,
                fontWeight: 600,
                cursor: 'pointer',
                whiteSpace: 'nowrap'
              }}
            >
              Send
            </button>
          </div>
        </section>
      </main>

      {/* Add CSS animation for loading spinner */}
      <style jsx>{`
        @keyframes spin {
          to { transform: rotate(360deg); }
        }
      `}</style>
    </div>
  );
}
