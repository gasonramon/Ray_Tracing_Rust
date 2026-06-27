<template>
  <div>
    <h1>Materials</h1>
    <p>
      Each sphere in the scene carries a material that decides what happens when a ray hits it.
      The <code>Material</code> trait has a single method, <code>scatter</code>, which takes the incoming
      ray and hit record. It either produces a new scattered ray with an attenuation color, or returns
      false if the ray was absorbed.
    </p>

    <pre><code><span class="keyword">pub trait</span> <span class="type">Material</span> <span class="punct">{</span>
    <span class="keyword">fn</span> <span class="fn-name">scatter</span><span class="punct">(</span>
        <span class="punct">&</span><span class="keyword">self</span><span class="punct">,</span>
        r_in<span class="punct">: &</span><span class="type">Ray</span><span class="punct">,</span>
        rec<span class="punct">: &</span><span class="type">Hitrecord</span><span class="punct">,</span>
        attenuation<span class="punct">: &mut</span> <span class="type">Color</span><span class="punct">,</span>
        scattered<span class="punct">:   &mut</span> <span class="type">Ray</span><span class="punct">,</span>
    <span class="punct">) -></span> <span class="type">bool</span><span class="punct">;</span>
<span class="punct">}</span></code></pre>

    <div class="mat-grid">
      <div v-for="m in materials" :key="m.name" class="mat-card" :style="{ borderColor: m.color }">
        <div class="mat-header" :style="{ background: m.color + '22', borderColor: m.color }">
          <span class="mat-dot" :style="{ background: m.color }"></span>
          <span class="mat-name">{{ m.name }}</span>
          <span class="mat-tag">{{ m.tag }}</span>
        </div>
        <p>{{ m.desc }}</p>
      </div>
    </div>

    <h2>Lambertian (diffuse)</h2>
    <p>
      Diffuse surfaces scatter light randomly in a hemisphere around the surface normal.
      The Lambertian model adds a random unit vector to the normal for the scatter direction.
      This creates the characteristic soft, matte look.
    </p>
    <pre><code><span class="keyword">fn</span> <span class="fn-name">scatter</span><span class="punct">(&</span><span class="keyword">self</span><span class="punct">, ...) -></span> <span class="type">bool</span> <span class="punct">{</span>
    <span class="keyword">let mut</span> scatter_direction <span class="punct">=</span> rec<span class="punct">.</span>normal <span class="punct">+</span> <span class="fn-name">random_unit_vector</span><span class="punct">();</span>

    <span class="comment">// guard: if random vector cancels the normal exactly</span>
    <span class="keyword">if</span> scatter_direction<span class="punct">.</span><span class="fn-name">near_zero</span><span class="punct">() {</span>
        scatter_direction <span class="punct">=</span> rec<span class="punct">.</span>normal<span class="punct">;</span>
    <span class="punct">}</span>

    <span class="punct">*</span>attenuation <span class="punct">=</span> <span class="keyword">self</span><span class="punct">.</span>albedo<span class="punct">;</span>
    <span class="punct">*</span>scattered   <span class="punct">=</span> <span class="type">Ray</span><span class="punct">::</span><span class="fn-name">new</span><span class="punct">(</span>rec<span class="punct">.</span>posisition<span class="punct">,</span> scatter_direction<span class="punct">);</span>
    <span class="keyword">true</span>
<span class="punct">}</span></code></pre>

    <h2>Metal (specular reflection)</h2>
    <p>
      Metal reflects the ray about the surface normal. A <code>fuzz</code> factor adds a small random
      perturbation to the reflected direction, so a higher value gives a blurrier mirror.
    </p>
    <div class="math-block">
      <strong>reflected</strong> = <strong>v</strong> − 2(<strong>v</strong> · <strong>n</strong>)<strong>n</strong>
    </div>
    <pre><code><span class="keyword">fn</span> <span class="fn-name">scatter</span><span class="punct">(&</span><span class="keyword">self</span><span class="punct">, ...) -></span> <span class="type">bool</span> <span class="punct">{</span>
    <span class="keyword">let</span> reflected <span class="punct">=</span> <span class="fn-name">reflect</span><span class="punct">(</span><span class="fn-name">unit_vector</span><span class="punct">(</span>r_in<span class="punct">.</span><span class="fn-name">get_direction</span><span class="punct">()),</span> rec<span class="punct">.</span>normal<span class="punct">);</span>
    <span class="punct">*</span>attenuation <span class="punct">=</span> <span class="keyword">self</span><span class="punct">.</span>albedo<span class="punct">;</span>
    <span class="punct">*</span>scattered   <span class="punct">=</span> <span class="type">Ray</span><span class="punct">::</span><span class="fn-name">new</span><span class="punct">(</span>
        rec<span class="punct">.</span>posisition<span class="punct">,</span>
        reflected <span class="punct">+</span> <span class="keyword">self</span><span class="punct">.</span>fuzz <span class="punct">*</span> <span class="fn-name">random_unit_in_sphere</span><span class="punct">(),</span>
    <span class="punct">);</span>
    <span class="comment">// only scatter if the reflected ray goes outward</span>
    <span class="fn-name">dot</span><span class="punct">(</span>scattered<span class="punct">.</span><span class="fn-name">get_direction</span><span class="punct">(),</span> rec<span class="punct">.</span>normal<span class="punct">) &gt;</span> <span class="number">0.0</span>
<span class="punct">}</span></code></pre>

    <h2>Dielectric (glass)</h2>
    <p>
      Glass both refracts and reflects depending on the angle of incidence. Refraction follows
      Snell's law. When <code>η·sin(θ) &gt; 1</code>, total internal reflection kicks in and no
      refracted ray is possible. Even below that threshold, reflection becomes more likely at
      shallow angles, which is approximated using Schlick's formula.
    </p>

    <h3>Schlick approximation</h3>
    <p>
      The full Fresnel equations are expensive to evaluate. Schlick's approximation gives a good
      polynomial fit for reflectance as a function of angle and refractive index:
    </p>
    <div class="math-block">
      R(θ) = R₀ + (1 − R₀)(1 − cosθ)⁵<br/>
      R₀ = ((n₁ − n₂) / (n₁ + n₂))²
    </div>
    <pre><code><span class="keyword">fn</span> <span class="fn-name">reflectance</span><span class="punct">(</span>cosine<span class="punct">:</span> <span class="type">f64</span><span class="punct">,</span> ref_idx<span class="punct">:</span> <span class="type">f64</span><span class="punct">) -></span> <span class="type">f64</span> <span class="punct">{</span>
    <span class="keyword">let mut</span> r0 <span class="punct">= (</span><span class="number">1.0</span> <span class="punct">-</span> ref_idx<span class="punct">) / (</span><span class="number">1.0</span> <span class="punct">+</span> ref_idx<span class="punct">);</span>
    r0 <span class="punct">=</span> r0 <span class="punct">*</span> r0<span class="punct">;</span>
    r0 <span class="punct">+ (</span><span class="number">1.0</span> <span class="punct">-</span> r0<span class="punct">) *</span> f64<span class="punct">::</span><span class="fn-name">powf</span><span class="punct">(</span><span class="number">1.0</span> <span class="punct">-</span> cosine<span class="punct">,</span> <span class="number">5.0</span><span class="punct">)</span>
<span class="punct">}</span></code></pre>
    <pre><code><span class="comment">// in scatter():</span>
