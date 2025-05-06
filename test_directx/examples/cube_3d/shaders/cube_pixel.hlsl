// Pixel shader with basic lighting

// Material properties buffer (matches Rust struct)
cbuffer MaterialBuffer : register(b1) {
    int colorType;              // 0=Solid, 1=LinearGradient
    float3 _padding1;           // Padding to align next float4
    float4 solidColor;          // RGBA
    float4 gradientColorStart;  // RGBA
    float4 gradientColorEnd;    // RGBA
    float3 gradientDirection;   // Normalized direction (Model Space)
    float _padding2;            // Padding
}

// Light properties buffer (matches Rust struct)
cbuffer LightBuffer : register(b2) {
    float3 lightDirection; // Direction TO the light (normalized)
    float _padding_light_dir;
    float4 lightColor;     // Light color/intensity
    float4 ambientColor;   // Ambient light color/intensity
}

struct PSInput {
    float4 position : SV_POSITION;
    float3 worldPosition : TEXCOORD0; // Received from vertex shader
    float3 worldNormal : TEXCOORD1;   // Received from vertex shader (Normalized)
};

float4 main(PSInput input) : SV_TARGET {
    float4 materialColor;

    // 1. Determine base material color (Solid or Gradient)
    if (colorType == 1) { // Linear Gradient
        // Gradient calculation still uses model-space position passed via worldPosition semantic
        // Consider passing actual world position if gradient should be world-aligned
        float dot_product = dot(input.worldPosition, gradientDirection);
        float gradient_t = saturate(dot_product + 0.5); // Map [-0.5, 0.5] -> [0, 1]
        materialColor = lerp(gradientColorStart, gradientColorEnd, gradient_t);
    } else { // Solid Color
        materialColor = solidColor;
    }

    // 2. Calculate Lighting
    // Ensure normal is normalized (should be done in VS, but safety check)
    float3 N = normalize(input.worldNormal);
    // Ensure light direction is normalized (should be done on CPU, but safety check)
    float3 L = normalize(lightDirection);
    
    // Calculate Diffuse Term (Lambertian)
    float diffuseFactor = saturate(dot(N, -L)); // Light direction is TO the light
    float4 diffuseLight = diffuseFactor * lightColor;

    // 3. Combine Lighting and Material Color
    // FinalColor = Ambient + Diffuse
    // Modulate material color with combined light
    float4 finalColor = materialColor * (ambientColor + diffuseLight);

    // Ensure alpha is 1.0 (or use materialColor.a)
    finalColor.a = 1.0;

    return finalColor;
} 