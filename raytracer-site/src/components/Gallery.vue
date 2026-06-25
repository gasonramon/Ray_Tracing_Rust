<template>
  <div>
    <h1>Gallery</h1>
    <p>
      Each render was produced by the Rust ray tracer — no post-processing.
      The final scene uses 500 samples per pixel and 50 bounce depth.
    </p>

    <div class="render-feature">
      <div class="render-placeholder big">
        <div class="placeholder-inner">
          <div class="placeholder-icon">◈</div>
          <div class="placeholder-title">Final Random Scene</div>
          <div class="placeholder-sub">1200 × 800 · 500 spp · 50 depth · ~484 spheres</div>
        </div>
      </div>
      <div class="render-meta">
        <div class="meta-row"><span class="meta-key">Resolution</span><span>1200 × 800</span></div>
        <div class="meta-row"><span class="meta-key">Samples / pixel</span><span>500</span></div>
        <div class="meta-row"><span class="meta-key">Max depth</span><span>50 bounces</span></div>
        <div class="meta-row"><span class="meta-key">Objects</span><span>~484 spheres</span></div>
        <div class="meta-row"><span class="meta-key">Camera FOV</span><span>20° (telephoto)</span></div>
        <div class="meta-row"><span class="meta-key">Aperture</span><span>0.1 (slight DoF)</span></div>
        <div class="meta-row"><span class="meta-key">Focus distance</span><span>10.0 units</span></div>
      </div>
    </div>

    <h2>Render stages</h2>
    <p>These placeholders show the progression of techniques added during development. Drop your own renders in the <code>public/</code> folder and update the paths below.</p>
    <div class="stage-grid">
      <div v-for="s in stages" :key="s.title" class="stage-card">
        <div class="stage-img">
          <div class="stage-placeholder">
            <span>{{ s.icon }}</span>
          </div>
        </div>
        <div class="stage-info">
          <div class="stage-title">{{ s.title }}</div>
          <div class="stage-desc">{{ s.desc }}</div>
          <div class="stage-tags">
            <span class="tag" v-for="t in s.tags" :key="t">{{ t }}</span>
          </div>
        </div>
      </div>
    </div>

    <hr class="divider" />

    <h2>How the PPM output works</h2>
    <p>The tracer writes a plain-text PPM file to stdout. Each pixel is three integers (R G B, 0-255), gamma-corrected with a square root to approximate a 2.2 gamma curve.</p>
    <pre><code><span class="comment">// write_color: average N samples, gamma correct, clamp, output as 8-bit int</span>
<span class="keyword">let</span> scale <span class="punct">=</span> <span class="number">1.0</span> <span class="punct">/</span> samples_per_pixel <span class="keyword">as</span> <span class="type">f64</span><span class="punct">;</span>
<span class="keyword">let</span> r <span class="punct">=</span> <span class="punct">(</span>color<span class="punct">.</span><span class="fn-name">get_x</span><span class="punct">() *</span> scale<span class="punct">).</span><span class="fn-name">sqrt</span><span class="punct">();</span>
<span class="keyword">let</span> g <span class="punct">=</span> <span class="punct">(</span>color<span class="punct">.</span><span class="fn-name">get_y</span><span class="punct">() *</span> scale<span class="punct">).</span><span class="fn-name">sqrt</span><span class="punct">();</span>
<span class="keyword">let</span> b <span class="punct">=</span> <span class="punct">(</span>color<span class="punct">.</span><span class="fn-name">get_z</span><span class="punct">() *</span> scale<span class="punct">).</span><span class="fn-name">sqrt</span><span class="punct">();</span>

<span class="fn-name">write!</span><span class="punct">(</span>out<span class="punct">,</span> <span class="string">"{} {} {}\n"</span><span class="punct">,</span>
    <span class="punct">(</span><span class="number">256.0</span> <span class="punct">*</span> r<span class="punct">.</span><span class="fn-name">clamp</span><span class="punct">(</span><span class="number">0.0</span><span class="punct">,</span> <span class="number">0.999</span><span class="punct">))</span> <span class="keyword">as</span> <span class="type">u32</span><span class="punct">,</span>
    <span class="punct">(</span><span class="number">256.0</span> <span class="punct">*</span> g<span class="punct">.</span><span class="fn-name">clamp</span><span class="punct">(</span><span class="number">0.0</span><span class="punct">,</span> <span class="number">0.999</span><span class="punct">))</span> <span class="keyword">as</span> <span class="type">u32</span><span class="punct">,</span>
    <span class="punct">(</span><span class="number">256.0</span> <span class="punct">*</span> b<span class="punct">.</span><span class="fn-name">clamp</span><span class="punct">(</span><span class="number">0.0</span><span class="punct">,</span> <span class="number">0.999</span><span class="punct">))</span> <span class="keyword">as</span> <span class="type">u32</span><span class="punct">,</span>
<span class="punct">);</span></code></pre>

    <div class="nav-footer">
      <RouterLink to="/camera" class="prev-link">← Camera</RouterLink>
      <RouterLink to="/" class="next-link">Back to Overview →</RouterLink>
    </div>
  </div>
