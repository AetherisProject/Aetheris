using System.Security.Cryptography;
using System.Text;
using Aetheris.Hub.Models;
using Aetheris.Hub.Services;
using Org.BouncyCastle.Crypto;
using Org.BouncyCastle.Crypto.Parameters;
using Org.BouncyCastle.Crypto.Signers;

namespace Aetheris.Hub.Controllers;

public sealed class DevicesController : ControllerBase
{
    private readonly IDeviceStore _deviceStore;
    private readonly string _accessToken;
    private readonly ILogger<DevicesController> _logger;

    public DevicesController(
        IDeviceStore deviceStore,
        IConfiguration configuration,
        ILogger<DevicesController> logger)
    {
        _deviceStore = deviceStore ?? throw new ArgumentNullException(nameof(deviceStore));
        _accessToken = configuration["AETHERIS_TOKEN"] ?? "dev-token";
        _logger = logger;
    }

    [HttpPost("/v1/devices")]
    public async Task<IActionResult> RegisterDevice([FromBody] DeviceRegistrationRequest request)
    {
        try
        {
            // Validate required fields
            if (string.IsNullOrWhiteSpace(request?.Name))
                return BadRequest(new { error = "name_required" });

            if (string.IsNullOrWhiteSpace(request.PublicKeyBase64))
                return BadRequest(new { error = "public_key_required" });

            if (string.IsNullOrWhiteSpace(request.SignatureBase64))
                return BadRequest(new { error = "signature_required" });

            // Verify the Ed25519 signature
            if (!VerifyDeviceSignature(request))
                return Unauthorized(new { error = "invalid_signature" });

            // Create device ID from public key hash
            var deviceId = ComputeDeviceId(request.PublicKeyBase64);

            // Check if device already exists
            var existing = await _deviceStore.GetDeviceAsync(deviceId);
            if (existing != null)
                return Conflict(new { error = "device_already_exists" });

            // Store the device
            var device = new Device(
                Id: deviceId,
                Name: request.Name,
                PublicKeyBase64: request.PublicKeyBase64,
                CreatedAt: DateTimeOffset.UtcNow
            );

            await _deviceStore.StoreDeviceAsync(device);

            return Created($"/v1/devices/{deviceId}", new DeviceRegistrationResponse(
                DeviceId: deviceId,
                Name: device.Name,
                CreatedAt: device.CreatedAt
            ));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Device registration failed");
            return StatusCode(500, new { error = "internal_error" });
        }
    }

    [HttpPost("/v1/devices/auth")]
    public async Task<IActionResult> AuthenticateDevice([FromBody] DeviceAuthRequest request)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(request?.DeviceId))
                return BadRequest(new { error = "device_id_required" });

            if (string.IsNullOrWhiteSpace(request.SignatureBase64))
                return BadRequest(new { error = "signature_required" });

            if (string.IsNullOrWhiteSpace(request.ChallengeNonceBase64))
                return BadRequest(new { error = "challenge_required" });

            // Get device
            var device = await _deviceStore.GetDeviceAsync(request.DeviceId);
            if (device == null)
                return NotFound(new { error = "device_not_found" });

            // Verify Ed25519 signature (challenge-based auth)
            if (!VerifyAuthSignature(device.PublicKeyBase64, request.SignatureBase64, request.ChallengeNonceBase64))
                return Unauthorized(new { error = "invalid_signature" });

            // Generate session token (in a real system, this would be JWT or similar)
            var sessionToken = Convert.ToBase64String(RandomNumberGenerator.GetBytes(32));

            return Ok(new { token = sessionToken });
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Device authentication failed");
            return StatusCode(500, new { error = "internal_error" });
        }
    }

    private bool VerifyDeviceSignature(DeviceRegistrationRequest request)
    {
        try
        {
            // Create a challenge message for registration
            var challenge = $"register:{request.Name}:{request.PublicKeyBase64}";
            var challengeBytes = Encoding.UTF8.GetBytes(challenge);

            var publicKeyBytes = Convert.FromBase64String(request.PublicKeyBase64);
            var signatureBytes = Convert.FromBase64String(request.SignatureBase64);

            // Parse Ed25519 public key
            var publicKey = new Ed25519PublicKeyParameters(publicKeyBytes, 0);

            // Verify Ed25519 signature
            var signer = new Ed25519Signer();
            signer.Init(false, publicKey);
            signer.BlockUpdate(challengeBytes, 0, challengeBytes.Length);
            return signer.VerifySignature(signatureBytes);
        }
        catch
        {
            return false;
        }
    }

    private bool VerifyAuthSignature(string publicKeyBase64, string signatureBase64, string challengeNonceBase64)
    {
        try
        {
            var challengeBytes = Convert.FromBase64String(challengeNonceBase64);
            var publicKeyBytes = Convert.FromBase64String(publicKeyBase64);
            var signatureBytes = Convert.FromBase64String(signatureBase64);

            // Parse Ed25519 public key
            var publicKey = new Ed25519PublicKeyParameters(publicKeyBytes, 0);

            // Verify Ed25519 signature
            var signer = new Ed25519Signer();
            signer.Init(false, publicKey);
            signer.BlockUpdate(challengeBytes, 0, challengeBytes.Length);
            return signer.VerifySignature(signatureBytes);
        }
        catch
        {
            return false;
        }
    }

    private string ComputeDeviceId(string publicKeyBase64)
    {
        var publicKeyBytes = Convert.FromBase64String(publicKeyBase64);
        using var sha256 = SHA256.Create();
        var hashBytes = sha256.ComputeHash(publicKeyBytes);
        return Convert.ToBase64String(hashBytes)[..22]; // Short ID
    }
}
