import React, { useState, useEffect } from 'react';
import { colors, spacing, typography, borderRadius, shadow } from '../components/AetherisComponents';

interface ModelInfo {
  id: string;
  label: string;
  description: string;
  max_tokens: number;
  usage: number;
  status: 'active' | 'maintenance' | 'disabled';
}

interface UserStats {
  user_id: string;
  email: string;
  requests: number;
  last_active: string;
}

const DEFAULT_MODELS: ModelInfo[] = [
  {
    id: 'puter/claude-fable-5',
    label: 'Fable 5',
    description: 'Puter Fable 5 model - Fast and efficient',
    max_tokens: 32000,
    usage: 0,
    status: 'active'
  },
  {
    id: 'puter/claude-fable-5.1',
    label: 'Fable 5.1',
    description: 'Puter Fable 5.1 model - Improved reasoning',
    max_tokens: 32000,
    usage: 0,
    status: 'active'
  },
  {
    id: 'puter/opus-5',
    label: 'Opus 5',
    description: 'Puter Opus 5 model - High intelligence',
    max_tokens: 32000,
    usage: 0,
    status: 'active'
  },
  {
    id: 'puter/sonnet-5',
    label: 'Sonnet 5',
    description: 'Puter Sonnet 5 model - Balanced performance',
    max_tokens: 64000,
    usage: 0,
    status: 'active'
  },
];

