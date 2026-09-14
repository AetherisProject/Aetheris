using Aetheris.Gateway;
using Aetheris.Gateway.Services;

namespace Aetheris.Gateway.Controllers;

[ApiController]
[Route("v1")]
public sealed class ModelsController : ControllerBase
{
    private readonly ProviderService _providerService;
    private readonly GatewayConfig _config;

    public ModelsController(ProviderService providerService, GatewayConfig config)
    {
        _providerService = providerService;
        _config = config;
    }

    [HttpGet("models")]
    public IActionResult GetModels()
    {
        // Merge aliases with provider models (3.6)
        var providerModels = _providerService.GetCatalog();
        
        var aliases = _config.Aliases.Select(a => new
        {
            id = a.Key,
            provider = a.Value.Provider,
            model = a.Value.Model,
            fallback = a.Value.FallbackTo,
        }).ToList();

        return Ok(new
        {
            aliases,
            providers = providerModels
        });
    }

    [HttpGet("models/{model}")]
    public IActionResult GetModel(string model)
    {
        // Check if it's an alias
        if (_config.Aliases.TryGetValue(model, out var alias))
        {
            var provider = _providerService.GetProvider(alias.Provider);
            var providerModel = provider?.Models.TryGetValue(alias.Model, out var pm) == true ? pm : null;
            
            return Ok(new
            {
                id = model,
                object = "model",
                created = DateTimeOffset.UtcNow.ToUnixTimeSeconds(),
                owned_by = alias.Provider,
                alias_of = alias.Model,
                fallback = alias.FallbackTo,
                capabilities = providerModel != null ? new
                {
                    context_length = providerModel.ContextLength,
                    max_output = providerModel.MaxOutput,
                    type = providerModel.Type
                } : null
            });
        }

        // Check if it's a provider model
        foreach (var provider in _providerService.Providers.Values)
        {
            if (provider.Models.TryGetValue(model, out var providerModel))
            {
                return Ok(new
                {
                    id = model,
                    object = "model",
                    created = DateTimeOffset.UtcNow.ToUnixTimeSeconds(),
                    owned_by = provider.Id,
                    capabilities = new
                    {
                        context_length = providerModel.ContextLength,
                        max_output = providerModel.MaxOutput,
                        type = providerModel.Type
                    }
                });
            }
        }

        return NotFound(new { error = $"Model '{model}' not found" });
    }
}