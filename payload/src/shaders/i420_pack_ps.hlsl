// Recording pack pixel shader: converts one eye's display-referred RGBA capture into a planar
// 4:2:0 YUV (I420) image packed into a single R8 render target, so one staging readback of that
// target yields the exact byte layout a `yuv4mpegpipe` frame wants.
//
// The target is `width` x `height * 3 / 2` pixels, `width` and `height` both even. Rows
// `[0, height)` are the full-resolution luma plane. The remaining `height / 2` rows hold the two
// quarter-size chroma planes back to back, each `width / 2` x `height / 2` samples laid out
// linearly: U first, then V. A row of the packed region therefore holds two chroma rows of one
// plane, and the plane boundary falls at row `height + height / 4`.
//
// Luma samples at the pixel centre; chroma samples at the centre of each 2x2 block, which with the
// bilinear sampler averages the block (the "centred" 4:2:0 siting, `C420jpeg` in y4m terms).
// The conversion is BT.709 with limited (video) range, the convention every consumer assumes for
// HD material. The source is scaled to the target size by the sampler, so a `scale` below one in
// the recorder simply gives a smaller target.

cbuffer PackParams : register(b0)
{
    // The output luma plane size (`width`, `height`), both even.
    uint2 size;
    uint2 padding;
};

Texture2D EyeTexture : register(t0);
SamplerState EyeSampler : register(s0);

float luma(float3 rgb)
{
    return 16.0 / 255.0 + (219.0 / 255.0) * dot(rgb, float3(0.2126, 0.7152, 0.0722));
}

float chroma_u(float3 rgb)
{
    return 128.0 / 255.0 + (224.0 / 255.0) * dot(rgb, float3(-0.1146, -0.3854, 0.5));
}

float chroma_v(float3 rgb)
{
    return 128.0 / 255.0 + (224.0 / 255.0) * dot(rgb, float3(0.5, -0.4542, -0.0458));
}

float main(float4 position : SV_Position, float2 uv : TEXCOORD0) : SV_Target
{
    uint x = (uint)position.x;
    uint y = (uint)position.y;
    float2 inv_size = 1.0 / float2(size);
    if (y < size.y)
    {
        float3 rgb = EyeTexture.Sample(EyeSampler, (float2(x, y) + 0.5) * inv_size).rgb;
        return luma(rgb);
    }
    uint half_width = size.x / 2;
    uint plane_samples = half_width * (size.y / 2);
    uint index = (y - size.y) * size.x + x;
    bool is_v = index >= plane_samples;
    uint sample_index = is_v ? index - plane_samples : index;
    uint cx = sample_index % half_width;
    uint cy = sample_index / half_width;
    float2 centre = (float2(cx, cy) * 2.0 + 1.0) * inv_size;
    float3 rgb = EyeTexture.Sample(EyeSampler, centre).rgb;
    return is_v ? chroma_v(rgb) : chroma_u(rgb);
}