export default function AdminDashboard() {
  const [models, setModels] = useState<ModelInfo[]>(DEFAULT_MODELS);
  const [users, setUsers] = useState<UserStats[]>([]);
  const [serverStatus, setServerStatus] = useState<'online' | 'offline' | 'checking'>('checking');
  const [stats, setStats] = useState({
    totalRequests: 0,
    activeUsers: 0,
    uptime: '00:00:00'
  });
  const [activeTab, setActiveTab] = useState<'models' | 'users' | 'analytics'>('models');

  // Check server status
  useEffect(() => {
    checkServerStatus();
    const interval = setInterval(checkServerStatus, 30000);
    return () => clearInterval(interval);
  }, []);

  const checkServerStatus = async () => {
    try {
      const response = await fetch('http://127.0.0.1:8008/models');
      setServerStatus(response.ok ? 'online' : 'offline');
    } catch {
      setServerStatus('offline');
    }
  };

  // Load sample user data
  useEffect(() => {
    // In a real implementation, this would fetch from an API
    const sampleUsers: UserStats[] = [
      { user_id: 'user_1', email: 'admin@aetheris.com', requests: 42, last_active: new Date().toISOString() },
      { user_id: 'user_2', email: 'test@example.com', requests: 15, last_active: new Date(Date.now() - 86400000).toISOString() },
    ];
    setUsers(sampleUsers);
    
    setStats({
      totalRequests: 1234,
      activeUsers: 2,
      uptime: '23:45:30'
    });
  }, []);

  // Toggle model status
  const toggleModelStatus = (modelId: string) => {
    setModels(prev => prev.map(model => 
      model.id === modelId 
        ? { 
            ...model, 
            status: model.status === 'active' ? 'maintenance' : 'active' 
          }
        : model
    ));
  };

  // Update model usage
  const updateModelUsage = (modelId: string, delta: number) => {
    setModels(prev => prev.map(model => 
      model.id === modelId 
        ? { ...model, usage: Math.max(0, model.usage + delta) }
        : model
    ));
  };

  // Get status color
  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active': return colors.success;
      case 'maintenance': return colors.warning;
      case 'disabled': return colors.error;
      default: return colors.textSecondary;
    }
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
            Aetheris Admin
          </h1>
          <p style={{ 
            color: colors.textSecondary, 
            fontSize: 14,
            margin: `${spacing.xsmall}px 0 0 0`
          }}>
            Model Management Dashboard
          </p>
        </div>
        <div style={{ 
          display: 'flex', 
          gap: spacing.small, 
          alignItems: 'center'
        }}>
          <span style={{ 
            fontSize: 12, 
            color: serverStatus === 'online' ? colors.success : colors.error
          }}>
            Server: {serverStatus}
          </span>
        </div>
      </header>

      {/* Stats Overview */}
      <section style={{ 
        padding: spacing.large,
        display: 'grid',
        gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))',
        gap: spacing.large,
        borderBottom: `1px solid ${colors.border}`
      }}>
        <div style={{ 
          background: colors.surface, 
          padding: spacing.medium,
          borderRadius: borderRadius.medium,
          textAlign: 'center'
        }}>
          <div style={{ 
            fontSize: 32, 
            fontWeight: 700, 
            color: colors.primary
          }}>
            {stats.totalRequests.toLocaleString()}
          </div>
          <div style={{ 
            fontSize: 12, 
            color: colors.textSecondary,
            marginTop: spacing.xsmall
          }}>
            Total Requests
          </div>
        </div>
        
        <div style={{ 
          background: colors.surface, 
          padding: spacing.medium,
          borderRadius: borderRadius.medium,
          textAlign: 'center'
        }}>
          <div style={{ 
            fontSize: 32, 
            fontWeight: 700, 
            color: colors.accent
          }}>
            {stats.activeUsers}
          </div>
          <div style={{ 
            fontSize: 12, 
            color: colors.textSecondary,
            marginTop: spacing.xsmall
          }}>
            Active Users
          </div>
        </div>
        
        <div style={{ 
          background: colors.surface, 
          padding: spacing.medium,
          borderRadius: borderRadius.medium,
          textAlign: 'center'
        }}>
          <div style={{ 
            fontSize: 32, 
            fontWeight: 700, 
            color: colors.secondary
          }}>
            {stats.uptime}
          </div>
          <div style={{ 
            fontSize: 12, 
            color: colors.textSecondary,
            marginTop: spacing.xsmall
          }}>
            Uptime
          </div>
        </div>
        
        <div style={{ 
          background: colors.surface, 
          padding: spacing.medium,
          borderRadius: borderRadius.medium,
          textAlign: 'center'
        }}>
          <div style={{ 
            fontSize: 32, 
            fontWeight: 700, 
            color: colors.cyberGlow
          }}>
            {models.filter(m => m.status === 'active').length}
          </div>
          <div style={{ 
            fontSize: 12, 
            color: colors.textSecondary,
            marginTop: spacing.xsmall
          }}>
            Active Models
          </div>
        </div>
      </section>

      {/* Tabs */}
      <nav style={{ 
        padding: `0 ${spacing.large}px`,
        borderBottom: `1px solid ${colors.border}`,
        display: 'flex',
        gap: spacing.large
      }}>
        <button
          onClick={() => setActiveTab('models')}
          style={{ 
            padding: `${spacing.small}px 0`,
            background: 'none',
            border: 'none',
            color: activeTab === 'models' ? colors.primary : colors.textSecondary,
            fontSize: 14,
            fontWeight: activeTab === 'models' ? 600 : 400,
            cursor: 'pointer',
            borderBottom: activeTab === 'models' ? `2px solid ${colors.primary}` : 'none'
          }}
        >
          Models
        </button>
        <button
          onClick={() => setActiveTab('users')}
          style={{ 
            padding: `${spacing.small}px 0`,
            background: 'none',
            border: 'none',
            color: activeTab === 'users' ? colors.primary : colors.textSecondary,
            fontSize: 14,
            fontWeight: activeTab === 'users' ? 600 : 400,
            cursor: 'pointer',
            borderBottom: activeTab === 'users' ? `2px solid ${colors.primary}` : 'none'
          }}
        >
          Users
        </button>
        <button
          onClick={() => setActiveTab('analytics')}
          style={{ 
            padding: `${spacing.small}px 0`,
            background: 'none',
            border: 'none',
            color: activeTab === 'analytics' ? colors.primary : colors.textSecondary,
            fontSize: 14,
            fontWeight: activeTab === 'analytics' ? 600 : 400,
            cursor: 'pointer',
            borderBottom: activeTab === 'analytics' ? `2px solid ${colors.primary}` : 'none'
          }}
        >
          Analytics
        </button>
      </nav>

      {/* Tab Content */}
      <main style={{ 
        flex: 1, 
        padding: spacing.large,
        overflowY: 'auto'
      }}>
        {activeTab === 'models' && (
          <div>
            <div style={{ 
              display: 'flex', 
              justifyContent: 'space-between', 
              alignItems: 'center',
              marginBottom: spacing.medium
            }}>
              <h2 style={{ 
                fontSize: 20, 
                fontWeight: 600, 
                color: colors.textPrimary,
                margin: 0
              }}>
                Model Management
              </h2>
            </div>
            
            <div style={{ 
              background: colors.surface, 
              borderRadius: borderRadius.medium,
              overflow: 'hidden'
            }}>
              <table style={{ width: '100%', borderCollapse: 'collapse' }}>
                <thead>
                  <tr style={{ background: colors.surfaceHover }}>
                    <th style={{ 
                      padding: spacing.medium, 
                      textAlign: 'left', 
                      fontSize: 12,
                      color: colors.textSecondary,
                      fontWeight: 600
                    }}>Model</th>
                    <th style={{ 
                      padding: spacing.medium, 
                      textAlign: 'left', 
                      fontSize: 12,
                      color: colors.textSecondary,
                      fontWeight: 600
                    }}>Description</th>
                    <th style={{ 
                      padding: spacing.medium, 
                      textAlign: 'left', 
                      fontSize: 12,
                      color: colors.textSecondary,
                      fontWeight: 600
                    }}>Max Tokens</th>
                    <th style={{ 
                      padding: spacing.medium, 
                      textAlign: 'left', 
                      fontSize: 12,
                      color: colors.textSecondary,
                      fontWeight: 600
                    }}>Usage</th>
                    <th style={{ 
                      padding: spacing.medium, 
                      textAlign: 'left', 
                      fontSize: 12,
                      color: colors.textSecondary,
                      fontWeight: 600
                    }}>Status</th>
                    <th style={{ 
                      padding: spacing.medium, 
                      textAlign: 'left', 
                      fontSize: 12,
                      color: colors.textSecondary,
                      fontWeight: 600
                    }}>Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {models.map(model => (
                    <tr key={model.id} style={{ borderBottom: `1px solid ${colors.border}` }}>
                      <td style={{ padding: spacing.medium, color: colors.textPrimary }}>
                        <strong>{model.label}</strong>
                        <div style={{ fontSize: 11, color: colors.textDisabled }}>
                          {model.id}
                        </div>
                      </td>
                      <td style={{ padding: spacing.medium, color: colors.textSecondary, fontSize: 13 }}>
                        {model.description}
                      </td>
                      <td style={{ padding: spacing.medium, color: colors.textPrimary }}>
                        {model.max_tokens.toLocaleString()}
                      </td>
                      <td style={{ padding: spacing.medium, color: colors.textPrimary }}>
                        {model.usage.toLocaleString()}
                      </td>
                      <td style={{ padding: spacing.medium }}>
                        <span style={{ 
                          padding: `${spacing.xsmall}px ${spacing.small}px`, 
                          background: getStatusColor(model.status) + '20',
                          color: getStatusColor(model.status),
                          borderRadius: borderRadius.xsmall,
                          fontSize: 11,
                          fontWeight: 600
                        }}>
                          {model.status.toUpperCase()}
                        </span>
                      </td>
                      <td style={{ padding: spacing.medium }}>
                        <button
                          onClick={() => toggleModelStatus(model.id)}
                          style={{ 
                            padding: `${spacing.xsmall}px ${spacing.small}px`, 
                            background: 'none',
                            border: `1px solid ${colors.border}`,
                            color: colors.textPrimary,
                            borderRadius: borderRadius.xsmall,
                            fontSize: 11,
                            cursor: 'pointer'
                          }}
                        >
                          {model.status === 'active' ? 'Disable' : 'Enable'}
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        )}

        {activeTab === 'users' && (
          <div>
            <div style={{ 
              display: 'flex', 
              justifyContent: 'space-between', 
              alignItems: 'center',
              marginBottom: spacing.medium
            }}>
              <h2 style={{ 
                fontSize: 20, 
                fontWeight: 600, 
                color: colors.textPrimary,
                margin: 0
              }}>
                User Management
              </h2>
            </div>
            
            <div style={{ 
              background: colors.surface, 
              borderRadius: borderRadius.medium,
              overflow: 'hidden'
            }}>
              <table style={{ width: '100%', borderCollapse: 'collapse' }}>
                <thead>
                  <tr style={{ background: colors.surfaceHover }}>
                    <th style={{ padding: spacing.medium, textAlign: 'left', fontSize: 12, color: colors.textSecondary, fontWeight: 600 }}>User</th>
                    <th style={{ padding: spacing.medium, textAlign: 'left', fontSize: 12, color: colors.textSecondary, fontWeight: 600 }}>Email</th>
                    <th style={{ padding: spacing.medium, textAlign: 'left', fontSize: 12, color: colors.textSecondary, fontWeight: 600 }}>Requests</th>
                    <th style={{ padding: spacing.medium, textAlign: 'left', fontSize: 12, color: colors.textSecondary, fontWeight: 600 }}>Last Active</th>
                  </tr>
                </thead>
                <tbody>
                  {users.map(user => (
                    <tr key={user.user_id} style={{ borderBottom: `1px solid ${colors.border}` }}>
                      <td style={{ padding: spacing.medium, color: colors.textPrimary }}>
                        {user.user_id}
                      </td>
                      <td style={{ padding: spacing.medium, color: colors.textPrimary }}>
                        {user.email}
                      </td>
                      <td style={{ padding: spacing.medium, color: colors.textPrimary }}>
                        {user.requests.toLocaleString()}
                      </td>
                      <td style={{ padding: spacing.medium, color: colors.textSecondary, fontSize: 12 }}>
                        {new Date(user.last_active).toLocaleString()}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        )}

        {activeTab === 'analytics' && (
          <div>
            <div style={{ 
              display: 'flex', 
              justifyContent: 'space-between', 
              alignItems: 'center',
              marginBottom: spacing.medium
            }}>
              <h2 style={{ 
                fontSize: 20, 
                fontWeight: 600, 
                color: colors.textPrimary,
                margin: 0
              }}>
                Analytics
              </h2>
            </div>
            
            <div style={{ 
              background: colors.surface, 
              borderRadius: borderRadius.medium,
              padding: spacing.large
            }}>
              <h3 style={{ 
                fontSize: 16, 
                fontWeight: 600, 
                color: colors.textPrimary,
                marginBottom: spacing.medium
              }}>
                Model Usage Statistics
              </h3>
              
              <div style={{ 
                display: 'grid', 
                gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))', 
                gap: spacing.medium
              }}>
                {models.map(model => (
                  <div key={model.id} style={{ 
                    background: colors.surfaceHover, 
                    padding: spacing.medium,
                    borderRadius: borderRadius.small
                  }}>
                    <div style={{ 
                      display: 'flex', 
                      justifyContent: 'space-between', 
                      alignItems: 'center',
                      marginBottom: spacing.small
                    }}>
                      <span style={{ fontWeight: 600, color: colors.textPrimary }}>
                        {model.label}
                      </span>
                      <span style={{ 
                        fontSize: 11, 
                        color: getStatusColor(model.status)
                      }}>
                        {model.status.toUpperCase()}
                      </span>
                    </div>
                    <div style={{ 
                      height: 8, 
                      background: colors.border, 
                      borderRadius: 4,
                      marginBottom: spacing.small,
                      overflow: 'hidden'
                    }}>
                      <div style={{ 
                        height: '100%', 
                        width: `${Math.min(100, (model.usage / model.max_tokens) * 100)}%`, 
                        background: colors.primary,
                        borderRadius: 4
                      }} />
                    </div>
                    <div style={{ 
                      display: 'flex', 
                      justifyContent: 'space-between',
                      fontSize: 11,
                      color: colors.textSecondary
                    }}>
                      <span>{model.usage.toLocaleString()} tokens</span>
                      <span>{model.max_tokens.toLocaleString()} max</span>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}
      </main>

      <footer style={{ 
        textAlign: 'center', 
        padding: spacing.large, 
        color: colors.textDisabled, 
        fontSize: 14,
        borderTop: `1px solid ${colors.border}`
      }}>
        Aetheris Admin Dashboard — Manage Puter LLM Models
      </footer>
    </div>
  );
}
