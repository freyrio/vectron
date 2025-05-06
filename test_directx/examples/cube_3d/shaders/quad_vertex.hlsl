// Simple vertex shader for 2D quads
#pragma pack_matrix(row_major) // Match Rust matrix layout expectations

// Combined Model * Ortho Projection matrix
cbuffer TransformBuffer2D : register(b0) { // Use b0, assuming it's rebound per-draw
    matrix transform; // Expecting a 3x3 stored in a 4x4 for alignment, or adjust cbuffer/Rust side
                      // For now, assume Rust sends a 4x4 padded matrix
}

struct VSInput {
    float2 position : POSITION;
    float4 color : COLOR;
};

struct VSOutput {
    float4 position : SV_POSITION;  // Position in clip space (-1 to 1)
    float4 color : COLOR;           // Vertex color to pass to pixel shader
};

VSOutput main(VSInput input) {
    VSOutput output;
    
    // Assume transform is Model * Ortho
    // Use homogeneous coordinates (x, y, 1)
    float3 pos_h = float3(input.position.xy, 1.0);
    
    // Transform vertex
    // HLSL matrix multiplication is vector * matrix (if matrix is row-major)
    float3 transformed_h = mul(pos_h, transform);
    
    // Output requires float4 (x, y, z, w)
    // For 2D orthographic, z is usually 0 to 1, w is 1
    output.position = float4(transformed_h.xy, 0.5, 1.0);
    
    // Pass color through
    output.color = input.color;
    
    return output;
} 