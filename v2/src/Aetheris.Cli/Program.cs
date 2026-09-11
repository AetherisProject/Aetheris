using System.Diagnostics;
using System.Text;
using Aetheris.Core.Vault;

// aeth — Aetheris CLI (baseline version; W5 upgrades to Spectre.Console + clipboard + JSON mode).
// Usage:
//   aeth init --vault keys.vault
//   aeth add password --vault keys.vault --title GitHub [--user me]
//   aeth add apikey  --vault keys.vault --provider openai --title "OpenAI prod"
//   aeth ls   [--vault keys.vault]
//   aeth get  <id|title> [--show] [--vault keys.vault]
//   aeth run  --env OPENAI_API_KEY=<id|title> -- <command> [args...]
// Master password comes from AETHERIS_PASS or an interactive masked prompt.

var vaultPath = ArgValue(args, "--vault") ?? "aetheris.vault";

if (args.Length == 0 || args[0] is "help" or "--help" or "-h") { Help(); return 0; }

switch (args[0])
{
    case "init":
    {
        var pass = ReadMasterPassword(confirm: true);
        using var store = VaultStore.Create(vaultPath, pass);
        Console.WriteLine($"✓ vault created: {vaultPath}");
        return 0;
    }
    case "add":
    {
        var kind = args.Length > 1 ? args[1] : "";
        using var store = VaultStore.Unlock(vaultPath, ReadMasterPassword(false));
        VaultItem item = kind switch
        {
            "password" => store.Add(ItemType.Password,
                Required(args, "--title"),
                new PasswordPayload(ArgValue(args, "--user") ?? "", ReadSecret("password"),
                    new List<string>(), null, null)),
            "apikey" => store.Add(ItemType.ApiKey,
                Required(args, "--title"),
                new ApiKeyPayload(Required(args, "--provider"), ReadSecret("api key"),
                    new List<string>(), null, null)),
            "note" => store.Add(ItemType.Note, Required(args, "--title"), new NotePayload(ReadSecret("note"))),
            _ => throw new Aetheris.Core.Crypto.AetherisException("add what? password | apikey | note"),
        };
        Console.WriteLine($"✓ added {item.Type} '{item.Title}' ({item.Id})");
        return 0;
    }
    case "ls":
    {
        using var store = VaultStore.Unlock(vaultPath, ReadMasterPassword(false));
        foreach (var i in store.Items)
            Console.WriteLine($"{i.Id[..8]}  {i.Type,-10}  {i.Title}");
        Console.WriteLine($"— {store.Items.Count} item(s), generation {store.Generation}");
        return 0;
    }
    case "get":
    {
        var id = args.Length > 1 ? args[1] : throw new ArgumentException("get <id|title>");
        using var store = VaultStore.Unlock(vaultPath, ReadMasterPassword(false));
        var item = store.Find(id) ?? throw new Aetheris.Core.Crypto.AetherisException($"'{id}' not found");
        var show = args.Contains("--show");
        Console.WriteLine($"{item.Type} '{item.Title}'  tags=[{string.Join(',', item.Tags)}]");
        Console.WriteLine(show
            ? item.PayloadJson
            : "(payload hidden — pass --show | W5: clipboard with auto-clear)");
        return 0;
    }
    case "run":
    {
        // Env injection: vault key is handed to the child process ONLY —
        // never printed, never in history.
        var envSpec = ArgValue(args, "--env") ?? throw new ArgumentException("run --env NAME=<id|title>");
        var eq = envSpec.IndexOf('=');
        if (eq < 1) throw new ArgumentException("--env must be NAME=<id|title>");
        var (envName, itemRef) = (envSpec[..eq], envSpec[(eq + 1)..]);

        var dash = Array.IndexOf(args, "--");
        if (dash < 0 || dash == args.Length - 1) throw new ArgumentException("missing '-- <command>'");

        using var store = VaultStore.Unlock(vaultPath, ReadMasterPassword(false));
        var item = store.Find(itemRef) ?? throw new Aetheris.Core.Crypto.AetherisException($"'{itemRef}' not found");
        var value = item.Type == ItemType.ApiKey ? item.Payload<ApiKeyPayload>().Key : item.Payload<PasswordPayload>().Password;

        var psi = new ProcessStartInfo(args[dash + 1], args[(dash + 2)..])
        {
            UseShellExecute = false,
            RedirectStandardOutput = false,
        };
        psi.Environment[envName] = value;
        using var process = Process.Start(psi)!;
        process.WaitForExit();
        Console.Error.WriteLine($"— aeth: injected {envName} from '{item.Title}' into child process (value never displayed)");
        return process.ExitCode;
    }
    default:
        Console.Error.WriteLine($"unknown command '{args[0]}'");
        Help();
        return 1;
}

static void Help() => Console.WriteLine("""
    aeth — Aetheris CLI
      init    --vault <path>
      add     password|apikey|note --title <t> [--user u] [--provider p] --vault <path>
      ls      --vault <path>
      get     <id|title> [--show] --vault <path>
      run     --env NAME=<id|title> -- <command> [args...]     (env injection, key never printed)
    """);

static string? ArgValue(string[] argv, string name)
{
    for (var i = 0; i < argv.Length - 1; i++)
        if (argv[i] == name) return argv[i + 1];
    return null;
}

static string Required(string[] argv, string name) =>
    ArgValue(argv, name) ?? throw new ArgumentException($"missing {name}");

static string ReadMasterPassword(bool confirm)
{
    var existing = Environment.GetEnvironmentVariable("AETHERIS_PASS");
    if (!string.IsNullOrEmpty(existing)) return existing;
    var first = ReadSecret("master password");
    if (!confirm) return first;
    var second = ReadSecret("confirm master password");
    if (first != second) throw new InvalidOperationException("passwords do not match");
    return first;
}

static string ReadSecret(string label)
{
    Console.Error.Write($"{label}: ");
    var sb = new StringBuilder();
    while (true)
    {
        var key = Console.ReadKey(intercept: true);
        if (key.Key == ConsoleKey.Enter) break;
        if (key.Key == ConsoleKey.Backspace && sb.Length > 0) sb.Length--;
        else if (!char.IsControl(key.KeyChar)) sb.Append(key.KeyChar);
    }
    Console.Error.WriteLine();
    var value = sb.ToString();
    if (value.Length == 0) throw new InvalidOperationException($"{label} must not be empty");
    return value;
}