<span class="keyword">let</span> cannot_refract <span class="punct">=</span> refraction_ratio <span class="punct">*</span> sin_theta <span class="punct">&gt;</span> <span class="number">1.0</span><span class="punct">;</span>
<span class="keyword">let</span> direction <span class="punct">=</span> <span class="keyword">if</span> cannot_refract
    <span class="punct">||</span> <span class="type">Self</span><span class="punct">::</span><span class="fn-name">reflectance</span><span class="punct">(</span>cos_theta<span class="punct">,</span> refraction_ratio<span class="punct">) &gt;</span> <span class="fn-name">random_double</span><span class="punct">()</span>
<span class="punct">{</span>
    <span class="fn-name">reflect</span><span class="punct">(</span>unit_direction<span class="punct">,</span> rec<span class="punct">.</span>normal<span class="punct">)</span>
<span class="punct">}</span> <span class="keyword">else</span> <span class="punct">{</span>
    <span class="fn-name">refract</span><span class="punct">(</span>unit_direction<span class="punct">,</span> rec<span class="punct">.</span>normal<span class="punct">,</span> refraction_ratio<span class="punct">)</span>
<span class="punct">};</span></code></pre>

    <div class="card" style="margin-top:2rem">
      <h3>Scene distribution</h3>
      <p style="margin:0">
        In the random scene, spheres are assigned materials based on a random roll:
      </p>
      <div class="dist-row" v-for="d in dist" :key="d.label">
        <div class="dist-bar" :style="{ width: d.pct, background: d.col }"></div>
        <span class="dist-label">{{ d.label }}</span>
        <span class="dist-pct">{{ d.pct }}</span>
      </div>
    </div>

    <div class="nav-footer">
      <RouterLink to="/rays" class="prev-link">← Rays</RouterLink>
      <RouterLink to="/camera" class="next-link">Next: Camera →</RouterLink>
    </div>
  </div>
