'use strict';

// A building budget belongs to a physical building, while the complete campus,
// terrace or industrial yard also pays the unchanged scene budget. Never divide
// an assembly's cost by its building count. Shared geometry is charged in full
// to every building that owns it, and once to the complete assembly.
function measureBuildingUnits(mesh, where = 'mesh') {
  const fail = message => { throw new Error(`${where}: ${message}`); };
  if (!mesh || !mesh.positions || mesh.positions.length % 9) fail('invalid triangle positions');
  const vertices = mesh.positions.length / 3;
  if (!Array.isArray(mesh.budgetBuildings)) fail('missing physical building declarations');
  if (!Array.isArray(mesh.budgetUnits) || !mesh.budgetUnits.length) fail('missing constituent ranges');
  const buildings = new Map();
  for (const b of mesh.budgetBuildings) {
    if (!b || typeof b.id !== 'string' || !b.id || typeof b.label !== 'string' || !b.label || buildings.has(b.id)) {
      fail('invalid or duplicate physical building');
    }
    buildings.set(b.id, {id:b.id, label:b.label, triangles:0, ownedTriangles:0, sharedTriangles:0});
  }
  const ids = new Set(), ranges = [], totals = {building:0, shared:0, prop:0, terrain:0};
  for (const unit of mesh.budgetUnits) {
    if (!unit || typeof unit.id !== 'string' || !unit.id || ids.has(unit.id)) fail('invalid or duplicate constituent id');
    ids.add(unit.id);
    if (!Object.hasOwn(totals, unit.kind)) fail(`unknown constituent kind for ${unit.id}`);
    if (!Number.isInteger(unit.first) || !Number.isInteger(unit.count) || unit.first < 0 || unit.count <= 0
        || unit.first % 3 || unit.count % 3 || unit.first + unit.count > vertices) fail(`invalid vertex range for ${unit.id}`);
    if (!Array.isArray(unit.buildingIds) || new Set(unit.buildingIds).size !== unit.buildingIds.length) fail(`invalid owners for ${unit.id}`);
    if (unit.kind === 'building' && unit.buildingIds.length !== 1) fail(`building constituent ${unit.id} must have one owner`);
    if (unit.kind === 'shared' && !unit.buildingIds.length) fail(`shared constituent ${unit.id} has no owners`);
    if ((unit.kind === 'prop' || unit.kind === 'terrain') && unit.buildingIds.length) fail(`nonbuilding constituent ${unit.id} has building owners`);
    const triangles = unit.count / 3;
    totals[unit.kind] += triangles;
    for (const id of unit.buildingIds) {
      const b = buildings.get(id);
      if (!b) fail(`unknown building ${id} for ${unit.id}`);
      b.triangles += triangles;
      b[unit.kind === 'shared' ? 'sharedTriangles' : 'ownedTriangles'] += triangles;
    }
    ranges.push({first:unit.first, end:unit.first + unit.count});
  }
  ranges.sort((a,b)=>a.first-b.first);
  let end = 0;
  for (const range of ranges) {
    if (range.first !== end) fail(range.first < end ? 'overlapping constituent ranges' : 'unassigned geometry between constituents');
    end = range.end;
  }
  if (end !== vertices) fail('unassigned geometry after constituents');
  const rows = [...buildings.values()];
  if (rows.some(b=>!b.triangles)) fail('declared building has no geometry');
  return {assemblyTriangles:vertices / 3, physicalBuildings:rows.length, buildings:rows, totals};
}

module.exports = {measureBuildingUnits};
