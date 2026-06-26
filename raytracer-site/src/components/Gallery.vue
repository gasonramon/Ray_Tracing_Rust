<template>
  <div>
    <h1>Gallery</h1>
    <p>
      The final scene — rendered entirely in software with no GPU or external rendering library.
      500 samples per pixel, 50 bounce depth, ~484 randomly placed spheres.
    </p>

    <div class="final-render">
      <img src="/final-scene.png" alt="Final ray traced scene" class="render-img" />
      <div class="render-caption">
        <div class="caption-grid">
          <div class="meta-item"><span class="meta-k">Resolution</span><span class="meta-v">1200 × 800</span></div>
          <div class="meta-item"><span class="meta-k">Samples / pixel</span><span class="meta-v">500</span></div>
          <div class="meta-item"><span class="meta-k">Max depth</span><span class="meta-v">50 bounces</span></div>
          <div class="meta-item"><span class="meta-k">Objects</span><span class="meta-v">~484 spheres</span></div>
          <div class="meta-item"><span class="meta-k">Camera FOV</span><span class="meta-v">20° telephoto</span></div>
          <div class="meta-item"><span class="meta-k">Aperture</span><span class="meta-v">0.1 (DoF)</span></div>
          <div class="meta-item"><span class="meta-k">Focus distance</span><span class="meta-v">10.0 units</span></div>
          <div class="meta-item"><span class="meta-k">Output format</span><span class="meta-v">PPM → JPG</span></div>
        </div>
      </div>
    </div>

    <h2>How the PPM output works</h2>
    <p>
      The tracer writes a plain-text PPM file to stdout. Each pixel is three integers (R G B, 0–255),
      gamma-corrected with a square root to approximate a 2.2 gamma curve.
    </p>
    <pre><code><span class="comment">// average N samples, gamma correct, clamp, write as 8-bit integers</span>
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

<style scoped>
.final-render {
  margin: 1.5rem 0 2.5rem;
  border: 1px solid var(--border);
  border-radius: 12px;
  overflow: hidden;
  background: #000;
}

.render-img {
  width: 100%;
  display: block;
  border-radius: 0;
}

.render-caption {
  padding: 1.2rem 1.5rem;
  background: var(--surface);
  border-top: 1px solid var(--border);
}

.caption-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
  gap: 0.75rem 1.5rem;
}

.meta-item {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}

.meta-k {
  font-size: 0.7rem;
  color: var(--muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.meta-v {
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.85rem;
  color: var(--accent2);
}

.nav-footer {
  margin-top: 3rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--border);
  display: flex;
  gap: 1rem;
}

.next-link, .prev-link {
  display: inline-flex;
  align-items: center;
  border-radius: 8px;
  padding: 0.6rem 1.2rem;
  font-weight: 600;
  font-size: 0.9rem;
  transition: opacity 0.15s;
}

.next-link { background: var(--accent); color: #fff; }
.prev-link { background: var(--surface); border: 1px solid var(--border); color: var(--text); }
.next-link:hover, .prev-link:hover { opacity: 0.8; }
</style>