</template>

<script setup>
const materials = [
  { name: 'Lambertian', tag: 'Diffuse',   color: '#a6e3a1', desc: 'Scatters incoming rays randomly around the surface normal. Absorbs some light based on its albedo color. Gives a soft, matte appearance.' },
  { name: 'Metal',      tag: 'Reflective', color: '#89b4fa', desc: 'Reflects rays about the surface normal. A fuzz parameter roughens the reflection for a brushed-metal look. Only scatters if the reflected ray faces outward.' },
  { name: 'Dielectric', tag: 'Glass',      color: '#89dceb', desc: 'Bends light via Snell\'s law and reflects at steep angles (Schlick approximation). Refractive index of 1.5.' },
]
const dist = [
  { label: 'Lambertian (diffuse)', pct: '80%', col: '#a6e3a1' },
  { label: 'Metal (reflective)',   pct: '15%', col: '#89b4fa' },
  { label: 'Dielectric (glass)',   pct: '5%',  col: '#89dceb' },
]
</script>

<style scoped>
.mat-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); gap: 1rem; margin: 1.5rem 0; }
.mat-card { background: var(--surface); border: 1px solid; border-radius: 12px; overflow: hidden; }
.mat-header { display: flex; align-items: center; gap: 0.6rem; padding: 0.8rem 1rem; border-bottom-width: 1px; border-bottom-style: solid; }
.mat-dot { width: 10px; height: 10px; border-radius: 50%; flex-shrink: 0; }
.mat-name { font-weight: 600; font-size: 0.95rem; }
.mat-tag { margin-left: auto; font-size: 0.72rem; color: var(--muted); }
.mat-card p { padding: 1rem; font-size: 0.85rem; color: var(--muted); margin: 0; }

.dist-row { display: flex; align-items: center; gap: 0.8rem; margin-top: 0.8rem; }
.dist-bar { height: 8px; border-radius: 4px; flex-shrink: 0; }
.dist-label { font-size: 0.85rem; flex: 1; }
.dist-pct { font-family: 'JetBrains Mono', monospace; font-size: 0.82rem; color: var(--muted); }

.nav-footer { margin-top: 3rem; padding-top: 1.5rem; border-top: 1px solid var(--border); display: flex; gap: 1rem; }
.next-link, .prev-link { display: inline-flex; align-items: center; border-radius: 8px; padding: 0.6rem 1.2rem; font-weight: 600; font-size: 0.9rem; transition: opacity 0.15s; }
.next-link { background: var(--accent); color: #fff; }
.prev-link { background: var(--surface); border: 1px solid var(--border); color: var(--text); }
.next-link:hover, .prev-link:hover { opacity: 0.8; }
</style>
