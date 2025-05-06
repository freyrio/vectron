#pragma pack_matrix(row_major)

// Updated constant buffer for transformations
cbuffer TransformBuffer : register(b0) {
    matrix model;
    matrix view;
    matrix projection;
    // Consider adding modelInverseTranspose if doing non-uniform scaling
}

struct VSInput {
    float3 position : POSITION;
    float3 normal : NORMAL; // Added normal input
};

struct VSOutput {
    float4 position : SV_POSITION;  // Position in clip space
    float3 worldPosition : TEXCOORD0; // World position for pixel shader calculations
    float3 worldNormal : TEXCOORD1;   // World normal for lighting calculations
};

VSOutput main(VSInput input) {
    VSOutput output;
    
    // Calculate world position
    float4 modelPos = float4(input.position, 1.0);
    float4 worldPos = mul(modelPos, model);
    output.worldPosition = worldPos.xyz;

    // Calculate clip space position
    float4 viewPos = mul(worldPos, view);
    float4 clipPos = mul(viewPos, projection);
    output.position = clipPos;

    // Transform normal to world space
    // Basic transformation (assumes uniform scaling or uses inverse transpose)
    // For non-uniform scaling, multiply by transpose(inverse(model)) instead of just model
    float3 worldNorm = normalize(mul(input.normal, (float3x3)model)); // Cast to 3x3 avoids transforming direction by translation
    output.worldNormal = worldNorm;
    
    return output;
} 