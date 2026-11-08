// Run npm ci && npm run assets:globe to rebuild the decorative illustration.
// Natural Earth map data is public domain. D3 performs the geographic projection.
import { geoOrthographic, geoPath, geoGraticule10 } from 'd3-geo';
import { feature } from 'topojson-client';
import { writeFile } from 'node:fs/promises';

const response = await fetch('https://cdn.jsdelivr.net/npm/world-atlas@2.0.2/land-110m.json');
if (!response.ok) throw new Error(`Map download: ${response.status}`);
const world = await response.json();
const projection = geoOrthographic().rotate([28, -14]).scale(245).translate([270, 270]);
const path = geoPath(projection);
const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 540 540">
<defs>
  <clipPath id="sphere"><circle cx="270" cy="270" r="245"/></clipPath>
  <radialGradient id="ocean" cx="32%" cy="25%" r="85%"><stop stop-color="#3974d5"/><stop offset=".45" stop-color="#1852b5"/><stop offset=".8" stop-color="#123e93"/><stop offset="1" stop-color="#102e6b"/></radialGradient>
  <linearGradient id="land" x1="0" y1="0" x2="1" y2="1"><stop stop-color="#b8bd85"/><stop offset=".5" stop-color="#98a677"/><stop offset="1" stop-color="#768963"/></linearGradient>
</defs>
<circle cx="270" cy="270" r="245" fill="url(#ocean)" stroke="#183f72" stroke-width="1.5"/>
<g clip-path="url(#sphere)">
  <g class="currents" fill="none" stroke="#a6d6f5" stroke-width="1.2" opacity=".22"><path d="M-30 166Q81 105 187 170T399 165T610 170M-30 174Q81 113 187 178T399 173T610 178M-30 311Q89 251 190 317T399 310T610 316M-30 319Q89 259 190 325T399 318T610 324M-30 422Q89 370 190 426T399 420T610 425"/></g>
  <path d="${path(geoGraticule10())}" fill="none" stroke="#c8e1ef" stroke-opacity=".25" stroke-width=".65"/>
  <path d="${path(feature(world, world.objects.land))}" fill="url(#land)" stroke="#d7d6a4" stroke-width=".65" stroke-linejoin="round"/>
</g>
<circle cx="270" cy="270" r="245" fill="none" stroke="#233f53" stroke-width="1.5"/>
<g fill="#d67145" stroke="#f7f4ec" stroke-width="3"><circle cx="152" cy="165" r="6"/><circle cx="362" cy="142" r="6"/><circle cx="332" cy="339" r="6"/></g>
<path d="M153 165Q207 64 360 143" fill="none" stroke="#f0d8a6" stroke-width="1.4" stroke-dasharray="4 5"/>
</svg>`;
await writeFile(new URL('../public/globe.svg', import.meta.url), svg);
console.log('Generated public/globe.svg from Natural Earth public-domain data.');
