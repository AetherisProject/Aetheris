using Microsoft.AspNetCore.Mvc;
using Aetheris.Gateway.Services;

namespace Aetheris.Gateway.Controllers;

[ApiController]
[Route("v1/gateway")]
public sealed class HealthController : ControllerBase
{
    private readonly HealthCheckService _healthService;

    public HealthController(HealthCheckService healthService)
    {
        _healthService = healthService;
    }

    [HttpGet("health")]
    public IActionResult GetHealth()
    {
        var status = _healthService.GetStatus();
        return Ok(status);
    }

    [HttpGet("health/{provider}")]
    public IActionResult GetProviderHealth(string provider)
    {
        var status = _healthService.GetProviderStatus(provider);
        if (status == null)
            return NotFound(new { error = $"Provider '{provider}' not found" });
        return Ok(status);
    }
}