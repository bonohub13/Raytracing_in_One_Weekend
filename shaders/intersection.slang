import common.types;

[shader("intersection")]
void main() {
  SphereAttr attr;
  uint material = 0;
  float hitT = 4.2;

  attr.normal = float3(0, 1, 0);

  ReportHit(hitT, material, attr);
}
