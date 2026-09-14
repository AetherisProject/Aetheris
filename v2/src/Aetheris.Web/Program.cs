using Aetheris.Web;
using Aetheris.Web.Services;
using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using Microsoft.Extensions.Configuration;

var builder = WebAssemblyHostBuilder.CreateDefault(args);
builder.RootComponents.Add<App>("#app");
builder.RootComponents.Add<HeadOutlet>("head::after");

// Configure services
builder.Services.AddScoped<VaultState>();
builder.Services.AddScoped<CryptoService>();

// Configure HttpClient for HubClient with base address from config
builder.Services.AddHttpClient<HubClient>(client =>
{
    var config = builder.Configuration;
    client.BaseAddress = new Uri(config["HubUrl"] ?? "http://127.0.0.1:8080");
    client.Timeout = TimeSpan.FromSeconds(30);
});

// Add standard HttpClient for other uses
builder.Services.AddScoped(sp => new HttpClient { 
    BaseAddress = new Uri(builder.HostEnvironment.BaseAddress) 
});

// Build and run
await builder.Build().RunAsync();
