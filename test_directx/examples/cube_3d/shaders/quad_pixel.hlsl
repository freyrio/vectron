// Simple pixel shader for 2D quads

struct PSInput {
    float4 position : SV_POSITION;
    float4 color : COLOR; // Color received from vertex shader
};

float4 main(PSInput input) : SV_TARGET {
    // Output the interpolated vertex color
    return input.color;
} 