</template>

<script setup>
const stages = [
  {
    icon: '◯', title: 'Basic sphere',
    desc: 'Single red sphere with normals shaded as colors. No lighting or materials.',
    tags: ['Sphere', 'Normals'],
  },
  {
    icon: '◑', title: 'Diffuse shading',
    desc: 'Lambertian scattering adds soft shadows and indirect lighting via random bounces.',
    tags: ['Lambertian', 'Anti-aliasing'],
  },
  {
    icon: '◈', title: 'Metal & glass',
    desc: 'Metal reflection and dielectric refraction with Schlick Fresnel.',
    tags: ['Metal', 'Dielectric', 'Schlick'],
  },
  {
    icon: '⊡', title: 'Positionable camera',
    desc: 'Camera placed at (13, 2, 3) looking at origin with 20° telephoto FOV.',
    tags: ['Camera', 'FOV'],
  },
  {
    icon: '⟳', title: 'Depth of field',
    desc: 'Thin-lens aperture model adds natural bokeh. Aperture 0.1, focus 10 units.',
    tags: ['DoF', 'Bokeh', 'Thin lens'],
  },
  {
    icon: '✦', title: 'Final random scene',
    desc: '484 randomly placed spheres with random materials. 500 samples per pixel.',
    tags: ['Final', '500 spp', '50 depth'],
  },
]
</script>

<style scoped>
.render-feature {
  display: grid; grid-template-columns: 1fr 220px; gap: 1.5rem; margin: 1.5rem 0;
  align-items: start;
}
.render-placeholder {
  background: linear-gradient(135deg, #1a1030 0%, #0d1a2a 100%);
  border: 1px solid var(--border); border-radius: 12px;
  display: flex; align-items: center; justify-content: center;
}
.render-placeholder.big { aspect-ratio: 3/2; }
.placeholder-inner { text-align: center; }
.placeholder-icon { font-size: 3rem; color: var(--accent); margin-bottom: 0.5rem; }
.placeholder-title { font-size: 1rem; font-weight: 600; margin-bottom: 0.25rem; }
.placeholder-sub { font-size: 0.78rem; color: var(--muted); }
.render-meta { background: var(--surface); border: 1px solid var(--border); border-radius: 12px; padding: 1rem; }
.meta-row { display: flex; flex-direction: column; padding: 0.5rem 0; border-bottom: 1px solid var(--border); font-size: 0.83rem; }
.meta-row:last-child { border-bottom: none; }
.meta-key { color: var(--muted); font-size: 0.72rem; margin-bottom: 0.1rem; }

.stage-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(260px, 1fr)); gap: 1rem; margin: 1.2rem 0; }
.stage-card { background: var(--surface); border: 1px solid var(--border); border-radius: 12px; overflow: hidden; }
.stage-img { aspect-ratio: 3/2; background: #0a0a10; display: flex; align-items: center; justify-content: center; }
.stage-placeholder { font-size: 2.5rem; color: var(--accent); opacity: 0.4; }
.stage-info { padding: 1rem; }
.stage-title { font-weight: 600; font-size: 0.92rem; margin-bottom: 0.3rem; }
.stage-desc { color: var(--muted); font-size: 0.82rem; margin-bottom: 0.6rem; }
.stage-tags { display: flex; flex-wrap: wrap; gap: 0.3rem; }

.nav-footer { margin-top: 3rem; padding-top: 1.5rem; border-top: 1px solid var(--border); display: flex; gap: 1rem; }
.next-link, .prev-link { display: inline-flex; align-items: center; border-radius: 8px; padding: 0.6rem 1.2rem; font-weight: 600; font-size: 0.9rem; transition: opacity 0.15s; }
.next-link { background: var(--accent); color: #fff; }
.prev-link { background: var(--surface); border: 1px solid var(--border); color: var(--text); }
.next-link:hover, .prev-link:hover { opacity: 0.8; }

@media (max-width: 600px) {
  .render-feature { grid-template-columns: 1fr; }
}
</style>
