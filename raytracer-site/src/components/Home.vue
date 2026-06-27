<template>
  <div>
    <div class="hero">
      <div class="hero-badge">Written in Rust</div>
      <h1>Ray Tracing<br /><span class="accent">from Scratch</span></h1>
      <p class="hero-sub">
        A software ray tracer built in Rust. Physically-based lighting, three material types,
        depth of field, and a random scene generator. No GPU, no external rendering library.
      </p>
      <div class="stat-row">
        <div class="stat"><span class="stat-n">500</span><span class="stat-l">samples/pixel</span></div>
        <div class="stat"><span class="stat-n">50</span><span class="stat-l">max ray depth</span></div>
        <div class="stat"><span class="stat-n">3</span><span class="stat-l">material types</span></div>
        <div class="stat"><span class="stat-n">1200px</span><span class="stat-l">output width</span></div>
      </div>
    </div>

    <hr class="divider" />

    <h2>How it works</h2>
    <p>
      Ray tracing works backwards from real light. Instead of photons leaving a light source, we shoot rays
      <em>from the camera</em> into the scene and follow them as they bounce off surfaces. For each pixel,
      many rays are cast with slight random offsets and their colors are averaged to produce smooth,
      anti-aliased results.
    </p>

    <div class="pipeline">
      <div class="step" v-for="(s, i) in steps" :key="i">
        <div class="step-num">{{ i + 1 }}</div>
        <div>
          <div class="step-title">{{ s.title }}</div>
          <div class="step-desc">{{ s.desc }}</div>
        </div>
      </div>
    </div>

    <hr class="divider" />

    <h2>Project structure</h2>
    <div class="file-tree">
      <div v-for="f in files" :key="f.name" class="file-row">
        <code class="file-name">{{ f.name }}</code>
        <span class="file-desc">{{ f.desc }}</span>
      </div>
    </div>

    <hr class="divider" />

    <h2>Explore the math</h2>
    <div class="topic-grid">
      <RouterLink v-for="t in topics" :key="t.to" :to="t.to" class="topic-card">
        <div class="topic-icon">{{ t.icon }}</div>
        <div class="topic-title">{{ t.title }}</div>
        <div class="topic-desc">{{ t.desc }}</div>
      </RouterLink>
    </div>
  </div>
</template>

<script setup>
const steps = [
  { title: 'Cast a ray', desc: 'For each pixel, fire a ray from the camera through the viewport into the scene.' },
  { title: 'Find intersections', desc: 'Test the ray against every object. The closest hit wins.' },
  { title: 'Scatter based on material', desc: 'Lambertian scatters randomly, metal reflects, glass refracts.' },
  { title: 'Recurse', desc: 'Follow the scattered ray until max depth or a miss (sky gradient).' },
  { title: 'Average samples', desc: 'Repeat 500× per pixel with slight random jitter for anti-aliasing.' },
]

const files = [
  { name: 'Vec3.rs',         desc: '3D vector with dot, cross, reflect, refract helpers' },
  { name: 'Ray.rs',          desc: 'Ray struct: origin + direction, parametric point P(t)' },
  { name: 'sphere.rs',       desc: 'Sphere hit test via quadratic discriminant' },
  { name: 'Hitteble.rs',     desc: 'Hittable trait + HitRecord (position, normal, material)' },
  { name: 'material.rs',     desc: 'Lambertian, Metal, Dielectric scatter implementations' },
  { name: 'camera.rs',       desc: 'Positionable camera with depth-of-field (aperture + focus)' },
  { name: 'Color.rs',        desc: 'Color type + gamma-corrected PPM output' },
  { name: 'main.rs',         desc: 'Scene setup, render loop, random scene generator' },
]

const topics = [
  { to: '/vectors',   icon: '→', title: 'Vectors & Math',  desc: 'Dot product, cross product, reflection and refraction formulas.' },
  { to: '/rays',      icon: '⟶', title: 'Rays & Spheres',  desc: 'Parametric rays and the quadratic sphere intersection test.' },
  { to: '/materials', icon: '◇', title: 'Materials',        desc: 'Diffuse scattering, specular reflection, Snell\'s law, Schlick.' },
  { to: '/camera',    icon: '⊡', title: 'Camera',           desc: 'Perspective projection, FOV, depth of field via thin lens.' },
]
</script>

<style scoped>
.hero {
  padding: 1rem 0 2rem;
}
.hero-badge {
  display: inline-block;
  background: #1a1535;
  color: var(--accent);
  border: 1px solid var(--accent);
  border-radius: 20px;
  padding: 0.25rem 0.9rem;
  font-size: 0.78rem;
  font-weight: 600;
  letter-spacing: 0.05em;
  margin-bottom: 1.2rem;
}
h1 { font-size: 3rem; }
.accent { color: var(--accent); }
.hero-sub { font-size: 1.05rem; color: var(--muted); max-width: 560px; margin-top: 1rem; }
.stat-row {
  display: flex; gap: 2rem; margin-top: 2rem; flex-wrap: wrap;
}
.stat { display: flex; flex-direction: column; }
.stat-n { font-size: 1.8rem; font-weight: 700; color: var(--accent2); line-height: 1; }
.stat-l { font-size: 0.78rem; color: var(--muted); margin-top: 0.2rem; }

.pipeline {
  display: flex; flex-direction: column; gap: 0.75rem; margin: 1.5rem 0;
}
.step {
  display: flex; align-items: flex-start; gap: 1rem;
  background: var(--surface); border: 1px solid var(--border); border-radius: 10px; padding: 1rem 1.2rem;
}
.step-num {
  background: var(--accent); color: #fff; border-radius: 50%; width: 28px; height: 28px;
  display: flex; align-items: center; justify-content: center; font-size: 0.8rem; font-weight: 700; flex-shrink: 0;
}
.step-title { font-weight: 600; font-size: 0.95rem; margin-bottom: 0.2rem; }
.step-desc { color: var(--muted); font-size: 0.88rem; }

.file-tree {
  display: flex; flex-direction: column; gap: 0.5rem; margin: 1rem 0;
}
.file-row {
  display: flex; align-items: baseline; gap: 1rem;
  padding: 0.6rem 1rem; background: var(--surface); border: 1px solid var(--border); border-radius: 8px;
}
.file-name { min-width: 140px; font-size: 0.85rem; }
.file-desc { color: var(--muted); font-size: 0.85rem; }

.topic-grid {
  display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 1rem; margin: 1.2rem 0;
}
.topic-card {
  background: var(--surface); border: 1px solid var(--border); border-radius: 12px;
  padding: 1.4rem 1.2rem; color: var(--text); transition: border-color 0.15s, background 0.15s;
  display: flex; flex-direction: column; gap: 0.4rem;
}
.topic-card:hover { border-color: var(--accent); background: #1a1535; }
.topic-icon { font-size: 1.5rem; }
.topic-title { font-weight: 600; font-size: 0.95rem; }
.topic-desc { color: var(--muted); font-size: 0.82rem; line-height: 1.4; }
</style>
