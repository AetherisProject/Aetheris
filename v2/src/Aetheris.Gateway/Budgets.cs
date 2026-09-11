using System.Collections.Concurrent;
using System.Text.Json;

namespace Aetheris.Gateway;

/// <summary>Tokens-per-day caps per model alias. Persisted as JSON; survives restarts.</summary>
public sealed class BudgetTracker
{
    private sealed record DaySpend(string Day, long Spent);
    private readonly string _path;
    private readonly Dictionary<string, long> _caps;
    private readonly ConcurrentDictionary<string, DaySpend> _spend = new();

    public BudgetTracker(string path, Dictionary<string, long> caps)
    {
        _path = path;
        _caps = caps;
        if (File.Exists(path))
        {
            var persisted = JsonSerializer.Deserialize<Dictionary<string, DaySpend>>(File.ReadAllText(path));
            if (persisted is not null)
                foreach (var (k, v) in persisted) _spend[k] = v;
        }
    }

    private static string Today => DateTimeOffset.UtcNow.ToString("yyyy-MM-dd");

    /// <returns>true if the alias may proceed; false when cap (tokens/day) is reached. 0 = unlimited.</returns>
    public bool Allow(string alias, long estimatedTokens)
    {
        if (!_caps.TryGetValue(alias, out var cap) || cap == 0) return true;
        var spend = _spend.GetOrAdd(alias, _ => new DaySpend(Today, 0));
        if (spend.Day != Today) spend = new DaySpend(Today, 0);
        return spend.Spent + estimatedTokens <= cap;
    }

    public void Report(string alias, long tokens)
    {
        _spend.AddOrUpdate(alias,
            _ => new DaySpend(Today, tokens),
            (_, old) => old.Day == Today ? old with { Spent = old.Spent + tokens } : new DaySpend(Today, tokens));
        Persist();
    }

    public object Snapshot() => _caps.ToDictionary(
        kv => kv.Key,
        kv => new
        {
            capPerDay = kv.Value,
            spentToday = _spend.TryGetValue(kv.Key, out var s) && s.Day == Today ? s.Spent : 0,
        });

    private void Persist()
    {
        try
        {
            Directory.CreateDirectory(Path.GetDirectoryName(_path) ?? ".");
            File.WriteAllText(_path, JsonSerializer.Serialize(_spend));
        }
        catch { /* budgets are best-effort */ }
    }
}

/// <summary>Bounded ring of REDACTED request logs. Keys never enter this structure.</summary>
public sealed class GatewayLog
{
    private readonly ConcurrentQueue<GatewayLogEntry> _entries = new();
    private const int Capacity = 200;

    public void Add(GatewayLogEntry entry)
    {
        _entries.Enqueue(entry);
        while (_entries.Count > Capacity && _entries.TryDequeue(out _)) { }
    }

    public IReadOnlyCollection<GatewayLogEntry> Entries => _entries.ToList();
}
