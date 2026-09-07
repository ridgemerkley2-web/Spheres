// Independent measurement of site-mesh.js against every numeric contract the
// P2 brief names. Deliberately NOT the check suite: it computes triangle counts,
// ground contact, monotonicity, the placeholder count and cross-process
// determinism from the buffers themselves, so a number quoted in a report is a
// number this script printed rather than one an assertion happened not to fail.
const path = require('node:path');
const crypto = require('node:crypto');
const cp = require('node:child_process');
const file = path.resolve(__dirname, '../../spheres-web/ui/site-mesh.js');
const site = require(file);

const STAGES = ['site', 'foundation', 'frame', 'enclosed', 'complete'];
const NEAR_MIN = 6000, NEAR_MAX = 40000, FAR_MIN = 100, FAR_MAX = 800;

const digestAll = () => {
  const h = crypto.createHash('sha256');
  for (const key of site.kinds()) {
    for (const stage of STAGES) {
      for (const lod of [0, 1]) {
        for (const level of [1, 3, 5]) {
          for (const status of site.statuses) {
            const m = site.build(key, stage, { lod, level, status, variant: level * 7 });
            h.update(Buffer.from(m.positions.buffer))
              .update(Buffer.from(m.normals.buffer))
              .update(Buffer.from(m.colors.buffer));
          }
        }
      }
    }
  }
  return h.digest('hex');
};
if (process.env.SITE_DIGEST_ONLY) { process.stdout.write(digestAll()); process.exit(0); }

const overNear = [], overFar = [], underNear = [], underFar = [], badGround = [], nonMono = [];
const rows = [];
let maxNear = 0, maxNearAt = '', maxFar = 0, maxFarAt = '';
let minNear = Infinity, minNearAt = '', minFar = Infinity, minFarAt = '';
for (const key of site.kinds()) {
  for (const lod of [0, 1]) {
    for (const level of [1, 2, 3, 4, 5]) {
      const counts = [];
      for (const stage of STAGES) {
        const m = site.build(key, stage, { lod, level });
        counts.push(m.triangleCount);
        if (m.bounds.min[1] !== 0) badGround.push(`${key}/${stage}/lod${lod}/L${level} min[1]=${m.bounds.min[1]}`);
        const n = m.triangleCount, at = `${key}/${stage}/L${level}`;
        if (lod === 0) {
          if (n > NEAR_MAX) overNear.push(`${at}=${n}`);
          if (n < NEAR_MIN) underNear.push(`${at}=${n}`);
          if (n > maxNear) { maxNear = n; maxNearAt = at; }
          if (n < minNear) { minNear = n; minNearAt = at; }
        } else {
          if (n > FAR_MAX) overFar.push(`${at}=${n}`);
          if (n < FAR_MIN) underFar.push(`${at}=${n}`);
          if (n > maxFar) { maxFar = n; maxFarAt = at; }
          if (n < minFar) { minFar = n; minFarAt = at; }
        }
      }
      if (!counts.every((v, i) => i === 0 || v >= counts[i - 1])) nonMono.push(`${key}/lod${lod}/L${level}: ${counts.join(' ')}`);
      // Monotonicity has to hold at every STATUS too: a stopped site draws a
      // stop board at the stages before hand-over and at none after it, so the
      // paused run is the one that goes backwards first.
      for (const status of site.statuses) {
        if (status === 'building') continue;
        const run = STAGES.map(stage => site.build(key, stage, { lod, level, status }).triangleCount);
        if (!run.every((v, i) => i === 0 || v >= run[i - 1])) nonMono.push(`${key}/lod${lod}/L${level}/${status}: ${run.join(' ')}`);
      }
      rows.push(`${key.padEnd(18)} lod${lod} L${level} ${counts.map(v => String(v).padStart(7)).join('')}`);
    }
  }
}
console.log('kind                lod lvl    site  found   frame  enclos  complet');
for (const r of rows) console.log(r);
console.log('');
console.log(`LOD0 range ${minNear} (${minNearAt}) .. ${maxNear} (${maxNearAt})    budget ${NEAR_MIN}..${NEAR_MAX}`);
console.log(`LOD1 range ${minFar} (${minFarAt}) .. ${maxFar} (${maxFarAt})    budget ${FAR_MIN}..${FAR_MAX}`);
console.log('over the 40,000 close ceiling :', overNear.length ? overNear.join(', ') : 'nothing');
console.log('under the 6,000 close floor   :', underNear.length ? underNear.join(', ') : 'nothing');
console.log('over the 800 map ceiling      :', overFar.length ? overFar.join(', ') : 'nothing');
console.log('under the 100 map floor       :', underFar.length ? underFar.join(', ') : 'nothing');
console.log('bounds.min[1] !== 0           :', badGround.length ? badGround.join(', ') : 'nothing (every mesh sits on Y=0)');
console.log('non-monotone stage runs       :', nonMono.length ? nonMono.join(' | ') : 'nothing');
const ph = site.kinds().filter(key => site.meta(key).placeholder);
console.log(`placeholder kinds             : ${ph.length}/13${ph.length ? ` (${ph.join(', ')})` : ''}`);

const local = digestAll();
const child = cp.execFileSync(process.execPath, [__filename], {
  env: Object.assign({}, process.env, { SITE_DIGEST_ONLY: '1' }), encoding: 'utf8',
}).trim();
console.log('determinism, two processes    :', local.slice(0, 16), 'vs', child.slice(0, 16),
  local === child ? 'IDENTICAL' : '*** DIFFERENT ***');
