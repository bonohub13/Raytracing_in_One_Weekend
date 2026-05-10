import common.types;

[shader("fragment")]
float4 main(VSOutput in) : SV_Target {
  return float4(in.uv[0], 1 - in.uv[1], 0, 1);
}
