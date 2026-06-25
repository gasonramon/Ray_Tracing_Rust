<template>
  <div>
    <h1>Vectors &amp; Math</h1>
    <p>Almost every calculation in ray tracing boils down to 3D vector operations. The <code>Vec3</code> struct is the backbone of the entire project.</p>

    <h2>The Vec3 struct</h2>
    <p>Three <code>f64</code> values stored in a fixed array. Because it's <code>Copy</code>, vectors are passed by value cheaply — no heap allocation.</p>
    <pre><code><span class="keyword">pub struct</span> <span class="type">Vec3</span> <span class="punct">{</span>
    e<span class="punct">: [</span><span class="type">f64</span><span class="punct">;</span> <span class="number">3</span><span class="punct">],</span>
<span class="punct">}</span>

<span class="comment">// Point3 is just a type alias — same memory layout</span>
<span class="keyword">pub type</span> <span class="type">Point3</span> <span class="punct">=</span> <span class="type">Vec3</span><span class="punct">;</span></code></pre>

    <h2>Dot product</h2>
    <p>
      The dot product of two vectors <strong>u</strong> and <strong>v</strong> gives a scalar that encodes
      how much they point in the same direction. It appears everywhere: sphere intersection, shading, Fresnel.
    </p>
    <div class="math-block">
      <strong>u · v</strong> = u<sub>x</sub>v<sub>x</sub> + u<sub>y</sub>v<sub>y</sub> + u<sub>z</sub>v<sub>z</sub>
    </div>
    <pre><code><span class="keyword">pub fn</span> <span class="fn-name">dot</span><span class="punct">(</span>u<span class="punct">:</span> <span class="type">Vec3</span><span class="punct">,</span> v<span class="punct">:</span> <span class="type">Vec3</span><span class="punct">) -></span> <span class="type">f64</span> <span class="punct">{</span>
    u.e[<span class="number">0</span>] <span class="punct">*</span> v.e[<span class="number">0</span>] <span class="punct">+</span> u.e[<span class="number">1</span>] <span class="punct">*</span> v.e[<span class="number">1</span>] <span class="punct">+</span> u.e[<span class="number">2</span>] <span class="punct">*</span> v.e[<span class="number">2</span>]
<span class="punct">}</span></code></pre>
    <p>When the dot product is negative the vectors point away from each other — used to determine front vs back face of a sphere.</p>

    <h2>Vector length</h2>
    <div class="math-block">
      |<strong>v</strong>| = √(v<sub>x</sub>² + v<sub>y</sub>² + v<sub>z</sub>²)
    </div>
    <p>The squared length is preferred when only comparisons are needed — it avoids the expensive square root.</p>
    <pre><code><span class="keyword">pub fn</span> <span class="fn-name">length</span><span class="punct">(&</span><span class="keyword">self</span><span class="punct">) -></span> <span class="type">f64</span> <span class="punct">{</span>
    f64<span class="punct">::</span><span class="fn-name">sqrt</span><span class="punct">(</span><span class="keyword">self</span><span class="punct">.</span><span class="fn-name">get_squared_length</span><span class="punct">())</span>
<span class="punct">}</span></code></pre>

    <h2>Unit vectors</h2>
    <p>A unit vector has length 1. Dividing by magnitude normalizes it. This is essential before using a vector as a direction.</p>
    <div class="math-block">
      <strong>v̂</strong> = <strong>v</strong> / |<strong>v</strong>|
    </div>
    <pre><code><span class="keyword">pub fn</span> <span class="fn-name">unit_vector</span><span class="punct">(</span>v<span class="punct">:</span> <span class="type">Vec3</span><span class="punct">) -></span> <span class="type">Vec3</span> <span class="punct">{</span>
    v <span class="punct">/</span> v<span class="punct">.</span><span class="fn-name">length</span><span class="punct">()</span>
<span class="punct">}</span></code></pre>

    <h2>Reflection</h2>
    <p>
      When a ray hits a mirror-like surface, it bounces symmetrically around the surface normal.
      The reflected direction is computed using the dot product to find how far the incoming vector
      projects onto the normal.
    </p>
    <div class="math-block">
      <strong>r</strong> = <strong>v</strong> − 2(<strong>v</strong> · <strong>n</strong>)<strong>n</strong>
    </div>
    <pre><code><span class="keyword">pub fn</span> <span class="fn-name">reflect</span><span class="punct">(</span>v<span class="punct">:</span> <span class="type">Vec3</span><span class="punct">,</span> n<span class="punct">:</span> <span class="type">Vec3</span><span class="punct">) -></span> <span class="type">Vec3</span> <span class="punct">{</span>
    v <span class="punct">-</span> <span class="number">2.0</span> <span class="punct">*</span> <span class="fn-name">dot</span><span class="punct">(</span>v<span class="punct">,</span> n<span class="punct">)</span> <span class="punct">*</span> n
<span class="punct">}</span></code></pre>

    <h2>Refraction (Snell's Law)</h2>
    <p>
      Light bends when passing between materials of different density (refractive index). Snell's law
      relates the incoming and outgoing angles:
    </p>
    <div class="math-block">
      η sin(θ) = η' sin(θ')
    </div>
    <p>The refracted ray is split into a perpendicular and parallel component relative to the normal:</p>
    <div class="math-block">
      <strong>r</strong><sub>⊥</sub> = (η/η')(<strong>uv</strong> + cos θ · <strong>n</strong>)<br/>
      <strong>r</strong><sub>∥</sub> = −√(1 − |<strong>r</strong><sub>⊥</sub>|²) · <strong>n</strong>
    </div>
    <pre><code><span class="keyword">pub fn</span> <span class="fn-name">refract</span><span class="punct">(</span>uv<span class="punct">:</span> <span class="type">Vec3</span><span class="punct">,</span> n<span class="punct">:</span> <span class="type">Vec3</span><span class="punct">,</span> etai_over_etat<span class="punct">:</span> <span class="type">f64</span><span class="punct">) -></span> <span class="type">Vec3</span> <span class="punct">{</span>
    <span class="keyword">let</span> cos_theta <span class="punct">=</span> f64<span class="punct">::</span><span class="fn-name">min</span><span class="punct">(</span><span class="fn-name">dot</span><span class="punct">(-</span>uv<span class="punct">,</span> n<span class="punct">),</span> <span class="number">1.0</span><span class="punct">);</span>
    <span class="keyword">let</span> r_out_perp <span class="punct">=</span> etai_over_etat <span class="punct">*</span> <span class="punct">(</span>uv <span class="punct">+</span> cos_theta <span class="punct">*</span> n<span class="punct">);</span>
    <span class="keyword">let</span> r_out_parallel <span class="punct">=</span> <span class="punct">-</span>f64<span class="punct">::</span><span class="fn-name">sqrt</span><span class="punct">(</span>f64<span class="punct">::</span><span class="fn-name">abs</span><span class="punct">(</span><span class="number">1.0</span> <span class="punct">-</span> r_out_perp<span class="punct">.</span><span class="fn-name">get_squared_length</span><span class="punct">()))</span> <span class="punct">*</span> n<span class="punct">;</span>
    r_out_parallel <span class="punct">+</span> r_out_perp
<span class="punct">}</span></code></pre>

    <div class="card">
      <h3>Cross product</h3>
      <p style="margin:0">
        Used in the camera to build an orthonormal basis (right, up, forward vectors) from <code>lookfrom</code>,
        <code>lookat</code> and the world-up vector. The cross product produces a vector perpendicular to both inputs.
      </p>
      <div class="math-block" style="margin-top:1rem">
        (<strong>u</strong> × <strong>v</strong>)<sub>x</sub> = u<sub>y</sub>v<sub>z</sub> − u<sub>z</sub>v<sub>y</sub><br/>
        (<strong>u</strong> × <strong>v</strong>)<sub>y</sub> = u<sub>z</sub>v<sub>x</sub> − u<sub>x</sub>v<sub>z</sub><br/>
        (<strong>u</strong> × <strong>v</strong>)<sub>z</sub> = u<sub>x</sub>v<sub>y</sub> − u<sub>y</sub>v<sub>x</sub>
      </div>
    </div>

    <div class="nav-footer">
      <RouterLink to="/rays" class="next-link">Next: Rays &amp; Spheres →</RouterLink>
    </div>
  </div>
</template>

<style scoped>
.nav-footer { margin-top: 3rem; padding-top: 1.5rem; border-top: 1px solid var(--border); }
.next-link {
  display: inline-flex; align-items: center; gap: 0.4rem;
  background: var(--accent); color: #fff; border-radius: 8px;
  padding: 0.6rem 1.2rem; font-weight: 600; font-size: 0.9rem;
  transition: opacity 0.15s;
}
.next-link:hover { opacity: 0.85; }
</style>
